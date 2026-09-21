//! Backup / restore for synthetic unencrypted vaults (schema v1 sources-only + v2 objects).

use std::fs;
use std::path::Path;

use medscale_contracts::ingest::{BackupManifest, BlobRef};
use medscale_contracts::objects::DigestSha256;

use crate::blob::FsBlobStore;
use crate::claim::assert_claim_path;
use crate::sqlite_meta::{AuthorityObjectRow, SourceMeta, SqliteMetaStore};
use crate::vault::SyntheticVault;

/// Create a backup artifact directory.
pub fn backup_vault(vault: &SyntheticVault, dest: &Path) -> Result<BackupManifest, String> {
    let dest = assert_claim_path(dest).map_err(|e| e.to_string())?;
    fs::create_dir_all(dest.join("blobs")).map_err(|e| e.to_string())?;
    let snapshot = vault.meta.snapshot_bytes().map_err(|e| e.to_string())?;
    let snapshot_digest = DigestSha256::of(&snapshot);
    fs::write(dest.join("metadata.snapshot"), &snapshot).map_err(|e| e.to_string())?;

    let mut blob_entries = Vec::new();
    let mut seen = std::collections::HashSet::<String>::new();
    for source in vault.meta.list_sources().map_err(|e| e.to_string())? {
        let bytes = vault
            .blobs
            .get_blob(&source.digest)
            .map_err(|e| e.to_string())?;
        let name = hex(&source.digest);
        if seen.insert(name.clone()) {
            fs::write(dest.join("blobs").join(&name), &bytes).map_err(|e| e.to_string())?;
            blob_entries.push(BlobRef {
                digest: source.digest,
                byte_length: source.byte_length,
            });
        }
    }
    for obj in vault
        .meta
        .list_authority_objects()
        .map_err(|e| e.to_string())?
    {
        if let Some(hex_d) = obj.content_digest_hex.as_ref()
            && seen.insert(hex_d.clone())
        {
            let digest = parse_hex(hex_d);
            let bytes = vault.blobs.get_blob(&digest).map_err(|e| e.to_string())?;
            fs::write(dest.join("blobs").join(hex_d), &bytes).map_err(|e| e.to_string())?;
            blob_entries.push(BlobRef {
                digest,
                byte_length: bytes.len() as u64,
            });
        }
    }

    let manifest = BackupManifest {
        schema_version: 4,
        vault_id: vault.vault_id.clone(),
        created_at: "1970-01-01T00:00:00Z".to_owned(),
        metadata_snapshot_digest: snapshot_digest,
        blob_entries,
        claim_scope_note: "synthetic-unencrypted-h0a".to_owned(),
    };
    let manifest_bytes = serde_json::to_vec_pretty(&manifest).map_err(|e| e.to_string())?;
    fs::write(dest.join("manifest.json"), manifest_bytes).map_err(|e| e.to_string())?;
    Ok(manifest)
}

/// Restore a backup into an empty destination vault root.
pub fn restore_vault(src: &Path, dest_vault_root: &Path) -> Result<(u64, u64), String> {
    let src = assert_claim_path(src).map_err(|e| e.to_string())?;
    let dest_vault_root = assert_claim_path(dest_vault_root).map_err(|e| e.to_string())?;
    if dest_vault_root.exists()
        && dest_vault_root
            .read_dir()
            .map(|mut d| d.next().is_some())
            .unwrap_or(false)
    {
        return Err("restore destination not empty".to_owned());
    }
    let manifest_bytes = fs::read(src.join("manifest.json")).map_err(|e| e.to_string())?;
    let manifest: BackupManifest =
        serde_json::from_slice(&manifest_bytes).map_err(|e| e.to_string())?;
    let snapshot = fs::read(src.join("metadata.snapshot")).map_err(|e| e.to_string())?;
    if DigestSha256::of(&snapshot) != manifest.metadata_snapshot_digest {
        return Err("tampered metadata snapshot".to_owned());
    }

    let vault =
        SyntheticVault::open(&manifest.vault_id, &dest_vault_root).map_err(|e| e.to_string())?;

    let mut sources = 0_u64;
    let mut blobs = 0_u64;
    for entry in &manifest.blob_entries {
        let name = hex(&entry.digest);
        let bytes = fs::read(src.join("blobs").join(&name)).map_err(|e| e.to_string())?;
        if DigestSha256::of(&bytes) != entry.digest {
            return Err("tampered blob".to_owned());
        }
        vault.blobs.put_blob(&bytes).map_err(|e| e.to_string())?;
        blobs += 1;
    }

    let snapshot_value: serde_json::Value =
        serde_json::from_slice(&snapshot).map_err(|e| e.to_string())?;
    let snapshot_schema = snapshot_value
        .get("schema_version")
        .and_then(|v| v.as_u64());
    if snapshot_schema == Some(4) {
        restore_v4(&vault, &snapshot_value, &mut sources)?;
    } else if snapshot_schema == Some(3) {
        restore_v3(&vault, &snapshot_value, &mut sources)?;
    } else if snapshot_schema == Some(2) {
        restore_v2(&vault, &snapshot_value, &mut sources)?;
    } else {
        let entries: Vec<serde_json::Value> =
            serde_json::from_slice(&snapshot).map_err(|e| e.to_string())?;
        for value in entries {
            insert_source_from_json(&vault, &value)?;
            sources += 1;
        }
    }
    let _ = FsBlobStore::open;
    let _ = SqliteMetaStore::open;
    Ok((sources, blobs))
}

fn restore_v2(
    vault: &SyntheticVault,
    snapshot: &serde_json::Value,
    sources: &mut u64,
) -> Result<(), String> {
    if let Some(entries) = snapshot.get("sources").and_then(|v| v.as_array()) {
        for value in entries {
            insert_source_from_json(vault, value)?;
            *sources += 1;
        }
    }
    let mut rows = Vec::new();
    if let Some(objects) = snapshot.get("objects").and_then(|v| v.as_array()) {
        for value in objects {
            rows.push(AuthorityObjectRow {
                object_id: value["object_id"].as_str().unwrap_or_default().to_owned(),
                object_class: value["object_class"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                realm_id: value["realm_id"].as_str().unwrap_or_default().to_owned(),
                authority_scope_id: value["authority_scope_id"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                body_json: value["body_json"].as_str().unwrap_or_default().to_owned(),
                content_digest_hex: value
                    .get("content_digest_hex")
                    .and_then(|v| v.as_str())
                    .map(str::to_owned),
                updated_seq: value["updated_seq"].as_u64().unwrap_or(0),
            });
        }
    }
    let next_seq = snapshot
        .get("next_seq")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    vault
        .meta
        .replace_authority_snapshot(&rows, next_seq)
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn restore_v3(
    vault: &SyntheticVault,
    snapshot: &serde_json::Value,
    sources: &mut u64,
) -> Result<(), String> {
    restore_v2(vault, snapshot, sources)?;
    // Spec 074 rows replay exactly (ids/revisions preserved); every row is
    // re-validated so a tampered snapshot fails closed instead of persisting.
    if let Some(projects) = snapshot.get("projects").and_then(|v| v.as_array()) {
        for value in projects {
            let project: medscale_contracts::project_graph::Project =
                serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            medscale_contracts::project_graph::validate_metadata_fields(
                &project.name,
                project.description.as_deref(),
            )
            .map_err(|e| e.to_string())?;
            if project.revision < 1 {
                return Err("tampered project revision".to_owned());
            }
            vault
                .meta
                .restore_project_row(&project)
                .map_err(|e| e.to_string())?;
        }
    }
    if let Some(experiments) = snapshot.get("experiments").and_then(|v| v.as_array()) {
        for value in experiments {
            let experiment: medscale_contracts::project_graph::Experiment =
                serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            medscale_contracts::project_graph::validate_metadata_fields(
                &experiment.name,
                experiment.description.as_deref(),
            )
            .map_err(|e| e.to_string())?;
            if experiment.revision < 1 {
                return Err("tampered experiment revision".to_owned());
            }
            vault
                .meta
                .restore_experiment_row(&experiment)
                .map_err(|e| e.to_string())?;
        }
    }
    if let Some(refs) = snapshot.get("refs").and_then(|v| v.as_array()) {
        for value in refs {
            let reference: medscale_contracts::project_graph::ProjectArtifactRef =
                serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            reference.artifact.validate().map_err(|e| e.to_string())?;
            if reference.revision < 1 {
                return Err("tampered ref revision".to_owned());
            }
            vault
                .meta
                .restore_ref_row(&reference)
                .map_err(|e| e.to_string())?;
        }
    }
    if let Some(edges) = snapshot.get("edges").and_then(|v| v.as_array()) {
        for value in edges {
            let edge: medscale_contracts::project_graph::ProjectGraphEdge =
                serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            edge.subject.validate().map_err(|e| e.to_string())?;
            edge.object.validate().map_err(|e| e.to_string())?;
            if edge.revision < 1 {
                return Err("tampered edge revision".to_owned());
            }
            vault
                .meta
                .restore_edge_row(&edge)
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn restore_v4(
    vault: &SyntheticVault,
    snapshot: &serde_json::Value,
    sources: &mut u64,
) -> Result<(), String> {
    restore_v3(vault, snapshot, sources)?;
    // Spec 075 rows replay exactly (ids/revisions/digests preserved); every
    // row is re-validated so a tampered snapshot fails closed instead of
    // persisting. Snapshots restore before releases so release scope
    // inheritance resolves.
    if let Some(entries) = snapshot.get("data_sources").and_then(|v| v.as_array()) {
        for value in entries {
            let manifest: medscale_contracts::data_sources::DataSourceManifest =
                serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            medscale_contracts::data_sources::validate_display_name(&manifest.display_name)
                .map_err(|e| e.to_string())?;
            manifest.locator.validate().map_err(|e| e.to_string())?;
            if matches!(
                manifest.locator,
                medscale_contracts::data_sources::SourceLocator::Database { .. }
            ) && manifest.credential_ref.is_some()
            {
                return Err(
                    "tampered source: database credential references are not admitted in 075"
                        .to_owned(),
                );
            }
            if manifest.capabilities.is_empty() {
                return Err("tampered source capabilities".to_owned());
            }
            if manifest.revision < 1 {
                return Err("tampered source revision".to_owned());
            }
            vault
                .meta
                .restore_data_source_row(&manifest)
                .map_err(|e| e.to_string())?;
        }
    }
    if let Some(entries) = snapshot.get("data_snapshots").and_then(|v| v.as_array()) {
        let parts = snapshot
            .get("data_snapshot_parts")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        for value in entries {
            let record: crate::data_sources::SnapshotRecord =
                serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            record.snapshot.validate().map_err(|e| e.to_string())?;
            record.schema.validate().map_err(|e| e.to_string())?;
            let id = record.snapshot.header.id.as_str().to_owned();
            let mut owned_parts = Vec::new();
            for part_value in &parts {
                let part: medscale_contracts::data_sources::SnapshotPart =
                    serde_json::from_value(part_value.clone()).map_err(|e| e.to_string())?;
                part.validate().map_err(|e| e.to_string())?;
                if part.snapshot_id.as_str() == id {
                    owned_parts.push(part);
                }
            }
            vault
                .meta
                .restore_snapshot_full(&record, &owned_parts)
                .map_err(|e| e.to_string())?;
        }
    }
    if let Some(entries) = snapshot.get("data_receipts").and_then(|v| v.as_array()) {
        for value in entries {
            let (kind, snap, receipt): (String, String, serde_json::Value) =
                serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            vault
                .meta
                .restore_receipt_row(&kind, &snap, &receipt)
                .map_err(|e| e.to_string())?;
        }
    }
    if let Some(entries) = snapshot.get("data_saved_views").and_then(|v| v.as_array()) {
        for value in entries {
            let (view, project): (
                medscale_contracts::data_sources::SavedDataView,
                medscale_contracts::objects::OpaqueId,
            ) = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            if view.revision < 1 {
                return Err("tampered view revision".to_owned());
            }
            vault
                .meta
                .restore_saved_view_row(&view, &project)
                .map_err(|e| e.to_string())?;
        }
    }
    if let Some(entries) = snapshot
        .get("data_transformations")
        .and_then(|v| v.as_array())
    {
        for value in entries {
            let record: crate::data_sources::TransformationRecord =
                serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            record.receipt.validate().map_err(|e| e.to_string())?;
            vault
                .meta
                .restore_transformation_row(&record)
                .map_err(|e| e.to_string())?;
        }
    }
    if let Some(entries) = snapshot.get("dataset_releases").and_then(|v| v.as_array()) {
        for value in entries {
            let release: medscale_contracts::data_sources::ReleaseManifest =
                serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            release.card.validate().map_err(|e| e.to_string())?;
            vault
                .meta
                .restore_dataset_release_row(&release)
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn insert_source_from_json(
    vault: &SyntheticVault,
    value: &serde_json::Value,
) -> Result<(), String> {
    let meta = SourceMeta {
        source_id: medscale_contracts::objects::OpaqueId::new(
            value["source_id"].as_str().unwrap_or_default(),
        ),
        realm_id: medscale_contracts::objects::RealmId::new(
            value["realm_id"].as_str().unwrap_or_default(),
        ),
        authority_scope_id: medscale_contracts::objects::AuthorityScopeId::new(
            value["authority_scope_id"].as_str().unwrap_or_default(),
        ),
        digest: parse_hex(value["digest_hex"].as_str().unwrap_or_default()),
        byte_length: value["byte_length"].as_u64().unwrap_or(0),
        media_type: value["media_type"].as_str().unwrap_or_default().to_owned(),
        visible: value["visible"].as_bool().unwrap_or(false),
        resource_type: value["resource_type"]
            .as_str()
            .unwrap_or_default()
            .to_owned(),
    };
    vault
        .blobs
        .verify(&meta.digest, meta.byte_length)
        .map_err(|e| e.to_string())?;
    vault.meta.insert_source(&meta).map_err(|e| e.to_string())?;
    Ok(())
}

fn hex(digest: &DigestSha256) -> String {
    digest
        .as_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn parse_hex(name: &str) -> DigestSha256 {
    let mut bytes = [0_u8; 32];
    for (i, chunk) in name.as_bytes().chunks(2).enumerate() {
        if i >= 32 {
            break;
        }
        let s = std::str::from_utf8(chunk).unwrap_or("00");
        bytes[i] = u8::from_str_radix(s, 16).unwrap_or(0);
    }
    DigestSha256::from_bytes(bytes)
}
