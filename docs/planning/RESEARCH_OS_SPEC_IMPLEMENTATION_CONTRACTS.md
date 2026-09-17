# MedScale Research OS Spec Implementation Contracts

**Status:** Planning contract. Candidate Specs 074-088 remain non-executable until canonical promotion.

This document defines the minimum implementation shape for each candidate unit. It exists to prevent future implementers from interpreting the roadmap as a feature wishlist.

All units inherit `RESEARCH_OS_MASTER_IMPLEMENTATION_CONTRACT.md` and `RESEARCH_OS_ACCEPTANCE_FRAMEWORK.md`.

## Program-wide implementation rule

For every promoted unit, implementation MUST follow this slice order unless the spec proves a smaller dependency graph:

```text
A. Contract/types
B. Storage/migration
C. Core authority/state machine
D. CLI vertical slice
E. Desktop vertical slice
F. Failure/security/performance evidence
G. Exact-revision closure
```

Do not build UI before the authoritative command/storage path exists. Do not add remote/server behavior before the local contract is proven.

---

# 074 — Project + Artifact Graph Foundation

## Dependency

Current canonical MedScale through Spec 073.

## Primary ownership

- `medscale-contracts`: Project/artifact/edge/experiment contracts.
- `medscale-storage`: project/artifact relation tables and migrations.
- `medscale-core`: project lifecycle and graph mutation/query authority.
- `medscale-cli`: project/artifact commands.
- `medscale-desktop`: Projects workspace and project context selector.

Do not create `medscale-project` until the spec proves existing crate boundaries are inadequate.

## Required contract types

At minimum:

```text
Project
ProjectStatus
Experiment
ArtifactDescriptor
ArtifactKind
ProjectGraphEdge
ProjectGraphPredicate
ProjectContext
ProjectSummary
```

All IDs use existing `OpaqueId`; provenance/header/digest semantics reuse existing contracts.

## Required Core commands

```text
ProjectCreate
ProjectGet
ProjectList
ProjectRename/UpdateMetadata
ProjectArchive
ExperimentCreate/Update
ArtifactAttach
ArtifactDetachReference
GraphEdgeCreate
GraphEdgeRemove/Tombstone
GraphNeighborsQuery
ProjectContextResolve
```

## Storage requirements

- Project identity unique and stable.
- Existing domain objects are referenced, not copied.
- Graph edge writes are revision/precondition guarded.
- Project deletion starts as archive/tombstone; destructive cleanup is a separate policy path.
- Migration must not change behavior of pre-074 CLI/Desktop routes.

## UI minimum

- Projects list.
- Create/open/archive.
- Active project context shown visibly.
- Project Overview with artifact counts based on real data.
- Artifact browser and relationship view may be simple list/tree first; no decorative graph required for acceptance.

## Tests

- deterministic create/list/open/archive;
- duplicate/idempotent create handling;
- object reference to existing canonical objects;
- edge cycle allowed/denied only according to predicate rules, never accidental recursion;
- migration from pre-074 vault;
- crash between object and edge writes cannot create half-attached authority;
- CLI/Desktop parity.

## Closure gate

A project can be created offline, existing MedScale objects can be referenced without duplication, graph relations survive restart, and old workflows remain functional.

---

# 075 — Collaboration Substrate

## Dependency

074 closed canonically.

## Primary ownership

Prefer modules under existing contracts/core/storage first. A dedicated collaboration crate is justified only if protocol/event logic becomes independently testable and I/O-free.

## Required contracts

```text
CollabEventEnvelope
Room
RoomMembership
ThreadRef
Message
MessageEdit
Task
TaskStatus
NoteDocument
ApprovalRequest
ApprovalDecision
PresenceEvent (ephemeral)
AgentParticipantIdentity
SyncCursor (local-only precursor)
```

## Event rule

Collaboration events reference canonical objects by exact ID/revision. They do not own those objects.

## Conflict semantics

- message create: append-only;
- message edit: explicit edit/replacement event;
- task update: optimistic concurrency;
- note/canvas: revision conflict creates conflict copy or explicit merge UI;
- artifact metadata: Core revision conflict only;
- presence/typing: ephemeral and disposable.

No CRDT in 075.

## Buzz adoption boundary

Study/adapt only isolated patterns that win against native implementation. Candidate areas:

- room/thread event shapes;
- agent-as-participant identity concepts;
- search/activity feed patterns;
- workflow/approval event separation;
- audit checkpoint concepts;
- media annotation structure.

Do not adopt Nostr as canonical clinical/research storage. Do not import Buzz Tauri/React UI.

## Tests

- membership denial;
- agent membership distinct from authority grants;
- message/task/note reference exact artifact revisions;
- stale task update -> Conflict;
- offline note conflict -> both revisions preserved;
- ephemeral events never become durable authority;
- audit chain/checkpoint detects mutation where declared.

## Closure gate

One local user can use rooms/tasks/notes/activity offline, agents have distinct identities, and collaboration cannot mutate referenced canonical artifacts implicitly.

---

# 076 — MedAgent Workbench

## Dependency

074 and 075 closed.

## Contracts

```text
AgentIdentity
AgentProfile
AgentCapabilityManifest
ContextManifest
AgentRun
AgentRunState
AgentTurn
ToolManifest
ToolInvocation
ToolReceipt
RunReceipt
AgentProposal
```

## Run state machine

```text
Created -> ContextResolved -> Admitted -> Running ->
Succeeded | Failed | Cancelled | Partial
```

A run that cannot resolve all required context fails before model invocation.

## Required first vertical slice

- one Project;
- one admitted local model Pack;
- text prompt;
- explicit selected project artifacts;
- zero network;
- read-only evidence lookup tool if already admitted;
- streamed output where runtime supports it;
- cancel;
- persisted RunReceipt and output artifact.

## Tool enforcement

Model output never directly calls Rust functions. Tool request is parsed into typed invocation, capability checked outside the model, validated, executed, and returned with a receipt.

## UI

- left project/context panel;
- central conversation/work area;
- evidence/tool activity panel;
- visible model/runtime identity;
- visible data/network posture;
- stop/cancel control;
- run history.

## Tests

- denied tool call;
- prompt injection in project document cannot grant tool;
- cancellation during generation;
- model process failure;
- stale/deleted context before run;
- no sensitive prompt body in operational logs by default;
- exact model Pack provenance in receipt.

## Closure gate

MedAgent can complete a real local, project-grounded, auditable run without network access or ambient vault access.

---

# 077 — Model Fleet + Compare

## Dependency

076 closed.

## Contracts

```text
AgentLane
LaneTransform
FleetRun
FleetRunState
LaneRunRef
ComparisonRequest
ComparisonObservation
ComparisonReport
```

## Dispatch rule

Each lane resolves its own model, context, tools, data and network policy. Fleet orchestration cannot widen the union of permissions and give every lane all capabilities.

## Comparison output

May report:

- agreement/disagreement;
- contradiction candidates;
- evidence references and overlap;
- unsupported claim candidates;
- abstentions;
- structured schema validity;
- latency/resource facts;
- tool calls;
- unavailable/unobserved dimensions.

Must not produce a default "winner" or treat majority as truth.

## Delegate adapter rule

External agent/delegate adapters are disabled for sensitive data by default. Initial adapters, if promoted, operate only on `PUBLIC` or specifically authorized `EXTERNAL_DEIDENTIFIED` inputs.

## Tests

- one lane fails, fleet remains Partial with exact lane state;
- permission sets differ per lane;
- comparison survives different output lengths/formats;
- citation/evidence comparison uses refs, not string similarity alone;
- unknown metrics remain unknown.

---

# 078 — Privacy Gate

## Dependency

074 and 076 closed; 077 optional dependency only for multi-lane testing.

## Contracts

```text
DataClass
PrivacyPolicyProfile
SensitiveSpan
RecognizerResult
PrivacyTransformPlan
PrivacyTransform
PseudonymMapRef
ResidualScanResult
DeidReceipt
EgressDecision
```

## Recognition layers

1. deterministic identifiers/patterns;
2. structured metadata/FHIR-aware fields;
3. admitted local NER/PII models;
4. optional contextual recognizers;
5. residual scan.

Recognition is evidence, not proof of completeness.

## Transformation modes

- redact;
- tokenize;
- generalize;
- pseudonymize;
- drop field/row only under explicit policy.

Source artifact remains immutable.

## Enforcement points

Must integrate with:

- MedAgent context;
- browser/network;
- export;
- Hub sharing;
- compute staging;
- analytics adapter materialization;
- connector requests.

## Tests

Synthetic multilingual corpus covering names, IDs, contacts, dates, addresses, record IDs, medications/context and free text. Include false-positive/false-negative reporting; do not publish a blanket "PHI removed" claim.

Revocation/re-identification tests required for reversible pseudonyms.

---

# 079 — AudioFlow Foundation

## Dependency

078 closed for privacy policy; 076 closed for voice-agent control integration.

## Ownership

Default plan:

- contracts in `medscale-contracts`;
- routing/authority in `medscale-core`;
- Pack/model identity via `medscale-pack`;
- native capture/audio orchestration may justify a dedicated `medscale-audio` crate only when the spec proves it is cohesive and reusable;
- heavyweight Python/custom-code engines remain isolated workers, never Core imports.

## Required contracts

```text
AudioSource
AudioSourceKind
AudioSession
CaptureState
CaptureHealth
AudioRouteRequest
AudioRouteDecision
AudioRouteReceipt
TranscriptRevision
TranscriptSegment
SpeakerLabel
DiarizationRevision
AudioEvidenceRef
VoiceInputMode = COMMAND | CONTEXT | DICTATION
TranscriptReceipt
```

## Capture state machine

```text
Idle -> PermissionPending -> Capturing -> Paused -> Stopping -> Finalized
                                  \-> Failed
```

Visible capture indicator is mandatory while Capturing/Paused.

## Runtime routing

Initial qualification must select, not assume, routes for:

- low-latency live STT;
- higher-quality offline/import STT;
- diarization/alignment.

Arabic, Arabic-English code switching and medical-sensitive tokens are benchmark dimensions where claimed.

## Donor use

VoiceStudio: engine/model orchestration, diagnostics, local speech service, streaming, dictation and output safety patterns.

Himsat: capture lineage, two-pass transcription, long-session stability, audio evidence mapping, runtime router research.

Wispral: COMMAND/CONTEXT/DICTATION, interruption and steering semantics.

Buzz: later huddle lifecycle/media annotations, not required for foundation.

## Transcript rule

Quality corrections create new revisions. Keep source audio digest/time mapping and previous transcript revisions.

## Tests

- microphone permission denied;
- device removed mid-session;
- six-hour soak target defined and measured on qualified platform before release claim;
- cancel/stop race;
- no hidden cloud fallback;
- route unavailable -> explicit Unavailable;
- medical number/unit/negation preservation benchmark;
- Arabic/code-switch benchmark;
- diarization speaker labels do not create persistent biometric identity.

---

# 080 — Analytics Gate

## Dependency

074 and 078 closed.

## Contracts

```text
AnalyticalView
AnalysisPlan
QueryRequest
QueryExecution
QueryReceipt
CohortDefinition
StatisticRequest
StatisticResult
AssumptionCheck
DerivedDataset
FigureArtifact
TableArtifact
```

## Engine rule

Qualify DataFusion against required workloads. If gaps exist, add bounded alternatives behind the same contract; do not expose engine-specific authority to UI.

## Execution policy

- AI proposes SQL/plan;
- parser/planner rejects writes/DDL in default path;
- only governed views are visible;
- source revisions are pinned at execution start;
- query result records exact engine/version/config;
- arbitrary Python/R runs through Compute only.

## Cohort rule

Cohort definitions are versioned artifacts separate from materialized datasets. Re-running a cohort against changed source revisions creates a new result identity.

## Tests

- SQL injection/tool prompt cannot escape governed views;
- query timeout/cancel;
- source revision changes during query -> pinned snapshot or explicit Conflict/Stale behavior;
- deterministic fixtures for aggregates/joins/window operations used by product;
- statistics fixtures validated against trusted independent reference implementation;
- not-run assumptions remain explicit.

---

# 081 — Knowledge + Research Canvas

## Dependency

074, 078, 080 closed; AudioFlow 079 if audio evidence is indexed.

## Contracts

```text
IndexManifest
ChunkRef
EmbeddingManifest
RetrievalPlan
RetrievalCandidate
RetrievalReceipt
EvidenceSpan
CanvasDocument
CanvasNode
CanvasEdge
LiteratureRecord
```

## Retrieval order

Use structured/project relations and lexical retrieval before or alongside vector retrieval. Vector search is optional, not authority.

## Permission rule

Authorization filters are applied before content disclosure. Indexes carry project/scope metadata; stale/deleted/revoked content is tombstoned and excluded.

## Canvas rule

Canvas nodes reference live artifacts/revisions. Copying data into a canvas body must be explicit and provenance-preserving.

## Tests

- stale embedding detection after source revision;
- cross-project permission leak prevention;
- deleted source removal from search/cache;
- exact page/span/audio timestamp links;
- retrieval with no evidence returns insufficient-evidence state.

---

# 082 — MedScale Hub

## Dependency

075 collaboration contracts, 078 privacy and 081 permission-aware project knowledge closed.

## Initial topology

Single Hub instance + multiple desktop clients. Do not start with multi-region or federation.

## Required protocol contracts

```text
HubHello/Capabilities
DeviceIdentity
ProjectMembership
SyncRequest
SyncBatch
SyncAck
ArtifactTransferOffer
ArtifactChunk/ResumeToken
ConflictResponse
RevocationNotice
HubHealth
```

## Hub trust boundary

Hub never becomes alternate Core. Core validates all synced canonical artifact admissions locally.

## Sync rules

- monotonic cursor per project stream;
- idempotent submission;
- exact revision/digest checks;
- resumable object transfer;
- no plaintext local-only artifact upload without policy grant;
- conflict state is returned, never silently overwritten.

## Tests

- two clients offline then conflicting edits;
- revoke user while connected;
- stale cursor/full resync;
- interrupted large dataset transfer/resume;
- malicious Hub provides wrong digest;
- tenant/project isolation across DB/search/cache/pubsub;
- backup/restore and upgrade rollback.

---

# 083 — AudioFlow Advanced

## Dependency

079 + 082 closed.

## Scope

- Audio Huddles;
- agent participation in huddles;
- live tasks/evidence proposals;
- TTS/listen-back;
- duplex voice where qualified;
- voice design/cloning with consent/provenance;
- time/frame anchored annotations;
- batch workflows;
- optional remote workers.

## Huddle rule

Huddle lifecycle is durable collaboration state; raw audio retention is a separate explicit decision. Joining a huddle does not imply permission to record/transcribe/export.

## Agent rule

Agents may listen/speak only when their room membership plus audio capability plus data-class policy allows it. Spoken agent output is labeled as synthetic/agent-originated.

## Voice cloning rule

Requires explicit subject permission/provenance record and export provenance/watermark policy. No implicit cloning from meeting audio.

---

# 084 — MedScale Compute

## Dependency

078 privacy closed; 076 tool/agent contracts closed. Analytics/audio may consume Compute later.

## Contracts

```text
ComputeJobManifest
WorkerCapabilityManifest
WorkerLease
WorkerHeartbeat
JobInputLease
JobState
ComputeReceipt
OutputCandidate
```

## Initial worker order

1. local subprocess/OS-sandbox worker;
2. self-hosted remote worker;
3. container/OpenSandbox qualification;
4. institutional schedulers later.

## Protocol

- worker registers capabilities;
- Core/Hub grants short-lived lease;
- inputs staged by exact digest;
- worker cannot browse vault;
- periodic heartbeat;
- cancel/revoke expires lease;
- outputs uploaded as candidates;
- Core validates/adopts output.

## Tests

OOM, timeout, crash, network loss, duplicate completion, late completion after revoke, malicious output schema, excessive log content, filesystem escape and egress denial.

---

# 085 — Research Packs

## Dependency

074 Project Graph, 076 tool model, 081 knowledge contracts, 084 worker model where executable extensions are needed.

## First proof order

1. Clinical Research Pack — proves migration of existing medical/research concepts without weakening Core.
2. AI Research Pack — proves datasets/models/evals/experiments.
3. Systematic Review Pack — proves literature/screening/extraction.
4. Imaging, Omics and Wet Lab after domain-specific source qualification.

This order may change only with a documented adoption/use-case decision.

## Pack contracts

```text
ResearchPackManifest
ArtifactSchemaExtension
ValidationRule
WorkflowTemplate
ViewDescriptor
ToolRequirement
ModelRequirement
ImportExportDescriptor
PackMigration
```

No arbitrary native UI code in trusted process by default.

---

# 086 — Institutional Adapters

## Dependency

082 Hub and 084 Compute stable.

## Adapter contract

Every adapter defines:

- identity/auth method;
- data classes accepted;
- read/write capabilities;
- idempotency semantics;
- timeout/Unknown handling;
- rate/resource limits;
- credentials storage;
- mapping/versioning;
- audit/receipt;
- offline/unavailable behavior.

## Candidate order

1. institutional identity/SSO;
2. object storage;
3. EHR/FHIR and LIMS/ELN read paths;
4. HPC/Slurm/Kubernetes compute;
5. advanced BI/search adapters;
6. write/action adapters only after read paths and authority contracts are proven.

Superset/OpenRAG/OpenSearch remain optional services over approved data, not Core dependencies.

---

# 087 — Federation

## Dependency

Institution-scale Hub/identity/policy/compute proven under 086.

## Default position

Federation is not required for first lab or institutional release. It remains research-gated until a concrete cross-institution use case is selected.

## Required research contracts before implementation

```text
FederatedIdentityBinding
SiteTrustPolicy
DataUseConstraint
FederatedJobManifest
FederatedResultEnvelope
Revocation/Expiry
CrossSiteProvenanceChain
```

No raw PHI transfer is assumed. Data stays at site by default.

---

# 088 — Whole-Platform Qualification

## Dependency

Only capabilities intended for the declared release need be included; unpromoted research/federation work cannot block an earlier product release unless canonical governance says otherwise.

## Required qualification campaigns

1. clean install / migration / rollback;
2. personal fully-offline workflow;
3. lab multi-user workflow;
4. authority/capability abuse campaign;
5. privacy/PHI egress campaign;
6. agent prompt/tool injection campaign;
7. browser/worker sandbox campaign;
8. analytics correctness/reproducibility campaign;
9. AudioFlow quality/privacy/long-session campaign;
10. Hub concurrency/conflict/revoke campaign;
11. backup/restore/disaster-recovery campaign;
12. accessibility and keyboard/focus campaign;
13. supported-platform performance/resource campaign;
14. donor/provenance/license/SBOM closure;
15. release packaging/signing/notarization only where real credentials/evidence exist.

## Final closure rule

`RESEARCH_OS_COMPLETE=true` or equivalent may be asserted only for an explicitly declared scope/release profile. It MUST list deferred/unqualified Packs, adapters or federation features rather than implying all roadmap research is complete.
