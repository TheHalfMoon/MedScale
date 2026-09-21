//! In-process Core Host authority facade (logical IPC API).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use medscale_contracts::AUTHORITY_SCHEMA_VERSION;
use medscale_contracts::envelopes::{
    AuthorityError, AuthorityRequest, AuthorityResponse, Capability, RequestBody, ResponseBody,
};
use medscale_contracts::objects::{
    ActionAuditKind, ActionAuditRecord, DerivedSourceArtifact, DigestSha256, EffectState,
    EvaluationRecord, LossClass, MedicalTime, ObjectHeader, OpaqueId, ProducerKind, Projection,
    Proposal, RepresentationKind, TimePrecision,
};

use crate::effects;
use crate::process::{LeaseError, LeaseRegistry, SessionEnforcement, SessionRegistry};

use super::identity::{create_identity_assertion, decide_identity_merge};
use super::ingest_ops;
use super::presentation;
use super::promote::{PromoteError, promote_proposal};
use super::source_ops::create_source_record;
use super::store::{InMemoryAuthorityStore, ScopeError, StoredObject};
use medscale_contracts::network::EgressAllowlistEntry;
use medscale_network::FixtureTransport;
use medscale_storage::{EncryptedVault, SyntheticVault};

const MAX_PREPARED_MODEL_CACHE: usize = 4;

/// In-process facade owning lease registry + in-memory store + optional vaults.
#[derive(Debug)]
pub struct CoreFacade {
    leases: LeaseRegistry,
    sessions: SessionRegistry,
    session_enforcement: SessionEnforcement,
    store: Mutex<InMemoryAuthorityStore>,
    vault: Mutex<Option<SyntheticVault>>,
    encrypted: Mutex<Option<EncryptedVault>>,
    allowlist: Mutex<Vec<EgressAllowlistEntry>>,
    packs: Mutex<medscale_pack::PackStore>,
    prepared_models: Mutex<HashMap<DigestSha256, Arc<medscale_pack::PreparedOnnxTokenClassifier>>>,
    mesc_epochs: Mutex<medscale_pack::MescEpochStore>,
}

impl Default for CoreFacade {
    fn default() -> Self {
        Self::new()
    }
}

impl CoreFacade {
    /// Creates a new facade with Spec 024 Strict session enforcement (fail-closed).
    #[must_use]
    pub fn new() -> Self {
        Self {
            leases: LeaseRegistry::new(),
            sessions: SessionRegistry::new(),
            session_enforcement: SessionEnforcement::Strict,
            store: Mutex::new(InMemoryAuthorityStore::default()),
            vault: Mutex::new(None),
            encrypted: Mutex::new(None),
            allowlist: Mutex::new(Vec::new()),
            packs: Mutex::new(medscale_pack::PackStore::default()),
            prepared_models: Mutex::new(HashMap::new()),
            mesc_epochs: Mutex::new(medscale_pack::MescEpochStore::new()),
        }
    }

    /// Engineering/test escape: Spec 018 legacy lease-only mutation without `session_id`.
    ///
    /// Not used by the qualified OS IPC host path. Prefer Strict (`CoreFacade::new()`).
    #[must_use]
    pub fn new_legacy_lease_only_engineering() -> Self {
        Self {
            session_enforcement: SessionEnforcement::LegacyLeaseOnlyEngineering,
            ..Self::new()
        }
    }

    /// In-process session registry (Spec 018 READY_BASE).
    #[must_use]
    pub fn sessions(&self) -> &SessionRegistry {
        &self.sessions
    }

    /// Active session enforcement mode (Spec 024).
    #[must_use]
    pub fn session_enforcement(&self) -> SessionEnforcement {
        self.session_enforcement
    }

    fn store(&self) -> std::sync::MutexGuard<'_, InMemoryAuthorityStore> {
        self.store
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn vault(&self) -> std::sync::MutexGuard<'_, Option<SyntheticVault>> {
        self.vault
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn encrypted(&self) -> std::sync::MutexGuard<'_, Option<EncryptedVault>> {
        self.encrypted
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Runs one Spec 074 project-graph operation with lease enforcement and
    /// vault-meta access (synthetic or encrypted). Surfaces never touch
    /// storage: this is the only path from request to project rows.
    fn pg<R>(
        &self,
        vault_id: &medscale_contracts::objects::VaultId,
        realm: medscale_contracts::objects::RealmId,
        scope: medscale_contracts::objects::AuthorityScopeId,
        session_id: Option<medscale_contracts::objects::OpaqueId>,
        op: impl FnOnce(super::project_graph::ProjectGraph<'_>) -> Result<R, AuthorityError>,
    ) -> Result<R, AuthorityError> {
        self.require_lease(vault_id)?;
        // Lock order vault -> store matches the existing multi-lock arms and
        // keeps no new lock ordering in the lane.
        let vault_guard = self.vault();
        let enc_guard = self.encrypted();
        let meta = if let Some(enc) = enc_guard.as_ref() {
            &enc.meta
        } else if let Some(vault) = vault_guard.as_ref() {
            &vault.meta
        } else {
            return Err(AuthorityError::VaultRequired);
        };
        let mut store = self.store();
        let packs_guard = self.packs();
        let store_ref: &mut InMemoryAuthorityStore = &mut store;
        let packs_ref: &medscale_pack::PackStore = &packs_guard;
        op(super::project_graph::ProjectGraph {
            store: store_ref,
            meta,
            packs: packs_ref,
            sessions: &self.sessions,
            leases: &self.leases,
            vault_id,
            realm,
            scope,
            session_id,
        })
    }

    /// Runs one Spec 075 data-source operation with lease enforcement and
    /// vault-meta/blob access (synthetic or encrypted). Surfaces never touch
    /// storage, drivers, or transports: this is the only path from request
    /// to data-source rows. Remote acquisition is brokered through the
    /// fixture transport; live hosts stay external-gate refused.
    fn ds<R>(
        &self,
        vault_id: &medscale_contracts::objects::VaultId,
        realm: medscale_contracts::objects::RealmId,
        scope: medscale_contracts::objects::AuthorityScopeId,
        session_id: Option<medscale_contracts::objects::OpaqueId>,
        op: impl FnOnce(super::data_sources::DataSources<'_>) -> Result<R, AuthorityError>,
    ) -> Result<R, AuthorityError> {
        self.require_lease(vault_id)?;
        // Lock order vault -> store matches the existing multi-lock arms and
        // keeps no new lock ordering in the lane.
        let vault_guard = self.vault();
        let enc_guard = self.encrypted();
        let backend = if let Some(enc) = enc_guard.as_ref() {
            super::data_sources::SnapshotBlobBackend::Encrypted(enc)
        } else if let Some(vault) = vault_guard.as_ref() {
            super::data_sources::SnapshotBlobBackend::Synthetic(vault)
        } else {
            return Err(AuthorityError::VaultRequired);
        };
        let mut store = self.store();
        let allow_guard = self.allowlist();
        let transport = FixtureTransport;
        op(super::data_sources::DataSources {
            store: &mut store,
            backend,
            allowlist: &allow_guard,
            transport: &transport,
            sessions: &self.sessions,
            leases: &self.leases,
            vault_id,
            realm,
            scope,
            session_id,
        })
    }

    /// Runs one Spec 076 collaboration operation with lease enforcement and
    /// vault-meta access (synthetic or encrypted). Surfaces never touch
    /// storage: this is the only path from request to collaboration rows.
    fn collab<R>(
        &self,
        vault_id: &medscale_contracts::objects::VaultId,
        realm: medscale_contracts::objects::RealmId,
        scope: medscale_contracts::objects::AuthorityScopeId,
        session_id: Option<medscale_contracts::objects::OpaqueId>,
        op: impl FnOnce(super::collaboration::Collab<'_>) -> Result<R, AuthorityError>,
    ) -> Result<R, AuthorityError> {
        self.require_lease(vault_id)?;
        // Lock order vault -> store matches the existing multi-lock arms and
        // keeps no new lock ordering in the lane.
        let vault_guard = self.vault();
        let enc_guard = self.encrypted();
        let meta = if let Some(enc) = enc_guard.as_ref() {
            &enc.meta
        } else if let Some(vault) = vault_guard.as_ref() {
            &vault.meta
        } else {
            return Err(AuthorityError::VaultRequired);
        };
        let mut store = self.store();
        let store_ref: &mut InMemoryAuthorityStore = &mut store;
        op(super::collaboration::Collab {
            store: store_ref,
            meta,
            sessions: &self.sessions,
            leases: &self.leases,
            vault_id,
            realm,
            scope,
            session_id,
        })
    }

    fn allowlist(&self) -> std::sync::MutexGuard<'_, Vec<EgressAllowlistEntry>> {
        self.allowlist
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn packs(&self) -> std::sync::MutexGuard<'_, medscale_pack::PackStore> {
        self.packs
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn prepared_models(
        &self,
    ) -> std::sync::MutexGuard<
        '_,
        HashMap<DigestSha256, Arc<medscale_pack::PreparedOnnxTokenClassifier>>,
    > {
        self.prepared_models
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn mesc_epochs(&self) -> std::sync::MutexGuard<'_, medscale_pack::MescEpochStore> {
        self.mesc_epochs
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn require_lease(
        &self,
        vault_id: &medscale_contracts::objects::VaultId,
    ) -> Result<(), AuthorityError> {
        if self.leases.holder(vault_id).is_none() {
            Err(AuthorityError::LeaseRequired)
        } else {
            Ok(())
        }
    }

    /// Dispatches a versioned authority request.
    pub fn dispatch(&self, req: AuthorityRequest) -> AuthorityResponse {
        let request_id = req.request_id.clone();
        // Spec 074 high-frequency graph ops provably leave the in-memory
        // snapshot untouched (see RequestBody::preserves_memory_snapshot);
        // skipping the rewrite is output-identical and keeps per-op cost
        // constant instead of O(full history).
        let skip_snapshot = req.body.preserves_memory_snapshot();
        let result = self.dispatch_inner(req).and_then(|body| {
            if skip_snapshot {
                Ok(body)
            } else {
                self.persist_open_vault()?;
                Ok(body)
            }
        });
        AuthorityResponse {
            schema_version: AUTHORITY_SCHEMA_VERSION,
            request_id,
            result,
        }
    }

    fn persist_open_vault(&self) -> Result<(), AuthorityError> {
        let store = self.store();
        if let Some(enc) = self.encrypted().as_ref() {
            return super::durable::sync_store_to_encrypted(enc, &store);
        }
        let vault = self.vault();
        let Some(vault) = vault.as_ref() else {
            return Ok(());
        };
        super::durable::sync_store_to_vault(vault, &store)
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

        // Spec 024: Strict requires session_id for mutating capabilities.
        // LegacyLeaseOnlyEngineering preserves Spec 018 opt-in validation-only-when-present.
        match self.session_enforcement {
            SessionEnforcement::Strict => {
                if req.capability.requires_client_session() {
                    let Some(session_id) = &req.session_id else {
                        return Err(AuthorityError::SessionRequired);
                    };
                    self.sessions
                        .validate(session_id, &req.vault_id, req.capability)?;
                } else if let Some(session_id) = &req.session_id {
                    self.sessions
                        .validate(session_id, &req.vault_id, req.capability)?;
                }
            }
            SessionEnforcement::LegacyLeaseOnlyEngineering => {
                if let Some(session_id) = &req.session_id {
                    self.sessions
                        .validate(session_id, &req.vault_id, req.capability)?;
                }
            }
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
            RequestBody::OpenSession {
                holder_id,
                granted,
                ttl_ticks,
            } => {
                let Some(lease_holder) = self.leases.holder(&req.vault_id) else {
                    return Err(AuthorityError::LeaseRequired);
                };
                if lease_holder != holder_id {
                    return Err(AuthorityError::NotHolder);
                }
                let (session_id, expires_at_tick) =
                    self.sessions
                        .open(&req.vault_id, holder_id, granted, ttl_ticks);
                Ok(ResponseBody::Session {
                    session_id,
                    expires_at_tick,
                })
            }
            RequestBody::RevokeSession { session_id } => {
                self.sessions.revoke(&session_id);
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
            RequestBody::AmendAssertion {
                prior_assertion_id,
                authorized_by,
                payload,
                effective_time,
                kind,
                rationale,
            } => {
                let mut store = self.store();
                let (assertion, amendment, audit) = super::amend::amend_assertion(
                    &mut store,
                    super::amend::AmendAssertionInput {
                        prior_assertion_id: &prior_assertion_id,
                        authorized_by,
                        payload,
                        effective_time,
                        kind,
                        rationale,
                        realm_id: &req.realm_id,
                        scope_id: &req.authority_scope_id,
                    },
                )
                .map_err(amend_err)?;
                Ok(ResponseBody::Amended {
                    assertion_id: assertion.header.id,
                    amendment_id: amendment.header.id,
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
                // READY_BASE: cannot leave Pending toward Sent without bound payload digest.
                if from == EffectState::Pending
                    && to == EffectState::Sent
                    && audit.payload_digest.is_none()
                {
                    return Err(AuthorityError::InvalidArgument {
                        message: "payload_digest required before Pending→Sent".to_owned(),
                    });
                }
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
            RequestBody::OpenSyntheticVault { vault_root } => {
                self.require_lease(&req.vault_id)?;
                if self.encrypted().is_some() {
                    return Err(AuthorityError::InvalidArgument {
                        message: "encrypted vault already open; close before opening synthetic"
                            .to_owned(),
                    });
                }
                let mut slot = self.vault();
                let mut store = self.store();
                ingest_ops::open_vault(&mut slot, &mut store, req.vault_id.as_str(), &vault_root)
            }
            RequestBody::CloseVault => {
                self.require_lease(&req.vault_id)?;
                let mut slot = self.vault();
                let store = self.store();
                ingest_ops::close_vault(&mut slot, &store)
            }
            RequestBody::IngestFhirSynthetic {
                media_type,
                bytes,
                fhir_version_hint,
                attach_validator_fixture_id,
            } => {
                self.require_lease(&req.vault_id)?;
                let vault_guard = self.vault();
                let vault = vault_guard.as_ref().ok_or(AuthorityError::VaultRequired)?;
                let mut store = self.store();
                ingest_ops::ingest_fhir(
                    vault,
                    &mut store,
                    req.realm_id,
                    req.authority_scope_id,
                    media_type,
                    bytes,
                    fhir_version_hint,
                    attach_validator_fixture_id,
                )
            }
            RequestBody::AttachValidatorEvidence {
                source_id,
                evaluator,
                outcome,
                issue_codes,
            } => {
                self.require_lease(&req.vault_id)?;
                let mut store = self.store();
                store
                    .get_scoped(&source_id, &req.realm_id, &req.authority_scope_id)
                    .map_err(scope_err)?;
                let eval_id = store.alloc_id("eval");
                store.insert(StoredObject::Evaluation(EvaluationRecord {
                    header: ObjectHeader {
                        id: eval_id.clone(),
                        schema_version: AUTHORITY_SCHEMA_VERSION,
                        realm_id: req.realm_id,
                        authority_scope_id: req.authority_scope_id,
                    },
                    target_refs: vec![source_id],
                    evaluator,
                    result: serde_json::json!({ "outcome": outcome, "issues": issue_codes, "evidence_only": true }),
                    evidence_only: true,
                }));
                Ok(ResponseBody::Created { object_id: eval_id })
            }
            RequestBody::RebuildProjection { kind, built_from } => {
                self.require_lease(&req.vault_id)?;
                let mut store = self.store();
                let vault_guard = self.vault();
                let body = if presentation::is_presentation_kind(&kind) {
                    // Prefer subject from first assertion in built_from, else treat first id as subject.
                    let subject_ref =
                        built_from
                            .first()
                            .cloned()
                            .ok_or(AuthorityError::InvalidArgument {
                                message: "built_from required".to_owned(),
                            })?;
                    // If first id is an assertion, use its subject; else use as subject_ref directly.
                    let subject = match store.get_scoped(
                        &subject_ref,
                        &req.realm_id,
                        &req.authority_scope_id,
                    ) {
                        Ok(StoredObject::Assertion(a)) => a.subject_ref.clone(),
                        _ => subject_ref.clone(),
                    };
                    let lookup = |d: &medscale_contracts::objects::DigestSha256| {
                        blob_lookup(vault_guard.as_ref(), d)
                    };
                    match kind.as_str() {
                        medscale_contracts::presentation::KIND_SUBJECT_TIMELINE_V1 => {
                            presentation::body_timeline(&presentation::build_timeline(
                                &store, &subject, &lookup,
                            ))
                        }
                        medscale_contracts::presentation::KIND_SUBJECT_BRIEF_V1 => {
                            presentation::body_brief(&presentation::build_brief(
                                &store, &subject, &lookup,
                            ))
                        }
                        medscale_contracts::presentation::KIND_SUBJECT_COVERAGE_V1 => {
                            presentation::body_coverage(&presentation::build_coverage(
                                &store, &subject, &lookup,
                            ))
                        }
                        _ => serde_json::json!({}),
                    }
                } else {
                    serde_json::json!({})
                };
                let id = store.alloc_id("proj");
                let proj = Projection {
                    header: ObjectHeader {
                        id: id.clone(),
                        schema_version: AUTHORITY_SCHEMA_VERSION,
                        realm_id: req.realm_id,
                        authority_scope_id: req.authority_scope_id,
                    },
                    projection_kind: kind,
                    built_from,
                    built_at: MedicalTime::new(
                        "1970-01-01T00:00:00Z",
                        TimePrecision::Instant,
                        false,
                    ),
                    body,
                    authoritative: false,
                };
                store.insert(StoredObject::Projection(proj));
                Ok(ResponseBody::Created { object_id: id })
            }
            RequestBody::ReadCanonicalVisibility { source_id } => {
                self.require_lease(&req.vault_id)?;
                let vault_guard = self.vault();
                let vault = vault_guard.as_ref().ok_or(AuthorityError::VaultRequired)?;
                let meta = vault
                    .meta
                    .get_source(&source_id)
                    .map_err(|_| AuthorityError::NotFound)?;
                if meta.realm_id != req.realm_id
                    || meta.authority_scope_id != req.authority_scope_id
                {
                    return Err(AuthorityError::WrongScope);
                }
                let blob_ok = vault.blobs.verify(&meta.digest, meta.byte_length).is_ok();
                Ok(ResponseBody::Visibility {
                    source_id,
                    visible: meta.visible && blob_ok,
                    content_digest: meta.digest,
                    byte_length: meta.byte_length,
                    blob_ok,
                })
            }
            RequestBody::VerifyBlob { digest } => {
                self.require_lease(&req.vault_id)?;
                let vault_guard = self.vault();
                let vault = vault_guard.as_ref().ok_or(AuthorityError::VaultRequired)?;
                let state = vault.blobs.state(&digest);
                let bytes = vault
                    .blobs
                    .get_blob(&digest)
                    .map_err(|_| AuthorityError::NotFound)?;
                Ok(ResponseBody::BlobVerified {
                    digest,
                    byte_length: bytes.len() as u64,
                    state,
                })
            }
            RequestBody::BackupVault { destination } => {
                self.require_lease(&req.vault_id)?;
                let vault_guard = self.vault();
                let vault = vault_guard.as_ref().ok_or(AuthorityError::VaultRequired)?;
                ingest_ops::backup(vault, &destination)
            }
            RequestBody::RestoreVault {
                source,
                destination,
            } => {
                self.require_lease(&req.vault_id)?;
                ingest_ops::restore(&source, &destination)
            }
            RequestBody::RunBlobGc => {
                self.require_lease(&req.vault_id)?;
                let vault_guard = self.vault();
                let vault = vault_guard.as_ref().ok_or(AuthorityError::VaultRequired)?;
                ingest_ops::gc(vault)
            }
            RequestBody::GetTimeline { subject_ref } => {
                self.require_lease(&req.vault_id)?;
                let store = self.store();
                let vault_guard = self.vault();
                let body = presentation::build_timeline(&store, &subject_ref, &|d| {
                    blob_lookup(vault_guard.as_ref(), d)
                });
                Ok(ResponseBody::Timeline {
                    body,
                    projection_id: None,
                })
            }
            RequestBody::GetBrief { subject_ref } => {
                self.require_lease(&req.vault_id)?;
                let store = self.store();
                let vault_guard = self.vault();
                let body = presentation::build_brief(&store, &subject_ref, &|d| {
                    blob_lookup(vault_guard.as_ref(), d)
                });
                Ok(ResponseBody::Brief {
                    body,
                    projection_id: None,
                })
            }
            RequestBody::GetCoverage { subject_ref } => {
                self.require_lease(&req.vault_id)?;
                let store = self.store();
                let vault_guard = self.vault();
                let body = presentation::build_coverage(&store, &subject_ref, &|d| {
                    blob_lookup(vault_guard.as_ref(), d)
                });
                Ok(ResponseBody::Coverage {
                    body,
                    projection_id: None,
                })
            }
            RequestBody::DrillDownPresentation {
                subject_ref,
                field_key,
                assertion_id,
            } => {
                self.require_lease(&req.vault_id)?;
                let store = self.store();
                let vault_guard = self.vault();
                let result = presentation::drill_down(
                    &store,
                    &subject_ref,
                    &field_key,
                    assertion_id.as_ref(),
                    &|d| blob_lookup(vault_guard.as_ref(), d),
                )
                .map_err(|e| match e {
                    presentation::DrillDownError::NotFound => AuthorityError::NotFound,
                    presentation::DrillDownError::UnhealthyEvidence => {
                        AuthorityError::InvalidArgument {
                            message: "unhealthy_evidence".to_owned(),
                        }
                    }
                })?;
                Ok(ResponseBody::DrillDown { result })
            }
            RequestBody::CreateEncryptedVault {
                vault_root,
                passphrase,
            } => {
                self.require_lease(&req.vault_id)?;
                if self.vault().is_some() {
                    return Err(AuthorityError::InvalidArgument {
                        message: "synthetic vault already open; close before creating encrypted"
                            .to_owned(),
                    });
                }
                let mut slot = self.encrypted();
                if slot.is_some() {
                    return Err(AuthorityError::InvalidArgument {
                        message: "encrypted vault already open".to_owned(),
                    });
                }
                let holder = self
                    .leases
                    .holder(&req.vault_id)
                    .ok_or(AuthorityError::LeaseRequired)?;
                let (vault, codes) = EncryptedVault::create(
                    req.vault_id.as_str(),
                    std::path::Path::new(&vault_root),
                    &passphrase,
                    holder.as_str(),
                    None,
                )
                .map_err(enc_err)?;
                // Spec 035: empty authority snapshot for new encrypted vault.
                {
                    let store = self.store();
                    super::durable::sync_store_to_encrypted(&vault, &store)?;
                }
                *slot = Some(vault);
                Ok(ResponseBody::EncryptedVaultReady {
                    vault_root,
                    recovery_codes: Some(codes.codes),
                })
            }
            RequestBody::OpenEncryptedVault {
                vault_root,
                passphrase,
                recovery_code,
            } => {
                self.require_lease(&req.vault_id)?;
                if self.vault().is_some() {
                    return Err(AuthorityError::InvalidArgument {
                        message: "synthetic vault already open; close before opening encrypted"
                            .to_owned(),
                    });
                }
                let mut slot = self.encrypted();
                if slot.is_some() {
                    return Err(AuthorityError::InvalidArgument {
                        message: "encrypted vault already open".to_owned(),
                    });
                }
                let holder = self
                    .leases
                    .holder(&req.vault_id)
                    .ok_or(AuthorityError::LeaseRequired)?;
                let path = std::path::Path::new(&vault_root);
                let vault = if let Some(code) = recovery_code {
                    EncryptedVault::open_with_recovery(path, &code, holder.as_str())
                } else if let Some(pw) = passphrase {
                    EncryptedVault::open_with_passphrase(path, &pw, holder.as_str())
                } else {
                    return Err(AuthorityError::MissingKeyMaterial);
                }
                .map_err(enc_err)?;
                {
                    let mut store = self.store();
                    super::durable::load_store_from_encrypted(&vault, &mut store)?;
                }
                *slot = Some(vault);
                Ok(ResponseBody::EncryptedVaultReady {
                    vault_root,
                    recovery_codes: None,
                })
            }
            RequestBody::CloseEncryptedVault => {
                self.require_lease(&req.vault_id)?;
                {
                    let enc = self.encrypted();
                    if let Some(vault) = enc.as_ref() {
                        let store = self.store();
                        super::durable::sync_store_to_encrypted(vault, &store)?;
                    }
                }
                let mut slot = self.encrypted();
                if let Some(vault) = slot.take() {
                    vault.close().map_err(enc_err)?;
                }
                Ok(ResponseBody::VaultClosed)
            }
            RequestBody::SetEgressAllowlist { entries } => {
                self.require_lease(&req.vault_id)?;
                let count = entries.len() as u32;
                *self.allowlist() = entries;
                Ok(ResponseBody::AllowlistSet { entries: count })
            }
            RequestBody::NetworkBrokerInvoke { request } => {
                self.require_lease(&req.vault_id)?;
                let allowlist = self.allowlist().clone();
                let outcome =
                    medscale_network::broker_invoke(&allowlist, &request, &FixtureTransport);
                let mut store = self.store();
                let audit_id = store.alloc_id("audit");
                let action = match outcome.decision {
                    medscale_contracts::network::BrokerDecision::Allow => "network_broker.attempt",
                    medscale_contracts::network::BrokerDecision::Deny => "network_broker.deny",
                };
                store.insert(StoredObject::Audit(ActionAuditRecord {
                    header: ObjectHeader {
                        id: audit_id.clone(),
                        schema_version: AUTHORITY_SCHEMA_VERSION,
                        realm_id: req.realm_id.clone(),
                        authority_scope_id: req.authority_scope_id.clone(),
                    },
                    kind: ActionAuditKind::Audit,
                    actor: OpaqueId::new("network-broker"),
                    action: action.to_owned(),
                    target_refs: vec![],
                    effect_state: None,
                    payload_digest: request.body_digest.clone(),
                    detail: Some(serde_json::json!({
                        "host": request.destination_host,
                        "path": request.destination_path,
                        "purpose": request.purpose,
                        "data_class": request.data_class,
                        "decision": outcome.decision,
                        "reason": outcome.reason,
                        "transport_sent": outcome.transport_sent,
                    })),
                }));
                let evaluation_id = if matches!(
                    request.purpose,
                    medscale_contracts::network::EgressPurpose::ProfileOracleFixture
                        | medscale_contracts::network::EgressPurpose::ConformanceEvidenceAttach
                        | medscale_contracts::network::EgressPurpose::IntegrityCheck
                ) {
                    let eval_id = store.alloc_id("eval");
                    store.insert(StoredObject::Evaluation(EvaluationRecord {
                        header: ObjectHeader {
                            id: eval_id.clone(),
                            schema_version: AUTHORITY_SCHEMA_VERSION,
                            realm_id: req.realm_id,
                            authority_scope_id: req.authority_scope_id,
                        },
                        target_refs: vec![audit_id.clone()],
                        evaluator: "medscale.network.fixture_oracle.v1".to_owned(),
                        result: serde_json::json!({
                            "evidence_only": true,
                            "reason": outcome.reason,
                            "fixture": outcome.fixture_body,
                        }),
                        evidence_only: true,
                    }));
                    Some(eval_id)
                } else {
                    None
                };
                Ok(ResponseBody::NetworkBroker {
                    result: medscale_contracts::network::NetworkBrokerResult {
                        decision: outcome.decision,
                        reason: outcome.reason,
                        audit_id,
                        evaluation_id,
                        transport_sent: outcome.transport_sent,
                        fixture_body: outcome.fixture_body,
                    },
                })
            }
            RequestBody::PacksInstallLocal { local_path } => {
                self.require_lease(&req.vault_id)?;
                let mut store = self.store();
                let audit_id = store.alloc_id("audit");
                match medscale_pack::admit_pack_dir(std::path::Path::new(&local_path)) {
                    Ok(manifest) => {
                        let pack_id = manifest.pack_id.clone();
                        if let Err(err) = self.packs().admit(manifest) {
                            let reason = err.reason();
                            store.insert(StoredObject::Audit(ActionAuditRecord {
                                header: ObjectHeader {
                                    id: audit_id.clone(),
                                    schema_version: AUTHORITY_SCHEMA_VERSION,
                                    realm_id: req.realm_id,
                                    authority_scope_id: req.authority_scope_id,
                                },
                                kind: ActionAuditKind::Audit,
                                actor: OpaqueId::new("pack-admit"),
                                action: "packs.install_local.deny".to_owned(),
                                target_refs: vec![],
                                effect_state: None,
                                payload_digest: None,
                                detail: Some(serde_json::json!({
                                    "local_path": local_path,
                                    "reason": reason,
                                })),
                            }));
                            return Ok(ResponseBody::PackAdmit {
                                result: medscale_contracts::packs::PackAdmitResult {
                                    admitted: false,
                                    reason,
                                    pack_id: None,
                                    audit_id,
                                },
                            });
                        }
                        store.insert(StoredObject::Audit(ActionAuditRecord {
                            header: ObjectHeader {
                                id: audit_id.clone(),
                                schema_version: AUTHORITY_SCHEMA_VERSION,
                                realm_id: req.realm_id,
                                authority_scope_id: req.authority_scope_id,
                            },
                            kind: ActionAuditKind::Audit,
                            actor: OpaqueId::new("pack-admit"),
                            action: "packs.install_local".to_owned(),
                            target_refs: vec![pack_id.clone()],
                            effect_state: None,
                            payload_digest: None,
                            detail: Some(serde_json::json!({
                                "local_path": local_path,
                                "admitted": true
                            })),
                        }));
                        Ok(ResponseBody::PackAdmit {
                            result: medscale_contracts::packs::PackAdmitResult {
                                admitted: true,
                                reason: medscale_contracts::packs::PackAdmitReason::Ok,
                                pack_id: Some(pack_id),
                                audit_id,
                            },
                        })
                    }
                    Err(err) => {
                        let reason = err.reason();
                        store.insert(StoredObject::Audit(ActionAuditRecord {
                            header: ObjectHeader {
                                id: audit_id.clone(),
                                schema_version: AUTHORITY_SCHEMA_VERSION,
                                realm_id: req.realm_id,
                                authority_scope_id: req.authority_scope_id,
                            },
                            kind: ActionAuditKind::Audit,
                            actor: OpaqueId::new("pack-admit"),
                            action: "packs.install_local.deny".to_owned(),
                            target_refs: vec![],
                            effect_state: None,
                            payload_digest: None,
                            detail: Some(serde_json::json!({
                                "local_path": local_path,
                                "reason": reason,
                            })),
                        }));
                        Ok(ResponseBody::PackAdmit {
                            result: medscale_contracts::packs::PackAdmitResult {
                                admitted: false,
                                reason,
                                pack_id: None,
                                audit_id,
                            },
                        })
                    }
                }
            }
            RequestBody::PacksList => {
                self.require_lease(&req.vault_id)?;
                Ok(ResponseBody::PackList {
                    packs: self.packs().list(),
                })
            }
            RequestBody::PacksPromote { pack_id, to } => {
                self.require_lease(&req.vault_id)?;
                match self.packs().promote(&pack_id, to) {
                    Ok(m) => Ok(ResponseBody::PackPromoted {
                        pack_id: m.pack_id,
                        state: m.promotion_state,
                    }),
                    Err(msg) => Err(AuthorityError::InvalidArgument {
                        message: msg.to_owned(),
                    }),
                }
            }
            RequestBody::PacksEvaluateLocal { request } => {
                self.require_lease(&req.vault_id)?;
                if !request.synthetic_only {
                    return Err(AuthorityError::ExternalGateRequired {
                        gate: "REAL_PHI_MODEL_RUNTIME".to_owned(),
                    });
                }
                let path = std::path::Path::new(&request.local_path);
                let manifest = medscale_pack::admit_pack_dir(path).map_err(|err| {
                    AuthorityError::InvalidArgument {
                        message: format!("pack evaluation admission failed: {err}"),
                    }
                })?;
                if manifest.pack_id != request.pack_id {
                    return Err(AuthorityError::DigestMismatch);
                }
                let stored = self
                    .packs()
                    .get(&request.pack_id)
                    .ok_or(AuthorityError::NotFound)?;
                if stored.content_digest != manifest.content_digest
                    || stored.pack_epoch != manifest.pack_epoch
                    || stored.version != manifest.version
                {
                    return Err(AuthorityError::DigestMismatch);
                }
                let max_tokens = usize::try_from(request.max_tokens).map_err(|_| {
                    AuthorityError::InvalidArgument {
                        message: "max_tokens is not representable".to_owned(),
                    }
                })?;
                let runtime =
                    medscale_pack::OnnxTokenClassifierRuntime::new(max_tokens).map_err(|err| {
                        AuthorityError::InvalidArgument {
                            message: format!("pack runtime configuration denied: {err}"),
                        }
                    })?;
                let digest = manifest.content_digest.clone();
                let cached = self
                    .prepared_models()
                    .get(&digest)
                    .filter(|prepared| prepared.matches_runtime_contract(&manifest))
                    .cloned();
                let (prepared, prepared_cache_hit) = if let Some(prepared) = cached {
                    if prepared.fixed_sequence_length() > max_tokens {
                        return Err(AuthorityError::InvalidArgument {
                            message:
                                "pack runtime token ceiling is below the prepared model requirement"
                                    .to_owned(),
                        });
                    }
                    (prepared, true)
                } else {
                    // Preparation can be expensive. Never hold the shared cache mutex while
                    // parsing/optimizing model bytes. Re-check after preparation to handle a
                    // concurrent request that may have populated the same digest.
                    let candidate = Arc::new(runtime.prepare(path, &manifest).map_err(|err| {
                        AuthorityError::InvalidArgument {
                            message: format!("pack runtime preparation failed: {err}"),
                        }
                    })?);
                    let mut cache = self.prepared_models();
                    if let Some(existing) = cache
                        .get(&digest)
                        .filter(|prepared| prepared.matches_runtime_contract(&manifest))
                        .cloned()
                    {
                        (existing, true)
                    } else {
                        if cache.len() >= MAX_PREPARED_MODEL_CACHE
                            && let Some(evict) = cache.keys().next().cloned()
                        {
                            cache.remove(&evict);
                        }
                        cache.insert(digest, Arc::clone(&candidate));
                        (candidate, false)
                    }
                };
                let evaluation =
                    prepared
                        .run(&manifest.pack_id, &request.input)
                        .map_err(|err| AuthorityError::InvalidArgument {
                            message: format!("pack runtime evaluation failed: {err}"),
                        })?;
                let mut store = self.store();
                let audit_id = store.alloc_id("audit");
                store.insert(StoredObject::Audit(ActionAuditRecord {
                    header: ObjectHeader {
                        id: audit_id.clone(),
                        schema_version: AUTHORITY_SCHEMA_VERSION,
                        realm_id: req.realm_id,
                        authority_scope_id: req.authority_scope_id,
                    },
                    kind: ActionAuditKind::Audit,
                    actor: OpaqueId::new("pack-runtime"),
                    action: "packs.evaluate_local".to_owned(),
                    target_refs: vec![manifest.pack_id.clone()],
                    effect_state: None,
                    payload_digest: None,
                    detail: Some(serde_json::json!({
                        "runtime": medscale_pack::ONNX_TOKEN_CLASSIFIER_RUNTIME_ID,
                        "input_bytes": request.input.len(),
                        "evidence_only": true,
                        "synthetic_only": true,
                        "prepared_cache_hit": prepared_cache_hit,
                    })),
                }));
                Ok(ResponseBody::PackEvaluation {
                    result: medscale_contracts::packs::PackEvaluationResult {
                        pack_id: evaluation.output.pack_id,
                        content_digest: manifest.content_digest,
                        runtime_id: medscale_pack::ONNX_TOKEN_CLASSIFIER_RUNTIME_ID.to_owned(),
                        prepared_cache_hit,
                        evidence_only: evaluation.output.evidence_only,
                        proposal_payload: evaluation.output.proposal_payload,
                        provenance: evaluation.provenance,
                        audit_id,
                    },
                })
            }
            RequestBody::DocumentIntake { request } => {
                self.require_lease(&req.vault_id)?;
                let mut store = self.store();
                Ok(ResponseBody::DocumentIntake {
                    result: super::document_ops::intake(
                        &mut store,
                        req.realm_id,
                        req.authority_scope_id,
                        request,
                    )?,
                })
            }
            RequestBody::OcrStub { request } => {
                self.require_lease(&req.vault_id)?;
                let mut store = self.store();
                Ok(ResponseBody::MediaStub {
                    result: super::document_ops::ocr_stub(
                        &mut store,
                        req.realm_id,
                        req.authority_scope_id,
                        request,
                    )?,
                })
            }
            RequestBody::AsrStub { request } => {
                self.require_lease(&req.vault_id)?;
                let mut store = self.store();
                Ok(ResponseBody::MediaStub {
                    result: super::document_ops::asr_stub(
                        &mut store,
                        req.realm_id,
                        req.authority_scope_id,
                        request,
                    )?,
                })
            }
            RequestBody::RetrieveLexical { request } => {
                self.require_lease(&req.vault_id)?;
                let mut store = self.store();
                Ok(ResponseBody::LexicalRetrieve {
                    result: super::retrieval::retrieve_lexical(
                        &mut store,
                        req.realm_id,
                        req.authority_scope_id,
                        request,
                    )?,
                })
            }
            RequestBody::CreateExternalActionIntent { request } => {
                let mut store = self.store();
                let id = store.alloc_id("intent");
                let audit = ActionAuditRecord {
                    header: ObjectHeader {
                        id: id.clone(),
                        schema_version: AUTHORITY_SCHEMA_VERSION,
                        realm_id: req.realm_id,
                        authority_scope_id: req.authority_scope_id,
                    },
                    kind: ActionAuditKind::ExternalActionIntent,
                    actor: request.actor,
                    action: request.action,
                    target_refs: request.target_refs,
                    effect_state: Some(EffectState::Pending),
                    payload_digest: Some(request.payload_digest),
                    detail: None,
                };
                store.insert(StoredObject::Audit(audit));
                Ok(ResponseBody::Created { object_id: id })
            }
            RequestBody::ListOutbox => {
                let store = self.store();
                let entries = store
                    .list_external_action_intents(&req.realm_id, &req.authority_scope_id)
                    .into_iter()
                    .filter_map(|a| {
                        let digest = a.payload_digest.clone()?;
                        Some(medscale_contracts::actions::OutboxEntry {
                            action_id: a.header.id.clone(),
                            action: a.action.clone(),
                            effect_state: a.effect_state.unwrap_or(EffectState::Pending),
                            payload_digest: digest,
                        })
                    })
                    .collect();
                Ok(ResponseBody::Outbox { entries })
            }
            RequestBody::NphiesInvoke { request: _ } => Err(AuthorityError::ExternalGateRequired {
                gate: "SPEC_014_WORKFLOW_EVIDENCE".to_owned(),
            }),
            RequestBody::OnlinePackAcquire { request } => {
                if !request.broker_required {
                    return Err(AuthorityError::InvalidArgument {
                        message: "online pack acquire requires broker_required=true".to_owned(),
                    });
                }
                Err(AuthorityError::ExternalGateRequired {
                    gate: "HF_ONLINE_PACK_DISTRIBUTION".to_owned(),
                })
            }
            RequestBody::MescArtifactAdmit { request } => {
                if !request.pack_path_required {
                    return Err(AuthorityError::InvalidArgument {
                        message: "MESC admit requires pack_path_required=true (ARTIFACT_IMPORT)"
                            .to_owned(),
                    });
                }
                Err(AuthorityError::ExternalGateRequired {
                    gate: "MESC_RELEASED_ARTIFACT".to_owned(),
                })
            }
            RequestBody::MescArtifactVerify { request } => {
                if request.release_dir.trim().is_empty() {
                    return Err(AuthorityError::InvalidArgument {
                        message: "MESC verify requires non-empty release_dir".to_owned(),
                    });
                }
                let path = std::path::Path::new(&request.release_dir);
                match medscale_pack::verify_mesc_release_dir(path) {
                    Ok(report) => {
                        let producer = report
                            .producer_id
                            .clone()
                            .unwrap_or_else(|| "unknown".to_owned());
                        let epoch = report.epoch.unwrap_or(0);
                        if let Err(e) = self.mesc_epochs().admit_epoch(&producer, epoch) {
                            return Ok(ResponseBody::MescVerify {
                                report: e.report().clone(),
                            });
                        }
                        debug_assert!(!report.product_admit_authorized);
                        Ok(ResponseBody::MescVerify { report })
                    }
                    Err(e) => Ok(ResponseBody::MescVerify {
                        report: e.report().clone(),
                    }),
                }
            }
            RequestBody::GetFhirSupportMatrix => Ok(ResponseBody::FhirSupportMatrix {
                matrix: medscale_contracts::fhir::FhirSupportMatrix::trusted_v1_ready_base(),
            }),
            RequestBody::ExportFhirLossAware { resource } => {
                Ok(ResponseBody::FhirLossAwareExport {
                    export: medscale_contracts::fhir::loss_aware_export(&resource),
                })
            }
            RequestBody::RejectProposal {
                proposal_id,
                actor,
                rationale,
            } => {
                self.require_lease(&req.vault_id)?;
                let mut store = self.store();
                let _proposal = store
                    .get_proposal(&proposal_id, &req.realm_id, &req.authority_scope_id)
                    .map_err(scope_err)?;
                let audit_id = store.alloc_id("audit");
                let audit = ActionAuditRecord {
                    header: ObjectHeader {
                        id: audit_id.clone(),
                        schema_version: AUTHORITY_SCHEMA_VERSION,
                        realm_id: req.realm_id,
                        authority_scope_id: req.authority_scope_id,
                    },
                    kind: ActionAuditKind::Audit,
                    actor,
                    action: medscale_contracts::workflow::PROPOSAL_REJECT_ACTION.to_owned(),
                    target_refs: vec![proposal_id.clone()],
                    effect_state: None,
                    payload_digest: None,
                    detail: Some(serde_json::json!({ "rationale": rationale })),
                };
                store.insert(StoredObject::Audit(audit));
                Ok(ResponseBody::Rejected {
                    proposal_id,
                    audit_id,
                })
            }
            RequestBody::AppendDisclosure {
                purpose,
                scope,
                subject_ref,
                artifact_refs,
                export_digest,
                note,
            } => {
                self.require_lease(&req.vault_id)?;
                let mut store = self.store();
                let disclosure_id = store.alloc_id("disclosure");
                let record = medscale_contracts::workflow::DisclosureRecord {
                    disclosure_id: disclosure_id.clone(),
                    purpose: purpose.clone(),
                    scope: scope.clone(),
                    subject_ref: subject_ref.clone(),
                    artifact_refs: artifact_refs.clone(),
                    export_digest: export_digest.clone(),
                    synthetic_only: true,
                    release_ready_claimed: false,
                    note: note.clone(),
                };
                let audit = ActionAuditRecord {
                    header: ObjectHeader {
                        id: disclosure_id,
                        schema_version: AUTHORITY_SCHEMA_VERSION,
                        realm_id: req.realm_id,
                        authority_scope_id: req.authority_scope_id,
                    },
                    kind: ActionAuditKind::Audit,
                    actor: OpaqueId::new("workflow-operator"),
                    action: medscale_contracts::workflow::DISCLOSURE_APPEND_ACTION.to_owned(),
                    target_refs: artifact_refs,
                    effect_state: None,
                    payload_digest: export_digest,
                    detail: Some(serde_json::to_value(&record).unwrap_or(serde_json::Value::Null)),
                };
                store.insert(StoredObject::Audit(audit));
                Ok(ResponseBody::DisclosureAppended { record })
            }
            RequestBody::ListDisclosures => {
                self.require_lease(&req.vault_id)?;
                let store = self.store();
                let records = store.list_disclosures(&req.realm_id, &req.authority_scope_id);
                Ok(ResponseBody::DisclosureList { records })
            }
            // Spec 074 project graph: every arm runs through `pg` (lease +
            // session gate already enforced above + meta + audit). No arm
            // touches storage except through `ProjectGraph` methods.
            RequestBody::ProjectCreate { name, description } => {
                let project = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut pg| pg.create_project(name, description),
                )?;
                Ok(ResponseBody::Project { project })
            }
            RequestBody::ProjectGet { project_id } => {
                let project = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |pg| pg.scoped_project_pub(&project_id),
                )?;
                Ok(ResponseBody::Project { project })
            }
            RequestBody::ProjectList {
                status,
                limit,
                cursor,
            } => {
                let (projects, next_cursor) = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |pg| pg.list_projects(status, limit, cursor),
                )?;
                Ok(ResponseBody::ProjectList {
                    projects,
                    next_cursor,
                })
            }
            RequestBody::ProjectUpdate {
                project_id,
                expected_revision,
                name,
                description,
            } => {
                let project = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut pg| pg.update_project(&project_id, expected_revision, name, description),
                )?;
                Ok(ResponseBody::Project { project })
            }
            RequestBody::ProjectArchive {
                project_id,
                expected_revision,
            } => {
                let project = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut pg| pg.archive_project(&project_id, expected_revision),
                )?;
                Ok(ResponseBody::Project { project })
            }
            RequestBody::ProjectRestore {
                project_id,
                expected_revision,
            } => {
                let project = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut pg| pg.restore_project(&project_id, expected_revision),
                )?;
                Ok(ResponseBody::Project { project })
            }
            RequestBody::ExperimentCreate {
                project_id,
                name,
                description,
            } => {
                let experiment = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut pg| pg.create_experiment(&project_id, name, description),
                )?;
                Ok(ResponseBody::Experiment { experiment })
            }
            RequestBody::ExperimentGet { experiment_id } => {
                let experiment = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |pg| pg.scoped_experiment_pub(&experiment_id),
                )?;
                Ok(ResponseBody::Experiment { experiment })
            }
            RequestBody::ExperimentList {
                project_id,
                limit,
                cursor,
            } => {
                let (experiments, next_cursor) = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |pg| pg.list_experiments(&project_id, limit, cursor),
                )?;
                Ok(ResponseBody::ExperimentList {
                    experiments,
                    next_cursor,
                })
            }
            RequestBody::ExperimentUpdate {
                experiment_id,
                expected_revision,
                name,
                description,
            } => {
                let experiment = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut pg| {
                        pg.update_experiment(&experiment_id, expected_revision, name, description)
                    },
                )?;
                Ok(ResponseBody::Experiment { experiment })
            }
            RequestBody::ExperimentArchive {
                experiment_id,
                expected_revision,
            } => {
                let experiment = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut pg| pg.archive_experiment(&experiment_id, expected_revision),
                )?;
                Ok(ResponseBody::Experiment { experiment })
            }
            RequestBody::ProjectAttach {
                project_id,
                experiment_id,
                artifact,
            } => {
                let reference = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut pg| pg.attach(&project_id, experiment_id, artifact),
                )?;
                Ok(ResponseBody::ProjectRef { reference })
            }
            RequestBody::ProjectDetach {
                ref_id,
                expected_revision,
            } => {
                let reference = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut pg| pg.detach(&ref_id, expected_revision),
                )?;
                Ok(ResponseBody::ProjectRef { reference })
            }
            RequestBody::ProjectListRefs {
                project_id,
                experiment_id,
                active_only,
                limit,
                cursor,
            } => {
                let (refs, next_cursor) = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |pg| pg.list_refs(&project_id, experiment_id, active_only, limit, cursor),
                )?;
                Ok(ResponseBody::ProjectRefList { refs, next_cursor })
            }
            RequestBody::GraphEdgeCreate {
                project_id,
                subject,
                predicate,
                object,
            } => {
                let edge = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut pg| pg.create_edge(&project_id, subject, predicate, object),
                )?;
                Ok(ResponseBody::GraphEdge { edge })
            }
            RequestBody::GraphEdgeRemove {
                edge_id,
                expected_revision,
            } => {
                let edge = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut pg| pg.remove_edge(&edge_id, expected_revision),
                )?;
                Ok(ResponseBody::GraphEdge { edge })
            }
            RequestBody::GraphNeighbors {
                project_id,
                start,
                predicates,
                direction,
                limit,
                cursor,
            } => {
                let page = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |pg| pg.neighbors(&project_id, start, predicates, direction, limit, cursor),
                )?;
                Ok(ResponseBody::GraphNeighbors { page })
            }
            RequestBody::ProjectContextResolve {
                project_id,
                experiment_id,
                refs_limit,
                graph_limit,
            } => {
                let context = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |pg| pg.context(&project_id, experiment_id, refs_limit, graph_limit),
                )?;
                Ok(ResponseBody::ProjectContext { context })
            }
            RequestBody::ProjectSummaryQuery { project_id } => {
                let summary = self.pg(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |pg| pg.project_summary(&project_id),
                )?;
                Ok(ResponseBody::ProjectSummary { summary })
            }
            // Spec 075 data source fabric: every arm runs through `ds`
            // (lease + session gate already enforced above + meta + blobs).
            // No arm touches storage, drivers, or transports except through
            // `DataSources` methods.
            RequestBody::DataSourceCreate {
                project_id,
                display_name,
                locator,
                credential_ref,
            } => {
                let source = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut ds| ds.create_source(project_id, display_name, locator, credential_ref),
                )?;
                Ok(ResponseBody::DataSource {
                    source: Box::new(source),
                })
            }
            RequestBody::DataSourceGet { source_id } => {
                let source = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |ds| ds.get_source(&source_id),
                )?;
                Ok(ResponseBody::DataSource {
                    source: Box::new(source),
                })
            }
            RequestBody::DataSourceList {
                project_id,
                limit,
                cursor,
            } => {
                let (sources, next_cursor) = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |ds| ds.list_sources(&project_id, limit, &cursor),
                )?;
                Ok(ResponseBody::DataSourceList {
                    sources,
                    next_cursor,
                })
            }
            RequestBody::DataSourceUpdate {
                source_id,
                expected_revision,
                display_name,
                credential_ref,
            } => {
                let source = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut ds| {
                        ds.update_source(
                            &source_id,
                            expected_revision,
                            display_name,
                            credential_ref,
                        )
                    },
                )?;
                Ok(ResponseBody::DataSource {
                    source: Box::new(source),
                })
            }
            RequestBody::DataSourceArchive {
                source_id,
                expected_revision,
            } => {
                let source = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut ds| ds.archive_source(&source_id, expected_revision),
                )?;
                Ok(ResponseBody::DataSource {
                    source: Box::new(source),
                })
            }
            RequestBody::SnapshotImport { source_id } => {
                let (snapshot, receipt) = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut ds| ds.import_source(&source_id),
                )?;
                Ok(ResponseBody::SnapshotImported {
                    snapshot: Box::new(snapshot.snapshot),
                    receipt,
                })
            }
            RequestBody::SnapshotPreview {
                source_id,
                max_rows,
            } => {
                let (schema, rows) = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |ds| ds.preview_source(&source_id, max_rows),
                )?;
                Ok(ResponseBody::SnapshotPreview { schema, rows })
            }
            RequestBody::SnapshotGet { snapshot_id } => {
                let record = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |ds| ds.get_snapshot(&snapshot_id),
                )?;
                Ok(ResponseBody::Snapshot {
                    snapshot: Box::new(record.snapshot),
                })
            }
            RequestBody::SnapshotList {
                source_id,
                limit,
                cursor,
            } => {
                let (snapshots, next_cursor) = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |ds| ds.list_snapshots(&source_id, limit, &cursor),
                )?;
                Ok(ResponseBody::SnapshotList {
                    snapshots,
                    next_cursor,
                })
            }
            RequestBody::SnapshotRows {
                snapshot_id,
                limit,
                cursor,
                filters,
                sort,
            } => {
                let page = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |ds| ds.snapshot_rows(&snapshot_id, limit, &cursor, &filters, &sort),
                )?;
                Ok(ResponseBody::SnapshotRows { page })
            }
            RequestBody::SnapshotRefresh {
                source_id,
                allow_schema_change,
            } => {
                let (receipt, record) = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut ds| ds.refresh_source(&source_id, allow_schema_change),
                )?;
                Ok(ResponseBody::SnapshotRefreshed {
                    receipt,
                    snapshot: record.map(|record| Box::new(record.snapshot)),
                })
            }
            RequestBody::SavedViewCreate {
                snapshot_id,
                view_kind,
                state,
            } => {
                let view = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut ds| ds.create_view(snapshot_id, view_kind, state),
                )?;
                Ok(ResponseBody::SavedView {
                    view: Box::new(view),
                })
            }
            RequestBody::SavedViewGet { view_id } => {
                let view = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |ds| ds.get_view(&view_id),
                )?;
                Ok(ResponseBody::SavedView {
                    view: Box::new(view),
                })
            }
            RequestBody::SavedViewList {
                snapshot_id,
                limit,
                cursor,
            } => {
                let (views, next_cursor) = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |ds| ds.list_views(&snapshot_id, limit, &cursor),
                )?;
                Ok(ResponseBody::SavedViewList { views, next_cursor })
            }
            RequestBody::SavedViewUpdate {
                view_id,
                expected_revision,
                state,
            } => {
                let view = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut ds| ds.update_view(&view_id, expected_revision, state),
                )?;
                Ok(ResponseBody::SavedView {
                    view: Box::new(view),
                })
            }
            RequestBody::TransformExecute {
                input_snapshot_ids,
                ops,
            } => {
                let (snapshot, receipt) = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut ds| ds.execute_transformation(input_snapshot_ids, ops),
                )?;
                Ok(ResponseBody::Transformed {
                    snapshot: Box::new(snapshot.snapshot),
                    receipt,
                })
            }
            RequestBody::DatasetReleaseCreate {
                snapshot_id,
                version,
                split_group,
                annotation_schema_ref,
                rights_state,
            } => {
                let release = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut ds| {
                        ds.create_release(
                            snapshot_id,
                            version,
                            split_group,
                            annotation_schema_ref,
                            rights_state,
                        )
                    },
                )?;
                Ok(ResponseBody::DatasetRelease {
                    release: Box::new(release),
                })
            }
            RequestBody::DatasetReleaseGet { release_id } => {
                let release = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |ds| ds.get_release(&release_id),
                )?;
                Ok(ResponseBody::DatasetRelease {
                    release: Box::new(release),
                })
            }
            RequestBody::DatasetReleaseList {
                project_id,
                limit,
                cursor,
            } => {
                let (releases, next_cursor) = self.ds(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |ds| ds.list_releases(&project_id, limit, &cursor),
                )?;
                Ok(ResponseBody::DatasetReleaseList {
                    releases,
                    next_cursor,
                })
            }
            // Spec 076 Collaboration Substrate: every mutation flows through
            // Core authority paths. Surfaces never write collaboration
            // storage directly. T076-03 slice only (participant/room/
            // membership); thread/message/task/note/approval/activity land
            // in later slices.
            RequestBody::ParticipantRegister {
                holder_id,
                kind,
                display_name,
                agent_profile_ref,
            } => {
                let participant = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut collab| {
                        collab.register_participant(
                            holder_id,
                            kind,
                            display_name,
                            agent_profile_ref,
                        )
                    },
                )?;
                Ok(ResponseBody::Participant {
                    participant: Box::new(participant),
                })
            }
            RequestBody::ParticipantGet { participant_id } => {
                let participant = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.get_participant(&participant_id),
                )?;
                Ok(ResponseBody::Participant {
                    participant: Box::new(participant),
                })
            }
            RequestBody::ParticipantRevoke {
                participant_id,
                expected_revision,
            } => {
                let participant = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut collab| collab.revoke_participant(&participant_id, expected_revision),
                )?;
                Ok(ResponseBody::Participant {
                    participant: Box::new(participant),
                })
            }
            RequestBody::RoomCreate {
                project_id,
                experiment_id,
                name,
            } => {
                let room = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |mut collab| collab.create_room(project_id, experiment_id, name),
                )?;
                Ok(ResponseBody::CollabRoom {
                    room: Box::new(room),
                })
            }
            RequestBody::RoomGet { room_id } => {
                let room = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.get_room(&room_id),
                )?;
                Ok(ResponseBody::CollabRoom {
                    room: Box::new(room),
                })
            }
            RequestBody::RoomList {
                project_id,
                status,
                limit,
                cursor,
            } => {
                let (rooms, next_cursor) = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.list_rooms(&project_id, status, limit.unwrap_or(25), cursor),
                )?;
                Ok(ResponseBody::CollabRoomList { rooms, next_cursor })
            }
            RequestBody::RoomRename {
                room_id,
                expected_revision,
                name,
            } => {
                let room = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.rename_room(&room_id, expected_revision, name),
                )?;
                Ok(ResponseBody::CollabRoom {
                    room: Box::new(room),
                })
            }
            RequestBody::RoomArchive {
                room_id,
                expected_revision,
            } => {
                let room = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.archive_room(&room_id, expected_revision),
                )?;
                Ok(ResponseBody::CollabRoom {
                    room: Box::new(room),
                })
            }
            RequestBody::RoomMembershipAdd {
                room_id,
                participant_id,
                role,
            } => {
                let membership = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.add_membership(&room_id, &participant_id, role),
                )?;
                Ok(ResponseBody::CollabMembership {
                    membership: Box::new(membership),
                })
            }
            RequestBody::RoomMembershipList { room_id } => {
                let memberships = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.list_memberships(&room_id),
                )?;
                Ok(ResponseBody::CollabMembershipList { memberships })
            }
            RequestBody::RoomMembershipRemove {
                room_id,
                membership_id,
                expected_revision,
            } => {
                let membership = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.remove_membership(&room_id, &membership_id, expected_revision),
                )?;
                Ok(ResponseBody::CollabMembership {
                    membership: Box::new(membership),
                })
            }
            // Spec 076 T076-04/05/06 slice: thread, message, task.
            RequestBody::ThreadOpen { room_id, anchor } => {
                let thread = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.open_thread(room_id, anchor),
                )?;
                Ok(ResponseBody::CollabThread {
                    thread: Box::new(thread),
                })
            }
            RequestBody::ThreadGet { thread_id } => {
                let thread = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.get_thread(&thread_id),
                )?;
                Ok(ResponseBody::CollabThread {
                    thread: Box::new(thread),
                })
            }
            RequestBody::ThreadList {
                room_id,
                limit,
                cursor,
            } => {
                let (threads, next_cursor) = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.list_threads(&room_id, limit.unwrap_or(25), cursor),
                )?;
                Ok(ResponseBody::CollabThreadList {
                    threads,
                    next_cursor,
                })
            }
            RequestBody::ThreadSetStatus {
                thread_id,
                expected_revision,
                status,
            } => {
                let thread = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.set_thread_status(&thread_id, expected_revision, status),
                )?;
                Ok(ResponseBody::CollabThread {
                    thread: Box::new(thread),
                })
            }
            RequestBody::MessagePost { thread_id, body } => {
                let message = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.post_message(&thread_id, body),
                )?;
                Ok(ResponseBody::CollabMessage {
                    message: Box::new(message),
                })
            }
            RequestBody::MessageList {
                thread_id,
                limit,
                after_seq,
            } => {
                let messages = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| {
                        collab.list_messages(
                            &thread_id,
                            limit.unwrap_or(50),
                            after_seq.unwrap_or(0),
                        )
                    },
                )?;
                Ok(ResponseBody::CollabMessageList { messages })
            }
            RequestBody::MessageEditBody {
                message_id,
                new_body,
            } => {
                let edit = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.edit_message(&message_id, new_body),
                )?;
                Ok(ResponseBody::CollabMessageEdit {
                    edit: Box::new(edit),
                })
            }
            RequestBody::MessageDelete { message_id } => {
                let edit = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.delete_message(&message_id),
                )?;
                Ok(ResponseBody::CollabMessageEdit {
                    edit: Box::new(edit),
                })
            }
            RequestBody::TaskCreate {
                room_id,
                anchor,
                title,
                description,
            } => {
                let task = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.create_task(room_id, anchor, title, description),
                )?;
                Ok(ResponseBody::CollabTask {
                    task: Box::new(task),
                })
            }
            RequestBody::TaskGet { task_id } => {
                let task = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.get_task(&task_id),
                )?;
                Ok(ResponseBody::CollabTask {
                    task: Box::new(task),
                })
            }
            RequestBody::TaskList {
                room_id,
                limit,
                cursor,
            } => {
                let (tasks, next_cursor) = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.list_tasks(&room_id, limit.unwrap_or(25), cursor),
                )?;
                Ok(ResponseBody::CollabTaskList { tasks, next_cursor })
            }
            RequestBody::TaskUpdate {
                task_id,
                expected_revision,
                status,
                assignee_participant_id,
            } => {
                let task = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| {
                        collab.update_task(
                            &task_id,
                            expected_revision,
                            status,
                            assignee_participant_id,
                        )
                    },
                )?;
                Ok(ResponseBody::CollabTask {
                    task: Box::new(task),
                })
            }
            // Spec 076 T076-07/08/09 slice: note, approval, activity.
            RequestBody::NoteCreate {
                room_id,
                title,
                body,
            } => {
                let note = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.create_note(room_id, title, body),
                )?;
                Ok(ResponseBody::CollabNote {
                    note: Box::new(note),
                })
            }
            RequestBody::NoteGet { note_id } => {
                let note = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.get_note(&note_id),
                )?;
                Ok(ResponseBody::CollabNote {
                    note: Box::new(note),
                })
            }
            RequestBody::NoteListRevisions { note_id } => {
                let revisions = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.list_note_revisions(&note_id),
                )?;
                Ok(ResponseBody::CollabNoteRevisionList { revisions })
            }
            RequestBody::NoteEdit {
                note_id,
                expected_revision,
                body,
            } => {
                let (note, new_revision, is_conflict_copy) = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.edit_note(&note_id, expected_revision, body),
                )?;
                Ok(ResponseBody::CollabNoteEdit {
                    note: Box::new(note),
                    new_revision: Box::new(new_revision),
                    is_conflict_copy,
                })
            }
            RequestBody::ApprovalRequestCreate {
                room_id,
                anchor,
                kind,
                assignee_participant_ids,
                blind_until_closed,
            } => {
                let request = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| {
                        collab.create_approval_request(
                            room_id,
                            anchor,
                            kind,
                            assignee_participant_ids,
                            blind_until_closed,
                        )
                    },
                )?;
                Ok(ResponseBody::CollabApprovalRequest {
                    request: Box::new(request),
                })
            }
            RequestBody::ApprovalRequestGet { request_id } => {
                let request = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.get_approval_request(&request_id),
                )?;
                Ok(ResponseBody::CollabApprovalRequest {
                    request: Box::new(request),
                })
            }
            RequestBody::ApprovalRequestWithdraw {
                request_id,
                expected_revision,
            } => {
                let request = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.withdraw_approval_request(&request_id, expected_revision),
                )?;
                Ok(ResponseBody::CollabApprovalRequest {
                    request: Box::new(request),
                })
            }
            RequestBody::ApprovalDecide {
                request_id,
                outcome,
                rationale,
            } => {
                let decision = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.decide_approval_request(&request_id, outcome, rationale),
                )?;
                Ok(ResponseBody::CollabApprovalDecision {
                    decision: Box::new(decision),
                })
            }
            RequestBody::ApprovalDecisionList { request_id } => {
                let decisions = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| collab.list_approval_decisions(&request_id),
                )?;
                Ok(ResponseBody::CollabApprovalDecisionList { decisions })
            }
            RequestBody::ActivityList {
                room_id,
                limit,
                after_seq,
            } => {
                let records = self.collab(
                    &req.vault_id,
                    req.realm_id,
                    req.authority_scope_id,
                    req.session_id,
                    |collab| {
                        collab.list_activity(&room_id, limit.unwrap_or(100), after_seq.unwrap_or(0))
                    },
                )?;
                Ok(ResponseBody::CollabActivityList { records })
            }
        }
    }
}

fn blob_lookup(
    vault: Option<&SyntheticVault>,
    digest: &medscale_contracts::objects::DigestSha256,
) -> Option<(Vec<u8>, bool)> {
    let vault = vault?;
    match vault.blobs.get_blob(digest) {
        Ok(bytes) => {
            let ok = vault.blobs.verify(digest, bytes.len() as u64).is_ok();
            Some((bytes, ok))
        }
        Err(_) => None,
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
                Capability::AmendAssertion,
                RequestBody::AmendAssertion { .. }
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
            | (
                Capability::OpenSyntheticVault,
                RequestBody::OpenSyntheticVault { .. }
            )
            | (Capability::CloseVault, RequestBody::CloseVault)
            | (
                Capability::IngestFhirSynthetic,
                RequestBody::IngestFhirSynthetic { .. }
            )
            | (
                Capability::AttachValidatorEvidence,
                RequestBody::AttachValidatorEvidence { .. }
            )
            | (
                Capability::RebuildProjection,
                RequestBody::RebuildProjection { .. }
            )
            | (
                Capability::ReadCanonicalVisibility,
                RequestBody::ReadCanonicalVisibility { .. }
            )
            | (Capability::VerifyBlob, RequestBody::VerifyBlob { .. })
            | (Capability::BackupVault, RequestBody::BackupVault { .. })
            | (Capability::RestoreVault, RequestBody::RestoreVault { .. })
            | (Capability::RunBlobGc, RequestBody::RunBlobGc)
            | (Capability::GetTimeline, RequestBody::GetTimeline { .. })
            | (Capability::GetBrief, RequestBody::GetBrief { .. })
            | (Capability::GetCoverage, RequestBody::GetCoverage { .. })
            | (
                Capability::DrillDownPresentation,
                RequestBody::DrillDownPresentation { .. }
            )
            | (
                Capability::CreateEncryptedVault,
                RequestBody::CreateEncryptedVault { .. }
            )
            | (
                Capability::OpenEncryptedVault,
                RequestBody::OpenEncryptedVault { .. }
            )
            | (
                Capability::CloseEncryptedVault,
                RequestBody::CloseEncryptedVault
            )
            | (
                Capability::NetworkBrokerInvoke,
                RequestBody::NetworkBrokerInvoke { .. }
            )
            | (
                Capability::SetEgressAllowlist,
                RequestBody::SetEgressAllowlist { .. }
            )
            | (
                Capability::PacksInstallLocal,
                RequestBody::PacksInstallLocal { .. }
            )
            | (Capability::PacksList, RequestBody::PacksList)
            | (Capability::PacksPromote, RequestBody::PacksPromote { .. })
            | (
                Capability::PacksEvaluateLocal,
                RequestBody::PacksEvaluateLocal { .. },
            )
            | (
                Capability::DocumentIntake,
                RequestBody::DocumentIntake { .. }
            )
            | (Capability::OcrStub, RequestBody::OcrStub { .. })
            | (Capability::AsrStub, RequestBody::AsrStub { .. })
            | (
                Capability::RetrieveLexical,
                RequestBody::RetrieveLexical { .. }
            )
            | (
                Capability::CreateExternalActionIntent,
                RequestBody::CreateExternalActionIntent { .. }
            )
            | (Capability::ListOutbox, RequestBody::ListOutbox)
            | (Capability::NphiesInvoke, RequestBody::NphiesInvoke { .. })
            | (
                Capability::OnlinePackAcquire,
                RequestBody::OnlinePackAcquire { .. }
            )
            | (
                Capability::MescArtifactAdmit,
                RequestBody::MescArtifactAdmit { .. }
            )
            | (
                Capability::MescArtifactVerify,
                RequestBody::MescArtifactVerify { .. }
            )
            | (Capability::OpenSession, RequestBody::OpenSession { .. })
            | (Capability::RevokeSession, RequestBody::RevokeSession { .. })
            | (
                Capability::GetFhirSupportMatrix,
                RequestBody::GetFhirSupportMatrix
            )
            | (
                Capability::ExportFhirLossAware,
                RequestBody::ExportFhirLossAware { .. }
            )
            | (
                Capability::RejectProposal,
                RequestBody::RejectProposal { .. }
            )
            | (
                Capability::AppendDisclosure,
                RequestBody::AppendDisclosure { .. }
            )
            | (Capability::ListDisclosures, RequestBody::ListDisclosures)
            | (Capability::ProjectCreate, RequestBody::ProjectCreate { .. })
            | (Capability::ProjectRead, RequestBody::ProjectGet { .. })
            | (Capability::ProjectRead, RequestBody::ProjectList { .. })
            | (Capability::ProjectUpdate, RequestBody::ProjectUpdate { .. })
            | (
                Capability::ProjectArchive,
                RequestBody::ProjectArchive { .. }
            )
            | (
                Capability::ProjectArchive,
                RequestBody::ProjectRestore { .. }
            )
            | (
                Capability::ExperimentCreate,
                RequestBody::ExperimentCreate { .. }
            )
            | (
                Capability::ExperimentRead,
                RequestBody::ExperimentGet { .. }
            )
            | (
                Capability::ExperimentRead,
                RequestBody::ExperimentList { .. }
            )
            | (
                Capability::ExperimentUpdate,
                RequestBody::ExperimentUpdate { .. }
            )
            | (
                Capability::ExperimentArchive,
                RequestBody::ExperimentArchive { .. }
            )
            | (
                Capability::ProjectArtifactAttach,
                RequestBody::ProjectAttach { .. }
            )
            | (
                Capability::ProjectArtifactDetach,
                RequestBody::ProjectDetach { .. }
            )
            | (Capability::ProjectRead, RequestBody::ProjectListRefs { .. })
            | (
                Capability::ProjectGraphMutate,
                RequestBody::GraphEdgeCreate { .. }
            )
            | (
                Capability::ProjectGraphMutate,
                RequestBody::GraphEdgeRemove { .. }
            )
            | (
                Capability::ProjectGraphRead,
                RequestBody::GraphNeighbors { .. }
            )
            | (
                Capability::ProjectRead,
                RequestBody::ProjectContextResolve { .. }
            )
            | (
                Capability::ProjectRead,
                RequestBody::ProjectSummaryQuery { .. }
            )
            | (
                Capability::DataSourceCreate,
                RequestBody::DataSourceCreate { .. }
            )
            | (
                Capability::DataSourceRead,
                RequestBody::DataSourceGet { .. }
            )
            | (
                Capability::DataSourceRead,
                RequestBody::DataSourceList { .. }
            )
            | (
                Capability::DataSourceUpdate,
                RequestBody::DataSourceUpdate { .. }
            )
            | (
                Capability::DataSourceArchive,
                RequestBody::DataSourceArchive { .. }
            )
            | (
                Capability::SnapshotImport,
                RequestBody::SnapshotImport { .. }
            )
            | (
                Capability::SnapshotPreview,
                RequestBody::SnapshotPreview { .. }
            )
            | (Capability::SnapshotRead, RequestBody::SnapshotGet { .. })
            | (Capability::SnapshotRead, RequestBody::SnapshotList { .. })
            | (Capability::SnapshotRead, RequestBody::SnapshotRows { .. })
            | (
                Capability::SnapshotRefresh,
                RequestBody::SnapshotRefresh { .. }
            )
            | (
                Capability::SavedViewCreate,
                RequestBody::SavedViewCreate { .. }
            )
            | (Capability::SavedViewRead, RequestBody::SavedViewGet { .. })
            | (Capability::SavedViewRead, RequestBody::SavedViewList { .. })
            | (
                Capability::SavedViewUpdate,
                RequestBody::SavedViewUpdate { .. }
            )
            | (
                Capability::TransformExecute,
                RequestBody::TransformExecute { .. }
            )
            | (
                Capability::DatasetReleaseCreate,
                RequestBody::DatasetReleaseCreate { .. }
            )
            | (
                Capability::DatasetReleaseRead,
                RequestBody::DatasetReleaseGet { .. }
            )
            | (
                Capability::DatasetReleaseRead,
                RequestBody::DatasetReleaseList { .. }
            )
            | (
                Capability::ParticipantRegister,
                RequestBody::ParticipantRegister { .. }
            )
            | (
                Capability::ParticipantRead,
                RequestBody::ParticipantGet { .. }
            )
            | (
                Capability::ParticipantRevoke,
                RequestBody::ParticipantRevoke { .. }
            )
            | (Capability::RoomCreate, RequestBody::RoomCreate { .. })
            | (Capability::RoomRead, RequestBody::RoomGet { .. })
            | (Capability::RoomRead, RequestBody::RoomList { .. })
            | (Capability::RoomUpdate, RequestBody::RoomRename { .. })
            | (Capability::RoomArchive, RequestBody::RoomArchive { .. })
            | (
                Capability::RoomMembershipManage,
                RequestBody::RoomMembershipAdd { .. }
            )
            | (
                Capability::RoomMembershipRead,
                RequestBody::RoomMembershipList { .. }
            )
            | (
                Capability::RoomMembershipManage,
                RequestBody::RoomMembershipRemove { .. }
            )
            | (Capability::ThreadCreate, RequestBody::ThreadOpen { .. })
            | (Capability::ThreadRead, RequestBody::ThreadGet { .. })
            | (Capability::ThreadRead, RequestBody::ThreadList { .. })
            | (
                Capability::ThreadResolve,
                RequestBody::ThreadSetStatus { .. }
            )
            | (Capability::MessagePost, RequestBody::MessagePost { .. })
            | (Capability::MessageRead, RequestBody::MessageList { .. })
            | (Capability::MessageEdit, RequestBody::MessageEditBody { .. })
            | (Capability::MessageEdit, RequestBody::MessageDelete { .. })
            | (Capability::TaskCreate, RequestBody::TaskCreate { .. })
            | (Capability::TaskRead, RequestBody::TaskGet { .. })
            | (Capability::TaskRead, RequestBody::TaskList { .. })
            | (Capability::TaskUpdate, RequestBody::TaskUpdate { .. })
            | (Capability::NoteCreate, RequestBody::NoteCreate { .. })
            | (Capability::NoteRead, RequestBody::NoteGet { .. })
            | (Capability::NoteRead, RequestBody::NoteListRevisions { .. })
            | (Capability::NoteUpdate, RequestBody::NoteEdit { .. })
            | (
                Capability::ApprovalRequestCreate,
                RequestBody::ApprovalRequestCreate { .. }
            )
            | (
                Capability::ApprovalRequestRead,
                RequestBody::ApprovalRequestGet { .. }
            )
            | (
                Capability::ApprovalWithdraw,
                RequestBody::ApprovalRequestWithdraw { .. }
            )
            | (
                Capability::ApprovalDecide,
                RequestBody::ApprovalDecide { .. }
            )
            | (
                Capability::ApprovalRequestRead,
                RequestBody::ApprovalDecisionList { .. }
            )
            | (Capability::ActivityRead, RequestBody::ActivityList { .. })
    )
}

fn enc_err(err: medscale_storage::EncryptedVaultError) -> AuthorityError {
    match err {
        medscale_storage::EncryptedVaultError::Claim(
            medscale_storage::ClaimError::SyncRootRefused(_),
        )
        | medscale_storage::EncryptedVaultError::Claim(medscale_storage::ClaimError::Escape) => {
            AuthorityError::PathOutsideClaim
        }
        medscale_storage::EncryptedVaultError::MissingKeyMaterial => {
            AuthorityError::MissingKeyMaterial
        }
        medscale_storage::EncryptedVaultError::LeaseHeld(holder) => AuthorityError::LeaseHeld {
            holder_id: OpaqueId::new(holder),
        },
        other => AuthorityError::InvalidArgument {
            message: other.to_string(),
        },
    }
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

fn amend_err(err: super::amend::AmendError) -> AuthorityError {
    match err {
        super::amend::AmendError::NotFound => AuthorityError::NotFound,
        super::amend::AmendError::WrongScope => AuthorityError::WrongScope,
        super::amend::AmendError::AlreadySuperseded => AuthorityError::IllegalTransition,
        super::amend::AmendError::InvalidTime(message) => {
            AuthorityError::InvalidArgument { message }
        }
    }
}
