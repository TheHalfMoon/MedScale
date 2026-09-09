//! Persist / reload `InMemoryAuthorityStore` through vault metadata (Spec 016).

use medscale_contracts::envelopes::AuthorityError;
use medscale_contracts::objects::{
    ActionAuditRecord, AmendmentRecord, ClinicalAssertion, DerivedSourceArtifact, DigestSha256,
    EvaluationRecord, IdentityAssertion, IdentityMergeDecision, Projection, Proposal, SourceRecord,
};
use medscale_storage::{AuthorityObjectRow, FsBlobStore, SyntheticVault};
use serde_json::Value;

use super::store::{InMemoryAuthorityStore, StoredObject};

pub fn sync_store_to_vault(
    vault: &SyntheticVault,
    store: &InMemoryAuthorityStore,
) -> Result<(), AuthorityError> {
    let mut rows = Vec::new();
    for obj in store.objects_raw().values() {
        match obj {
            StoredObject::Source(s) => {
                vault
                    .blobs
                    .put_blob(&s.bytes)
                    .map_err(|e| AuthorityError::InvalidArgument {
                        message: e.to_string(),
                    })?;
                let _ = vault.meta.upsert_source(&medscale_storage::SourceMeta {
                    source_id: s.header.id.clone(),
                    realm_id: s.header.realm_id.clone(),
                    authority_scope_id: s.header.authority_scope_id.clone(),
                    digest: s.content_digest.clone(),
                    byte_length: s.bytes.len() as u64,
                    media_type: s.media_type.clone(),
                    visible: true,
                    resource_type: "authority_source".to_owned(),
                });
            }
            StoredObject::Derived(d) => {
                vault
                    .blobs
                    .put_blob(&d.bytes)
                    .map_err(|e| AuthorityError::InvalidArgument {
                        message: e.to_string(),
                    })?;
            }
            _ => {}
        }
        rows.push(to_row(obj, store.next_seq())?);
    }
    vault
        .meta
        .replace_authority_snapshot(&rows, store.next_seq())
        .map_err(|e| AuthorityError::InvalidArgument {
            message: e.to_string(),
        })?;
    Ok(())
}

pub fn load_store_from_vault(
    vault: &SyntheticVault,
    store: &mut InMemoryAuthorityStore,
) -> Result<(), AuthorityError> {
    store.clear();
    let next_seq = vault
        .meta
        .get_next_seq()
        .map_err(|e| AuthorityError::InvalidArgument {
            message: e.to_string(),
        })?;
    store.set_next_seq(next_seq);
    let rows =
        vault
            .meta
            .list_authority_objects()
            .map_err(|e| AuthorityError::InvalidArgument {
                message: e.to_string(),
            })?;
    if rows.is_empty() {
        // v1 vault: reconstruct Source envelopes from sources + blobs.
        reconstruct_sources_from_meta(vault, store)?;
        sync_store_to_vault(vault, store)?;
        return Ok(());
    }
    for row in rows {
        let obj = from_row(&row, &vault.blobs)?;
        store.insert(obj);
    }
    Ok(())
}

fn reconstruct_sources_from_meta(
    vault: &SyntheticVault,
    store: &mut InMemoryAuthorityStore,
) -> Result<(), AuthorityError> {
    let sources = vault
        .meta
        .list_sources()
        .map_err(|e| AuthorityError::InvalidArgument {
            message: e.to_string(),
        })?;
    let mut max_seq = store.next_seq();
    for meta in sources {
        let bytes =
            vault
                .blobs
                .get_blob(&meta.digest)
                .map_err(|e| AuthorityError::InvalidArgument {
                    message: format!("content missing for {}: {e}", meta.source_id.as_str()),
                })?;
        if DigestSha256::of(&bytes) != meta.digest {
            return Err(AuthorityError::DigestMismatch);
        }
        store.insert(StoredObject::Source(SourceRecord {
            header: medscale_contracts::objects::ObjectHeader {
                id: meta.source_id.clone(),
                schema_version: medscale_contracts::AUTHORITY_SCHEMA_VERSION,
                realm_id: meta.realm_id.clone(),
                authority_scope_id: meta.authority_scope_id.clone(),
            },
            bytes,
            content_digest: meta.digest,
            media_type: meta.media_type,
            acquired_at: None,
            provenance_note: Some(format!("reconstructed-from-sources:{}", meta.resource_type)),
        }));
        max_seq = max_seq.max(estimate_seq(meta.source_id.as_str()));
    }
    if max_seq > store.next_seq() {
        store.set_next_seq(max_seq);
    }
    Ok(())
}

fn estimate_seq(id: &str) -> u64 {
    id.rsplit('-')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

fn to_row(obj: &StoredObject, updated_seq: u64) -> Result<AuthorityObjectRow, AuthorityError> {
    let (class, realm, scope, id, digest, body) = match obj {
        StoredObject::Source(s) => {
            let mut v = serde_json::to_value(s).map_err(|e| AuthorityError::InvalidArgument {
                message: e.to_string(),
            })?;
            if let Some(map) = v.as_object_mut() {
                map.insert("bytes".to_owned(), Value::Array(vec![]));
            }
            (
                "source".to_owned(),
                s.header.realm_id.as_opaque().as_str().to_owned(),
                s.header.authority_scope_id.as_opaque().as_str().to_owned(),
                s.header.id.as_str().to_owned(),
                Some(s.content_digest.to_hex()),
                v,
            )
        }
        StoredObject::Derived(d) => {
            let mut v = serde_json::to_value(d).map_err(|e| AuthorityError::InvalidArgument {
                message: e.to_string(),
            })?;
            if let Some(map) = v.as_object_mut() {
                map.insert("bytes".to_owned(), Value::Array(vec![]));
            }
            (
                "derived".to_owned(),
                d.header.realm_id.as_opaque().as_str().to_owned(),
                d.header.authority_scope_id.as_opaque().as_str().to_owned(),
                d.header.id.as_str().to_owned(),
                Some(d.content_digest.to_hex()),
                v,
            )
        }
        StoredObject::Proposal(p) => row_parts("proposal", p)?,
        StoredObject::Assertion(a) => row_parts("assertion", a)?,
        StoredObject::Audit(a) => row_parts("audit", a)?,
        StoredObject::Identity(a) => row_parts("identity", a)?,
        StoredObject::Merge(a) => row_parts("merge", a)?,
        StoredObject::Evaluation(a) => row_parts("evaluation", a)?,
        StoredObject::Projection(a) => row_parts("projection", a)?,
        StoredObject::Amendment(a) => row_parts("amendment", a)?,
    };
    let body_json = serde_json::to_string(&body).map_err(|e| AuthorityError::InvalidArgument {
        message: e.to_string(),
    })?;
    Ok(AuthorityObjectRow {
        object_id: id,
        object_class: class,
        realm_id: realm,
        authority_scope_id: scope,
        body_json,
        content_digest_hex: digest,
        updated_seq,
    })
}

type RowParts = (String, String, String, String, Option<String>, Value);

fn row_parts<T: serde::Serialize + HasHeader>(
    class: &'static str,
    value: &T,
) -> Result<RowParts, AuthorityError> {
    let v = serde_json::to_value(value).map_err(|e| AuthorityError::InvalidArgument {
        message: e.to_string(),
    })?;
    let h = value.header_parts();
    Ok((class.to_owned(), h.0, h.1, h.2, None, v))
}

trait HasHeader {
    fn header_parts(&self) -> (String, String, String);
}

macro_rules! impl_header {
    ($t:ty) => {
        impl HasHeader for $t {
            fn header_parts(&self) -> (String, String, String) {
                (
                    self.header.realm_id.as_opaque().as_str().to_owned(),
                    self.header
                        .authority_scope_id
                        .as_opaque()
                        .as_str()
                        .to_owned(),
                    self.header.id.as_str().to_owned(),
                )
            }
        }
    };
}

impl_header!(Proposal);
impl_header!(ClinicalAssertion);
impl_header!(ActionAuditRecord);
impl_header!(IdentityAssertion);
impl_header!(IdentityMergeDecision);
impl_header!(EvaluationRecord);
impl_header!(Projection);
impl_header!(AmendmentRecord);

fn from_row(row: &AuthorityObjectRow, blobs: &FsBlobStore) -> Result<StoredObject, AuthorityError> {
    let value: Value =
        serde_json::from_str(&row.body_json).map_err(|e| AuthorityError::InvalidArgument {
            message: format!("corrupt object body: {e}"),
        })?;
    match row.object_class.as_str() {
        "source" => {
            let mut source: SourceRecord =
                serde_json::from_value(value).map_err(|e| AuthorityError::InvalidArgument {
                    message: format!("corrupt source: {e}"),
                })?;
            let digest = source.content_digest.clone();
            let bytes = blobs
                .get_blob(&digest)
                .map_err(|e| AuthorityError::InvalidArgument {
                    message: format!("content missing: {e}"),
                })?;
            if DigestSha256::of(&bytes) != digest {
                return Err(AuthorityError::DigestMismatch);
            }
            source.bytes = bytes;
            Ok(StoredObject::Source(source))
        }
        "derived" => {
            let mut derived: DerivedSourceArtifact =
                serde_json::from_value(value).map_err(|e| AuthorityError::InvalidArgument {
                    message: format!("corrupt derived: {e}"),
                })?;
            let digest = derived.content_digest.clone();
            let bytes = blobs
                .get_blob(&digest)
                .map_err(|e| AuthorityError::InvalidArgument {
                    message: format!("content missing: {e}"),
                })?;
            if DigestSha256::of(&bytes) != digest {
                return Err(AuthorityError::DigestMismatch);
            }
            derived.bytes = bytes;
            Ok(StoredObject::Derived(derived))
        }
        "proposal" => Ok(StoredObject::Proposal(
            serde_json::from_value(value).map_err(|e| AuthorityError::InvalidArgument {
                message: e.to_string(),
            })?,
        )),
        "assertion" => Ok(StoredObject::Assertion(
            serde_json::from_value(value).map_err(|e| AuthorityError::InvalidArgument {
                message: e.to_string(),
            })?,
        )),
        "audit" => Ok(StoredObject::Audit(serde_json::from_value(value).map_err(
            |e| AuthorityError::InvalidArgument {
                message: e.to_string(),
            },
        )?)),
        "identity" => Ok(StoredObject::Identity(
            serde_json::from_value(value).map_err(|e| AuthorityError::InvalidArgument {
                message: e.to_string(),
            })?,
        )),
        "merge" => Ok(StoredObject::Merge(serde_json::from_value(value).map_err(
            |e| AuthorityError::InvalidArgument {
                message: e.to_string(),
            },
        )?)),
        "evaluation" => Ok(StoredObject::Evaluation(
            serde_json::from_value(value).map_err(|e| AuthorityError::InvalidArgument {
                message: e.to_string(),
            })?,
        )),
        "projection" => Ok(StoredObject::Projection(
            serde_json::from_value(value).map_err(|e| AuthorityError::InvalidArgument {
                message: e.to_string(),
            })?,
        )),
        "amendment" => Ok(StoredObject::Amendment(
            serde_json::from_value(value).map_err(|e| AuthorityError::InvalidArgument {
                message: e.to_string(),
            })?,
        )),
        other => Err(AuthorityError::InvalidArgument {
            message: format!("unknown object class {other}"),
        }),
    }
}
