# MedScale Clinical + Research Intelligence OS — Implementation-Ready Master Plan

**Date:** 2026-09-19  
**Status:** `PLANNING_CANDIDATE_ONLY` — implementation-ready planning, not implementation authority  
**Program range:** existing Research OS V2 candidate Specs 075–092  
**Prerequisite already closed:** Spec 074 Project + Artifact Graph Foundation  
**North star:** A local-first, privacy-first Clinical + Research Intelligence OS that combines the workflow value of ambient clinical documentation, evidence-grounded medical research, local medical AI, data workbenches, reproducible analytics, knowledge graphs, collaborative research, and institutional interoperability without creating a second authority plane.

---

# 1. Product definition

MedScale is not:

- a generic medical chatbot;
- an EHR replacement;
- a cloud-only scribe;
- a vector database with a UI;
- an Airtable fork;
- an AFFiNE fork;
- a Graphify fork;
- a collection of unrelated research tools.

MedScale is:

> **One local clinical and research workspace with one authority/provenance model, many qualified local engines, explicit external capabilities, and evidence attached to every consequential derived output.**

The product target is intentionally broader than any single competitor:

```text
Ambient clinical workflow
+ evidence-grounded medical intelligence
+ local medical model ecosystem
+ patient/clinical graph
+ documents and audio
+ datasets and cohort work
+ reproducible analytics
+ literature and research canvas
+ agents and model comparison
+ team collaboration
+ institutional adapters
= MedScale
```

No parity/surpass claim is authorized by this plan.

---

# 2. System invariants

These are non-negotiable across Specs 075–092.

## INV-01 — One authority path

All durable authority-changing state flows through existing MedScale Core contracts. No UI, worker, graph engine, browser, model, collaboration server, or donor becomes a second clinical/research authority.

## INV-02 — Local-first useful core

A useful core path must remain available with product network egress disabled.

Remote literature, remote EHR access, Hub sync, institutional adapters, model downloads, dataset refresh, and optional external providers are explicit capabilities, not hidden dependencies.

## INV-03 — No hidden cloud fallback

If a local ASR/model/OCR/analytics route is unavailable, the UI reports `UNAVAILABLE` or offers an explicit policy-approved alternative. It never silently sends patient/research content to a remote provider.

## INV-04 — Source ≠ derivation ≠ review ≠ effect

At minimum distinguish:

```text
SOURCE
DETERMINISTIC_DERIVATION
MODEL_INFERENCE
HUMAN_REVIEWED
ACTION_PROPOSAL
EXTERNAL_EFFECT
```

One state never silently promotes itself to the next.

## INV-05 — Graph is projection

Clinical/Research Graph state is rebuildable derived state. It references canonical source/artifact IDs and never silently becomes the canonical patient record.

## INV-06 — Workspace is view

Document, canvas, table, timeline, graph and dashboard views reference shared MedScale object IDs. They do not create competing truth stores.

## INV-07 — Evidence near claims

A consequential generated claim must be able to resolve to relevant source spans/objects, model/tool/run identity, limitations and review state.

## INV-08 — Unknown is first-class

Unknown, stale, conflicting, partial, not measured, denied, unsupported and unavailable remain distinct states.

## INV-09 — Heavy/untrusted runtimes isolated

Browser automation, document parsing/OCR where hostile, arbitrary code, Python/R, large native inference engines and extension runtimes receive staged inputs and bounded capabilities rather than ambient vault/DB access.

## INV-10 — External writes are durable controlled effects

EHR writes, messages, orders, exports, remote actions, dataset uploads and institutional mutations use explicit intent, idempotency, durable state, reconciliation and `UNKNOWN` outcome semantics.

## INV-11 — No real PHI before authority

Current repository truth keeps `REAL_PHI = NOT_AUTHORIZED`. Specs may build and qualify with synthetic/permitted non-PHI fixtures until canonical privacy/release authority changes.

## INV-12 — MESC remains separate

MESC is not a MedScale implementation dependency, completion condition or release gate.

---

# 3. Product planes

## 3.1 Trusted Authority Plane

Existing Rust-owned MedScale Core remains responsible for:

- canonical IDs;
- artifact/source identity;
- encrypted persistence;
- authority and review state;
- audit/evidence receipts;
- migrations/recovery;
- capability decisions;
- action/effect reconciliation.

## 3.2 Clinical Context Plane

Owns projections and workflows around:

- Patient/Subject;
- Encounter;
- Practitioner/Role;
- conditions/problems;
- medications;
- allergies;
- observations/results;
- procedures;
- documents;
- transcript/audio;
- tasks/actions;
- evidence relationships.

It is not an EHR database replacement.

## 3.3 Intelligence Plane

Contains qualified:

- local LLMs;
- local NER;
- local de-identification;
- ASR;
- diarization;
- rerankers;
- embedding models when admitted;
- VLMs;
- evidence classification models;
- OCR/document engines.

All enter through versioned Pack/runtime contracts.

## 3.4 Evidence Plane

Owns:

- source acquisition;
- literature/guideline references;
- retrieval plans;
- citations;
- evidence spans;
- support/contradiction relationships;
- applicability;
- recency;
- retraction state;
- evidence-strength dimensions;
- retrieval/evaluation receipts.

## 3.5 Data + Analytics Plane

Owns:

- Data Sources;
- immutable snapshots;
- Data Workbench;
- cohorts;
- transformations;
- SQL;
- reproducible statistics;
- tables/figures;
- R integration;
- analysis receipts.

## 3.6 Workspace Plane

Owns views and compositions:

- project workspace;
- patient/encounter workspace;
- document;
- canvas;
- table;
- timeline;
- graph;
- evidence ledger;
- manuscript/report;
- comments/tasks/review.

## 3.7 Execution Plane

Owns:

- model workers;
- document/OCR workers;
- browser workers;
- compute jobs;
- extension runtime;
- capability-scoped tools;
- resource budgets;
- cancellation/timeout.

## 3.8 Integration Plane

Owns admitted adapters for:

- FHIR/SMART;
- HL7 v2 when qualified;
- CDA/CCDA where justified;
- EHR/vendor APIs;
- DICOM/DICOMweb metadata/content where justified;
- databases/warehouses;
- object stores;
- institutional identity;
- messaging and other controlled effects.

---

# 4. Canonical cross-program contracts

Names below are planning contracts. Each promoted spec must bind them to exact live Rust types/modules before implementation.

## 4.1 EvidenceRef

```text
EvidenceRef {
  evidence_id
  source_object_id
  source_revision
  source_kind
  locator
  observed_time?
  effective_time?
  digest?
  classification
}
```

Locator examples:

- audio time span;
- transcript segment/span;
- FHIR resource + field/path;
- document page + region/span;
- dataset snapshot + row/column identity;
- URL snapshot + text span;
- paper/guideline section.

## 4.2 DerivedClaim

```text
DerivedClaim {
  claim_id
  text_or_structured_value
  claim_kind
  derivation_class
  supporting_evidence[]
  contradicting_evidence[]
  model_or_tool_run?
  applicability?
  uncertainty?
  review_state
  created_at
}
```

## 4.3 ContextManifest

Explicit input set for a model/agent/tool run:

```text
project
patient?/encounter?
artifact refs
data snapshot refs
evidence refs
allowed tools
allowed network classes
classification
retention/log policy
model/runtime
budget
```

No ambient vault access.

## 4.4 ModelPack / RuntimeProfile

Must bind:

- model artifact identity;
- model family/task;
- source/license/rights;
- checksum;
- tokenizer/preprocessor;
- runtime version;
- hardware compatibility;
- memory/resource profile;
- language/domain profile;
- benchmark evidence;
- admission state.

## 4.5 AudioSession / TranscriptRevision

Audio is source evidence; transcript is derived revision.

Each transcript revision binds:

- source audio digest;
- segment timestamps;
- speaker state;
- ASR model/runtime;
- language;
- quality flags;
- parent revision when refined;
- correction/review state.

## 4.6 ClinicalGraphEdge

Each edge binds:

- source node;
- target node;
- relation;
- authority class;
- source evidence;
- effective time;
- derivation;
- model/tool run if inferred;
- review state;
- confidence when meaningful;
- supersession.

## 4.7 DataSnapshot

Immutable analytical/research input binding:

- source;
- source revision;
- acquisition query/file selection;
- schema fingerprint;
- row/file counts;
- content digest(s);
- classification;
- adapter identity/version;
- creation time;
- import/refresh receipt.

## 4.8 AnalysisRun

Binds:

- exact DataSnapshots;
- cohort/view/filter definition;
- method/query/code;
- parameters;
- random seed where applicable;
- runtime/environment;
- package lock identities where relevant;
- output artifacts;
- assumptions/checks;
- limitations;
- run receipt.

## 4.9 EffectIntent

For any external write:

- target adapter/resource;
- exact payload digest;
- actor;
- authorization;
- idempotency key;
- preconditions;
- expiry;
- state machine;
- reconciliation strategy.

---

# 5. Architecture decisions

## ADR-CR-001 — Clinical Graph is rebuildable projection

No mandatory Neo4j/FalkorDB server in the trusted local core. Begin with MedScale-owned graph projection/storage appropriate to local scale; qualify an external graph engine only when measured need exists.

## ADR-CR-002 — Retrieval is hybrid, not vector-first

Default retrieval combines:

1. deterministic metadata/source filters;
2. lexical retrieval;
3. structured Clinical/Project Graph traversal;
4. optional embeddings/vector retrieval if benchmarked;
5. reranking if justified.

No vector index is authority.

## ADR-CR-003 — AFFiNE is UX/donor research, not a platform dependency

Study/adapt document/canvas/table/block interaction patterns. Direct code transfer requires exact path license review. MedScale retains native product authority and storage contracts.

## ADR-CR-004 — Data Workbench is MedScale-owned

Do not embed Airtable/NocoDB/Grist/Baserow as the patient/research authority. Adapt qualified concepts/components behind MedScale dataset contracts.

## ADR-CR-005 — Grist + Baserow are preferred Data Workbench donors

Grist Community and Baserow OSE are stronger initial study/adaptation candidates because their open-source licensing is more compatible with selective reuse than current NocoDB/Teable application licensing.

## ADR-CR-006 — Scribe is offline-first

Capture, ASR, diarization, note draft, linked evidence and local save must have a fully local path.

## ADR-CR-007 — Two-pass ASR

Use low-latency streaming transcription plus higher-accuracy refinement. Never overwrite the earlier transcript without version lineage.

## ADR-CR-008 — Note is proposal until clinician review

A generated clinical note does not become a signed/approved clinical artifact through model completion alone.

## ADR-CR-009 — Linked Evidence Everywhere

Span-level evidence linkage is a platform primitive used by scribe, evidence answers, analytics, research writing and patient-context reasoning.

## ADR-CR-010 — Evidence quality is multidimensional

Do not reduce evidence appraisal to one unexplained score. Keep dimensions such as study design, bias, consistency, directness, precision, applicability, recency, jurisdiction and contradiction state inspectable.

## ADR-CR-011 — Literature rights are explicit

Metadata/citations may be stored where permitted. Full-text acquisition obeys source/license/organization rights. Restricted publisher content is never assumed redistributable.

## ADR-CR-012 — Terminology/code systems are Pack-governed

SNOMED CT, LOINC, ICD, CPT, RxNorm, ATC and other terminology/coding assets each require explicit version/rights/source rules. No license is inferred from technical accessibility.

## ADR-CR-013 — Patient identity never silently merges

Record linkage creates candidate matches with evidence. Human or explicit authoritative matching confirms merges when required.

## ADR-CR-014 — Clinical facts do not use blind CRDT semantics

CRDT/eventual merge may be appropriate for notes/canvas collaboration; canonical clinical facts use authority-aware revision/conflict semantics.

## ADR-CR-015 — Analysis is snapshot-bound

Completed analytical results never silently change because a live database/source changed.

## ADR-CR-016 — Arbitrary code stays outside trusted Desktop/Core

Python, R, shell and untrusted generated code execute only through MedScale Compute or an external tool with staged inputs and explicit publication back.

## ADR-CR-017 — Browser content is hostile input

Web pages, snippets and downloaded files cannot alter system instructions, tool grants, network scope or patient context access.

## ADR-CR-018 — Remote inference is not default

Future remote model providers, if ever admitted, sit behind Privacy Gate with explicit egress classification and user/institution policy. No automatic fallback.

## ADR-CR-019 — Mobile is companion first

Mobile should prioritize capture, review, evidence lookup, patient instructions and task continuity before attempting full desktop research/analytics parity.

## ADR-CR-020 — Institutional scale does not invalidate personal mode

Hub, institutional adapters and federation are optional expansions. Personal single-device local use remains a valid architecture target.

## ADR-CR-021 — Revenue intelligence cannot rewrite clinical truth

HCC, CDI, DRG, E&M, coding and payer-gap outputs are reviewable derived proposals. Financial/reimbursement optimization never changes source clinical facts or elevates a diagnosis without clinical documentation/evidence.

## ADR-CR-022 — Prior authorization is a controlled effect

MedScale may draft and assemble evidence locally. Payer submission, status polling, appeal and downstream routing are institution-adapter effects with explicit identity, policy/rule version, idempotency and UNKNOWN/reconciliation semantics.

## ADR-CR-023 — Trial matching is a candidate search, not eligibility authority

Clinical-trial matching maps patient/source evidence to explicit registry criteria and surfaces unresolved criteria. A match is a candidate for clinician/research-coordinator review, never an automatic eligibility or enrollment decision.

## ADR-CR-024 — Communications are optional adapters

Calls, messages, voicemail, fax and similar channels may support clinical operations, but they do not become a mandatory MedScale cloud/telephony layer. Each channel has explicit identity, consent, data-minimization, retention and delivery-effect semantics.

## ADR-CR-025 — Research intent and amendments are durable artifacts

Protocols, inclusion/exclusion criteria, search strategies, analysis plans and amendments are versioned Project artifacts. Post-hoc changes remain visible rather than rewriting preregistered intent.

## ADR-CR-026 — Clinical tools are deterministic/versioned when possible

Risk calculators, unit/dose calculations, guideline pathways and checklists use explicit inputs, units, formula/rule version, source references and test vectors. Model assistance may suggest or prefill inputs but does not replace deterministic execution or source validation.

## ADR-CR-027 — Annotation provenance distinguishes human and model labels

Dataset labels record schema version, source item, annotator identity/type, revision and review/adjudication state. Model-generated labels are proposals, never silently human ground truth.

## ADR-CR-028 — Training output is never automatically admitted

Any fine-tuned/trained model produced by Compute begins as a candidate artifact/ModelPack. It must pass normal rights, provenance, safety, quality, resource and Pack admission gates before it can appear as an admitted clinical/research runtime.

---

# 6. User-visible product surfaces

## Clinical

- Home / Research Command Center
- Patient Search / Subject Context
- Patient Workspace
- Encounter Workspace
- Local Scribe
- Clinical Evidence Copilot
- Clinical Tools / Calculators / Pathways
- Problems / Medications / Results / Documents
- Clinical Graph
- Orders / Tasks / Action Proposals
- Patient Instructions
- Coding / Documentation Review
- CDI / Pre-Bill Review
- Prior Authorization
- Care / Risk Signals
- Nursing Workspace
- Pre-Round / Inpatient Summary
- Clinical Trial Candidates
- Secure Communications (optional institution adapter)
- Audit / Evidence Inspector

## Research

- Projects
- Papers / Literature Library
- Research Canvas
- Data Sources
- Data Workbench
- Cohort Builder
- Analytics Gate
- R Workspace
- Experiments
- Models / Model Compare
- MedAgent
- Research Packs
- Systematic Review / Screening
- Study Extraction / Risk of Bias
- Manuscript / Reports
- Dataset Annotation / Adjudication
- Model Research / Evaluation
- Team / Review

## Platform

- Model Packs
- Privacy Gate
- Governed Browse
- AudioFlow
- Compute
- Hub
- Integrations
- Extensions
- Settings / Storage / Security / Backups
- Diagnostics / Evidence

---

# 7. Implementation program

The current 075–092 numbering remains the program skeleton. The following task decomposition is the implementation-ready refinement.

---

# Spec 075 — Data Source Fabric + Data Workbench Foundation

**Hard dependency:** 074.

## Goal

One governed source/snapshot contract for every later data consumer, plus the first native table/grid workbench over immutable snapshots.

## Implementation slices

### 075-00 Live contract binding
- bind source/snapshot types to current 074 artifact graph;
- freeze IDs, classifications, receipts and migration plan;
- decide exact crate/module placement from live tree.

### 075-01 Local file acquisition
- CSV/TSV;
- JSON/JSONL;
- Parquet;
- Arrow IPC/Feather if qualified;
- XLSX through isolated/qualified parser path;
- hostile-input and size/depth limits;
- immutable snapshot receipts.

### 075-02 Database read adapters
- PostgreSQL;
- MySQL/MariaDB;
- SQL Server;
- external SQLite;
- read-only default;
- secrets as opaque CredentialRef;
- bounded schema/table discovery;
- snapshot materialization;
- query receipts.

### 075-03 Remote dataset sources
- Hugging Face datasets by exact repo/revision/files;
- Kaggle datasets by exact identifier/version/files;
- no trusted remote code;
- resume/partial/corruption semantics;
- cache is non-authoritative.

### 075-04 Data Workbench grid
- schema;
- typed columns;
- sort/filter/group;
- row detail;
- saved views;
- local edits only for MedScale-owned research tables, never direct mutation of external clinical sources;
- revision history.

### 075-05 Import mapping
- source-to-column mapping;
- type inference as proposal;
- validation;
- missing/invalid/unknown handling;
- FHIR resource materialized view import.

### 075-06 Workbench view contracts
- Grid;
- Form;
- Gallery;
- Kanban;
- Calendar/time;
- summary/aggregate view.

### 075-07 Transformation foundation
- deterministic column transformations;
- formula/view-only computations;
- published transformation -> new DataSnapshot;
- lineage receipt.

### 075-08 Recovery/performance
- interrupted import;
- disk exhaustion;
- source loss;
- source schema change;
- large-table pagination/virtualization;
- resource budgets.

### 075-09 Dataset release / annotation-schema foundation
- versioned Dataset Card;
- immutable dataset release manifest;
- split/group metadata;
- annotation schema identity/version;
- annotation task references remain collaboration-owned rather than inventing a second task system;
- privacy/rights state remains explicit.

### 075-10 Qualification
- local/offline file path;
- DB snapshot path;
- HF/Kaggle exact-revision path;
- secret leakage tests;
- malformed data tests;
- migration/reopen;
- provenance;
- dataset-release/version lineage.

## Closure gate

A user can create a Project, import local/DB/approved remote data into immutable snapshots, inspect it in a native workbench, derive a versioned transformed snapshot and reproduce the lineage without cloud dependency for local sources.

---

# Spec 076 — Collaboration Substrate

**Hard dependency:** 074.

## Slices

### 076-00 Identity/event contracts
Human/service/agent participant references; artifact-revision references; collaboration event envelope.

### 076-01 Threads/comments/review
Comment on exact artifact/span/claim revisions; resolve/reopen; mention/assignment semantics.

### 076-02 Tasks/decisions
Tasks, review requests, approvals, decisions, due state, optimistic revision.

### 076-03 Notes/canvas collaboration foundation
Revisioned user-authored documents; conflict copies; explicit merge; no clinical-fact blind merge.

### 076-04 Activity/audit
Local searchable timeline; tamper-evident checkpoints; source refs.

### 076-05 Offline/restart
Single-device collaboration semantics work without Hub; durable restart and conflict simulation.

### 076-06 Research review / annotation adjudication
Reuse collaboration contracts for dual screening, annotation review, conflict queues, adjudication and reviewer provenance. Do not create a separate reviewer/assignment authority for systematic reviews or datasets.

### 076-07 Qualification
Agent participation cannot imply authority; canonical artifacts cannot be mutated through collaboration events; blinded/independent review state and adjudication provenance survive restart.

---

# Spec 077 — MedAgent + Evidence Copilot Foundation

**Hard dependency:** 074.  
**Integration:** 075 snapshots when available; 076 activity semantics before team closure.

## Slices

### 077-00 Agent run contract
ContextManifest, ToolManifest, ToolInvocation, RunReceipt, cancellation, budgets.

### 077-01 One local model lane
Admitted local text model; no network; explicit context; artifact output.

### 077-02 Tool execution
Read-only Project/artifact/evidence tools first; no external effects.

### 077-03 Evidence answer schema
Question, decomposed subquestion/PICO where relevant, claims, citations, evidence spans, limitations, uncertainty.

### 077-04 Local source evidence
Answer over admitted local documents/FHIR/project artifacts with source-linked claims.

### 077-05 Patient-context mode
Explicit patient/encounter ContextManifest; bounded longitudinal inputs; no ambient patient search.

### 077-06 Research mode
Papers/notes/datasets/artifacts; save answer/claim/evidence as project artifacts.

### 077-07 Interaction UX
Conversation + outcome pane + evidence inspector; interrupt/steer; provenance visible.

### 077-08 Failure/safety
Insufficient evidence; conflicting evidence; model unavailable; tool denied; context stale.

### 077-09 Clinical tool invocation
Discover and invoke admitted deterministic ClinicalTool manifests through explicit typed inputs. Model may propose a relevant tool or prefill sourced values; missing/ambiguous inputs cannot be guessed silently.

### 077-10 Qualification
Real local run, exact model/runtime/tool receipts, no hidden network or vault enumeration.

---

# Spec 078 — Model Fleet + Compare

**Hard dependency:** 077.

## Slices

### 078-00 Pack/runtime inventory
Unified model catalog with task/language/resource/rights/admission metadata.

### 078-01 Runtime router
Evidence-based route selection for text/NER/embedding/reranker/VLM families.

### 078-02 Parallel lanes
Two or more local lanes with independent context/tool/resource policy.

### 078-03 Comparison
Side-by-side output, citation overlap, contradiction candidates, unsupported claims, schema validity, abstention, latency/resources.

### 078-04 Hardware scheduler
CPU/GPU/Apple acceleration profile; memory reservation; cancellation/OOM behavior.

### 078-05 Benchmark harness
Same task, same context, declared hardware; no automatic “winner” without explicit metric.

### 078-06 Qualification
No permission union, partial failure explicit, unknown remains unknown.

---

# Spec 079 — Privacy Gate

**Hard dependency:** 074 + 077.

## Slices

### 079-00 Data classification
PUBLIC, PROJECT, TEAM_PROTECTED, CLINICAL_SENSITIVE, PHI/PII candidate and repository-defined classes bound to live authority.

### 079-01 Local detection
Structured/FHIR-aware recognizers + deterministic patterns + admitted local NER model.

### 079-02 Transformations
Redact, tokenize, pseudonymize, generalize, drop; immutable output artifact; no source overwrite.

### 079-03 Consent + purpose
Capture/use/share/export purpose metadata; institution policy hook; consent is not inferred from UI presence.

### 079-04 Retention/deletion
Raw audio, transcript, derived note, browser capture, dataset, model logs; tombstone/provenance semantics.

### 079-05 Egress gate
Model, browser, connector, Hub, Compute, R, export, extension boundaries.

### 079-06 Deid receipts
Transformation rules, source revision, detector versions, residual scan, uncertainty.

### 079-07 Benchmarks
Synthetic/permitted multilingual PHI/PII corpus; precision/recall by category; Arabic/English.

### 079-08 Qualification
Denied export/browse/model/provider path cannot leak content; reversible map keys separated.

---

# Spec 080 — Governed Browse + Medical Literature Acquisition

**Hard dependency:** 077 + 079 + existing network broker authority.

## Slices

### 080-00 Browse contracts
Network capability, BrowseRequest, BrowseReceipt, SourceCapture.

### 080-01 Medical search adapters
PubMed/NCBI APIs where permitted; Crossref/OpenAlex metadata; direct URL/DOI/PMID capture; no rights bypass.

### 080-02 Web retrieval
HTTP/source capture with URL, retrieval time, digest, headers/provenance where useful.

### 080-03 Deterministic browser
Playwright-class worker behind capability policy.

### 080-04 Hostile-content defense
Prompt injection, hidden instructions, redirects, private network/SSRF, credential exfiltration.

### 080-05 Download quarantine
PDF/data/media -> candidate source -> 075/document validation; never immediate trusted artifact.

### 080-06 Evidence capture UX
Save source/span/snapshot into Project; attach citation metadata.

### 080-07 Rights/retraction metadata
License/access class; publication version; retraction/correction state when available.

### 080-08 Qualification
Public/deidentified retrieval works; protected content egress denied; receipts complete.

---

# Spec 081 — AudioFlow Foundation + Local Scribe MVP

**Hard dependency:** 077 + 079.

## Slices

### 081-00 Capture contract
AudioSource, AudioSession, explicit capture state, device selection, encrypted rolling storage, retention.

### 081-01 Audio conditioning
VAD, levels, clipping/noise state, channel handling, session health.

### 081-02 Streaming ASR
At least one qualified local engine behind AudioRuntime.

### 081-03 Refinement ASR
Second pass; transcript revision; timestamp preservation; no source rewrite.

### 081-04 Diarization
Local speaker separation and reviewable Clinician/Patient/Caregiver/Interpreter/Unknown roles.

### 081-05 Clinical speech benchmark
Medication/dose/numeric/unit/negation/date/lab/specialty metrics plus WER/CER and Arabic-English code switching.

### 081-06 Scribe context bridge
Encounter transcript -> explicit patient/encounter ContextManifest -> MedAgent note draft.

### 081-07 Linked note evidence
Material note spans link to transcript/audio and other patient source objects.

### 081-08 Clinician review
Draft/edit/review/save; unsupported/conflict flags; generated note remains proposal until reviewed.

### 081-09 Offline qualification
Network disabled: capture -> transcript -> note draft -> linked evidence -> local save.

---

# Spec 082 — Analytics Gate + Cohort Builder

**Hard dependency:** 074 + 075 + 079.

## Slices

### 082-00 Analysis contracts
AnalysisDefinition, QueryReceipt, AnalysisRun, Figure/Table artifacts.

### 082-01 Native SQL
DataFusion or evidence-selected alternative; read-only; exact snapshot pinning.

### 082-02 Cohort Builder
Transparent inclusion/exclusion criteria; unknown handling; filter-stage counts; saved cohort definition.

### 082-03 Statistics foundation
Descriptive statistics, common tests/intervals as qualified; assumption state explicit.

### 082-04 Visualization
Tables/charts with data snapshot, query/method and run provenance.

### 082-05 Workbench integration
Saved Data Workbench view -> immutable snapshot -> analysis.

### 082-06 AI assistance
NL query/method/formula proposal only; deterministic validator/executor.

### 082-07 Reproducibility
Restart/replay; source refresh does not alter old run; deterministic seed where applicable.

### 082-08 Research synthesis analytics
Support structured systematic-review data, inter-rater agreement, effect-size/meta-analysis workflows and model-evaluation statistics through exact DataSnapshot/AnalysisRun bindings. Advanced methods may use R/Compute when native support is not justified.

### 082-09 Qualification
Independent fixtures for statistical correctness; systematic-review/meta-analysis oracle cases; split/leakage checks where model research uses the Analytics plane; large-data budgets; cancellation/timeout.

---

# Spec 083 — Clinical Graph + Knowledge/Research Canvas

**Hard dependency:** 074 + 075 + 077 + 079.  
**Integration:** web evidence after 080, audio spans after 081, analytics after 082.

## Slices

### 083-00 Projection contracts
ClinicalGraphNode/Edge, authority classes, temporal semantics, rebuild/version metadata.

### 083-01 Structured graph extraction
FHIR/project/artifact/evidence nodes; deterministic mappings.

### 083-02 Inferred relationships
Model-produced edges explicitly `INFERRED`; rationale/run identity; review/confirmation path.

### 083-03 Graph queries
Neighbors, path, explain, temporal filter, evidence resolution; Graphify-inspired behavior under MedScale semantics.

### 083-04 Retrieval index
Lexical first; graph traversal; optional vectors only after benchmark; stale/tombstone handling.

### 083-05 Evidence ledger
Claim, supporting/contradicting sources, applicability, recency, retraction, limitations.

### 083-06 Workspace block model
Document, canvas, table, graph, timeline blocks referencing canonical IDs.

### 083-07 Research Canvas
Papers, notes, datasets, hypotheses, analyses, figures, decisions, manuscript sections.

### 083-08 Patient Workspace
Summary, timeline, encounters, scribe, problems, meds, results, documents, evidence, graph, tasks.

### 083-09 AFFiNE donor qualification
Exact files/components only if needed and license-compatible; otherwise native reimplementation.

### 083-10 Systematic review + manuscript workflow
Versioned protocol, search receipts, screening states, inclusion/exclusion reasons, study extraction, risk-of-bias artifacts, evidence synthesis links, PRISMA-style event-derived flow and manuscript/report artifact composition.

### 083-11 Qualification
Graph rebuild consistency; no cross-project leaks; no silent identity merge; workspace views preserve object identity; protocol/screening/extraction/manuscript references remain revision-consistent.

---

# Spec 084 — MedScale Hub

**Hard dependency:** 074 + 076 + 079.

## Slices

### 084-00 Protocol/identity
Versioned handshake, user/device/team/project scope.

### 084-01 Collaboration sync
Rooms/threads/tasks/notes/activity.

### 084-02 Object/artifact sync
Policy-approved resumable transfer with hashes.

### 084-03 Offline/conflict
Monotonic cursor, retries, idempotency, conflict responses.

### 084-04 Revocation/deletion
Access revocation, tombstone propagation, honest backup retention.

### 084-05 Self-host deployment
Single-Hub first; backup/restore; upgrade/rollback.

### 084-06 Qualification
Two clients, offline edits, reconnect, revoked user, tenant isolation, malicious digest.

---

# Spec 085 — MedScale Compute

**Hard dependency:** 077 + 079.

## Slices

### 085-00 Job contract
ComputeJobManifest, staged inputs, resources, network/filesystem/secret policy.

### 085-01 Local worker
Bounded process/container/sandbox selected by platform evidence.

### 085-02 Model/document workers
General worker protocol for heavier admitted jobs.

### 085-03 Python
Python execution only inside Compute; environment/lock evidence.

### 085-04 GPU scheduling
Device selection, VRAM budget, cancellation/OOM.

### 085-05 Output admission
Digest/schema/classification/policy validation before artifact creation.

### 085-06 Remote self-host worker
Only after local proof; authenticated/staged; no vault mount.

### 085-07 Model research jobs
Permit bounded evaluation and, only when rights/privacy/resource policy allows, training/fine-tuning jobs over staged exact dataset releases. Job output is candidate artifact only and cannot self-admit into Model Fleet.

### 085-08 Qualification
Filesystem escape, egress denial, timeout/OOM/crash/late result, reproducibility, data-split access controls, training-output candidate/admission separation.

---

# Spec 086 — R Workspace

**Hard dependency:** 075 + 079 + 082 + 085.

## Slices

- exact DataSnapshot staging;
- Arrow/Parquet default interchange;
- .Rproj + scripts + outputs + manifest;
- renv.lock digest and restore state;
- Open in RStudio / Positron / folder;
- Rscript through Compute only;
- output publish/admission path;
- RRunReceipt;
- offline package limitations;
- qualification with deterministic fixture analyses.

---

# Spec 087 — Community Extensions

**Hard dependency:** 079 + 084 + 085.

## Slices

- extension manifest;
- publisher/provenance/signature;
- capability declarations;
- WASM/isolated runtime benchmark and admission;
- no ambient vault/keychain/network;
- capability diff/re-consent on update;
- extension UI descriptor boundaries;
- offline/manual install;
- registry metadata separated from client authority;
- revoke/quarantine/rollback;
- malicious extension qualification.

---

# Spec 088 — AudioFlow Advanced + Clinical Documentation Expansion

**Hard dependency:** 081 + 084 for team/shared workflows where used.  
**Uses:** 077/078/079/083/090 contracts where available.

## Slices

### 088-00 Specialty templates
Primary care, internal medicine, emergency, psychiatry, pediatrics and additional specialties through declarative versioned templates.

### 088-01 Clinician style
Learned formatting/style from clinician-approved edits without training on PHI outside local policy.

### 088-02 Pre-visit
Local patient-context summary with exact source links and freshness state.

### 088-03 Active voice assistant
Read-only chart/evidence questions; command proposals; explicit tool policy.

### 088-04 Coding proposals
ICD and other code systems only with licensed/versioned terminology Packs; evidence and review.

### 088-05 Orders/tasks
Proposed orders/tasks with exact context and explicit approval; external write waits for 090.

### 088-06 Nursing
Assessment/flowsheet/handoff draft proposals linked to transcript/source evidence.

### 088-07 Patient instructions
Clinician-reviewed plain-language after-visit materials, multilingual where qualified.

### 088-08 Revenue-cycle intelligence
Documentation/coding discrepancy candidates with exact source evidence. Cover, when licensed and qualified:
- inpatient CDI before discharge;
- final coded diagnosis / DRG discrepancy review;
- outpatient diagnosis/HCC risk-gap candidates;
- MEAT-style documentation support;
- E&M level proposals/rationale;
- pre-bill review.

No reimbursement, coding-compliance or payer-acceptance claim follows from a suggestion.

### 088-09 Pre-round / inpatient context
Source-linked pre-round summaries, new/changed results, medications, problems, unresolved conflicts and care-team activity. Critical omission and freshness are measurable states.

### 088-10 Clinical-trial candidate matching
Use local Evidence/Research Packs or Governed Browse registry sources to map explicit patient context to trial criteria. Every criterion is `MATCHED`, `NOT_MATCHED`, `UNKNOWN` or `UNAVAILABLE`; final eligibility remains external/human authority.

### 088-11 Prior-authorization drafting
Generate clinician-reviewable medical-necessity/prior-auth drafts from exact patient evidence, planned treatment and versioned payer/policy context where available. No network submission in this slice.

### 088-12 Discharge / order-set proposals
Evidence- and patient-context-linked discharge-planning and order-set candidates remain reviewable ActionProposals. External effects belong to 090.

### 088-13 Multilingual clinical qualification
Arabic, English, code switching and additional language Packs through measured admission.

### 088-14 Long-session/recovery
Multi-hour sessions, crash/restart, capture-device changes, low-power degradation.

---

# Spec 089 — Research Packs + Evidence Packs

**Hard dependency:** 074 + 075 + 077 + 082 + 083.

## Pack classes

- specialty evidence Pack;
- guideline Pack;
- terminology Pack;
- literature corpus Pack;
- clinical template Pack;
- benchmark/evaluation Pack;
- research-method Pack;
- systematic-review method/template Pack;
- ClinicalTool Pack;
- dataset schema/validation Pack;
- dataset/evaluation benchmark Pack.

## Slices

- signed/versioned manifest;
- source/rights/license metadata;
- exact content digests;
- update/supersession/revocation;
- citation and evidence metadata;
- declarative validators/workflows;
- local indexing;
- no arbitrary trusted-process code;
- migration/uninstall behavior;
- offline installation;
- qualification of one end-to-end evidence Pack.

---

# Spec 090 — Institutional Adapters + Clinical Effect Integration

**Hard dependency:** 075 + 079 + 084 + 085 + 089.

## Foundation adapters

### Clinical data
- FHIR R4/SMART;
- existing MedScale FHIR/network contracts;
- HL7 v2 read/write where institutionally required and profile-qualified;
- CDA/CCDA import where justified;
- DICOM/DICOMweb metadata/content adapter when promoted.

### Identity
- institution SSO/OIDC/SAML as separate identity adapter;
- patient/subject matching remains evidence-based;
- no silent record merge.

### EHR effects
- note export/writeback;
- task/order proposal submission where authorized;
- result/document attachment;
- durable outbox;
- idempotency;
- unknown/reconcile state.

### Payer / authorization effects
- prior-authorization submission only after approved draft/review;
- status polling;
- denial/appeal workflow as explicit states;
- payer/rules/profile identity and version where available;
- idempotent retries and lost-response reconciliation;
- no automatic claim that medical necessity was accepted.

### Communication effects
Optional, separately qualified adapters for institution-approved calls/messages/voicemail/fax/email or equivalent channels:
- verified sender/patient/delegate identity;
- channel-specific consent and data minimization;
- delivery status / UNKNOWN state;
- no patient content in ordinary logs;
- no requirement for a MedScale-hosted communications cloud.

### Institutional data
- warehouses/databases/object stores;
- source-specific capability and data-class policy.

## Clinical-effect slices

- draft note -> clinician-approved artifact -> adapter payload;
- action proposal -> clinician approval -> EffectIntent;
- prior-auth draft -> approved request -> submit/status/denial/appeal/reconcile;
- approved patient communication -> channel adapter -> delivery/UNKNOWN/reconcile;
- delivery acknowledgement;
- timeout/unknown;
- reconciliation;
- amendment/correction.

## Qualification

- profile/version mismatch;
- expired auth;
- lost network after submit;
- duplicate retry;
- partial/unknown response;
- mapping loss;
- revoked permission;
- audit completeness.

---

# Spec 091 — Federation

**Hard dependency:** 090.

Federation is optional and later.

## Foundation

- site-owned data by default;
- explicit query/aggregate contracts;
- provenance per site/input;
- no raw cross-site transfer by default;
- statistical validity and privacy method per use case;
- site policy intersection;
- revocation/expiry/partition behavior;
- no global patient identity assumption.

Candidate use cases:

- multi-site research aggregates;
- model/evaluation metrics;
- approved federated analysis.

No federated training is implied.

---

# Spec 092 — Whole-Platform Qualification

**Hard dependency:** all promoted/required predecessors.

This spec does not add major new product capability. It proves the integrated product.

## 092 campaigns

### 092-A Authority
No model/graph/browser/collab/worker can bypass Core authority.

### 092-B Privacy
No hidden egress; local-mode packet/network proof; secret/PHI log scans; deletion/retention proof.

### 092-C Local/offline
Scribe, patient review, local evidence, Data Workbench and analytics remain useful without network.

### 092-D Scribe quality
Execute the governed protocol in `LOCAL_MEDICAL_SCRIBE_EVALUATION_PROTOCOL_2026-09-19.md`: medical ASR + diarization + note-grounding + Linked Evidence + critical omissions + clinician review workload + offline/privacy/resource campaigns.

### 092-E Evidence quality
Execute `EVIDENCE_ENGINE_EVALUATION_PROTOCOL_2026-09-19.md`: citation identity, claim support, retrieval, evidence quality, contradiction, applicability, guideline/jurisdiction conflict, retraction/correction, rights and deep-synthesis fixtures.

### 092-F Analytics + research reproducibility
Snapshot reproducibility, statistical correctness, R/Compute publication and provenance, systematic-review replay, protocol amendment history, screening/extraction consistency, meta-analysis oracle cases and manuscript artifact freshness.

### 092-G Graph/workspace
Projection rebuild, temporal queries, identity/conflict safety, cross-view ID consistency.

### 092-H Interoperability
FHIR/SMART and any promoted institutional adapters with exact-version conformance evidence, including any promoted payer/prior-auth and communication adapters.

### 092-I Security
Browser prompt injection, malicious documents, extension escape, compute escape, credential exfiltration, SSRF, path traversal and hostile model outputs.

### 092-J Recovery
Vault restore, schema migrations, interrupted imports, audio crash recovery, Hub restore, Pack rollback.

### 092-K Performance
Declared hardware classes; p50/p95; RAM/VRAM; long-session; large data; local model startup and throughput.

### 092-L Accessibility/localization
Keyboard/focus, screen reader semantics where platform supports, reduced motion, long strings, RTL/Arabic, contrast, touch targets.

### 092-M Packaging/update
Signed/checksummed artifacts, SBOM/NOTICE, Pack update/revocation, app upgrade/rollback.

### 092-N Human validation
Clinician and researcher usability/safety study protocol using authorized non-production/synthetic or appropriately approved data. Research workflow validation includes screening/adjudication, extraction, analysis and report traceability.

### 092-N2 Clinical tools / model research
Deterministic clinical-tool test vectors, medication/reference coverage honesty, dataset split/leakage checks, annotation provenance and training-output-to-Pack admission separation.

### 092-O Release truth
Reconcile every release/privacy/multi-client gate. No release claim from partial success.

---

# 8. Vertical milestones

Specs remain the governance units; milestones are product integration targets.

## M1 — Local Research Workbench
075 + 077 + 078 + 079:
- project data;
- local model;
- evidence-linked answers over local sources;
- Data Workbench foundation;
- privacy gating.

## M2 — Local Clinical Scribe
081 + relevant 077/078/079:
- offline capture;
- transcript;
- diarization;
- note draft;
- linked evidence;
- clinician review.

## M3 — Evidence + Knowledge OS
080 + 083 + 089:
- literature acquisition;
- evidence claims/citations;
- Clinical/Research Graph;
- canvas/library;
- evidence Packs.

## M4 — Reproducible Analytics
082 + 085 + 086:
- cohorts;
- SQL/stats/charts;
- Python/R via bounded Compute;
- reproducible reports.

## M5 — Team / Lab
076 + 084:
- review/comments/tasks;
- self-hosted collaboration;
- multi-device/lab workflows.

## M6 — Advanced Clinical Workflow
088 + 090:
- pre-visit;
- specialty/nursing;
- coding/order/task proposals;
- patient instructions;
- EHR/institution adapters.

## M7 — Ecosystem
087 + optional 091:
- extensions;
- institutional/site scale;
- federation only when justified.

## M8 — Qualified Product
092:
- integrated evidence needed before any release/private/offline/parity claims.

---

# 9. Missing-domain protections

Every promoted spec must explicitly answer these questions.

## Identity
- Which patient/subject/project identity is used?
- Can two sources refer to the same entity?
- How is candidate matching reviewed?
- What prevents silent merge?

## Time
- source time;
- effective clinical time;
- ingestion time;
- model/run time;
- timezone/precision;
- amended/corrected time.

## Rights
- source license;
- model license;
- terminology license;
- paper/full-text rights;
- dataset terms;
- redistribution limitations.

## Privacy
- classification;
- purpose;
- retention;
- egress;
- logs/caches;
- deletion;
- backups.

## Provenance
- exact source revision;
- transformation;
- engine/model;
- parameters;
- reviewer;
- output digest.

## Failure
- denied;
- unavailable;
- timeout;
- partial;
- stale;
- conflicting;
- unknown;
- corrupt;
- unsupported version.

## Recovery
- restart;
- migration;
- backup;
- rollback;
- interrupted operation;
- late external result.

If a promoted spec does not answer all applicable domains, it is not implementation-ready.

---

# 10. Donor/source strategy

Adoption hierarchy:

```text
MedScale native contract
  -> reference pattern
  -> reimplement/adapt
  -> bounded dependency
  -> isolated worker
  -> selective copied component
```

Wholesale donor-platform absorption is rejected by default.

New source qualification is recorded separately in:
- `RESEARCH_OS_V2_SOURCE_QUALIFICATION_LEDGER_2026-09-19.md`.

---

# 11. UI integration

The product design lane remains separate from backend authority.

V0/Identity V2 governs visual system.

Implementation requirements:

- one component/token system;
- light/dark parity;
- patient/clinical/evidence states never communicated by color alone;
- source/evidence state near generated outputs;
- local/network/capture state visible but not theatrical;
- graph is an optional lens, never mandatory navigation;
- Data Workbench supports dense research tables;
- Scribe optimizes for capture/review, not dashboard cards;
- Research Canvas optimizes for composition and provenance.

V0 code is reference until explicitly accepted/ported into production authority.

---

# 12. Promotion protocol

This plan is deliberately implementation-ready but not self-authorizing.

For every candidate:

1. fetch live `main`;
2. verify predecessor closure;
3. verify spec number is unused/current;
4. bind planning contracts to exact repository types/files;
5. write promoted Spec Kit package;
6. include task list, migration, threat-model delta, qualification commands and evidence paths;
7. record donor decisions and exact revisions;
8. implement smallest vertical slice;
9. qualify exact candidate head;
10. merge normally;
11. verify post-main;
12. update BUILD_QUEUE;
13. only then promote the next dependency-ready unit.

No agent may interpret this master plan as authority to implement all 075–092 on one branch.

---

# 13. Definition of program success

MedScale eventually succeeds only if evidence establishes all applicable claims, including:

- local scribe genuinely works offline;
- no default patient-content egress;
- note claims trace to source evidence;
- medical speech errors are measured, not hidden by generic WER;
- evidence answers identify sources and claim support;
- contradictory evidence is visible;
- patient/source identity is safe;
- datasets are versioned and analyses reproducible;
- graph/workspace views preserve canonical object identity;
- local model replacement does not change authority semantics;
- institutional effects are durable and reconcilable;
- security boundaries survive hostile web/doc/model/extension inputs;
- application update/recovery is proven;
- accessibility/localization are measured;
- release statements match exact evidence.

Until those gates are proven, MedScale is a progressively implemented research/product platform, not a clinically validated replacement claim.
