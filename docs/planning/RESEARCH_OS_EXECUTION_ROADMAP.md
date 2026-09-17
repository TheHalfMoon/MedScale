# MedScale Research OS Execution Roadmap

**Status:** Candidate roadmap only. This document does not promote or authorize a specification.

## 1. Sequencing principle

The Research OS expansion is a dependency graph, not one giant feature branch and not a blindly serial queue.

The Project/Artifact/Authority substrate must land first. After the cross-cutting privacy boundary exists, bounded product planes may advance in parallel only when their contracts are independent and their predecessor evidence is closed.

The numbering below assumes canonical MedScale remains closed through Spec 073 at promotion time. Before promotion, reverify live governance and renumber if the canonical frontier moved. Never reuse a number already owned by live main.

## 2. Program dependency graph

```text
074 Project + Artifact Graph Foundation
  |
  +--> 075 Collaboration Substrate
  |      |
  |      +-------------------------------> 083 MedScale Hub
  |
  +--> 076 MedAgent Workbench
         |
         +--> 077 Model Fleet + Compare
         |
         +--> 078 Privacy Gate
                |
                +--> 079 Governed Browse
                +--> 080 AudioFlow Foundation
                +--> 081 Analytics Gate
                +--> 082 Knowledge + Research Canvas
                +--> 085 MedScale Compute
                |
                +--> 083 MedScale Hub  (also requires 075)

080 + 083 -------------------------------> 084 AudioFlow Advanced
074 + 076 + 081 + 082 -------------------> 086 Research Packs
078 + 083 + 085 + 086 -------------------> 087 Institutional Adapters
087 -------------------------------------> 088 Federation
074-088 ---------------------------------> 089 Whole-Platform Qualification
```

Integration dependencies are stricter than build dependencies. Example: Knowledge may implement local document retrieval before AudioFlow closes, but audio-timestamp retrieval is not qualified until both 080 and 082 integration evidence exists.

## 3. Parallelism rules

Parallel work is allowed only when:

1. predecessor contracts are closed on canonical main;
2. neither branch edits the same authority/storage schema without an agreed contract;
3. each branch has independent migrations/recovery/evidence;
4. integration acceptance is explicitly assigned to one owning spec;
5. no branch assumes an unmerged sibling behavior.

Default: prefer sequential promotion for 074-078. After 078, 079/080/081/082/083/085 may be shaped independently, but implementation concurrency is decided from live changed-path/dependency truth.

---

# 074 — Project + Artifact Graph Foundation

## Goal
Create the durable organizing ontology on which the expansion depends without replacing existing MedScale authority objects.

## Required outcomes
- `Project`, `Experiment`, `ArtifactDescriptor`, typed Project Graph relations and Project context contracts;
- reuse existing `OpaqueId`, `ObjectHeader`, digest/provenance/audit semantics;
- stable revisions and project ownership references;
- existing specialized/FHIR/Pack/authority objects are referenced, not flattened or copied;
- offline Project workspace requires no Hub/network;
- migration from pre-074 vault preserves current CLI/Desktop semantics;
- archive/tombstone behavior and recovery are explicit;
- CLI + native Desktop project vertical slice.

## Non-goals
No team server, agent IDE, analytics, audio or RAG implementation.

## Closure focus
Deterministic lifecycle, migration/reopen, provenance, compatibility, graph integrity, no duplicate authority/ID model.

---

# 075 — Collaboration Substrate

## Depends on
074.

## Goal
Create local-first collaboration semantics that later sync through Hub: rooms, threads, tasks, notes, activity, approvals and participant identities.

## Required outcomes
- MedScale-owned collaboration event envelope;
- rooms/threads/messages/tasks/note revisions/approval contracts;
- human/service/agent participant identity references without granting clinical/research authority;
- exact artifact revision references;
- optimistic concurrency for tasks;
- conflict-copy/user-resolution behavior for note documents;
- ephemeral presence separated from durable history;
- searchable local activity;
- tamper-evident audit/checkpoints;
- selective Buzz donor qualification if code/patterns are transferred.

## Non-goals
No mandatory Hub, no CRDT, no Nostr replacement for canonical data, no Tauri/React shell.

## Closure focus
Local single-user/offline collaboration works; collaboration cannot implicitly mutate canonical artifacts.

---

# 076 — MedAgent Workbench

## Depends on
074. 075 participant/activity integration is preferred before final closure; a promoted spec must state whether it is a hard predecessor based on live contracts.

## Goal
Make MedAgent the governed intelligence workspace inside a Project.

## Required outcomes
- split-pane agent IDE/workbench;
- explicit `ContextManifest` rather than ambient project/vault access;
- one admitted local model Pack lane;
- typed tool manifests/invocations/receipts;
- proposal/evidence-only model output semantics;
- run state machine and RunReceipt;
- inspectable model/runtime/data/network posture;
- cancel/interrupt/steer;
- run history as Project artifacts;
- browser/external providers remain denied until 079.

## Closure focus
A real local project-grounded run completes without network, ambient vault access or hidden authority.

---

# 077 — Model Fleet + Compare

## Depends on
076.

## Goal
Run multiple bounded lanes and compare observable results without inventing a winner or treating consensus as truth.

## Required outcomes
- `AgentLane`, `FleetRun`, lane-specific context/tool/data/network policy;
- multiple admitted local model lanes;
- same-task and role-specialized lane semantics;
- optional delegate adapters only on policy-approved non-sensitive/transformed data;
- side-by-side results;
- agreement/disagreement, contradiction candidates, evidence/citation overlap, unsupported-claim candidates, abstention, schema validity, tool/runtime/resource facts;
- partial lane failure remains explicit;
- comparison artifact inherits the most restrictive participating input/output classification unless a typed privacy transformation proves otherwise.

## Closure focus
No permission union across lanes; unknown stays unknown; no default quality/intelligence score.

---

# 078 — Privacy Gate

## Depends on
074 + 076. 077 is not a hard prerequisite.

## Goal
Make sensitive-data classification, de-identification and egress decisions a reusable product boundary.

## Required outcomes
- canonical data-class semantics (`LOCAL_PHI`, `TEAM_PROTECTED`, `EXTERNAL_DEIDENTIFIED`, `PUBLIC` or canonically refined equivalents);
- deterministic + structured/FHIR-aware + admitted local model recognizers;
- redact/tokenize/generalize/pseudonymize/drop transformations under policy;
- immutable transformed artifacts; source never overwritten;
- residual scan + explicit uncertainty;
- `DeidReceipt` and re-identification audit capability;
- policy enforcement hooks for model, browser, export, Hub, compute, connector and analytics-adapter boundaries;
- synthetic/permitted multilingual benchmark corpus;
- no product claim that automated scanning proves all PHI/PII absent.

## Closure focus
Fail-closed boundary decisions, classification propagation, reversible-map key separation, denial tests.

---

# 079 — Governed Browse

## Depends on
076 + 078 + existing MedScale Network Broker authority.

## Goal
Give MedAgent/research workflows web search and browser capability without bypassing privacy, network authority or evidence provenance.

## Foundation scope
Read/retrieve/research first. External side-effecting browser actions are not foundation behavior.

## Required outcomes
- `BrowseRequest`, `BrowsePolicy`, `BrowseRoute`, `BrowseEvidence`, `BrowseReceipt` contracts;
- search/HTTP acquisition through admitted network broker;
- deterministic browser automation (Playwright-class) behind a bounded worker/adapter when required;
- agentic browser fallback only when deterministic navigation is insufficient and explicitly permitted;
- browser context is hostile/untrusted input and cannot change instructions/capabilities;
- `PUBLIC` may browse under network policy;
- `EXTERNAL_DEIDENTIFIED` requires valid transform/egress decision;
- `LOCAL_PHI` and `TEAM_PROTECTED` are denied from public browsing routes by default;
- credential handles stay outside model prompts; login/MFA uses explicit human takeover/session boundary;
- exact URL/source, retrieval time, route/tool identity, content hash where obtainable, selected evidence spans and snapshot/retention policy;
- robots/terms/site-specific constraints represented by adapter policy where applicable;
- cancel/timeout/redirect/download/file-type limits;
- download ingestion returns quarantined candidate/source bytes for normal MedScale validation rather than trusted content;
- CLI + MedAgent Browse panel/activity + evidence linking.

## Explicit non-goals
- no general autonomous purchasing/submission/clinical-system writes;
- no unrestricted browser profile/cookie import;
- no hidden credential extraction;
- no public browsing of sensitive raw project context;
- no claim that web content is clinical truth.

## Closure focus
Public/deidentified research retrieval works with receipts and prompt-injection/egress/credential/redirect/download abuse tests. Sensitive egress fails closed.

---

# 080 — AudioFlow Foundation

## Depends on
076 + 078.

## Goal
Add local-first audio capture, transcription, diarization, dictation and voice control as evidence-bearing Project capabilities.

## Required outcomes
- `AudioSession`, `AudioSource`, `TranscriptRevision`, `DiarizationRevision`, `AudioEvidenceRef`;
- native capture/control contract and imported audio/video path;
- visible capture state and capture-health semantics;
- source digest + retention policy;
- conditioning/VAD ownership;
- evidence-selected `VoiceRuntimeRouter` routes;
- at least one qualified streaming local STT path and one offline-quality path;
- diarization/alignment;
- medical number/unit/negation terminology benchmark where claimed;
- Arabic + Arabic-English code-switch evaluation dimension;
- non-destructive transcript revision ledger;
- explicit `COMMAND` / `CONTEXT` / `DICTATION` semantics for MedAgent;
- interruption/cancellation;
- VoiceStudio/Himsat/Wispral donor qualification;
- no default wake word/always-listening.

## Closure focus
No silent cloud fallback; route unavailable is explicit; source audio and transcript lineage remain reproducible.

---

# 081 — Analytics Gate

## Depends on
074 + 078.

## Goal
Provide reproducible native data analysis without making a BI server or unrestricted notebook a Desktop dependency.

## Required outcomes
- governed analytical views over exact Project artifact revisions;
- Arrow/Parquet interchange where appropriate;
- DataFusion qualification or documented bounded alternative behind same contract;
- SQL editor + natural-language query proposal path;
- parser/planner default-deny for writes/DDL;
- cohort/query builder;
- `QueryReceipt` with exact SQL/plan/input revisions/engine/config;
- derived Dataset/Table/Figure artifacts with classification propagation;
- statistical operations with independent correctness fixtures and explicit assumption/not-run state;
- native table/chart surfaces;
- privacy enforcement before adapter/export materialization;
- arbitrary Python/R/shell deferred to 085 Compute.

## Closure focus
Reproducibility, read-only default, cancellation/timeouts, source-revision pinning, independent statistical correctness.

---

# 082 — Knowledge + Research Canvas

## Depends on
074 + 076 + 078.

## Goal
Create permission-aware project knowledge, retrieval and visual research composition without treating vector search as authority.

## Required outcomes
- `IndexManifest`, source/chunk/parser identity and stale/tombstone semantics;
- embedded/local lexical search first;
- vector retrieval only if benchmark demonstrates value;
- structured Project Graph + lexical + optional vector retrieval plan;
- authorization before disclosure and cache reuse;
- exact page/span/revision evidence links;
- `RetrievalReceipt`;
- Research Canvas with live artifact references rather than silent copies;
- literature/library workflow;
- MedAgent grounding over exact evidence;
- explicit insufficient-evidence state;
- contradiction/missing-evidence inspection.

## Integration gates
- web evidence is indexed only after 079 closes and its BrowseReceipt contract is admitted;
- audio timestamp evidence is indexed only after 080 closes;
- analytics derived artifacts are indexed only after 081 closes.

## Closure focus
No cross-project leaks, stale-index honesty, deletion/tombstone propagation, evidence spans survive restart/rebuild.

---

# 083 — MedScale Hub

## Depends on
074 + 075 + 078.

## Goal
Enable user-controlled multi-device lab/team collaboration without a mandatory MedScale cloud.

## Required outcomes
- self-hostable single-Hub topology first;
- team/project membership + invitations;
- versioned Hub handshake and device identity;
- sync of rooms/threads/tasks/note revisions/activity;
- artifact metadata + policy-approved resumable object transfer;
- monotonic cursors/idempotent submission/conflict responses;
- tenant/project scope applied before DB/search/cache/pubsub/object lookup;
- protected Hub data encrypted at rest; transport encryption mandatory outside loopback/dev;
- explicit operator trust statement (not zero-knowledge unless a later E2EE design proves it);
- revocation propagation; late results/data quarantined or denied;
- deletion/tombstone propagation with honest backup-retention semantics and no silent resurrection;
- workflow/audit coordination;
- offline-first reconnect/conflict behavior;
- backup/restore + schema upgrade/rollback;
- Personal single-laptop mode remains fully valid.

## Candidate deployments
Workstation, LAN server, on-prem server, private VPC.

## Closure focus
Two-client offline/conflict/reconnect, revocation, tenant isolation, malicious digest, backup/restore, upgrade rollback.

---

# 084 — AudioFlow Advanced

## Depends on
080 + 075 + 083 for shared huddle behavior.

## Goal
Turn AudioFlow into full team audio intelligence after capture/transcript/Hub foundations are proven.

## Required outcomes
- Project audio huddles;
- humans + explicitly scoped agents in audio sessions;
- permission to join is distinct from permission to record/transcribe/export;
- live transcript/evidence/task proposals;
- TTS/listen-back;
- optional duplex agent voice;
- voice design/cloning only with explicit subject permission/provenance;
- synthetic-agent/voice output labeling and export provenance/watermark policy;
- media time/frame annotations;
- batch workflows;
- optional remote audio workers through Compute once available;
- persistent speaker identity opt-in/encrypted/deletable.

## Closure focus
Consent/permission separation, duplex interruption, synthetic-origin labeling, retention/deletion and long-session team behavior.

---

# 085 — MedScale Compute

## Depends on
076 + 078. Local worker foundation does not require Hub; remote lab-worker integration may later integrate 083.

## Goal
Scale bounded jobs from local CPU/GPU to lab/institutional compute without widening vault authority.

## Required outcomes
- `ComputeJobManifest`, worker capability/lease/heartbeat/input lease/output candidate/receipt contracts;
- local bounded worker first;
- exact input staging; no vault mount/ambient key store;
- filesystem/network/secret/resource policy;
- timeout/OOM/crash/cancel/lost/partial semantics;
- output schema/digest validation before Core admission;
- late completion after revocation quarantined;
- reproducible runtime/environment identity;
- log redaction;
- self-hosted remote worker after local proof;
- container/OpenSandbox/OS sandbox choice evidence-selected by workload/platform;
- institutional schedulers deferred to 087.

## Closure focus
Filesystem escape, egress denial, malicious output, duplicate/late completion, crash/OOM/timeout and reproducible output evidence.

---

# 086 — Research Packs

## Depends on
074 + 076 + 081 + 082. 085 is required only for Pack features that execute arbitrary/heavy worker code.

## Goal
Expand to lab/research domains without bloating or weakening Core.

## Initial proof order
1. Clinical Research;
2. AI Research;
3. Systematic Review;
4. Imaging;
5. Omics;
6. Wet Lab.

## Required outcomes
- versioned declarative `ResearchPackManifest`;
- artifact schema/validation/workflow/view descriptors;
- tool/model requirements;
- evidence semantics;
- import/export boundaries;
- migrations/upgrades;
- uninstall disables Pack but preserves domain data until explicit export/delete;
- no arbitrary trusted-process UI/native code by default;
- executable extension uses normal tool/worker authority;
- clear distinction from executable/model Packs in naming/contracts/UI.

## Closure focus
At least the first Pack proves domain extension without alternate authority, unsafe UI plugins or destructive migration.

---

# 087 — Institutional Adapters

## Depends on
078 + 083 + 085 + 086.

## Goal
Connect MedScale to organizational infrastructure through explicit adapters without making them required for Personal/Lab operation.

## Candidate adapter families
- institutional identity/SSO binding to stable local MedScale identity;
- object storage;
- LIMS/ELN;
- EHR/FHIR systems;
- HPC/Slurm/Kubernetes;
- institutional model registry;
- Superset/BI over approved materialized views;
- OpenRAG/OpenSearch-class scale services;
- notification/communication systems;
- approved side-effecting browser/connector workflows where APIs are unavailable.

## Required posture
Every adapter gets destination/data-class/capability/credential/effect-state/provenance contracts. External writes preserve `Unknown` when confirmation is uncertain and never retry blindly.

---

# 088 — Federation

## Depends on
087.

## Goal
Research and, only if proven, enable controlled cross-institution collaboration without requiring a central MedScale data cloud.

## Research/qualification areas
- signed project/artifact bundles;
- cross-institution identity/trust;
- minimal policy vocabulary and unknown-policy deny;
- data-stays-at-site default;
- federated query/aggregate/model tasks only when statistically/privacy valid;
- provenance across sites;
- consent/data-use/retention/export constraints;
- revocation/expiry;
- result aggregation without hidden source loss.

Federation remains non-release-blocking until explicitly promoted by product governance.

---

# 089 — Whole-Platform Qualification

## Depends on
Every Research OS capability claimed for the target release. Deferred optional features are listed explicitly rather than faked as qualified.

## Goal
Qualify the integrated Research OS, not merely the existence of feature code.

## Required evidence families
- authority/capability bypass tests;
- privacy/PHI/PII boundary tests;
- model/agent/fleet authority and context isolation;
- browser prompt-injection/egress/credential/download abuse;
- Hub tenancy/auth/revocation/conflict/deletion/backup;
- AudioFlow capture/privacy/quality/long-session behavior on declared populations/tasks;
- Analytics correctness/reproducibility;
- Knowledge permission/staleness/evidence grounding;
- collaboration concurrency/recovery;
- Compute isolation;
- migration/backup/restore/upgrade rollback;
- Research Pack migration/removal/security;
- accessibility;
- performance/resource envelopes by deployment tier;
- donor/license/SBOM/provenance closure;
- platform packaging/signing where applicable;
- explicit unsupported/external/deferred axes.

## Release statement rule
No Research OS release claim may be broader than the exact deployment tiers, platforms, data classes and evidence qualified by this unit.

---

## 4. Promotion rule

This roadmap is not self-authorizing. A candidate unit becomes executable only when canonical governance promotes a bounded specification using the Future Spec Template and exact live repository truth.

A promotion must:

1. verify the number/name is unused;
2. bind exact predecessor closure evidence;
3. bind exact current crate/module/storage paths;
4. close architecture defaults through the Decision Resolution Register;
5. identify any evidence-selected dependencies and their qualification harness;
6. define migration/recovery and test/evidence commands;
7. leave adjacent candidate units non-executable.

Live repository truth always overrides this roadmap.
