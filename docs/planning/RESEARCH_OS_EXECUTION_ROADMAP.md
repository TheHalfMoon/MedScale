# MedScale Research OS Execution Roadmap V2

**Status:** Candidate roadmap only. This document does not itself promote or authorize a specification.  
**Amendment authority:** `RESEARCH_OS_PROGRAM_AMENDMENT_001_DATA_EXTENSIONS.md`  
**Current promoted Research OS unit:** Spec 074 only, through its separate founder promotion.  
**Candidate future range:** 075-092, subject to live frontier reconciliation at every promotion.

## 1. Sequencing principle

Research OS is a dependency graph, not one giant feature branch.

The durable Project/Artifact substrate lands first. The Data Source Fabric then gives every later plane one governed way to discover, preview, import, snapshot, classify, and reference data. Collaboration, MedAgent, privacy, browsing, audio, analytics, knowledge, Hub, Compute, R, community extensions, domain Packs, institutional adapters and federation build on those foundations without creating alternate authority planes.

Every promoted spec must reverify live `main`, bind candidate contracts to actual repository paths/types, and obtain separate implementation authority. Candidate numbering is never authority by itself.

## 2. Program dependency graph

```text
074 Project + Artifact Graph Foundation                  [PROMOTED separately]
 |
 +--> 075 Data Source Fabric
 |
 +--> 076 Collaboration Substrate ---------------------> 084 MedScale Hub
 |
 +--> 077 MedAgent Workbench --> 078 Model Fleet + Compare
 |       |
 |       +--> 079 Privacy Gate
 |               |
 |               +--> 080 Governed Browse
 |               +--> 081 AudioFlow Foundation
 |               +--> 082 Analytics Gate <----------- 075
 |               +--> 083 Knowledge + Research Canvas <- 075
 |               +--> 084 MedScale Hub <------------- 076
 |               +--> 085 MedScale Compute
 |
075 + 079 + 082 + 085 --------------------------------> 086 R Workspace
079 + 084 + 085 --------------------------------------> 087 Community Extensions
081 + 084 --------------------------------------------> 088 AudioFlow Advanced
074 + 075 + 077 + 082 + 083 --------------------------> 089 Research Packs
075 + 079 + 084 + 085 + 089 --------------------------> 090 Institutional Adapters
090 ---------------------------------------------------> 091 Federation
074-091 -----------------------------------------------> 092 Whole-Platform Qualification
```

Integration dependencies are stricter than implementation dependencies. A subsystem may build its local foundation once hard predecessors close, but cross-plane features are not qualified until every owning predecessor contract is canonical.

## 3. Parallelism rules

Parallel work is allowed only when all of the following are true:

1. every hard predecessor is closed on canonical `main`;
2. branches do not independently mutate the same authority/storage schema without a frozen shared contract;
3. every branch has its own migration/recovery/evidence lane;
4. integration acceptance is assigned to one owning spec;
5. no branch assumes unmerged sibling behavior;
6. shared contract changes are merged before dependent implementation begins;
7. runtime capability expansion is not inferred from planning prose.

Default: prefer sequential promotion through 079. After Privacy Gate closes, 080/081/082/083/085 may be shaped independently when changed-path and contract truth permit it.

---

# 074 — Project + Artifact Graph Foundation

## Goal
Create the durable organizing ontology on which the Research OS expansion depends without replacing existing MedScale authority objects.

## Required outcomes
- `Project`, `Experiment`, artifact references, bounded typed Project Graph relations, Project context and summary contracts;
- reuse existing ID, scope, digest, audit and provenance semantics;
- stable optimistic revisions;
- specialized/FHIR/Pack/evidence objects are referenced, not flattened or copied;
- encrypted local persistence and migration/reopen/recovery;
- CLI and native Desktop vertical slice;
- Personal mode remains fully offline.

## Non-goals
No databases/connectors, team server, MedAgent, analytics, audio, RAG, R, plugin ecosystem, Hub or network expansion.

## Closure focus
Lifecycle, migration/reopen, compatibility, graph integrity, bounded traversal, no duplicate authority/ID/provenance model.

---

# 075 — Data Source Fabric

## Depends on
074.

## Goal
Create one governed acquisition/connection model for local files, databases and approved remote dataset providers so every later data-consuming subsystem uses the same source identity, credential, snapshot, schema and provenance contracts.

## Foundation adapters
1. local files/folders: CSV/TSV, Parquet, Arrow IPC/Feather where qualified, JSON/JSONL;
2. local data engines/files: external SQLite source and evidence-qualified DuckDB path;
3. relational read connectors: PostgreSQL, MySQL/MariaDB, SQL Server; generic ODBC later if justified;
4. Kaggle dataset acquisition;
5. Hugging Face Hub dataset acquisition.

## Required contracts
- `DataSourceId`;
- `DataSourceKind`;
- `DataSourceManifest`;
- `CredentialRef`;
- `SourceLocation`;
- `SourceCapability`;
- `SourceSchema` / `SchemaFingerprint`;
- `SourceRevision`;
- `AcquisitionMode`;
- `ImportPlan`;
- `DataSnapshot`;
- `ImportReceipt` / `RefreshReceipt`;
- `SourceHealth`;
- explicit mutable/live versus immutable/snapshotted semantics.

## Required behavior
- first-class Desktop **Data Sources** workspace;
- provider-specific setup over one MedScale-owned contract;
- secrets stored only behind the admitted secret-store boundary;
- read/discover/preview/import first; no default remote writes;
- bounded schema/table/file discovery and preview;
- exact source/version/revision/file selection where provider supports it;
- disk-size and row/file estimates before materialization when available;
- hostile-input validation and quarantine before admission;
- immutable snapshot lineage for reproducible analysis;
- classification/policy attached to imports and snapshots;
- source revision, schema fingerprint, adapter identity/version and digests captured in receipts;
- attach resulting snapshots/references to Projects through 074 contracts;
- cache/temp files follow explicit retention/cleanup policy;
- provider outage/rate limit/auth expiry/partial download/corruption are explicit states.

## Database rules
- foundation is read-only;
- connection string metadata and `CredentialRef` are separate;
- passwords/tokens never appear in logs, project manifests, prompts or receipts;
- schema/table discovery is permission-bounded;
- user-authored read query support, if admitted, must reject DDL/DML/multi-statement escape by default;
- snapshot materialization records exact query/table selection and transaction/isolation facts available from the source;
- live preview is not automatically reproducible evidence.

## Kaggle/Hugging Face rules
- exact dataset identifier + version/revision/files are recorded where obtainable;
- inspect metadata/license/card before acquisition when provider exposes them;
- no arbitrary provider-supplied code in the trusted import path;
- no `trust_remote_code`-class behavior by default;
- Python provider clients, if required, run behind a bounded adapter/worker rather than becoming a trusted Desktop dependency;
- credentials remain opaque handles;
- external dataset content is evidence/source material, not clinical truth.

## Non-goals
No database writes, Kaggle submission, HF push/upload, warehouse administration, ETL orchestration platform, arbitrary remote code, or mandatory cloud dependency.

## Closure focus
Offline local import, database read/snapshot, Kaggle and HF exact-version acquisition, credential safety, hostile-input handling, interrupted/resumed acquisition where supported, reproducibility receipts and no hidden network fallback.

---

# 076 — Collaboration Substrate

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
No mandatory Hub, no Nostr replacement for canonical data, no blanket CRDT requirement, no WebView shell.

## Closure focus
Local single-user/offline collaboration works; collaboration cannot implicitly mutate canonical artifacts.

---

# 077 — MedAgent Workbench

## Depends on
074. Integration with 076 participant/activity semantics should be bound before final closure if live contracts require it.

## Goal
Make MedAgent the governed intelligence workspace inside a Project.

## Required outcomes
- split-pane agent IDE/workbench;
- explicit `ContextManifest` rather than ambient vault access;
- one admitted local model Pack lane;
- typed tool manifests/invocations/receipts;
- proposal/evidence-only output semantics;
- run state machine and RunReceipt;
- inspectable model/runtime/data/network posture;
- cancel/interrupt/steer;
- run history as Project artifacts;
- browser/external providers remain denied until 080;
- source snapshots from 075 may be selected only by explicit Project/context references.

## Closure focus
A real local project-grounded run completes without network, ambient vault access or hidden authority.

---

# 078 — Model Fleet + Compare

## Depends on
077.

## Goal
Run multiple bounded lanes and compare observable results without inventing a winner or treating consensus as truth.

## Required outcomes
- `AgentLane`, `FleetRun`, lane-specific context/tool/data/network policy;
- multiple admitted local model lanes;
- same-task and role-specialized lane semantics;
- optional delegate adapters only on policy-approved non-sensitive/transformed data;
- side-by-side results;
- agreement/disagreement, contradiction candidates, evidence/citation overlap, unsupported-claim candidates, abstention, schema validity and resource/runtime facts;
- partial lane failure remains explicit;
- comparison inherits the most restrictive participating classification unless a typed privacy transform proves otherwise.

## Closure focus
No permission union across lanes; unknown stays unknown; no default intelligence/quality score.

---

# 079 — Privacy Gate

## Depends on
074 + 077. 078 is not a hard prerequisite.

## Goal
Make data classification, de-identification and egress decisions a reusable boundary for models, sources, browsing, Hub, Compute, exports, R and extensions.

## Required outcomes
- canonical data-class semantics;
- deterministic + structured/FHIR-aware + admitted local-model recognizers;
- redact/tokenize/generalize/pseudonymize/drop transformations under policy;
- immutable transformed artifacts; source never overwritten;
- residual scan + explicit uncertainty;
- `DeidReceipt` and re-identification audit capability;
- policy hooks for model, source, browser, export, Hub, compute, R workspace, connector and extension boundaries;
- synthetic/permitted multilingual benchmark corpus;
- no product claim that automation proves all PHI/PII absent.

## Closure focus
Fail-closed boundary decisions, classification propagation, reversible-map key separation, source/import denial tests and external-egress denial tests.

---

# 080 — Governed Browse

## Depends on
077 + 079 + existing MedScale Network Broker authority.

## Goal
Give MedAgent/research workflows web search and browser capability without bypassing privacy, network authority or evidence provenance.

## Foundation scope
Read/retrieve/research first. Side-effecting browser actions are a later explicit capability.

## Required outcomes
- typed request/policy/route/evidence/receipt contracts;
- search/HTTP acquisition through admitted network broker;
- deterministic automation behind a bounded worker when required;
- agentic fallback only when deterministic navigation is insufficient and explicitly permitted;
- hostile web content cannot change instructions/capabilities;
- credentials remain outside model prompts;
- explicit human takeover for login/MFA/session boundaries;
- URL/time/tool/content-hash/evidence-span provenance;
- cancellation/timeouts/redirect/download/file-type limits;
- SSRF/private-network/credential-exfiltration defenses;
- downloads return quarantined candidate bytes for normal 075 import/validation paths rather than trusted artifacts.

## Closure focus
Public/deidentified retrieval works with receipts; sensitive egress, prompt injection, credential leakage, redirect abuse and malicious downloads fail closed.

---

# 081 — AudioFlow Foundation

## Depends on
077 + 079.

## Goal
Add local-first capture, transcription, diarization, dictation and voice control as evidence-bearing Project capabilities.

## Required outcomes
- audio source/session/transcript/diarization/evidence contracts;
- native capture/control and imported media path;
- visible capture/health state;
- immutable source digest + retention policy;
- conditioning/VAD ownership;
- evidence-selected runtime router;
- streaming local STT + offline-quality path;
- diarization/alignment;
- medical terminology/number/unit/negation benchmark where claimed;
- Arabic and Arabic-English code-switch evaluation;
- non-destructive transcript revisions;
- explicit `COMMAND` / `CONTEXT` / `DICTATION` semantics;
- interruption/cancellation;
- VoiceStudio/Himsat/Wispral donor qualification;
- no default wake word or always-listening.

## Closure focus
No silent cloud fallback; unavailable routes remain explicit; source audio and transcript lineage are reproducible.

---

# 082 — Analytics Gate

## Depends on
074 + 075 + 079.

## Goal
Provide reproducible native analysis over exact Project/DataSnapshot revisions without making an external BI server or unrestricted scripting runtime a Desktop dependency.

## Required outcomes
- governed analytical views over exact immutable inputs;
- Arrow/Parquet interchange where appropriate;
- DataFusion qualification or bounded alternative behind the same contract;
- SQL editor + natural-language query proposal path;
- parser/planner default-deny for writes/DDL;
- cohort/query builder;
- `QueryReceipt` with exact query/plan/input revisions/engine/config;
- derived Dataset/Table/Figure artifacts with classification propagation;
- statistical operations with independent correctness fixtures;
- native tables/charts;
- cancellation/timeouts/resource limits;
- source/snapshot refresh never silently mutates a completed analytical run;
- R/Python/shell execution deferred to 085/086.

## Closure focus
Reproducibility, read-only default, immutable source pinning, independent statistical correctness and honest assumption/not-run states.

---

# 083 — Knowledge + Research Canvas

## Depends on
074 + 075 + 077 + 079.

## Goal
Create permission-aware project knowledge, retrieval and visual research composition without treating vector search as authority.

## Required outcomes
- index manifest with exact source/chunk/parser identity and stale/tombstone semantics;
- embedded/local lexical search first;
- vector retrieval only if benchmark demonstrates value;
- structured Project Graph + lexical + optional vector plan;
- authorization before disclosure/cache reuse;
- exact page/span/revision/data-snapshot evidence links;
- `RetrievalReceipt`;
- Research Canvas with live references rather than silent copies;
- literature/library workflow;
- explicit insufficient-evidence state;
- contradiction/missing-evidence inspection.

## Integration gates
- web evidence only after 080;
- audio timestamps only after 081;
- analytics derivatives only after 082;
- imported datasets use 075 snapshot lineage.

## Closure focus
No cross-project leaks, stale-index honesty, deletion/tombstone propagation and stable evidence links after restart/rebuild.

---

# 084 — MedScale Hub

## Depends on
074 + 076 + 079.

## Goal
Enable user-controlled multi-device lab/team collaboration without requiring a MedScale-hosted cloud.

## Required outcomes
- self-hostable single-Hub topology first;
- team/project membership + invitations;
- versioned handshake and device identity;
- sync of collaboration artifacts;
- policy-approved resumable object transfer;
- monotonic cursors/idempotent submission/conflict responses;
- tenant/project scope before DB/search/cache/pubsub/object lookup;
- encryption at rest for protected Hub data and transport encryption outside loopback/dev;
- explicit operator trust statement;
- revocation propagation;
- tombstone/deletion propagation with honest backup-retention semantics;
- workflow/audit coordination;
- offline reconnect/conflict behavior;
- backup/restore + schema upgrade/rollback;
- Personal single-laptop mode remains valid.

## Closure focus
Two-client offline/conflict/reconnect, revocation, tenant isolation, malicious digest, backup/restore and upgrade rollback.

---

# 085 — MedScale Compute

## Depends on
077 + 079. Local worker foundation does not require Hub.

## Goal
Scale bounded jobs from local CPU/GPU to lab/institutional compute without widening vault authority.

## Required outcomes
- job/worker/lease/input/output/receipt contracts;
- local bounded worker first;
- exact input staging; no vault mount or ambient key store;
- filesystem/network/secret/resource policy;
- timeout/OOM/crash/cancel/lost/partial semantics;
- output schema/digest validation before Core admission;
- revoked/late results quarantined;
- reproducible runtime/environment identity;
- log redaction;
- self-hosted remote worker after local proof;
- sandbox choice evidence-selected by workload/platform;
- institutional schedulers deferred to 090.

## Closure focus
Filesystem escape, egress denial, malicious output, duplicate/late completion, crash/OOM/timeout and reproducible output evidence.

---

# 086 — R Workspace

## Depends on
075 + 079 + 082 + 085.

## Goal
Make R a first-class, reproducible research environment while keeping unrestricted R code outside the trusted Desktop/Core process.

## Required outcomes
- `RWorkspaceManifest`;
- exact input `DataSnapshot` references/digests;
- staged workspace with `data/`, `scripts/`, `outputs/`, `.Rproj`, `renv.lock`, workspace manifest and README;
- Arrow/Parquet interchange by default where appropriate;
- **Open in RStudio**, **Open in Positron**, and **Open folder** actions where installed;
- external IDE receives staged workspace only, never vault master key or canonical DB handle;
- `Rscript` execution only through Compute authority;
- `RRunReceipt` with input digests, script digest, R version, platform/runtime identity, lockfile digest, package restore state, command, exit state, log/output digests and classification;
- explicit output **Publish to MedScale** path with schema/digest/policy validation;
- no ambient watching/import of arbitrary workspace outputs;
- deterministic workspace regeneration from the same manifest where external dependencies remain available;
- explicit offline package-restoration limitations;
- institutional Posit Workbench/Job Launcher deferred to 090.

## Non-goals
No embedded R interpreter in trusted Desktop, no unrestricted package installer with ambient network, no direct vault write from R, no promise that all CRAN/Bioconductor packages reproduce forever.

## Closure focus
Exact snapshot staging, external IDE isolation, Compute-mediated script execution, `renv`/runtime evidence, output admission and no vault/secret escape.

---

# 087 — Community Extensions

## Depends on
079 + 084 + 085. 080 is required for extensions requesting public-web/browser capabilities.

## Goal
Create an Obsidian-quality community ecosystem with healthcare/research-grade signing, sandboxing, capability review and rollback.

## Product surfaces
- **MedScale Extensions** — platform;
- **Community Registry** — Hub discover/install/publish surface;
- **Extension SDK** — developer contracts/tooling;
- **Extension Pack** — signed distributable artifact, distinct from Research Packs and model Packs.

## Extension contribution classes
- data-source connectors;
- importers/exporters;
- MedAgent tools;
- analysis actions;
- visualization contributions;
- Research Pack augmentations;
- AudioFlow processors;
- institutional adapters;
- declarative UI/action contributions where no executable code is required.

## Required outcomes
- versioned `ExtensionManifest` with publisher, API/version bounds, digest/signature/provenance, license, entrypoint kind, declared capabilities, network/data/filesystem/device policies, config schema, migrations and rollback metadata;
- deny ambient vault DB, master key, keychain, filesystem, network, clipboard, microphone/camera, Project data and Core mutation APIs;
- explicit capability grant and visible capability diff on install/update;
- signed package verification;
- sandboxed WASM runtime if qualified, isolated worker process, or declarative contribution; arbitrary native trusted-process library loading denied by default;
- Host API is narrow, versioned and mediated by Core/network/Compute authority;
- extension crash/timeout/quarantine/disable/rollback states;
- SBOM/license/dependency/provenance scanning;
- registry review tiers and malicious/revoked package handling;
- offline/manual Extension Pack install remains possible under equivalent verification;
- extensions cannot self-update, silently escalate capabilities or bypass policy;
- community source/data connector extensions still emit 075-compatible manifests/receipts;
- no extension inherits the invoking user's full authority automatically.

## Non-goals
No arbitrary JS/native plugin code inside trusted Slint/Desktop process, no unreviewed marketplace auto-execution, no mandatory Hub for local/manual extension use, no plugin-owned clinical authority.

## Closure focus
Capability escape, malicious package, signature failure, dependency confusion, capability escalation, filesystem/network denial, crash isolation, rollback, offline install verification and registry revocation.

---

# 088 — AudioFlow Advanced

## Depends on
081 + 076 + 084. Compute integration is required for remote/heavy audio workers when enabled.

## Goal
Turn AudioFlow into full team audio intelligence after capture/transcript/Hub foundations are proven.

## Required outcomes
- Project audio huddles;
- humans + explicitly scoped agents;
- join permission distinct from record/transcribe/export permission;
- live transcript/evidence/task proposals;
- TTS/listen-back;
- optional duplex agent voice;
- voice design/cloning only with explicit subject permission/provenance;
- synthetic audio labeling/export provenance;
- media time/frame annotations;
- batch workflows;
- optional remote audio workers;
- persistent speaker identity opt-in/encrypted/deletable.

## Closure focus
Consent separation, duplex interruption, synthetic-origin labeling, retention/deletion and long-session team behavior.

---

# 089 — Research Packs

## Depends on
074 + 075 + 077 + 082 + 083. 085 is required for Pack features that execute heavy/arbitrary worker code.

## Goal
Expand to research/lab domains without bloating or weakening Core.

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
- source/data requirements through 075 contracts;
- tool/model requirements;
- evidence semantics;
- import/export boundaries;
- migrations/upgrades;
- uninstall disables Pack but preserves domain data until explicit export/delete;
- declarative contributions preferred;
- executable additions use normal Extension/Compute authority rather than trusted-process arbitrary code;
- clear distinction between Research Pack, Extension Pack and model/runtime Pack.

## Closure focus
At least the first Pack proves domain extension without alternate authority, unsafe UI plugins, shadow data-source model or destructive migration.

---

# 090 — Institutional Adapters

## Depends on
075 + 079 + 084 + 085 + 089. R institutional integration also depends on 086; extension-based adapters must comply with 087.

## Goal
Connect MedScale to organizational infrastructure through explicit optional adapters without making them required for Personal/Lab operation.

## Candidate families
- institutional identity/SSO;
- S3-compatible/object storage, cloud object stores and warehouses;
- LIMS/ELN;
- EHR/FHIR systems where not already covered by current clinical integrations;
- HPC/Slurm/Kubernetes;
- institutional model registry;
- Posit Workbench / Job Launcher;
- Superset/BI over approved materialized views;
- OpenRAG/OpenSearch-class scale services;
- notification/communication systems;
- approved side-effecting browser/connector workflows where APIs are unavailable.

## Required posture
Every adapter gets destination, data-class, capability, credential, effect-state and provenance contracts. External writes preserve `Unknown` when confirmation is uncertain and never retry blindly.

---

# 091 — Federation

## Depends on
090.

## Goal
Research and, only if proven, enable controlled cross-institution collaboration without requiring a central MedScale data cloud.

## Candidate outcomes
- signed Project/DataSnapshot/Evidence bundles;
- institution identity/trust roots;
- least-privilege cross-institution capabilities;
- selective data release rather than automatic vault replication;
- federated/remote analysis only when threat/privacy/reproducibility requirements are independently qualified;
- revocation/expiry/audit semantics;
- no claim of regulatory compliance merely because federation exists.

## Closure focus
Cross-institution isolation, provenance preservation, revocation, partial failure, policy disagreement and no hidden central authority.

---

# 092 — Whole-Platform Qualification

## Depends on
All promoted/required 074-091 planes included in the intended release profile.

## Goal
Run integrated qualification against the actual release candidate rather than assuming subsystem passes compose automatically.

## Required campaigns
- Project/artifact integrity;
- source/import/database/Kaggle/HF acquisition and snapshot reproducibility;
- collaboration/Hub conflict and revocation;
- MedAgent/Fleet authority isolation;
- Privacy Gate and egress;
- Governed Browse hostile-content/credential/download behavior;
- AudioFlow privacy/long-session/runtime routing;
- Analytics correctness/reproducibility;
- Knowledge retrieval permission/staleness;
- Compute escape/resource/late-result behavior;
- R workspace isolation/reproducibility/output admission;
- extension signature/capability/sandbox/rollback/registry attacks;
- Research Pack migrations/domain isolation;
- institutional adapter effect-state/credential/policy behavior;
- federation trust/revocation if included;
- accessibility, RTL, minimum-window and failure-state UX;
- migration/backup/restore/upgrade from representative earlier versions;
- Personal/Lab/Research Center/Institution deployment profiles;
- performance/resource/storage/network measurements;
- exact-head and post-merge evidence.

## Terminal truth
Whole-platform qualification may establish only what the tested profile and evidence actually prove. It must not manufacture HIPAA/GDPR/PDPL, clinical effectiveness or universal performance claims.

---

## 4. Promotion discipline

For every candidate 075+:

1. reverify current main/frontier;
2. confirm semantic name and candidate number remain unused;
3. bind contracts to real code paths/types;
4. resolve evidence-selected technology choices through the Decision Resolution Register;
5. define migration/recovery and rollback;
6. define exact security/threat delta;
7. create focused verification/evidence plan;
8. promote only that bounded unit through explicit founder/canonical authority;
9. never infer authorization for the next unit from closure of the current unit.

## 5. Current execution rule

At the time of this roadmap amendment:

```text
SPEC_074_IMPLEMENTATION_AUTHORIZED=true
SPEC_075_PLUS_IMPLEMENTATION_AUTHORIZED=false
REAL_PHI_AUTHORIZED=false
MESC_MUTATION_AUTHORIZED=false
```

The V2 roadmap changes future planning. It does not expand the already-promoted Scope 074.