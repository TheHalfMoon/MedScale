//! In-process Core Host authority facade (logical IPC API).

use std::sync::Mutex;

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
use crate::process::{LeaseError, LeaseRegistry, SessionRegistry};

use super::identity::{create_identity_assertion, decide_identity_merge};
use super::ingest_ops;
use super::presentation;
use super::promote::{PromoteError, promote_proposal};
use super::source_ops::create_source_record;
use super::store::{InMemoryAuthorityStore, ScopeError, StoredObject};
use medscale_contracts::network::EgressAllowlistEntry;
use medscale_network::FixtureTransport;
use medscale_storage::{EncryptedVault, SyntheticVault};

/// In-process facade owning lease registry + in-memory store + optional vaults.
#[derive(Debug, Default)]
pub struct CoreFacade {
    leases: LeaseRegistry,
    sessions: SessionRegistry,
    store: Mutex<InMemoryAuthorityStore>,
    vault: Mutex<Option<SyntheticVault>>,
    encrypted: Mutex<Option<EncryptedVault>>,
    allowlist: Mutex<Vec<EgressAllowlistEntry>>,
    packs: Mutex<medscale_pack::PackStore>,
}

impl CoreFacade {
    /// Creates a new facade instance (one per process/host in Spec 002).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// In-process session registry (Spec 018 READY_BASE).
    #[must_use]
    pub fn sessions(&self) -> &SessionRegistry {
        &self.sessions
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
        let result = self.dispatch_inner(req).and_then(|body| {
            self.persist_open_vault()?;
            Ok(body)
        });
        AuthorityResponse {
            schema_version: AUTHORITY_SCHEMA_VERSION,
            request_id,
            result,
        }
    }

    fn persist_open_vault(&self) -> Result<(), AuthorityError> {
        let vault = self.vault();
        let Some(vault) = vault.as_ref() else {
            return Ok(());
        };
        let store = self.store();
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

        // Spec 018 READY_BASE: session_id is opt-in. When present, validate; when
        // absent, legacy lease-only callers continue without a session.
        if let Some(session_id) = &req.session_id {
            self.sessions
                .validate(session_id, &req.vault_id, req.capability)?;
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
                *slot = Some(vault);
                Ok(ResponseBody::EncryptedVaultReady {
                    vault_root,
                    recovery_codes: None,
                })
            }
            RequestBody::CloseEncryptedVault => {
                self.require_lease(&req.vault_id)?;
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
                        self.packs().insert(manifest);
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
            RequestBody::GetFhirSupportMatrix => Ok(ResponseBody::FhirSupportMatrix {
                matrix: medscale_contracts::fhir::FhirSupportMatrix::trusted_v1_ready_base(),
            }),
            RequestBody::ExportFhirLossAware { resource } => {
                Ok(ResponseBody::FhirLossAwareExport {
                    export: medscale_contracts::fhir::loss_aware_export(&resource),
                })
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
