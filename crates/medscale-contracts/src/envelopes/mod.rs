//! Versioned authority facade request/response envelopes.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::AUTHORITY_SCHEMA_VERSION;
use crate::actions::{CreateExternalActionIntentRequest, NphiesInvokeRequest, OutboxEntry};
use crate::documents::{
    AsrStubRequest, DocumentIntakeRequest, DocumentIntakeResult, MediaStubResult, OcrStubRequest,
};
use crate::evidence::{LexicalRetrieveRequest, LexicalRetrieveResult};
use crate::ingest::{BackupManifest, IngestReceipt};
use crate::mesc::{MescArtifactAdmitRequest, MescArtifactVerifyRequest, MescVerifyReport};
use crate::network::{EgressAllowlistEntry, NetworkBrokerRequest, NetworkBrokerResult};
use crate::objects::{AmendmentKind, DigestSha256, EffectState, MedicalTime, OpaqueId, VaultId};
use crate::online_packs::OnlinePackAcquireRequest;
use crate::packs::{PackAdmitResult, PackManifestV0, PackPromotionState};
use crate::presentation::{DrillDownResult, SubjectBriefV1, SubjectCoverageV1, SubjectTimelineV1};
use crate::workflow::DisclosureRecord;

/// Capability required to execute a facade operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    AcquireLease,
    ReleaseLease,
    Ping,
    CreateSourceRecord,
    CreateDerivedArtifact,
    CreateProposal,
    PromoteProposal,
    CreateIdentityAssertion,
    DecideIdentityMerge,
    AppendAudit,
    TransitionEffect,
    ReadObject,
    OpenSyntheticVault,
    CloseVault,
    IngestFhirSynthetic,
    AttachValidatorEvidence,
    RebuildProjection,
    ReadCanonicalVisibility,
    VerifyBlob,
    BackupVault,
    RestoreVault,
    RunBlobGc,
    GetTimeline,
    GetBrief,
    GetCoverage,
    DrillDownPresentation,
    CreateEncryptedVault,
    OpenEncryptedVault,
    CloseEncryptedVault,
    NetworkBrokerInvoke,
    SetEgressAllowlist,
    PacksInstallLocal,
    PacksList,
    PacksPromote,
    DocumentIntake,
    OcrStub,
    AsrStub,
    RetrieveLexical,
    CreateExternalActionIntent,
    ListOutbox,
    NphiesInvoke,
    OnlinePackAcquire,
    MescArtifactAdmit,
    MescArtifactVerify,
    OpenSession,
    RevokeSession,
    AmendAssertion,
    GetFhirSupportMatrix,
    ExportFhirLossAware,
    RejectProposal,
    AppendDisclosure,
    ListDisclosures,
}

impl Capability {
    /// Lease/session bootstrap ops that must work before a client session exists.
    #[must_use]
    pub const fn is_session_bootstrap(self) -> bool {
        matches!(
            self,
            Self::AcquireLease | Self::ReleaseLease | Self::OpenSession | Self::RevokeSession
        )
    }

    /// Read-only / health ops that do not require a client session under Strict mode.
    #[must_use]
    pub const fn is_read_or_health(self) -> bool {
        matches!(
            self,
            Self::Ping
                | Self::ReadObject
                | Self::GetTimeline
                | Self::GetBrief
                | Self::GetCoverage
                | Self::DrillDownPresentation
                | Self::PacksList
                | Self::ListOutbox
                | Self::ListDisclosures
                | Self::ReadCanonicalVisibility
                | Self::VerifyBlob
                | Self::GetFhirSupportMatrix
        )
    }

    /// True when Spec 024 Strict enforcement requires a live `session_id`.
    #[must_use]
    pub const fn requires_client_session(self) -> bool {
        !self.is_session_bootstrap() && !self.is_read_or_health()
    }

    /// Broad grant set for CLI / IPC operator sessions (synthetic READY_BASE).
    #[must_use]
    pub fn operator_grants() -> Vec<Self> {
        vec![
            Self::Ping,
            Self::CreateSourceRecord,
            Self::CreateDerivedArtifact,
            Self::CreateProposal,
            Self::PromoteProposal,
            Self::CreateIdentityAssertion,
            Self::DecideIdentityMerge,
            Self::AppendAudit,
            Self::TransitionEffect,
            Self::ReadObject,
            Self::OpenSyntheticVault,
            Self::CloseVault,
            Self::IngestFhirSynthetic,
            Self::AttachValidatorEvidence,
            Self::RebuildProjection,
            Self::ReadCanonicalVisibility,
            Self::VerifyBlob,
            Self::BackupVault,
            Self::RestoreVault,
            Self::RunBlobGc,
            Self::GetTimeline,
            Self::GetBrief,
            Self::GetCoverage,
            Self::DrillDownPresentation,
            Self::CreateEncryptedVault,
            Self::OpenEncryptedVault,
            Self::CloseEncryptedVault,
            Self::NetworkBrokerInvoke,
            Self::SetEgressAllowlist,
            Self::PacksInstallLocal,
            Self::PacksList,
            Self::PacksPromote,
            Self::DocumentIntake,
            Self::OcrStub,
            Self::AsrStub,
            Self::RetrieveLexical,
            Self::CreateExternalActionIntent,
            Self::ListOutbox,
            Self::NphiesInvoke,
            Self::OnlinePackAcquire,
            Self::MescArtifactAdmit,
            Self::MescArtifactVerify,
            Self::AmendAssertion,
            Self::GetFhirSupportMatrix,
            Self::ExportFhirLossAware,
            Self::RejectProposal,
            Self::AppendDisclosure,
            Self::ListDisclosures,
            Self::RevokeSession,
        ]
    }
}

/// Request body variants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum RequestBody {
    AcquireLease {
        client_id: OpaqueId,
        holder_id_hint: Option<OpaqueId>,
    },
    ReleaseLease {
        holder_id: OpaqueId,
    },
    Ping,
    CreateSourceRecord {
        media_type: String,
        bytes: Vec<u8>,
    },
    CreateDerivedArtifact {
        source_id: OpaqueId,
        transform_id: String,
        transform_version: String,
        bytes: Vec<u8>,
    },
    CreateProposal {
        subject_ref: Option<OpaqueId>,
        claim_kind: String,
        payload: Value,
        evidence_refs: Vec<OpaqueId>,
    },
    PromoteProposal {
        proposal_id: OpaqueId,
        authorized_by: OpaqueId,
        subject_ref: OpaqueId,
    },
    CreateIdentityAssertion {
        subject_id: OpaqueId,
        identifier_system: String,
        identifier_value: String,
    },
    DecideIdentityMerge {
        surviving_subject_id: OpaqueId,
        merged_subject_ids: Vec<OpaqueId>,
        authorized_by: OpaqueId,
        rationale: String,
    },
    AppendAudit {
        actor: OpaqueId,
        action: String,
        target_refs: Vec<OpaqueId>,
        detail: Option<Value>,
    },
    TransitionEffect {
        action_id: OpaqueId,
        to: EffectState,
        reconcile_token: Option<String>,
    },
    ReadObject {
        object_id: OpaqueId,
    },
    OpenSyntheticVault {
        vault_root: String,
    },
    CloseVault,
    IngestFhirSynthetic {
        media_type: String,
        bytes: Vec<u8>,
        fhir_version_hint: Option<String>,
        attach_validator_fixture_id: Option<String>,
    },
    AttachValidatorEvidence {
        source_id: OpaqueId,
        evaluator: String,
        outcome: String,
        issue_codes: Vec<String>,
    },
    RebuildProjection {
        kind: String,
        built_from: Vec<OpaqueId>,
    },
    ReadCanonicalVisibility {
        source_id: OpaqueId,
    },
    VerifyBlob {
        digest: DigestSha256,
    },
    BackupVault {
        destination: String,
    },
    RestoreVault {
        source: String,
        destination: String,
    },
    RunBlobGc,
    GetTimeline {
        subject_ref: OpaqueId,
    },
    GetBrief {
        subject_ref: OpaqueId,
    },
    GetCoverage {
        subject_ref: OpaqueId,
    },
    DrillDownPresentation {
        subject_ref: OpaqueId,
        field_key: String,
        assertion_id: Option<OpaqueId>,
    },
    CreateEncryptedVault {
        vault_root: String,
        passphrase: String,
    },
    OpenEncryptedVault {
        vault_root: String,
        passphrase: Option<String>,
        recovery_code: Option<String>,
    },
    CloseEncryptedVault,
    NetworkBrokerInvoke {
        request: NetworkBrokerRequest,
    },
    SetEgressAllowlist {
        entries: Vec<EgressAllowlistEntry>,
    },
    PacksInstallLocal {
        local_path: String,
    },
    PacksList,
    PacksPromote {
        pack_id: OpaqueId,
        to: PackPromotionState,
    },
    DocumentIntake {
        request: DocumentIntakeRequest,
    },
    OcrStub {
        request: OcrStubRequest,
    },
    AsrStub {
        request: AsrStubRequest,
    },
    RetrieveLexical {
        request: LexicalRetrieveRequest,
    },
    CreateExternalActionIntent {
        request: CreateExternalActionIntentRequest,
    },
    ListOutbox,
    NphiesInvoke {
        request: NphiesInvokeRequest,
    },
    OnlinePackAcquire {
        request: OnlinePackAcquireRequest,
    },
    MescArtifactAdmit {
        request: MescArtifactAdmitRequest,
    },
    MescArtifactVerify {
        request: MescArtifactVerifyRequest,
    },
    OpenSession {
        holder_id: OpaqueId,
        granted: Vec<Capability>,
        ttl_ticks: u64,
    },
    RevokeSession {
        session_id: OpaqueId,
    },
    AmendAssertion {
        prior_assertion_id: OpaqueId,
        authorized_by: OpaqueId,
        payload: Value,
        effective_time: Option<MedicalTime>,
        kind: AmendmentKind,
        rationale: String,
    },
    GetFhirSupportMatrix,
    ExportFhirLossAware {
        resource: Value,
    },
    RejectProposal {
        proposal_id: OpaqueId,
        actor: OpaqueId,
        rationale: String,
    },
    AppendDisclosure {
        purpose: String,
        scope: String,
        subject_ref: Option<OpaqueId>,
        artifact_refs: Vec<OpaqueId>,
        export_digest: Option<DigestSha256>,
        note: Option<String>,
    },
    ListDisclosures,
}

/// Successful response body variants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "ok", rename_all = "snake_case")]
pub enum ResponseBody {
    Lease {
        holder_id: OpaqueId,
        vault_id: VaultId,
    },
    Pong {
        schema_version: u32,
    },
    Created {
        object_id: OpaqueId,
    },
    Promoted {
        assertion_id: OpaqueId,
        audit_id: OpaqueId,
    },
    Effect {
        action_id: OpaqueId,
        state: EffectState,
    },
    Object {
        /// JSON encoding of the object; never includes DB/key handles.
        value: Value,
    },
    Released,
    VaultOpened {
        vault_root: String,
    },
    VaultClosed,
    Ingested {
        receipt: IngestReceipt,
    },
    Visibility {
        source_id: OpaqueId,
        visible: bool,
        content_digest: DigestSha256,
        byte_length: u64,
        blob_ok: bool,
    },
    BlobVerified {
        digest: DigestSha256,
        byte_length: u64,
        state: crate::ingest::BlobState,
    },
    Backup {
        manifest: BackupManifest,
    },
    Restored {
        sources_restored: u64,
        blobs_restored: u64,
    },
    Gc {
        tombstoned: u64,
        swept: u64,
    },
    Timeline {
        body: SubjectTimelineV1,
        projection_id: Option<OpaqueId>,
    },
    Brief {
        body: SubjectBriefV1,
        projection_id: Option<OpaqueId>,
    },
    Coverage {
        body: SubjectCoverageV1,
        projection_id: Option<OpaqueId>,
    },
    DrillDown {
        result: DrillDownResult,
    },
    EncryptedVaultReady {
        vault_root: String,
        recovery_codes: Option<Vec<String>>,
    },
    NetworkBroker {
        result: NetworkBrokerResult,
    },
    AllowlistSet {
        entries: u32,
    },
    PackAdmit {
        result: PackAdmitResult,
    },
    PackList {
        packs: Vec<PackManifestV0>,
    },
    PackPromoted {
        pack_id: OpaqueId,
        state: PackPromotionState,
    },
    DocumentIntake {
        result: DocumentIntakeResult,
    },
    MediaStub {
        result: MediaStubResult,
    },
    LexicalRetrieve {
        result: LexicalRetrieveResult,
    },
    Outbox {
        entries: Vec<OutboxEntry>,
    },
    Session {
        session_id: OpaqueId,
        expires_at_tick: u64,
    },
    Amended {
        assertion_id: OpaqueId,
        amendment_id: OpaqueId,
        audit_id: OpaqueId,
    },
    FhirSupportMatrix {
        matrix: crate::fhir::FhirSupportMatrix,
    },
    FhirLossAwareExport {
        export: crate::fhir::FhirLossAwareExport,
    },
    Rejected {
        proposal_id: OpaqueId,
        audit_id: OpaqueId,
    },
    DisclosureAppended {
        record: DisclosureRecord,
    },
    DisclosureList {
        records: Vec<DisclosureRecord>,
    },
    MescVerify {
        report: MescVerifyReport,
    },
}

/// Authority error vocabulary (fail closed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum AuthorityError {
    Unauthorized,
    AlreadyHeld {
        holder_id: OpaqueId,
    },
    NotHolder,
    NotHeld,
    NotFound,
    WrongScope,
    IllegalTransition,
    UnknownRequiresReconcile,
    DigestMismatch,
    LeaseRequired,
    VaultRequired,
    LexicalReject {
        reason: String,
    },
    VersionReject {
        got: Option<String>,
    },
    PathOutsideClaim,
    InvalidArgument {
        message: String,
    },
    MissingKeyMaterial,
    LeaseHeld {
        holder_id: OpaqueId,
    },
    BrokerDenied {
        reason: crate::network::BrokerReasonCode,
    },
    ExternalGateRequired {
        gate: String,
    },
    PackDenied {
        reason: crate::packs::PackAdmitReason,
    },
    SessionRequired,
    SessionExpired,
    SessionRevoked,
    SessionDenied,
}

/// Versioned authority request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorityRequest {
    pub schema_version: u32,
    pub request_id: OpaqueId,
    pub vault_id: VaultId,
    pub authority_scope_id: crate::objects::AuthorityScopeId,
    pub realm_id: crate::objects::RealmId,
    pub capability: Capability,
    pub deadline_ms: u64,
    pub max_response_bytes: u64,
    /// Client session (Spec 024 Strict: required for mutating capabilities).
    /// Absent is only valid for bootstrap/read/health, or LegacyLeaseOnlyEngineering.
    #[serde(default)]
    pub session_id: Option<OpaqueId>,
    pub body: RequestBody,
}

impl AuthorityRequest {
    /// Builds a request at the current schema version.
    #[must_use]
    pub fn new(
        request_id: OpaqueId,
        vault_id: VaultId,
        realm_id: crate::objects::RealmId,
        authority_scope_id: crate::objects::AuthorityScopeId,
        capability: Capability,
        body: RequestBody,
    ) -> Self {
        Self {
            schema_version: AUTHORITY_SCHEMA_VERSION,
            request_id,
            vault_id,
            authority_scope_id,
            realm_id,
            capability,
            deadline_ms: 5_000,
            max_response_bytes: 1_048_576,
            session_id: None,
            body,
        }
    }
}

/// Versioned authority response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorityResponse {
    pub schema_version: u32,
    pub request_id: OpaqueId,
    pub result: Result<ResponseBody, AuthorityError>,
}
