# Research OS V2 Exact Repository Implementation Map — 2026-09-19

**Status:** `PLANNING_CANDIDATE_ONLY`  
**Baseline inspected:** `main@8db4ac089db2869da9ff6570ad7da91139987321`  
**Tree inspected:** `9b0ee757f39016ad6a6da3786b5c6c972197c742`  
**Purpose:** turn the Clinical + Research Intelligence master plan into a repository-shaped implementation map. Live repository truth must be rechecked at each promotion.

## 1. Existing trusted boundaries

Current workspace:

```text
crates/medscale-contracts
crates/medscale-core
crates/medscale-cli
crates/medscale-storage
crates/medscale-fhir
crates/medscale-keys
crates/medscale-desktop
crates/medscale-network
crates/medscale-pack
```

Existing authority direction remains:

```text
contracts
   ^
   |
storage / fhir / keys / network / pack
   ^
   |
 core
   ^
   |
cli / desktop
```

Do not introduce a Research OS service that bypasses this structure.

## 2. Existing high-value anchors to reuse

### Contracts

Existing source/object/evidence foundations:

```text
crates/medscale-contracts/src/objects/
crates/medscale-contracts/src/evidence/mod.rs
crates/medscale-contracts/src/documents/mod.rs
crates/medscale-contracts/src/network/mod.rs
crates/medscale-contracts/src/actions/mod.rs
crates/medscale-contracts/src/packs/mod.rs
crates/medscale-contracts/src/project_graph.rs
crates/medscale-contracts/src/worker_policy/mod.rs
```

Reuse current IDs, scope, source, time/effect and evaluation/projection/audit semantics before creating new primitives.

### Core

Existing authority anchors:

```text
crates/medscale-core/src/authority/facade.rs
crates/medscale-core/src/authority/project_graph.rs
crates/medscale-core/src/authority/retrieval.rs
crates/medscale-core/src/authority/document_ops.rs
crates/medscale-core/src/authority/source_ops.rs
crates/medscale-core/src/effects/mod.rs
crates/medscale-core/src/cli_session.rs
crates/medscale-core/src/process/
crates/medscale-core/src/validate/
```

New product commands should normally enter through the existing authority facade/session pattern.

### Storage

Existing persistence anchors:

```text
crates/medscale-storage/src/encrypted_vault.rs
crates/medscale-storage/src/sqlite_meta.rs
crates/medscale-storage/src/sealed_blob.rs
crates/medscale-storage/src/blob.rs
crates/medscale-storage/src/migrate.rs
crates/medscale-storage/src/backup.rs
crates/medscale-storage/src/gc.rs
crates/medscale-storage/src/project_graph.rs
```

Do not create a second SQLite/vault/blob lifecycle.

### Network and credentials

```text
crates/medscale-network/src/adapters.rs
crates/medscale-network/src/allowlist.rs
crates/medscale-network/src/transport.rs

crates/medscale-keys/src/keystore.rs
crates/medscale-keys/src/provider.rs
```

All product egress and external credentials remain brokered.

### Pack/runtime

```text
crates/medscale-pack/src/format.rs
crates/medscale-pack/src/runtime.rs
crates/medscale-pack/src/onnx_runtime.rs
crates/medscale-pack/src/store.rs
```

Extend this model/runtime trust system before creating an independent model/audio/VLM store.

### Desktop

Current native surfaces:

```text
crates/medscale-desktop/src/patient_workspace.rs
crates/medscale-desktop/src/population_insights.rs
crates/medscale-desktop/src/product_intelligence.rs
crates/medscale-desktop/src/project_workspace.rs
crates/medscale-desktop/src/utility_surfaces.rs
crates/medscale-desktop/src/workflow_studio.rs
crates/medscale-desktop/ui/app.slint
crates/medscale-desktop/ui/components.slint
crates/medscale-desktop/ui/theme.slint
```

Keep `main.rs` composition-focused. New Research OS surfaces should become bounded adapter modules plus Slint surfaces/components.

---

# 3. Spec 075 — Data Source Fabric + Data Workbench

## Contract placement

Preferred new module:

```text
crates/medscale-contracts/src/data_sources/
  mod.rs
  source.rs
  schema.rs
  snapshot.rs
  receipt.rs
  health.rs
  workbench.rs
```

Reuse:
- object IDs/scope/digests;
- Project artifact refs;
- source identity;
- existing authority/evidence primitives.

## Core placement

```text
crates/medscale-core/src/data_sources/
  mod.rs
  discover.rs
  import.rs
  snapshot.rs
  transform.rs
  workbench.rs
```

Core owns:
- source admission;
- snapshot admission;
- Project attachment;
- transformation publication;
- classification propagation.

Provider adapters never write storage directly.

## Storage placement

Extend:

```text
crates/medscale-storage/src/migrate.rs
crates/medscale-storage/src/sqlite_meta.rs
crates/medscale-storage/src/blob.rs
```

Candidate semantic families:
- data_sources;
- data_snapshots;
- snapshot_parts/files;
- saved_views;
- transformations;
- import/refresh receipts.

Large imported bytes use existing blob/sealed-blob patterns where applicable.

## Network

Add provider routes behind `medscale-network`; no Desktop HTTP clients.

## CLI

Current CLI already split Project work into `src/project.rs`. 075 should continue that refactor rather than grow `main.rs`.

Candidate:
```text
crates/medscale-cli/src/data_source.rs
crates/medscale-cli/src/data_workbench.rs
```

## Desktop

Candidate:
```text
src/data_sources.rs
src/data_workbench.rs
ui/data_sources.slint
ui/data_workbench.slint
```

Do not put formula/data authority into Slint callbacks.

## Tests

Candidate focused tests:
```text
medscale-contracts/tests/data_sources_075.rs
medscale-storage/tests/data_sources_075.rs
medscale-core/tests/data_sources_075.rs
medscale-core/tests/data_workbench_075.rs
```

---

# 4. Spec 076 — Collaboration

## Contracts
```text
crates/medscale-contracts/src/collaboration/
```

## Core
```text
crates/medscale-core/src/collaboration/
```

## Storage
Current encrypted metadata store; semantic families for rooms/threads/comments/tasks/note revisions/approvals/activity.

## Desktop
```text
src/collaboration.rs
ui/collaboration.slint
```

Collaboration references exact Project/artifact revisions.

Do not store canonical clinical facts as collaborative document blocks.

---

# 5. Spec 077 — MedAgent + Evidence Copilot

## Contracts
```text
crates/medscale-contracts/src/agents/
crates/medscale-contracts/src/evidence/    # extend when claim/evidence semantics fit
```

## Core
```text
crates/medscale-core/src/agents/
crates/medscale-core/src/authority/retrieval.rs  # extend/reconcile, do not fork retrieval authority
```

## Pack
Use `medscale-pack` for admitted local models.

## Process/tool execution
Reuse `crates/medscale-core/src/process/` and worker-policy contracts before adding a new execution substrate.

## Desktop
```text
src/medagent.rs
ui/medagent.slint
```

Default visual composition:
conversation/task lane + outcome/artifact pane + optional evidence/context inspector.

## Test focus
- explicit ContextManifest;
- no ambient vault enumeration;
- model unavailable;
- tool denied;
- insufficient/conflicting evidence;
- run cancellation/restart semantics.

---

# 6. Spec 078 — Model Fleet + Compare

## Contracts
Extend Packs/runtime contracts or add:
```text
crates/medscale-contracts/src/model_fleet/
```
only if lane/comparison semantics do not fit existing Pack contracts.

## Core
```text
crates/medscale-core/src/model_fleet/
```

## Pack
Extend:
```text
medscale-pack/src/format.rs
medscale-pack/src/runtime.rs
medscale-pack/src/store.rs
```

Do not bypass Pack admission by loading arbitrary model repos from Desktop.

## Desktop
```text
src/model_center.rs
src/model_compare.rs
ui/model_center.slint
ui/model_compare.slint
```

---

# 7. Spec 079 — Privacy Gate

## Contracts
```text
crates/medscale-contracts/src/privacy/
```

Reuse existing source/scope/evidence classes where semantics already exist.

## Core
```text
crates/medscale-core/src/privacy/
```

Core owns:
- classification;
- transformation admission;
- egress decisions;
- DeidReceipt creation/admission;
- policy evaluation.

## Models
OpenMed-inspired NER/de-id models enter as admitted local Packs/workers, not Python trusted authority.

## Storage/keys
Reversible pseudonym maps, if admitted, use separately scoped encrypted/key authority. Never put reversible maps in normal audit records.

## Desktop
Privacy inspection is a cross-cutting surface, not a separate competing settings DB.

---

# 8. Spec 080 — Governed Browse + Literature

## Contracts
```text
crates/medscale-contracts/src/browse/
```
or extend existing network/evidence contracts when cleaner.

## Core
```text
crates/medscale-core/src/browse/
```

## Network
All retrieval extends:
```text
crates/medscale-network/src/
```

Candidate adapter modules:
- PubMed/NCBI;
- Crossref;
- OpenAlex;
- ClinicalTrials.gov;
- bounded generic HTTP/source capture.

## Browser worker
Do not add browser automation to Core.

If a deterministic browser becomes necessary, add an isolated worker/process only after 080 freezes the IPC/capability contract. It receives:
- approved URL/origin;
- credential handle only when explicitly authorized;
- network policy;
- output/download budget.

## Downloads
Browser downloads become quarantined source candidates and re-enter 075/document admission.

---

# 9. Spec 081 — AudioFlow + Local Scribe MVP

## Contracts
```text
crates/medscale-contracts/src/audio/
```

Types:
- AudioSource;
- AudioSession;
- TranscriptRevision;
- DiarizationRevision;
- AudioEvidenceRef;
- AudioRuntimeProfile.

## Core
```text
crates/medscale-core/src/audio/
```

Core owns session state, lineage, route admission and transcript/note artifact relationships. Raw DSP/ASR engine internals stay outside Core when heavy.

## Pack/runtime
Extend `medscale-pack` for ASR/diarization/model metadata rather than a second model store.

## Storage
Audio uses existing content-addressed/sealed blob patterns after retention/classification policy.

Transcript revisions use canonical encrypted metadata/blob strategy.

## Worker
Candidate engines such as whisper.cpp/sherpa-onnx remain behind a bounded audio worker/runtime adapter as required by exact dependency/security evidence.

## Desktop
```text
src/audioflow.rs
src/scribe.rs
ui/audioflow.slint
ui/scribe.slint
```

Scribe UI consumes Core results; it does not invoke ASR/model processes directly.

## Qualification
Follow `LOCAL_MEDICAL_SCRIBE_EVALUATION_PROTOCOL_2026-09-19.md`.

---

# 10. Spec 082 — Analytics + Cohort Builder

## Contracts
```text
crates/medscale-contracts/src/analytics/
```

## Core
```text
crates/medscale-core/src/analytics/
```

## Existing collision to reconcile
```text
crates/medscale-desktop/src/population_insights.rs
```

Inspect and migrate/reuse current population behavior; do not create a competing analytics truth.

## Execution
Native read-only SQL/statistics may use an admitted Rust engine. Python/R do not run in trusted Desktop/Core.

## Desktop
```text
src/analytics.rs
src/cohort_builder.rs
ui/analytics.slint
ui/cohort_builder.slint
```

---

# 11. Spec 083 — Clinical Graph + Knowledge/Research Canvas

## Contracts
Candidate:
```text
crates/medscale-contracts/src/knowledge/
crates/medscale-contracts/src/clinical_graph/
```

Before adding a second graph module, reconcile with:
```text
crates/medscale-contracts/src/project_graph.rs
```

If the generic typed-edge contract can be extended safely, prefer shared primitives plus domain-specific relation enums.

## Core
```text
crates/medscale-core/src/knowledge/
crates/medscale-core/src/clinical_graph/
```
or extend `authority/project_graph.rs` if that preserves cohesion.

## Storage
Projection/index storage must be rebuildable and explicitly non-authoritative.

Do not require Neo4j/FalkorDB or another graph server for Personal mode.

## Retrieval
Extend/reconcile:
```text
crates/medscale-core/src/authority/retrieval.rs
```

## Desktop
```text
src/knowledge_canvas.rs
src/clinical_graph.rs
ui/knowledge_canvas.slint
ui/clinical_graph.slint
```

Graph always has accessible table/list/timeline alternatives.

---

# 12. Spec 084 — Hub

New server crates are allowed only here or later, after promoted proof that existing crates cannot cleanly own the deployment boundary.

Candidate:
```text
crates/medscale-hub-server
```

Wire contracts should remain in `medscale-contracts` unless a separately versioned public/server contract truly requires its own crate.

Client egress still routes through `medscale-network`.

Local Core stays authoritative when Hub is offline.

---

# 13. Spec 085 — Compute

A new isolated worker binary/crate is justified if the promoted spec demonstrates platform/process isolation value.

Candidate:
```text
crates/medscale-compute-worker
```

Core owns job admission and output publication; worker owns execution only.

Reuse:
- worker policy contracts;
- process/session utilities;
- OS sandbox work;
- current effect/audit primitives.

Never give worker:
- vault root;
- canonical DB handle;
- key-store enumeration;
- unrestricted network.

---

# 14. Spec 086 — R Workspace

## Contracts/Core
```text
medscale-contracts/src/r_workspace/
medscale-core/src/r_workspace/
```

## Execution
`Rscript` goes through 085 Compute.

## External IDE
Desktop may launch a staged workspace path in RStudio/Positron if installed. The external IDE receives no vault/key/DB capability.

## Desktop
```text
src/r_workspace.rs
ui/r_workspace.slint
```

---

# 15. Spec 087 — Extensions

## Contracts
```text
medscale-contracts/src/extensions/
```

## Core
```text
medscale-core/src/extensions/
```

## Runtime
Candidate new isolated host:
```text
crates/medscale-extension-host
```
only after runtime benchmark/admission.

No arbitrary extension code in Desktop/Core.

UI extensions use MedScale-owned declarative descriptors initially.

---

# 16. Spec 088 — Advanced Clinical Documentation

Primarily composes existing 077/078/079/081/083/089 contracts.

Do not create a monolithic `clinical_ai` crate.

Candidate Core module:
```text
crates/medscale-core/src/clinical_workflows/
```
only for orchestration that does not fit audio/agents/evidence.

Candidate Desktop surfaces:
```text
src/clinical_workflows.rs
src/nursing_workspace.rs
src/revenue_intelligence.rs
ui/clinical_workflows.slint
ui/nursing_workspace.slint
ui/revenue_intelligence.slint
```

Coding/risk/CDI outputs are proposals. No billing rules live as unversioned UI code.

---

# 17. Spec 089 — Research/Evidence Packs

Extend or compose current Pack provenance but keep research/evidence content semantics distinct from executable model Packs.

Candidate contracts:
```text
medscale-contracts/src/research_packs/
```

Storage:
- manifest;
- rights/license;
- content digests;
- index version;
- update/revoke state.

Executable behavior must route through existing tool/Compute/extension boundaries.

---

# 18. Spec 090 — Institutional Adapters

## FHIR
Reuse/extend:
```text
crates/medscale-fhir/
crates/medscale-contracts/src/fhir/
```

## Network
All adapters route through:
```text
crates/medscale-network/
```

## Effects
Reuse:
```text
crates/medscale-core/src/effects/mod.rs
crates/medscale-contracts/src/actions/mod.rs
```

Do not build a second outbox/action state machine for prior authorization, messaging or EHR writes.

## Candidate adapter families

```text
FHIR/SMART
HL7 v2
CDA/C-CDA
DICOM/DICOMweb
payer/prior-auth
institution-approved communication channels
warehouse/object-store
identity/SSO
```

Each exact adapter is separately profile/version/security/rights qualified.

---

# 19. Spec 091 — Federation

Do not add federation crates before 090 is closed and a specific use case is approved.

Prefer protocol contracts plus bounded worker/Hub implementation rather than a second distributed authority layer.

---

# 20. Spec 092 — Qualification

Qualification should mostly add fixtures/harnesses/evidence, not new product architecture.

Reuse existing test/evidence conventions:
```text
crates/medscale-core/tests/
crates/medscale-storage/tests/
crates/medscale-contracts/tests/
crates/medscale-desktop/tests/
evidence/<spec>-.../
```

Required evaluation protocols:
- `LOCAL_MEDICAL_SCRIBE_EVALUATION_PROTOCOL_2026-09-19.md`;
- `EVIDENCE_ENGINE_EVALUATION_PROTOCOL_2026-09-19.md`.

---

# 21. New crate budget

Expected default before 084:

```text
NEW_CRATES_075_083 = 0 unless exact dependency/isolation evidence justifies one
```

Most capabilities should begin as modules in existing crates.

Likely justified new process/crate candidates later:
- Hub server;
- Compute worker;
- Extension host;
- isolated browser/audio/document workers only if packaging/dependency boundaries cannot be maintained as binaries/modules in existing infrastructure.

Every new crate must document:
- why existing ownership cannot fit;
- dependency direction;
- authority boundary;
- packaging/runtime lifecycle;
- test/evidence surface;
- removal/exit strategy.

---

# 22. First implementation file-set expectation — Spec 075

When 075 is promoted, the first contract-only slice should normally touch only a bounded subset similar to:

```text
crates/medscale-contracts/src/lib.rs
crates/medscale-contracts/src/data_sources/...
crates/medscale-contracts/tests/data_sources_075.rs
specs/075-.../
evidence/075-.../BASELINE.md
```

Storage/Core/CLI/Desktop files enter only in their dependency-ordered slices.

Do not create every 075–092 module at once.

---

# 23. Live-map drift rule

At every promotion:

1. fetch `main`;
2. list actual workspace members and domain modules;
3. compare to this map;
4. update the active spec with exact changed paths;
5. treat renamed/moved live code as authority over this planning file;
6. record deviations explicitly.

This map exists to prevent architecture invention, not to force stale paths.
