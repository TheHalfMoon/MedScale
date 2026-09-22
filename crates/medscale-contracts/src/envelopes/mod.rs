//! Versioned authority facade request/response envelopes.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::AUTHORITY_SCHEMA_VERSION;
use crate::actions::{CreateExternalActionIntentRequest, NphiesInvokeRequest, OutboxEntry};
use crate::collaboration::{
    ActivityRecord, AnchorTarget, ApprovalDecision, ApprovalDecisionOutcome, ApprovalKind,
    ApprovalRequest, MembershipRole, Message, MessageEdit, NoteDocument, NoteRevision,
    ParticipantIdentity, ParticipantKind, Room, RoomMembership, RoomStatus, Task, TaskStatus,
    ThreadRef, ThreadStatus,
};
use crate::data_sources::{
    DataSourceManifest, DataSourceSummary, DataViewKind, DatasetReleaseSummary, FilterExpr,
    RefreshReceipt, ReleaseManifest, SavedDataView, SavedViewSummary, SnapshotRowPage,
    SnapshotSummary, SortKey, SourceLocator, SourceSchema, TransformOp, TransformationReceipt,
    ViewState,
};
use crate::documents::{
    AsrStubRequest, DocumentIntakeRequest, DocumentIntakeResult, MediaStubResult, OcrStubRequest,
};
use crate::evidence::{LexicalRetrieveRequest, LexicalRetrieveResult};
use crate::ingest::{BackupManifest, IngestReceipt};
use crate::medagent::{AgentCapabilityManifest, AgentIdentity, ToolKind};
use crate::mesc::{MescArtifactAdmitRequest, MescArtifactVerifyRequest, MescVerifyReport};
use crate::network::{EgressAllowlistEntry, NetworkBrokerRequest, NetworkBrokerResult};
use crate::objects::{AmendmentKind, DigestSha256, EffectState, MedicalTime, OpaqueId, VaultId};
use crate::online_packs::OnlinePackAcquireRequest;
use crate::packs::{
    PackAdmitResult, PackEvaluationRequest, PackEvaluationResult, PackManifestV0,
    PackPromotionState,
};
use crate::presentation::{DrillDownResult, SubjectBriefV1, SubjectCoverageV1, SubjectTimelineV1};
use crate::project_graph::{
    ArtifactDescriptor, Experiment, ExperimentSummary, GraphDirection, GraphEndpoint,
    GraphNeighborPage, Project, ProjectArtifactRef, ProjectContext, ProjectGraphEdge,
    ProjectGraphPredicate, ProjectStatus, ProjectSummary, ReferenceResolution, ResolvedArtifactRef,
};
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
    PacksEvaluateLocal,
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
    ProjectCreate,
    ProjectRead,
    ProjectUpdate,
    ProjectArchive,
    ExperimentCreate,
    ExperimentRead,
    ExperimentUpdate,
    ExperimentArchive,
    ProjectArtifactAttach,
    ProjectArtifactDetach,
    ProjectGraphRead,
    ProjectGraphMutate,
    DataSourceCreate,
    DataSourceRead,
    DataSourceUpdate,
    DataSourceArchive,
    SnapshotImport,
    SnapshotPreview,
    SnapshotRead,
    SnapshotRefresh,
    SavedViewCreate,
    SavedViewRead,
    SavedViewUpdate,
    TransformExecute,
    DatasetReleaseCreate,
    DatasetReleaseRead,
    // Spec 076: Collaboration Substrate (T076-03 slice: participant, room,
    // membership only; thread/message/task/note/approval/activity
    // capabilities land in later slices).
    ParticipantRegister,
    ParticipantRead,
    ParticipantRevoke,
    RoomCreate,
    RoomRead,
    RoomUpdate,
    RoomArchive,
    RoomMembershipManage,
    RoomMembershipRead,
    // Spec 076 T076-04/05/06 slice: thread, message, task.
    ThreadCreate,
    ThreadRead,
    ThreadResolve,
    MessagePost,
    MessageRead,
    MessageEdit,
    TaskCreate,
    TaskRead,
    TaskUpdate,
    // Spec 076 T076-07/08/09 slice: note, approval, activity.
    NoteCreate,
    NoteRead,
    NoteUpdate,
    ApprovalRequestCreate,
    ApprovalRequestRead,
    ApprovalDecide,
    ApprovalWithdraw,
    ActivityRead,
    // Spec 077: MedAgent Workbench (T077-03 slice: AgentIdentity +
    // AgentCapabilityManifest only; ContextManifest/AgentRun/tool/receipt/
    // proposal capabilities land in later slices).
    AgentIdentityRegister,
    AgentIdentityRead,
    AgentIdentityRevoke,
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
                | Self::ProjectRead
                | Self::ExperimentRead
                | Self::ProjectGraphRead
                | Self::DataSourceRead
                | Self::SnapshotPreview
                | Self::SnapshotRead
                | Self::SavedViewRead
                | Self::DatasetReleaseRead
                | Self::ParticipantRead
                | Self::RoomRead
                | Self::RoomMembershipRead
                | Self::ThreadRead
                | Self::MessageRead
                | Self::TaskRead
                | Self::NoteRead
                | Self::ApprovalRequestRead
                | Self::ActivityRead
                | Self::AgentIdentityRead
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
            Self::PacksEvaluateLocal,
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
            Self::ProjectCreate,
            Self::ProjectRead,
            Self::ProjectUpdate,
            Self::ProjectArchive,
            Self::ExperimentCreate,
            Self::ExperimentRead,
            Self::ExperimentUpdate,
            Self::ExperimentArchive,
            Self::ProjectArtifactAttach,
            Self::ProjectArtifactDetach,
            Self::ProjectGraphRead,
            Self::ProjectGraphMutate,
            Self::DataSourceCreate,
            Self::DataSourceRead,
            Self::DataSourceUpdate,
            Self::DataSourceArchive,
            Self::SnapshotImport,
            Self::SnapshotPreview,
            Self::SnapshotRead,
            Self::SnapshotRefresh,
            Self::SavedViewCreate,
            Self::SavedViewRead,
            Self::SavedViewUpdate,
            Self::TransformExecute,
            Self::DatasetReleaseCreate,
            Self::DatasetReleaseRead,
            Self::ParticipantRegister,
            Self::ParticipantRead,
            Self::ParticipantRevoke,
            Self::RoomCreate,
            Self::RoomRead,
            Self::RoomUpdate,
            Self::RoomArchive,
            Self::RoomMembershipManage,
            Self::RoomMembershipRead,
            Self::ThreadCreate,
            Self::ThreadRead,
            Self::ThreadResolve,
            Self::MessagePost,
            Self::MessageRead,
            Self::MessageEdit,
            Self::TaskCreate,
            Self::TaskRead,
            Self::TaskUpdate,
            Self::NoteCreate,
            Self::NoteRead,
            Self::NoteUpdate,
            Self::ApprovalRequestCreate,
            Self::ApprovalRequestRead,
            Self::ApprovalDecide,
            Self::ApprovalWithdraw,
            Self::ActivityRead,
            Self::AgentIdentityRegister,
            Self::AgentIdentityRead,
            Self::AgentIdentityRevoke,
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
    PacksEvaluateLocal {
        request: PackEvaluationRequest,
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
    // Spec 074: every mutation flows through Core authority paths. Surfaces
    // never write project storage directly.
    ProjectCreate {
        name: String,
        description: Option<String>,
    },
    ProjectGet {
        project_id: OpaqueId,
    },
    ProjectList {
        status: Option<ProjectStatus>,
        limit: Option<u32>,
        cursor: Option<String>,
    },
    ProjectUpdate {
        project_id: OpaqueId,
        expected_revision: u64,
        name: Option<String>,
        description: Option<Option<String>>,
    },
    ProjectArchive {
        project_id: OpaqueId,
        expected_revision: u64,
    },
    ProjectRestore {
        project_id: OpaqueId,
        expected_revision: u64,
    },
    ExperimentCreate {
        project_id: OpaqueId,
        name: String,
        description: Option<String>,
    },
    ExperimentGet {
        experiment_id: OpaqueId,
    },
    ExperimentList {
        project_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    },
    ExperimentUpdate {
        experiment_id: OpaqueId,
        expected_revision: u64,
        name: Option<String>,
        description: Option<Option<String>>,
    },
    ExperimentArchive {
        experiment_id: OpaqueId,
        expected_revision: u64,
    },
    ProjectAttach {
        project_id: OpaqueId,
        experiment_id: Option<OpaqueId>,
        artifact: ArtifactDescriptor,
    },
    ProjectDetach {
        ref_id: OpaqueId,
        expected_revision: u64,
    },
    ProjectListRefs {
        project_id: OpaqueId,
        experiment_id: Option<OpaqueId>,
        active_only: bool,
        limit: Option<u32>,
        cursor: Option<String>,
    },
    GraphEdgeCreate {
        project_id: OpaqueId,
        subject: GraphEndpoint,
        predicate: ProjectGraphPredicate,
        object: GraphEndpoint,
    },
    GraphEdgeRemove {
        edge_id: OpaqueId,
        expected_revision: u64,
    },
    GraphNeighbors {
        project_id: OpaqueId,
        start: GraphEndpoint,
        predicates: Option<Vec<ProjectGraphPredicate>>,
        direction: Option<GraphDirection>,
        limit: Option<u32>,
        cursor: Option<String>,
    },
    ProjectContextResolve {
        project_id: OpaqueId,
        experiment_id: Option<OpaqueId>,
        refs_limit: Option<u32>,
        graph_limit: Option<u32>,
    },
    ProjectSummaryQuery {
        project_id: OpaqueId,
    },
    // Spec 075: every mutation flows through Core authority paths. Surfaces
    // never write data-source storage directly.
    DataSourceCreate {
        project_id: OpaqueId,
        display_name: String,
        locator: SourceLocator,
        credential_ref: Option<OpaqueId>,
    },
    DataSourceGet {
        source_id: OpaqueId,
    },
    DataSourceList {
        project_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    },
    DataSourceUpdate {
        source_id: OpaqueId,
        expected_revision: u64,
        display_name: Option<String>,
        credential_ref: Option<Option<OpaqueId>>,
    },
    DataSourceArchive {
        source_id: OpaqueId,
        expected_revision: u64,
    },
    SnapshotImport {
        source_id: OpaqueId,
    },
    SnapshotPreview {
        source_id: OpaqueId,
        max_rows: Option<u32>,
    },
    SnapshotGet {
        snapshot_id: OpaqueId,
    },
    SnapshotList {
        source_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    },
    SnapshotRows {
        snapshot_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
        filters: Vec<FilterExpr>,
        sort: Vec<SortKey>,
    },
    SnapshotRefresh {
        source_id: OpaqueId,
        allow_schema_change: bool,
    },
    SavedViewCreate {
        snapshot_id: OpaqueId,
        view_kind: DataViewKind,
        state: ViewState,
    },
    SavedViewGet {
        view_id: OpaqueId,
    },
    SavedViewList {
        snapshot_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    },
    SavedViewUpdate {
        view_id: OpaqueId,
        expected_revision: u64,
        state: ViewState,
    },
    TransformExecute {
        input_snapshot_ids: Vec<OpaqueId>,
        ops: Vec<TransformOp>,
    },
    DatasetReleaseCreate {
        snapshot_id: OpaqueId,
        version: String,
        split_group: Option<String>,
        annotation_schema_ref: Option<String>,
        rights_state: crate::data_sources::RightsState,
    },
    DatasetReleaseGet {
        release_id: OpaqueId,
    },
    DatasetReleaseList {
        project_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    },
    // Spec 076: every mutation flows through Core authority paths. Surfaces
    // never write collaboration storage directly. T076-03 slice only.
    ParticipantRegister {
        holder_id: OpaqueId,
        kind: ParticipantKind,
        display_name: String,
        agent_profile_ref: Option<OpaqueId>,
    },
    ParticipantGet {
        participant_id: OpaqueId,
    },
    ParticipantRevoke {
        participant_id: OpaqueId,
        expected_revision: u64,
    },
    RoomCreate {
        project_id: OpaqueId,
        experiment_id: Option<OpaqueId>,
        name: String,
    },
    RoomGet {
        room_id: OpaqueId,
    },
    RoomList {
        project_id: OpaqueId,
        status: Option<RoomStatus>,
        limit: Option<u32>,
        cursor: Option<String>,
    },
    RoomRename {
        room_id: OpaqueId,
        expected_revision: u64,
        name: String,
    },
    RoomArchive {
        room_id: OpaqueId,
        expected_revision: u64,
    },
    RoomMembershipAdd {
        room_id: OpaqueId,
        participant_id: OpaqueId,
        role: MembershipRole,
    },
    RoomMembershipList {
        room_id: OpaqueId,
    },
    RoomMembershipRemove {
        room_id: OpaqueId,
        membership_id: OpaqueId,
        expected_revision: u64,
    },
    // Spec 076 T076-04/05/06 slice: thread, message, task.
    ThreadOpen {
        room_id: OpaqueId,
        anchor: AnchorTarget,
    },
    ThreadGet {
        thread_id: OpaqueId,
    },
    ThreadList {
        room_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    },
    ThreadSetStatus {
        thread_id: OpaqueId,
        expected_revision: u64,
        status: ThreadStatus,
    },
    MessagePost {
        thread_id: OpaqueId,
        body: String,
    },
    MessageList {
        thread_id: OpaqueId,
        limit: Option<u32>,
        after_seq: Option<u64>,
    },
    MessageEditBody {
        message_id: OpaqueId,
        new_body: String,
    },
    MessageDelete {
        message_id: OpaqueId,
    },
    TaskCreate {
        room_id: OpaqueId,
        anchor: Option<AnchorTarget>,
        title: String,
        description: Option<String>,
    },
    TaskGet {
        task_id: OpaqueId,
    },
    TaskList {
        room_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    },
    TaskUpdate {
        task_id: OpaqueId,
        expected_revision: u64,
        status: TaskStatus,
        assignee_participant_id: Option<OpaqueId>,
    },
    // Spec 076 T076-07/08/09 slice: note, approval, activity.
    NoteCreate {
        room_id: OpaqueId,
        title: String,
        body: String,
    },
    NoteGet {
        note_id: OpaqueId,
    },
    NoteListRevisions {
        note_id: OpaqueId,
    },
    NoteEdit {
        note_id: OpaqueId,
        expected_revision: u64,
        body: String,
    },
    ApprovalRequestCreate {
        room_id: OpaqueId,
        anchor: AnchorTarget,
        kind: ApprovalKind,
        assignee_participant_ids: Vec<OpaqueId>,
        blind_until_closed: bool,
    },
    ApprovalRequestGet {
        request_id: OpaqueId,
    },
    ApprovalRequestWithdraw {
        request_id: OpaqueId,
        expected_revision: u64,
    },
    ApprovalDecide {
        request_id: OpaqueId,
        outcome: ApprovalDecisionOutcome,
        rationale: Option<String>,
    },
    ApprovalDecisionList {
        request_id: OpaqueId,
    },
    ActivityList {
        room_id: OpaqueId,
        limit: Option<u32>,
        after_seq: Option<u64>,
    },
    // Spec 077: MedAgent Workbench. Every mutation flows through Core
    // authority paths; surfaces never write medagent storage directly.
    // T077-03 slice only (AgentIdentity + AgentCapabilityManifest);
    // ContextManifest/AgentRun/tool/receipt/proposal land in later slices.
    AgentIdentityRegister {
        project_id: OpaqueId,
        pack_id: OpaqueId,
        display_name: String,
        granted_tool_kinds: Vec<ToolKind>,
    },
    AgentIdentityGet {
        agent_id: OpaqueId,
    },
    AgentIdentityList {
        project_id: OpaqueId,
        limit: Option<u32>,
    },
    AgentIdentityRevoke {
        agent_id: OpaqueId,
        expected_revision: u64,
    },
}

impl RequestBody {
    /// True for the Spec 074 graph mutations plus the Spec 075 snapshot
    /// import/refresh/transform ops that provably leave the in-memory
    /// authority snapshot untouched (durable sqlite rows plus
    /// content-addressed blobs and revisioned receipts only; 075 ids come
    /// from the sqlite sequence). The facade skips the frozen full-snapshot
    /// rewrite for these ops; skipping is output-identical because the
    /// snapshot bytes cannot change. Locked by the Core snapshot-stability
    /// test; any op that gains a memory write must leave this set.
    #[must_use]
    pub fn preserves_memory_snapshot(&self) -> bool {
        matches!(
            self,
            Self::ProjectAttach { .. }
                | Self::ProjectDetach { .. }
                | Self::GraphEdgeCreate { .. }
                | Self::GraphEdgeRemove { .. }
                | Self::SnapshotImport { .. }
                | Self::SnapshotRefresh { .. }
                | Self::TransformExecute { .. }
        )
    }
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
    PackEvaluation {
        result: PackEvaluationResult,
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
    // Spec 074 typed results (revisioned, scope-checked, no raw payloads).
    Project {
        project: Project,
    },
    ProjectList {
        projects: Vec<ProjectSummary>,
        next_cursor: Option<String>,
    },
    Experiment {
        experiment: Experiment,
    },
    ExperimentList {
        experiments: Vec<ExperimentSummary>,
        next_cursor: Option<String>,
    },
    ProjectRef {
        reference: ProjectArtifactRef,
    },
    ProjectRefList {
        refs: Vec<ResolvedArtifactRef>,
        next_cursor: Option<String>,
    },
    GraphEdge {
        edge: ProjectGraphEdge,
    },
    GraphNeighbors {
        page: GraphNeighborPage,
    },
    ProjectContext {
        context: ProjectContext,
    },
    ProjectSummary {
        summary: ProjectSummary,
    },
    // Spec 075 typed results (immutable snapshots, scope-checked, no secrets).
    // Large payloads are boxed: `large_enum_variant` keeps every variant
    // small while the payloads themselves stay owned typed values.
    DataSource {
        source: Box<DataSourceManifest>,
    },
    DataSourceList {
        sources: Vec<DataSourceSummary>,
        next_cursor: Option<String>,
    },
    SnapshotImported {
        snapshot: Box<crate::data_sources::DataSnapshot>,
        receipt: crate::data_sources::ImportReceipt,
    },
    SnapshotPreview {
        schema: SourceSchema,
        rows: Vec<Vec<crate::data_sources::CellValue>>,
    },
    Snapshot {
        snapshot: Box<crate::data_sources::DataSnapshot>,
    },
    SnapshotList {
        snapshots: Vec<SnapshotSummary>,
        next_cursor: Option<String>,
    },
    SnapshotRows {
        page: SnapshotRowPage,
    },
    SnapshotRefreshed {
        receipt: RefreshReceipt,
        snapshot: Option<Box<crate::data_sources::DataSnapshot>>,
    },
    SavedView {
        view: Box<SavedDataView>,
    },
    SavedViewList {
        views: Vec<SavedViewSummary>,
        next_cursor: Option<String>,
    },
    Transformed {
        snapshot: Box<crate::data_sources::DataSnapshot>,
        receipt: TransformationReceipt,
    },
    DatasetRelease {
        release: Box<ReleaseManifest>,
    },
    DatasetReleaseList {
        releases: Vec<DatasetReleaseSummary>,
        next_cursor: Option<String>,
    },
    // Spec 076 typed results (revisioned, membership-gated, no secrets).
    // T076-03 slice only.
    Participant {
        participant: Box<ParticipantIdentity>,
    },
    CollabRoom {
        room: Box<Room>,
    },
    CollabRoomList {
        rooms: Vec<Room>,
        next_cursor: Option<String>,
    },
    CollabMembership {
        membership: Box<RoomMembership>,
    },
    CollabMembershipList {
        memberships: Vec<RoomMembership>,
    },
    CollabThread {
        thread: Box<ThreadRef>,
        resolution: ReferenceResolution,
    },
    CollabThreadList {
        threads: Vec<ThreadRef>,
        resolutions: Vec<ReferenceResolution>,
        next_cursor: Option<String>,
    },
    CollabMessage {
        message: Box<Message>,
    },
    CollabMessageList {
        messages: Vec<Message>,
    },
    CollabMessageEdit {
        edit: Box<MessageEdit>,
    },
    CollabTask {
        task: Box<Task>,
    },
    CollabTaskList {
        tasks: Vec<Task>,
        next_cursor: Option<String>,
    },
    CollabNote {
        note: Box<NoteDocument>,
    },
    CollabNoteRevisionList {
        revisions: Vec<NoteRevision>,
    },
    CollabNoteEdit {
        note: Box<NoteDocument>,
        new_revision: Box<NoteRevision>,
        is_conflict_copy: bool,
    },
    CollabApprovalRequest {
        request: Box<ApprovalRequest>,
    },
    CollabApprovalDecision {
        decision: Box<ApprovalDecision>,
    },
    CollabApprovalDecisionList {
        decisions: Vec<ApprovalDecision>,
    },
    CollabActivityList {
        records: Vec<ActivityRecord>,
    },
    // Spec 077: MedAgent Workbench typed results. T077-03 slice only.
    MedAgentIdentity {
        identity: Box<AgentIdentity>,
        capabilities: Box<AgentCapabilityManifest>,
    },
    MedAgentIdentityList {
        identities: Vec<AgentIdentity>,
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
    /// Optimistic-concurrency conflict: stale `expected_revision` or duplicate.
    /// No write was performed.
    Conflict {
        message: String,
    },
    /// A pinned attachment binding no longer matches its canonical target.
    StaleReference {
        message: String,
    },
    /// Durable Project Graph bytes failed integrity validation.
    Corrupt {
        message: String,
    },
    /// Unsupported durable schema or unknown future authority value (fail closed).
    UnsupportedSchema {
        message: String,
    },
    /// Dependency or target temporarily unresolvable.
    Unavailable {
        message: String,
    },
    /// Caller-requested cancellation completed before commit. No write was
    /// performed. Never a substitute for timeout/unavailable mapping.
    Cancelled {
        message: String,
    },
    /// Unexpected internal failure (never a substitute for a typed variant).
    Internal {
        message: String,
    },
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
