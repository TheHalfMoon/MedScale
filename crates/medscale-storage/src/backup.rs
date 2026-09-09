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
        if let Some(hex_d) = obj.content_digest_hex.as_ref() {
            if seen.insert(hex_d.clone()) {
                let digest = parse_hex(hex_d);
                let bytes = vault.blobs.get_blob(&digest).map_err(|e| e.to_string())?;
                fs::write(dest.join("blobs").join(hex_d), &bytes).map_err(|e| e.to_string())?;
                blob_entries.push(BlobRef {
                    digest,
                    byte_length: bytes.len() as u64,
                });
            }
        }
    }

    let manifest = BackupManifest {
        schema_version: 2,
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
    if snapshot_value
        .get("schema_version")
        .and_then(|v| v.as_u64())
        == Some(2)
    {
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
