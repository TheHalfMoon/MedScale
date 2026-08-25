//! In-process Core Host authority facade (logical IPC API).

use std::sync::Mutex;

use medscale_contracts::AUTHORITY_SCHEMA_VERSION;
use medscale_contracts::envelopes::{
    AuthorityError, AuthorityRequest, AuthorityResponse, Capability, RequestBody, ResponseBody,
};
use medscale_contracts::objects::{
    ActionAuditKind, ActionAuditRecord, DerivedSourceArtifact, DigestSha256, EffectState,
    LossClass, ObjectHeader, ProducerKind, Proposal, RepresentationKind,
};

use crate::effects;
use crate::process::{LeaseError, LeaseRegistry};

use super::identity::{create_identity_assertion, decide_identity_merge};
use super::promote::{PromoteError, promote_proposal};
use super::source_ops::create_source_record;
use super::store::{InMemoryAuthorityStore, ScopeError, StoredObject};

/// In-process facade owning lease registry + in-memory store.
#[derive(Debug, Default)]
pub struct CoreFacade {
    leases: LeaseRegistry,
    store: Mutex<InMemoryAuthorityStore>,
}

impl CoreFacade {
    /// Creates a new facade instance (one per process/host in Spec 002).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn store(&self) -> std::sync::MutexGuard<'_, InMemoryAuthorityStore> {
        self.store
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Dispatches a versioned authority request.
    pub fn dispatch(&self, req: AuthorityRequest) -> AuthorityResponse {
        let request_id = req.request_id.clone();
        let result = self.dispatch_inner(req);
        AuthorityResponse {
            schema_version: AUTHORITY_SCHEMA_VERSION,
            request_id,
            result,
        }
    }

    fn dispatch_inner(&self, req: AuthorityRequest) -> Result<ResponseBody, AuthorityError> {
        if req.schema_version != AUTHORITY_SCHEMA_VERSION {
            return Err(AuthorityError::InvalidArgument {
                message: "unsupported schema_version".to_owned(),
            });
        }
        if !capability_matches(&req.capability, &req.body) {
            return Err(AuthorityError::Unauthorized);
        }

        match req.body {
            RequestBody::AcquireLease {
                client_id,
                holder_id_hint,
            } => {
                let holder = self
                    .leases
                    .acquire(&req.vault_id, client_id, holder_id_hint)
                    .map_err(lease_err)?;
                Ok(ResponseBody::Lease {
                    holder_id: holder,
                    vault_id: req.vault_id,
                })
            }
            RequestBody::ReleaseLease { holder_id } => {
                self.leases
                    .release(&req.vault_id, &holder_id)
                    .map_err(lease_err)?;
                Ok(ResponseBody::Released)
            }
            RequestBody::Ping => Ok(ResponseBody::Pong {
                schema_version: AUTHORITY_SCHEMA_VERSION,
            }),
            RequestBody::CreateSourceRecord { media_type, bytes } => {
                let mut store = self.store();
                let record = create_source_record(
                    &mut store,
                    req.realm_id,
                    req.authority_scope_id,
                    media_type,
                    bytes,
                );
                Ok(ResponseBody::Created {
                    object_id: record.header.id,
                })
            }
            RequestBody::CreateDerivedArtifact {
                source_id,
                transform_id,
                transform_version,
                bytes,
            } => {
                let mut store = self.store();
                store
                    .get_source(&source_id, &req.realm_id, &req.authority_scope_id)
                    .map_err(scope_err)?;
                let id = store.alloc_id("derived");
                let digest = DigestSha256::of(&bytes);
                let artifact = DerivedSourceArtifact {
                    header: ObjectHeader {
                        id: id.clone(),
                        schema_version: AUTHORITY_SCHEMA_VERSION,
                        realm_id: req.realm_id,
                        authority_scope_id: req.authority_scope_id,
                    },
                    source_id,
                    transform_id,
                    transform_version,
                    loss_class: LossClass::Unknown,
                    representation: RepresentationKind::NormalizedText,
                    bytes,
                    content_digest: digest,
                    parent_span_map_ref: None,
                };
                store.insert(StoredObject::Derived(artifact));
                Ok(ResponseBody::Created { object_id: id })
            }
            RequestBody::CreateProposal {
                subject_ref,
                claim_kind,
                payload,
                evidence_refs,
            } => {
                let mut store = self.store();
                let id = store.alloc_id("proposal");
                let proposal = Proposal {
                    header: ObjectHeader {
                        id: id.clone(),
                        schema_version: AUTHORITY_SCHEMA_VERSION,
                        realm_id: req.realm_id,
                        authority_scope_id: req.authority_scope_id,
                    },
                    subject_ref,
                    claim_kind,
                    payload,
                    confidence: None,
                    evidence_refs,
                    producer: ProducerKind::Rule,
                };
                store.insert(StoredObject::Proposal(proposal));
                Ok(ResponseBody::Created { object_id: id })
            }
            RequestBody::PromoteProposal {
                proposal_id,
                authorized_by,
                subject_ref,
            } => {
                let mut store = self.store();
                let (assertion, audit) = promote_proposal(
                    &mut store,
                    &proposal_id,
                    authorized_by,
                    subject_ref,
                    &req.realm_id,
                    &req.authority_scope_id,
                )
                .map_err(promote_err)?;
                Ok(ResponseBody::Promoted {
                    assertion_id: assertion.header.id,
                    audit_id: audit.header.id,
                })
            }
            RequestBody::CreateIdentityAssertion {
                subject_id,
                identifier_system,
                identifier_value,
            } => {
                let mut store = self.store();
                let record = create_identity_assertion(
                    &mut store,
                    req.realm_id,
                    req.authority_scope_id,
                    subject_id,
                    identifier_system,
                    identifier_value,
                );
                Ok(ResponseBody::Created {
                    object_id: record.header.id,
                })
            }
            RequestBody::DecideIdentityMerge {
                surviving_subject_id,
                merged_subject_ids,
                authorized_by,
                rationale,
            } => {
                let mut store = self.store();
                let record = decide_identity_merge(
                    &mut store,
                    req.realm_id,
                    req.authority_scope_id,
                    surviving_subject_id,
                    merged_subject_ids,
                    authorized_by,
                    rationale,
                )
                .map_err(|message| AuthorityError::InvalidArgument {
                    message: message.to_owned(),
                })?;
                Ok(ResponseBody::Created {
                    object_id: record.header.id,
                })
            }
            RequestBody::AppendAudit {
                actor,
                action,
                target_refs,
                detail,
            } => {
                let mut store = self.store();
                let id = store.alloc_id("audit");
                let audit = ActionAuditRecord {
                    header: ObjectHeader {
                        id: id.clone(),
                        schema_version: AUTHORITY_SCHEMA_VERSION,
                        realm_id: req.realm_id,
                        authority_scope_id: req.authority_scope_id,
                    },
                    kind: ActionAuditKind::Audit,
                    actor,
                    action,
                    target_refs,
                    effect_state: None,
                    payload_digest: None,
                    detail,
                };
                store.insert(StoredObject::Audit(audit));
                Ok(ResponseBody::Created { object_id: id })
            }
            RequestBody::TransitionEffect {
                action_id,
                to,
                reconcile_token,
            } => {
                let mut store = self.store();
                let audit = store
                    .get_audit_mut(&action_id)
                    .ok_or(AuthorityError::NotFound)?;
                if audit.kind != ActionAuditKind::ExternalActionIntent {
                    // Promote plain audit into effect-bearing intent if needed.
                    audit.kind = ActionAuditKind::ExternalActionIntent;
                    if audit.effect_state.is_none() {
                        audit.effect_state = Some(EffectState::Pending);
                    }
                }
                let from = audit.effect_state.unwrap_or(EffectState::Pending);
                let next = effects::transition(from, to, reconcile_token.as_deref()).ok_or(
                    if from == EffectState::Unknown
                        && reconcile_token.as_deref().unwrap_or("").is_empty()
                    {
                        AuthorityError::UnknownRequiresReconcile
                    } else {
                        AuthorityError::IllegalTransition
                    },
                )?;
                audit.effect_state = Some(next);
                Ok(ResponseBody::Effect {
                    action_id,
                    state: next,
                })
            }
            RequestBody::ReadObject { object_id } => {
                let store = self.store();
                let obj = store
                    .get_scoped(&object_id, &req.realm_id, &req.authority_scope_id)
                    .map_err(scope_err)?;
                Ok(ResponseBody::Object {
                    value: obj.to_json(),
                })
            }
        }
    }
}

fn capability_matches(cap: &Capability, body: &RequestBody) -> bool {
    matches!(
        (cap, body),
        (Capability::AcquireLease, RequestBody::AcquireLease { .. })
            | (Capability::ReleaseLease, RequestBody::ReleaseLease { .. })
            | (Capability::Ping, RequestBody::Ping)
            | (
                Capability::CreateSourceRecord,
                RequestBody::CreateSourceRecord { .. }
            )
            | (
                Capability::CreateDerivedArtifact,
                RequestBody::CreateDerivedArtifact { .. }
            )
            | (
                Capability::CreateProposal,
                RequestBody::CreateProposal { .. }
            )
            | (
                Capability::PromoteProposal,
                RequestBody::PromoteProposal { .. }
            )
            | (
                Capability::CreateIdentityAssertion,
                RequestBody::CreateIdentityAssertion { .. }
            )
            | (
                Capability::DecideIdentityMerge,
                RequestBody::DecideIdentityMerge { .. }
            )
            | (Capability::AppendAudit, RequestBody::AppendAudit { .. })
            | (
                Capability::TransitionEffect,
                RequestBody::TransitionEffect { .. }
            )
            | (Capability::ReadObject, RequestBody::ReadObject { .. })
    )
}

fn lease_err(err: LeaseError) -> AuthorityError {
    match err {
        LeaseError::AlreadyHeld { holder_id } => AuthorityError::AlreadyHeld { holder_id },
        LeaseError::NotHolder => AuthorityError::NotHolder,
        LeaseError::NotHeld => AuthorityError::NotHeld,
    }
}

fn scope_err(err: ScopeError) -> AuthorityError {
    match err {
        ScopeError::NotFound => AuthorityError::NotFound,
        ScopeError::WrongScope => AuthorityError::WrongScope,
    }
}

fn promote_err(err: PromoteError) -> AuthorityError {
    match err {
        PromoteError::NotFound => AuthorityError::NotFound,
        PromoteError::WrongScope => AuthorityError::WrongScope,
    }
}
