# Research OS V2 — Clinical Intelligence Expansion

**Date:** 2026-09-19  
**Status:** `PLANNING_CANDIDATE_ONLY` — not implementation authority  
**Founder direction:** MedScale should become a local-first, privacy-first alternative to the combined value of Abridge, OpenEvidence, and OpenMed, while adding first-class research, datasets, analytics, knowledge graph, collaboration, and reproducibility.

## 1. Product thesis

MedScale should not become three cloned products glued together.

The target is one **Clinical + Research Intelligence OS** with one trusted local authority model and multiple coordinated experiences:

```text
Clinical conversation
        |
        v
Local Scribe --------> Patient / Encounter Workspace
        |                        |
        v                        v
Transcript + Note -----> Clinical Graph Projection
        |                        |
        +----------+-------------+
                   |
                   v
       Evidence + Research Engine
                   |
          +--------+--------+
          |                 |
          v                 v
     Data Workbench     Analytics Gate
          |                 |
          +--------+--------+
                   |
                   v
        Project + Artifact Graph
                   |
                   v
   Models / Agents / Documents / Team
```

The defining promise is:

> **Abridge-class ambient workflow + OpenEvidence-class evidence intelligence + OpenMed-class local medical AI, with user-owned data, local execution by default, and evidence/provenance across every derived output.**

This is a product direction, not a current capability claim.

## 2. Non-negotiable boundary

Existing MedScale authority remains intact:

- Rust-owned trusted authority path.
- local-first and privacy-first;
- runtime network default deny unless explicitly brokered;
- FHIR/interchange never silently becomes canonical authority;
- derived graph state never silently becomes source truth;
- model output remains proposal/evidence, not clinical authority;
- consequential orders, codes, actions, patient instructions, exports, and external writes require explicit review/authorization;
- unknown/conflict/stale/unresolved states remain first-class;
- no real PHI is authorized until the repository's privacy/release gates establish that authority;
- MESC remains a separate project and is not a MedScale dependency or release gate.

## 3. Competitive capability floor

### Abridge capability floor

Public Abridge materials currently describe:

- ambient conversation capture and AI-drafted clinical documentation;
- outpatient, inpatient, emergency and multi-specialty workflows;
- multilingual clinical conversations;
- direct EHR workflow integration;
- Linked Evidence tying AI-drafted note content back to source information;
- contextual notes using previous encounters, health-system guidelines and clinician preferences;
- problem prediction/grouping aligned to coding language;
- proposed actionable outputs such as medical orders for clinician review;
- clinician, nursing and revenue-cycle product experiences;
- clinical decision support and Care Signals;
- pre-visit and pre-round summaries;
- conversational nursing flowsheet drafts and care-team context;
- inpatient CDI and pre-bill diagnosis/DRG discrepancy review;
- HCC/risk-gap and MEAT-style documentation support;
- clinical-trial matching direction;
- payer-provider prior-authorization workflow direction;
- enterprise governance, analytics and reporting.

Primary references:
- https://www.abridge.com/
- https://www.abridge.com/platform/clinicians
- https://www.abridge.com/platform/nursing
- https://www.abridge.com/platform/revenue-cycle
- https://www.abridge.com/keynote
- https://www.abridge.com/press-release/pre-bill-review-for-cdi-and-coding-teams
- https://www.abridge.com/press-release/abridge-availity-collaboration-announcement

MedScale target: reproduce the workflow value locally where technically and legally possible, while providing stronger local provenance, configurable retention, offline operation, and open model/runtime choice.

### OpenEvidence capability floor

Current public and peer-reviewed references describe:

- clinician-focused clinical question answering;
- retrieval-augmented answers grounded in curated biomedical sources;
- explicit per-answer citations;
- evidence-strength grading/visualization;
- publisher, guideline, society and systematic-review content partnerships;
- voice question/answer mode;
- continuing education dashboard/credits;
- patient-facing TakeHome educational material;
- mobile and web access;
- Visits-style patient-context documentation and document management;
- deep/extended evidence consultation workflows;
- privacy-oriented clinician-patient communication/Dialer category capability;
- evolving coding, discharge/order-set and administrative assistance reported in current product reviews.

Primary references:
- https://www.nature.com/articles/s41746-026-03077-4
- https://www.nature.com/articles/s44401-026-00142-8
- https://takehome.openevidence.com/
- https://apps.apple.com/us/app/openevidence/id6612007783
- https://www.newswise.com/articles/openevidence-launches-evidencegrade-empowering-physicians-to-see-the-strength-of-cited-evidence-beneath-each-ai-answer
- https://www.newswise.com/articles/openevidence-wide-releases-ai-integrated-doctor-dialer-for-privacy-centric-doctor-patient-telemedicine-calls-messaging-and-voicemail-in-one-unified-clinical-platform-with-live-clinical-decision-ai-deeply-integrated

MedScale target: build an evidence engine whose citation existence, claim support, evidence quality, contradiction state, recency, jurisdiction, and patient applicability can be inspected separately instead of collapsing them into one confidence score.

### OpenMed capability floor

OpenMed currently provides a strong local medical-model capability floor including:

- local clinical NER;
- PII/PHI de-identification;
- local/on-device model execution;
- multilingual medical models;
- Apple MLX and Python-oriented paths;
- model catalog/runtime patterns;
- medical vision-language and specialist model families;
- multimodal/structured healthcare inputs including document, image, DICOM, HL7 v2, CDA/C-CDA and FHIR-oriented utilities;
- policy-aware de-identification, signed audit/reporting concepts and explicit air-gapped local execution.

Primary references:
- https://github.com/maziyarpanahi/openmed
- https://openmed.life/docs/

MedScale target: use OpenMed as a qualified donor/model ecosystem rather than making it clinical authority.

## 4. The MedScale differentiator: Linked Evidence Everywhere

Abridge demonstrates why transcript-linked notes matter. MedScale should extend that concept across the entire product.

Any consequential derived statement should be able to resolve to a provenance chain such as:

```text
Generated sentence
 -> source transcript segment
 -> audio time span
 -> encounter
 -> patient source identity
 -> prior FHIR field / document page / lab observation
 -> retrieved paper or guideline
 -> model/run identity
 -> transformation / extraction receipt
 -> review state
```

The user should be able to ask:

- Where did this sentence come from?
- Was it spoken, imported, inferred, or clinician-confirmed?
- Which patient record fields contributed?
- Which paper/guideline supports it?
- Is any source contradictory or stale?
- Which model/runtime created the derived claim?
- Has a clinician reviewed or edited it?
- What changed since the previous version?

This is stronger than a citation badge and should become a MedScale product signature.

## 5. Unified Clinical Graph

MedScale should add a **derived Clinical Graph Projection** over canonical source objects.

Candidate node families:

```text
Patient / Subject
Encounter
Practitioner
Organization
Condition
Observation
Medication
Allergy
Procedure
DiagnosticReport
ImagingStudy metadata
Document
AudioSession
TranscriptSegment
ClinicalNote
Guideline
Paper
Citation
Claim
EvidenceItem
Dataset
Cohort
AnalysisRun
ModelRun
Artifact
Task
Decision
ActionProposal
```

Candidate edge classes:

- `EXTRACTED` — explicit in source data;
- `MAPPED` — deterministic mapping, for example FHIR-to-projection;
- `DERIVED` — deterministic derived transformation;
- `INFERRED` — probabilistic/model-produced relation;
- `CLINICIAN_CONFIRMED` — explicit human confirmation;
- `CONTRADICTS` / `SUPPORTS` — evidence relationship;
- `SUPERSEDES` / `AMENDS` — version/clinical correction relationship.

Every edge carries source provenance, time, transformation identity, review state and authority class.

**Critical rule:** the Clinical Graph is a projection/index. It does not replace canonical FHIR/source custody, Core authority, or signed artifact identity.

## 6. Graphify + AFFiNE synthesis

Use Graphify and AFFiNE for different jobs.

### Graphify-inspired engine

Study/adapt:

- real graph traversal rather than vector-only retrieval;
- explained edges;
- explicit `EXTRACTED` versus `INFERRED` relations;
- shortest-path/explain/query operations;
- community/subgraph discovery;
- one graph spanning heterogeneous artifacts;
- local processing where possible.

Reference:
- https://github.com/Graphify-Labs/graphify

Do not copy Graphify's software-code domain model into clinical truth. MedScale owns healthcare semantics and provenance.

### AFFiNE-inspired workspace

Study/adapt:

- one object rendered as document, canvas, table or linked block;
- docs + whiteboard/canvas + database-style views;
- local-first workspace expectations;
- self-hosting;
- collaboration and block-level composition.

Reference:
- https://github.com/toeverything/AFFiNE

MedScale should not wholesale embed AFFiNE. Exact file-level licensing/provenance review is required before any direct code transfer.

### Combined result

```text
MedScale canonical objects
        |
        +--> Clinical Graph Projection (Graphify-inspired)
        |
        +--> Workspace Views (AFFiNE-inspired)
                  |
                  +--> document
                  +--> canvas
                  +--> table
                  +--> timeline
                  +--> graph
                  +--> evidence ledger
```

All views reference the same MedScale object/artifact IDs. They do not create competing truth stores.

## 7. Data Workbench: Airtable-class capability without Airtable authority

The Data Source Fabric should grow into a native **MedScale Data Workbench**.

Preferred donor study order:

1. **Grist Community** — Apache-2.0 relational spreadsheet patterns, local/self-hosted operation.
2. **Baserow OSE** — MIT open-source core, headless/API-first no-code database patterns.
3. **NocoDB** — feature reference only unless licensing changes; current main/develop source is under Sustainable Use License with commercial-use restrictions.
4. **Teable** — feature reference/selective MIT-package study only; core applications are AGPL-3.0 with additional brand terms.

MedScale should own its data semantics instead of embedding an Airtable clone.

Required Data Workbench experiences:

- grid/table view;
- forms;
- gallery;
- kanban;
- calendar/time views;
- linked records/relations;
- formulas/computed fields;
- schema and type inference;
- validation rules;
- data dictionary;
- filtering, grouping, sorting and saved views;
- revisions/snapshots;
- row/column provenance and lineage;
- imports from CSV/XLSX/JSON/Parquet;
- FHIR resource/materialized views;
- SQL/database read adapters through governed Data Source Fabric;
- export;
- permissions;
- pseudonymized analytical views;
- no-code transformations that compile to reproducible run receipts;
- dataset snapshots as first-class Project/Artifact Graph objects.

## 8. Local Medical Scribe

The scribe must be useful with no MedScale cloud.

Pipeline:

```text
Explicit capture state
 -> local audio capture
 -> VAD / quality monitoring
 -> streaming ASR first pass
 -> speaker diarization
 -> timestamped transcript
 -> local clinical entity extraction
 -> prior-context assembly
 -> local note draft
 -> unsupported-claim / contradiction checks
 -> Linked Evidence mapping
 -> clinician edit/review
 -> approved FHIR/EHR export or action proposal
```

Detailed architecture is defined in `LOCAL_MEDICAL_SCRIBE_PLAN.md`.

User-owned Himsat and Wispral provide strong planning/research patterns for capture, local speech, diarization, evidence-linked session memory and STT benchmarking. Their code and authority remain separate unless exact donor qualification approves a bounded transfer.

## 9. Evidence Copilot

The OpenEvidence-class experience should include:

- clinical question decomposition;
- rights-aware local/open literature ingestion;
- PubMed/PMC/open-access-first acquisition;
- optional institution-licensed source adapters without redistributing restricted content;
- hybrid retrieval: lexical + semantic + citation/clinical graph + reranking;
- explicit source filters: guideline, systematic review, RCT, cohort, case report, regulator, society;
- per-claim citations;
- citation existence verification;
- citation-to-claim support verification;
- contradiction detection;
- evidence-strength assessment with individual dimensions rather than only one letter;
- population/intervention/outcome applicability;
- jurisdiction/guideline applicability;
- recency and retraction state;
- uncertainty and missing-evidence explanation;
- local voice mode;
- patient education / TakeHome-like output after clinician review;
- saved evidence collections and Research Packs;
- transparent Deep Consult mode with an inspectable research plan, search/retrieval trail, source set, contradictions, unresolved gaps and immutable evidence snapshot;
- clinical-trial candidate search with explicit criterion mapping and unresolved eligibility;
- evidence-aware encounter documentation without conflating external literature with patient-specific facts.

Source acquisition and rights rules are defined in `MEDICAL_EVIDENCE_SOURCE_STRATEGY_2026-09-19.md`. Evidence quality must be evaluated using `EVIDENCE_ENGINE_EVALUATION_PROTOCOL_2026-09-19.md`.

CME/MOC credit issuance is not a software-only capability and must remain an external accreditation/partnership gate.

## 10. Patient + encounter workspace

Patient workspace candidate views:

```text
Summary
Timeline
Encounters
Scribe
Problems
Medications
Allergies
Results
Documents
Evidence
Graph
Tasks
Data
Analytics
Audit
```

The workspace should distinguish:

- source facts;
- derived summaries;
- model suggestions;
- unresolved conflicts;
- outdated information;
- clinician-confirmed updates.

No graph visualization should become the only way to inspect a patient.

## 11. Analytics + research

Analytics Gate should become a reproducible local analysis plane, not a decorative dashboard.

Every analysis result should bind:

- dataset snapshot;
- cohort/filter definition;
- code/query/method;
- package/runtime identity;
- parameters;
- random seed where relevant;
- generated tables/figures;
- model/tool identity when AI-assisted;
- run receipt;
- limitations;
- publication/export artifact.

Research Canvas should combine:

- papers;
- citations;
- notes;
- datasets;
- hypotheses;
- protocols;
- analyses;
- figures;
- canvases;
- graph views;
- decisions;
- manuscript/report artifacts.

## 12. Existing Research OS V2 mapping

Do not create unnecessary top-level specs if the current 075–092 candidate sequence can own the work.

| Candidate | Expanded ownership |
|---|---|
| 075 Data Source Fabric | connectors, snapshots, Data Workbench foundation, table/views/imports |
| 076 Collaboration Substrate | local-first workspace collaboration, comments, review, shared blocks |
| 077 MedAgent Workbench | Evidence Copilot, clinical agent lanes, patient-aware tool context |
| 078 Model Fleet + Compare | OpenMed/model packs, local runtime fleet, quality/resource comparison |
| 079 Privacy Gate | PHI/PII policy, de-identification, export gating, local/remote boundary |
| 080 Governed Browse | medical web/literature browse, capture, provenance, source receipts |
| 081 AudioFlow Foundation | scribe capture, VAD, ASR, diarization, transcript authority |
| 082 Analytics Gate | reproducible analytics, cohort/data analysis, visualization |
| 083 Knowledge + Research Canvas | AFFiNE-inspired block/canvas/table views + Clinical Graph lenses |
| 084 MedScale Hub | connectors/extensions/catalog UX |
| 085 MedScale Compute | isolated local jobs and heavier analysis/model workers |
| 086 R Workspace | R/Posit external-tool integration and reproducibility |
| 087 Community Extensions | sandboxed extension ecosystem |
| 088 AudioFlow Advanced | specialty scribe templates, code-switching, pre-round context, nursing/documentation flows, CDI/coding/risk-gap proposals, prior-auth drafts, discharge/order-set proposals |
| 089 Research Packs | local evidence corpora, guidelines, terminology, trial-registry/evidence snapshots, reusable research/evidence packs |
| 090 Institutional Adapters | EHR/SMART/FHIR/vendor adapters, payer/prior-auth effects, optional secure communication channels, enterprise identity/governance |
| 091 Federation | optional encrypted institution/site collaboration after local core proves safe |
| 092 Whole-Platform Qualification | cross-surface privacy, accuracy, safety, performance and release evidence |

No candidate 075+ is implementation-authorized by this planning document.

## 13. Sibling-project leverage

High-value public sibling patterns identified in the founder's GitHub estate:

- Himsat — local conversation capture, transcription, diarization, evidence-linked memory and document intelligence;
- Wispral — local streaming STT bakeoffs and voice interaction research;
- commandMed — medical-model safety, evidence/tool separation and multilingual evaluation;
- Ecra — governed browser/search/knowledge/action patterns;
- Signthos — local PDF/OCR/document-processing security;
- Zyara — patient/provider graph, FHIR boundaries, consent and healthcare workflow patterns;
- commandF — FHIR/interoperability verification;
- Sentrdel — evidence/provenance/security classification;
- Golam / Kernux / Tarif — local agent, capability and execution authority patterns;
- SpecGrain / Diffcipline / Winds / Kodac / Delethos / Ascout — bounded execution and proof-before-done methodology.

Private sibling repositories may inform planning internally, but private source details must not be published or copied into MedScale without explicit disclosure/provenance authority.

## 14. What should be measurably better

MedScale should aim to prove, not merely claim:

- useful scribe operation with network disconnected;
- raw encounter audio never leaving the device in default configuration;
- exact note-span -> transcript/audio/source linkage;
- patient-context derivation with visible source fields and timestamps;
- evidence answers with citation-existence and claim-support checks;
- evidence-strength dimensions with uncertainty rather than false precision;
- versioned local datasets and reproducible analytics;
- one patient/research object rendered consistently across timeline/table/canvas/graph views;
- local model substitution without changing clinical authority semantics;
- explicit per-tool/per-connector capabilities;
- offline degradation that remains useful rather than failing closed unnecessarily;
- clear performance/resource budgets for ordinary workstation hardware.

These become qualification targets only when their owning specs are promoted.

## 15. Planning next step

Before promoting 075, reconcile this expansion with:

- `RESEARCH_OS_PROGRAM_AMENDMENT_001_DATA_EXTENSIONS.md`;
- `RESEARCH_OS_EXECUTION_ROADMAP.md`;
- `RESEARCH_OS_V2_DECISION_REGISTER.md`;
- `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md`;
- `SOURCE_ADOPTION_MATRIX.md`;
- `OSS_CODE_ABSORPTION_MATRIX_V2.md`;
- the separate Orca donor planning PR if it has not yet landed;
- the MedScale Identity/V0 lane without mixing visual work into backend authority.

Then promote only the next dependency-ready bounded unit through canonical governance.
