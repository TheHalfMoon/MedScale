# Research OS V2 Spec Implementation Contracts

**Status:** Planning contract — not implementation authority  
**Applies to:** candidate Specs 075-092 under Program Amendment 001  
**Does not change:** already-promoted Spec 074 scope or authority

This document is the candidate-unit implementation contract for Research OS V2. It supersedes V1 candidate-number ownership at 075+ for planning purposes. Every future promoted unit still requires a fresh live-truth promotion packet bound to exact repository paths, current canonical main, tests, migrations, evidence, and current governance.

All units inherit:

- `RESEARCH_OS_MASTER_IMPLEMENTATION_CONTRACT.md`;
- `RESEARCH_OS_PROGRAM_AMENDMENT_001_DATA_EXTENSIONS.md`;
- `RESEARCH_OS_V2_IMPLEMENTATION_CONTRACT_ADDENDUM.md`;
- `RESEARCH_OS_V2_REPOSITORY_MAP_ADDENDUM.md`;
- `RESEARCH_OS_V2_VERIFICATION_ADDENDUM.md`;
- `RESEARCH_OS_ACCEPTANCE_FRAMEWORK.md`;
- `RESEARCH_OS_BUILD_RULES.md`.

## Universal slice order

Unless a promoted spec proves a smaller safe dependency graph, implementation MUST proceed:

```text
A. live-truth reconciliation + contract freeze
B. typed contracts / schema definitions
C. persistence / migration / recovery
D. Core authority + policy state machine
E. focused contract/storage/Core tests
F. CLI vertical slice
G. native Desktop vertical slice
H. security/failure/adversarial qualification
I. performance/scale qualification
J. exact-range review + exact-head CI
K. normal merge + post-main verification
```

No UI-first implementation. No direct storage access from adapters, extensions, IDE bridges, agents, workers, or provider clients. No future unit may silently create a second identity, credential, provenance, audit, policy, source, or artifact authority model.

---

# 075 — Data Source Fabric

## Hard dependency
074 `CLOSED_CANONICAL`.

## Goal
Create one governed source/import/snapshot plane for local files, databases, Kaggle, Hugging Face datasets, and later institutional/object-store adapters.

## Mandatory contracts

```text
DataSourceId
DataSourceKind
DataSourceManifest
CredentialRef
SourceLocation
SourceCapability
SourceSchema
SchemaFingerprint
SourceRevision
AcquisitionMode
ImportPlan
ImportState
DataSnapshot
SnapshotManifest
MutableSourceBinding
ImportReceipt
RefreshReceipt
SourceHealth
AdapterIdentity
```

## Minimum source kinds

Foundation acceptance requires:

1. local CSV/TSV;
2. local Parquet;
3. local JSON/JSONL where bounded parser support exists;
4. SQLite source distinct from the MedScale vault;
5. PostgreSQL read connector;
6. one additional relational connector selected from MySQL/MariaDB or SQL Server based on live platform/test evidence;
7. Kaggle dataset acquisition;
8. Hugging Face dataset acquisition.

Other providers remain adapter backlog unless promoted explicitly.

## State machines

Connection/source:

```text
Defined -> CredentialBound -> Checked -> Ready
                         \-> AuthRequired | Unavailable | Denied | Invalid
```

Acquisition:

```text
Planned -> Admitted -> Acquiring -> Validating -> Materializing -> Complete
                         |              |               |
                         +-> Paused     +-> Corrupt     +-> Conflict
                         +-> Cancelled  +-> Rejected    +-> Failed
                         +-> Failed
```

## Authority rules

- credentials are opaque references only;
- source adapters never open/write the canonical vault directly;
- source browsing/preview is bounded and read-only by default;
- remote network access goes through admitted network authority;
- live preview is not reproducible evidence;
- canonical Analytics/R/RAG use immutable `DataSnapshot` where possible;
- mutable live-source runs must declare partial reproducibility explicitly;
- source deletion never deletes already-materialized snapshots implicitly;
- source refresh creates a new snapshot identity unless byte-for-byte/content-addressed identity proves equivalence.

## Database rules

- driver sessions default read-only where supported;
- no DDL/DML foundation path;
- parameter binding for user values;
- typed metadata discovery, not raw interpolated SQL;
- per-query timeout/row/byte limits;
- pool keys include authority scope, source identity, and credential identity;
- cancellation closes or invalidates the active statement/session safely;
- server errors are redacted before durable logging;
- transaction/isolation/snapshot semantics are recorded where exposed.

## Local import rules

- stable-read/race detection between preview and import;
- digest after stable read;
- parser identity/version recorded;
- archive traversal, symlink escape, decompression-bomb, row/field/depth limits when archives/folders are admitted;
- imported raw material is preserved or content-addressed before lossy normalization.

## Kaggle/Hugging Face rules

- exact dataset identifier/version/revision/file selection recorded;
- no `trust_remote_code` equivalent in the trusted path;
- provider SDKs requiring Python run isolated or are replaced by narrow native HTTP/client adapters;
- terms/license metadata captured when provider exposes it;
- cancellation/resume where provider/protocol allows;
- provider credentials never enter agent/model/plugin context;
- acquired files quarantined before normal parser admission.

## CLI minimum

```text
medscale source add
medscale source list
medscale source inspect
medscale source health
medscale source preview
medscale source import
medscale source refresh
medscale snapshot list
medscale snapshot show
```

## Desktop minimum

First-class **Data Sources** workspace with:

```text
Connected
Local
Databases
Kaggle
Hugging Face
Imports
Snapshots
Credentials/Access
Health
```

UI must visibly distinguish `Live Preview` from `Frozen Snapshot`.

## Migration

Additive only. No existing artifact is rewritten as a DataSource. Existing imported datasets remain valid. Connector disable/removal must not invalidate already-materialized snapshots.

## Required failure tests

- wrong/expired credentials;
- denied network;
- provider rate limit;
- source changed between preview/import;
- schema drift;
- partial interrupted download;
- corrupt/truncated file;
- insufficient disk;
- DB timeout/cancel;
- SQL injection attempt;
- attempted DDL/DML;
- Kaggle/HF remote-code path denied;
- credential leakage scan.

## Closure gate
A researcher can create a Project-attached immutable snapshot from local data, a relational database, Kaggle, and Hugging Face with exact source/revision/schema/digest/provenance and no ambient credential or vault authority.

---

# 076 — Collaboration Substrate

## Hard dependency
074.

## Goal
Local-first rooms, threads, tasks, notes, approvals, activity, and participant identities that later sync through Hub.

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
NoteRevision
ApprovalRequest
ApprovalDecision
PresenceEvent
ParticipantIdentity
AgentParticipantIdentity
ActivityRecord
SyncCursor
```

## Rules

- collaboration references exact artifact IDs/revisions; it never owns canonical artifact payloads;
- messages are append-only plus explicit edits;
- task/note mutations are revision-guarded;
- note conflict preserves both revisions or forces explicit merge;
- presence/typing is ephemeral and not authority evidence;
- agents are first-class participants but not first-class authorities;
- Buzz patterns may be selectively adapted; Nostr/Buzz is not canonical research/clinical storage.

## Closure gate
Rooms/tasks/notes/activity work fully offline for one local user, with exact artifact references, conflict honesty, and no implicit canonical mutation.

---

# 077 — MedAgent Workbench

## Hard dependency
074. Integration with 076 is required for participant/activity features in the final product surface.

## Goal
A governed project-aware agent IDE with explicit context, tools, receipts, and local model execution.

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

## Minimum vertical slice
One Project, one admitted local model Pack, text prompt, explicit selected artifacts, zero network, cancel/interrupt, persisted output/receipt, visible model/runtime/data posture.

## Rules

- no ambient vault access;
- model output never directly calls Rust functions;
- every tool invocation is typed, policy-checked, executed outside the model, receipted, then returned;
- external/browser providers remain denied until owning later specs;
- runs bind exact context revisions and source snapshots.

## Closure gate
A local Project-grounded run completes audibly/auditably with exact context/model/tool provenance and no hidden network or authority widening.

---

# 078 — Model Fleet + Compare

## Hard dependency
077.

## Goal
Multiple bounded lanes plus observable comparison without declaring a winner or treating majority as truth.

## Contracts

```text
AgentLane
LanePolicy
LaneTransform
FleetRun
FleetRunState
LaneRunRef
ComparisonRequest
ComparisonObservation
ComparisonReport
```

## Rules

- each lane resolves its own context/tools/data/network grants;
- no permission union across lanes;
- external delegates default to PUBLIC or explicitly admitted deidentified input only;
- comparison reports evidence overlap, contradiction candidates, unsupported claims, abstention, schema validity, latency/resource/tool facts, and unknowns;
- no default quality score/winner.

## Closure gate
At least two distinct admitted lanes run the same task, partial failure is explicit, and comparison preserves evidence/unknown semantics.

---

# 079 — Privacy Gate

## Hard dependency
074 + 077.

## Goal
Cross-cutting classification, de-identification, pseudonymization, and egress decisions for all later network/Hub/Compute/source/extension paths.

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

## Rules

- source artifact immutable;
- deterministic + structured/FHIR-aware + admitted local model recognition;
- automated scanning never claims guaranteed absence of PHI/PII;
- reversible mappings use separate key material and explicit re-identification audit;
- hooks must exist for Browse, Data Sources export/write paths, Hub, Compute, R, Extensions, Analytics adapter materialization.

## Closure gate
Synthetic multilingual privacy fixtures prove fail-closed egress, transformation lineage, revocation, and no silent plaintext leakage.

---

# 080 — Governed Browse

## Hard dependency
077 + 079 + existing network broker authority.

## Goal
Evidence-bearing public/deidentified web research without leaking secrets or accepting web content as authority.

## Contracts

```text
BrowseRequest
BrowseIntentKind
BrowsePolicyDecision
BrowseRoute
BrowseSession
BrowseSessionState
BrowseNavigationStep
BrowseDownloadCandidate
BrowseEvidenceItem
BrowseReceipt
CredentialHandleRef
HumanTakeoverRequest
```

## Routing
HTTP/search first, deterministic browser second, agentic browser only when necessary and explicitly admitted.

## Foundation non-goals
No purchasing/submission/clinical-system writes; no unrestricted browser profile import; no sensitive raw Project context egress.

## Closure gate
A public/deidentified research browse produces inspectable evidence receipts while redirect/SSRF/prompt-injection/credential/download abuse tests fail closed.

---

# 081 — AudioFlow Foundation

## Hard dependency
077 + 079.

## Goal
Local-first audio capture/import, STT, diarization, timestamped transcript lineage, dictation, and explicit voice-command semantics.

## Contracts

```text
AudioSource
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
VoiceInputMode
TranscriptReceipt
```

## Rules

- no hidden cloud fallback;
- route selection evidence-based;
- source audio digest/time mapping preserved;
- transcript corrections create new revisions;
- `COMMAND`, `CONTEXT`, and `DICTATION` are explicit modes;
- speaker diarization labels do not imply biometric identity;
- VoiceStudio/Himsat/Wispral components qualify behind MedScale contracts.

## Closure gate
Live and imported local audio paths produce reproducible transcript/evidence lineage with explicit device/runtime/failure states.

---

# 082 — Analytics Gate

## Hard dependency
074 + 075 + 079.

## Goal
Native reproducible analysis over exact snapshots/governed views.

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

## Rules

- governed views bind exact `DataSnapshot` revisions;
- DataFusion/Arrow qualifies against required workloads; alternatives stay behind same contract;
- AI may propose SQL, never bypass parser/planner policy;
- default query path rejects writes/DDL;
- arbitrary R/Python/shell waits for Compute/R Workspace;
- derived artifacts inherit classification and provenance.

## Closure gate
Read-only SQL/cohort/statistical workflows reproduce from exact snapshots with validated fixtures, cancellation, stale-source honesty, and receipts.

---

# 083 — Knowledge + Research Canvas

## Hard dependency
074 + 075 + 077 + 079.

## Goal
Permission-aware retrieval, evidence graph linkage, literature workflow, and visual Research Canvas.

## Contracts

```text
IndexManifest
IndexSourceRef
ChunkRef
RetrievalPlan
RetrievalResult
RetrievalReceipt
Canvas
CanvasNode
CanvasEdge
EvidenceSpanRef
```

## Rules

- authorization before retrieval/cache reuse;
- lexical/local index first; vector only if benchmark justifies;
- source snapshot/document/audio/web revision pinned;
- stale/tombstoned index state explicit;
- Canvas stores live references, not silent copies;
- insufficient evidence is a first-class result.

## Closure gate
Project retrieval survives restart/rebuild, preserves exact evidence spans/revisions, and cross-project leak tests fail closed.

---

# 084 — MedScale Hub

## Hard dependency
074 + 076 + 079.

## Goal
Self-hosted multi-device lab/team sync without mandatory MedScale cloud.

## Contracts

```text
HubIdentity
DeviceIdentity
HubHandshake
TenantScope
ProjectMembership
SyncEnvelope
SyncCursor
SyncConflict
ObjectTransferManifest
RevocationEvent
HubAuditCheckpoint
```

## Rules

- single-Hub topology first;
- tenant/project scope before DB/search/cache/pubsub/object lookup;
- resumable policy-approved object transfer;
- idempotent submission/monotonic cursors;
- offline conflict/reconnect explicit;
- revocation blocks/quarantines late work;
- operator trust model explicit; do not claim zero-knowledge without proof;
- backup/restore and schema rollback required.

## Closure gate
Two clients demonstrate offline edits, reconnect, conflict handling, revocation, tenant isolation, backup/restore, and no mandatory cloud dependency.

---

# 085 — MedScale Compute

## Hard dependency
077 + 079.

## Goal
Bounded local/lab worker execution without mounting the vault or widening credentials.

## Contracts

```text
ComputeJobManifest
ComputeCapabilityProfile
WorkerIdentity
WorkerLease
InputLease
Heartbeat
OutputCandidate
ComputeReceipt
```

## Rules

- exact input staging only;
- no canonical vault mount/master key;
- explicit filesystem/network/secret/resource policy;
- timeout/OOM/crash/cancel/lost/late states;
- output digest/schema validation before Core admission;
- reproducible runtime/environment identity;
- sandbox selection evidence-based.

## Closure gate
Local worker survives adversarial filesystem/network/output/crash/OOM/timeout tests and returns only validated candidates.

---

# 086 — R Workspace

## Hard dependency
075 + 079 + 082 + 085.

## Goal
First-class reproducible R/RStudio/Posit workflow without embedding unrestricted R in trusted Desktop/Core.

## Contracts

```text
RWorkspaceManifest
RWorkspaceState
RRuntimeIdentity
RLockfileRef
RLaunchRequest
RLaunchReceipt
RRunRequest
RRunReceipt
RPublishRequest
RPublishReceipt
```

## Foundation modes

1. `Open in RStudio` external process;
2. `Open in Positron` external process;
3. `Open Folder`;
4. `Run R Script` only through Compute.

## Workspace rules

- exact staged `DataSnapshot` inputs;
- read-only staging where feasible;
- no vault DB/key/credential store mounted;
- `.Rproj`, `renv.lock`, R version, platform, script digest captured;
- workspace regeneration deterministic within external package availability constraints;
- no ambient directory watcher auto-import.

## Publication

```text
external output
-> validation
-> digest/schema
-> classification
-> output contract
-> Core admission
-> Project artifact
-> RRunReceipt/RPublishReceipt
```

## Failure tests

R missing, IDE missing, lock restore failure, package unavailable, network denied, script crash, malicious output, output contract mismatch, stale input snapshot, cancelled Compute run.

## Closure gate
A user launches a reproducible R workspace from exact MedScale snapshots, runs/publishes output explicitly, and no R process receives ambient vault authority.

---

# 087 — Community Extensions

## Hard dependency
079 + 084 + 085. Data-source extension integration additionally depends on 075.

## Goal
Obsidian-like discoverability/community contribution with healthcare-grade capability isolation.

## Contracts

```text
ExtensionPack
ExtensionManifest
ExtensionPublisher
ExtensionCapability
ExtensionGrant
ExtensionInstallRecord
ExtensionRuntimeIdentity
ExtensionRuntimeReceipt
ExtensionRegistryEntry
ExtensionRevocation
HostApiVersion
```

## Extension classes

- declarative command/action;
- settings/schema;
- read-only panel/visualization spec;
- data-source provider;
- importer/exporter;
- MedAgent tool;
- analysis action;
- Research Pack augmentation;
- AudioFlow processor;
- institutional adapter.

## Runtime rules

- default capabilities empty;
- no vault DB/key/keychain/arbitrary filesystem/network/clipboard/mic/camera/Project data/Core mutation access;
- typed, scoped, revocable grants with data-class ceiling;
- executable code runs in qualified WASM sandbox or isolated worker;
- no arbitrary native library injection into Desktop/Core;
- UI contributions are declarative/typed, not arbitrary trusted web/native code;
- extensions cannot bypass owning Data Source/Browse/Analytics/Audio/Compute/Privacy authority.

## Install/update pipeline

```text
acquire
-> digest
-> signature/provenance
-> API compatibility
-> license/SBOM/dependency scan
-> static/malware policy
-> requested capability diff
-> user/admin consent
-> sandbox activation
```

Capability expansion on update requires re-consent. Self-update without MedScale admission is forbidden.

## Registry rules

- immutable release identity/digest;
- publisher identity;
- compatibility/API version;
- requested capabilities visible before install;
- revocation/quarantine state;
- offline/manual verified installation remains possible;
- registry listing does not imply medical correctness or safety certification.

## Developer mode
Developer mode may admit unsigned/local extensions only under a visibly separate trust posture, with explicit warnings and no silent promotion to normal/community trust.

## Closure gate
A third-party sample extension can be built, packaged, installed, denied a non-granted capability, upgraded with capability re-consent, revoked, and run without direct trusted-process/vault access.

---

# 088 — AudioFlow Advanced

## Hard dependency
081 + 084. Compute integration when used additionally depends on 085.

## Goal
Team huddles, TTS/listen-back, duplex agent voice, media annotations, batch audio workflows, optional voice design/cloning under explicit consent.

## Rules

Join/record/transcribe/export permissions are distinct. Synthetic voice/agent audio is labeled. Persistent speaker identity is opt-in, encrypted, deletable. Voice cloning requires subject permission/provenance.

## Closure gate
Multi-user huddle + transcript + task/evidence proposals work with consent separation, interruption, labeling, retention, and deletion proof.

---

# 089 — Research Packs

## Hard dependency
074 + 075 + 077 + 082 + 083. Executable/heavy Pack features also require 085/087 as appropriate.

## Goal
Domain semantics without Core bloat or hidden executable trust.

## Contracts

```text
ResearchPackManifest
ResearchArtifactSchema
ResearchWorkflowDescriptor
ResearchViewDescriptor
ResearchEvidenceRule
ResearchPackMigration
ResearchPackDependency
```

## Rules

Research Packs are domain/schema/workflow packages; Extensions are executable/integration packages. Installing a Research Pack never silently installs/grants executable Extensions. Domain proof order begins Clinical Research, AI Research, Systematic Review, then Imaging/Omics/Wet Lab.

## Closure gate
At least one domain Pack adds real workflows/artifact semantics with migration/uninstall preservation and no alternate authority plane.

---

# 090 — Institutional Adapters

## Hard dependency
075 + 079 + 084 + 085 + 089. 087 required when an adapter is community-delivered executable code.

## Goal
Optional organizational integration: SSO, object storage, LIMS/ELN, EHR/FHIR, HPC, model registry, BI/search services, Posit Workbench, and approved side-effecting connectors.

## Rules

Every adapter declares destination, data class, credential handle, capabilities, effect state, provenance, retry/idempotency semantics, and `Unknown` remote-write state where confirmation is incomplete. Personal mode never requires institutional services.

## Closure gate
At least one identity/storage/compute institutional path is proven end-to-end with outage/revocation/rollback and no authority split.

---

# 091 — Federation

## Hard dependency
090.

## Goal
Controlled cross-institution project/data/analysis exchange without requiring a central MedScale data cloud.

## Required research/implementation gates

- institution identity/trust anchors;
- signed export/project bundles;
- explicit data-sharing policy/consent;
- provenance preservation across institutions;
- revocation/tombstone semantics;
- partial/federated analysis only after threat/privacy validation;
- no automatic raw-PHI federation.

## Closure gate
A bounded multi-institution synthetic scenario proves signed exchange, provenance, revocation, policy denial, and no hidden central authority.

---

# 092 — Whole-Platform Qualification

## Hard dependency
074-091 as actually promoted/implemented for the target release profile.

## Goal
Integrated qualification of Research OS as a coherent product rather than separately passing features.

## Required campaigns

- Project/artifact migration/recovery;
- Data Source import/snapshot/refresh across local/DB/Kaggle/HF;
- MedAgent/Fleet/Privacy/Browse interplay;
- AudioFlow evidence and consent;
- Analytics/R/Knowledge reproducibility;
- Hub offline/conflict/revocation;
- Compute isolation;
- Extension capability escape/update/revocation;
- Research Pack migrations;
- institutional adapter outage/rollback;
- accessibility and minimum-window behavior;
- Windows/Linux/macOS qualification as applicable;
- synthetic load/soak;
- exact-head security/supply-chain CI;
- release/rollback/backup restore.

## Terminal rule
No subsystem's local PASS substitutes for integrated evidence. Pending/external/unavailable stays explicit. Whole-platform qualification may establish a release profile only for the exact features/platforms/data classes actually proven.
