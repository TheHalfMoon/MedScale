//! Synthetic FHIR ingest + vault operations for Spec 003.

use medscale_contracts::envelopes::{AuthorityError, ResponseBody};
use medscale_contracts::ingest::{BlobRef, IngestOutcome, IngestReceipt};
use medscale_contracts::objects::{
    ActionAuditKind, ActionAuditRecord, DigestSha256, EvaluationRecord, IdentityAssertion,
    ObjectHeader, OpaqueId, SourceRecord,
};
use medscale_contracts::{AUTHORITY_SCHEMA_VERSION, FHIR_R4_VERSION};
use medscale_fhir::{LexicalError, extract_patient_identifiers, gate_fhir_json};
use medscale_storage::{SourceMeta, SyntheticVault, backup_vault, restore_vault, run_gc};
use serde_json::json;

use super::store::{InMemoryAuthorityStore, StoredObject};

pub fn open_vault(
    vault_slot: &mut Option<SyntheticVault>,
    vault_id: &str,
    vault_root: &str,
) -> Result<ResponseBody, AuthorityError> {
    let vault = SyntheticVault::open(vault_id, std::path::Path::new(vault_root)).map_err(|e| {
        let msg = e.to_string();
        if msg.contains("sync") || msg.contains("refused") {
            AuthorityError::PathOutsideClaim
        } else {
            AuthorityError::InvalidArgument { message: msg }
        }
    })?;
    *vault_slot = Some(vault);
    Ok(ResponseBody::VaultOpened {
        vault_root: vault_root.to_owned(),
    })
}

pub fn close_vault(vault_slot: &mut Option<SyntheticVault>) -> ResponseBody {
    *vault_slot = None;
    ResponseBody::VaultClosed
}

#[allow(clippy::too_many_arguments)]
pub fn ingest_fhir(
    vault: &SyntheticVault,
    memory: &mut InMemoryAuthorityStore,
    realm_id: medscale_contracts::objects::RealmId,
    authority_scope_id: medscale_contracts::objects::AuthorityScopeId,
    media_type: String,
    bytes: Vec<u8>,
    fhir_version_hint: Option<String>,
    attach_validator_fixture_id: Option<String>,
) -> Result<ResponseBody, AuthorityError> {
    let report =
        gate_fhir_json(&media_type, &bytes, fhir_version_hint.as_deref()).map_err(lexical_err)?;

    let digest = DigestSha256::of(&bytes);
    if let Some(existing) = vault
        .meta
        .find_by_digest(&authority_scope_id, &digest)
        .map_err(|e| AuthorityError::InvalidArgument {
            message: e.to_string(),
        })?
    {
        let receipt = IngestReceipt {
            receipt_id: memory.alloc_id("receipt"),
            outcome: IngestOutcome::Duplicate,
            source_id: Some(existing.source_id),
            content_digest: Some(digest.clone()),
            byte_length: Some(existing.byte_length),
            blob_ref: Some(BlobRef {
                digest,
                byte_length: existing.byte_length,
            }),
            evaluation_refs: vec![],
            identity_refs: vec![],
            error: None,
        };
        return Ok(ResponseBody::Ingested { receipt });
    }

    let blob = vault
        .blobs
        .put_blob(&bytes)
        .map_err(|e| AuthorityError::InvalidArgument {
            message: e.to_string(),
        })?;
    vault
        .blobs
        .verify(&blob.digest, blob.byte_length)
        .map_err(|_| AuthorityError::DigestMismatch)?;

    let source_id = memory.alloc_id("src");
    let source = SourceRecord {
        header: ObjectHeader {
            id: source_id.clone(),
            schema_version: AUTHORITY_SCHEMA_VERSION,
            realm_id: realm_id.clone(),
            authority_scope_id: authority_scope_id.clone(),
        },
        bytes: bytes.clone(),
        content_digest: digest.clone(),
        media_type: media_type.clone(),
        acquired_at: None,
        provenance_note: Some(format!(
            "fhir {} {}",
            report.fhir_version, report.resource_type
        )),
    };
    memory.insert(StoredObject::Source(source));

    vault
        .meta
        .insert_source(&SourceMeta {
            source_id: source_id.clone(),
            realm_id: realm_id.clone(),
            authority_scope_id: authority_scope_id.clone(),
            digest: digest.clone(),
            byte_length: blob.byte_length,
            media_type,
            visible: true,
            resource_type: report.resource_type.clone(),
        })
        .map_err(|e| AuthorityError::InvalidArgument {
            message: e.to_string(),
        })?;

    let mut identity_refs = Vec::new();
    for (system, value) in extract_patient_identifiers(&bytes) {
        let id = memory.alloc_id("ident");
        let assertion = IdentityAssertion {
            header: ObjectHeader {
                id: id.clone(),
                schema_version: AUTHORITY_SCHEMA_VERSION,
                realm_id: realm_id.clone(),
                authority_scope_id: authority_scope_id.clone(),
            },
            subject_id: OpaqueId::new(format!("subject:{system}:{value}")),
            identifier_system: system,
            identifier_value: value,
            confidence: None,
            evidence_refs: vec![source_id.clone()],
        };
        memory.insert(StoredObject::Identity(assertion));
        identity_refs.push(id);
    }

    let mut evaluation_refs = Vec::new();
    if let Some(fixture_id) = attach_validator_fixture_id {
        let eval_id = memory.alloc_id("eval");
        let evaluation = EvaluationRecord {
            header: ObjectHeader {
                id: eval_id.clone(),
                schema_version: AUTHORITY_SCHEMA_VERSION,
                realm_id: realm_id.clone(),
                authority_scope_id: authority_scope_id.clone(),
            },
            target_refs: vec![source_id.clone()],
            evaluator: format!("fixture-oracle:{fixture_id}"),
            result: json!({
                "outcome": "pass",
                "fhirVersion": FHIR_R4_VERSION,
                "evidence_only": true
            }),
            evidence_only: true,
        };
        memory.insert(StoredObject::Evaluation(evaluation));
        evaluation_refs.push(eval_id);
    }

    let audit_id = memory.alloc_id("audit");
    memory.insert(StoredObject::Audit(ActionAuditRecord {
        header: ObjectHeader {
            id: audit_id,
            schema_version: AUTHORITY_SCHEMA_VERSION,
            realm_id,
            authority_scope_id,
        },
        kind: ActionAuditKind::Audit,
        actor: OpaqueId::new("ingest"),
        action: "ingest_fhir_synthetic".to_owned(),
        target_refs: vec![source_id.clone()],
        effect_state: None,
        payload_digest: Some(digest.clone()),
        detail: None,
    }));

    let receipt = IngestReceipt {
        receipt_id: memory.alloc_id("receipt"),
        outcome: IngestOutcome::Accepted,
        source_id: Some(source_id),
        content_digest: Some(digest),
        byte_length: Some(blob.byte_length),
        blob_ref: Some(blob),
        evaluation_refs,
        identity_refs,
        error: None,
    };
    Ok(ResponseBody::Ingested { receipt })
}

pub fn backup(vault: &SyntheticVault, destination: &str) -> Result<ResponseBody, AuthorityError> {
    let manifest = backup_vault(vault, std::path::Path::new(destination)).map_err(|e| {
        if e.contains("sync") || e.contains("refused") {
            AuthorityError::PathOutsideClaim
        } else {
            AuthorityError::InvalidArgument { message: e }
        }
    })?;
    Ok(ResponseBody::Backup { manifest })
}

pub fn restore(source: &str, destination: &str) -> Result<ResponseBody, AuthorityError> {
    let (sources_restored, blobs_restored) = restore_vault(
        std::path::Path::new(source),
        std::path::Path::new(destination),
    )
    .map_err(|e| {
        if e.contains("tamper") {
            AuthorityError::DigestMismatch
        } else {
            AuthorityError::InvalidArgument { message: e }
        }
    })?;
    Ok(ResponseBody::Restored {
        sources_restored,
        blobs_restored,
    })
}

pub fn gc(vault: &SyntheticVault) -> Result<ResponseBody, AuthorityError> {
    let stats = run_gc(&vault.meta, &vault.blobs, 1)
        .map_err(|e| AuthorityError::InvalidArgument { message: e })?;
    Ok(ResponseBody::Gc {
        tombstoned: stats.tombstoned,
        swept: stats.swept,
    })
}

fn lexical_err(err: LexicalError) -> AuthorityError {
    match err {
        LexicalError::Version(got) => AuthorityError::VersionReject { got },
        other => AuthorityError::LexicalReject {
            reason: other.to_string(),
        },
    }
}
