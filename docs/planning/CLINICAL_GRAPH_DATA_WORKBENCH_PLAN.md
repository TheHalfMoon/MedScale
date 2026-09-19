# Clinical Graph + Data Workbench Plan

**Date:** 2026-09-19  
**Status:** `PLANNING_CANDIDATE_ONLY`  
**Purpose:** define how Graphify-inspired graph intelligence, AFFiNE-inspired workspace views, and Airtable-class data tooling can strengthen MedScale without creating competing clinical truth stores.

## 1. One object model, many views

The central rule is:

> **Do not build a graph database product, a wiki product, a spreadsheet product, and a clinical record product as separate authorities.**

MedScale should keep one canonical authority path and expose multiple projections/views over the same objects.

```text
Canonical MedScale source + artifact IDs
                |
        +-------+--------+
        |                |
        v                v
Clinical Graph       Dataset Projection
        |                |
        +-------+--------+
                |
                v
        Workspace View Layer
                |
      +---------+----------+---------+---------+
      |         |          |         |         |
      v         v          v         v         v
   Timeline   Table      Canvas     Graph     Note
```

## 2. Clinical Graph authority model

The graph is a **derived projection**.

It may accelerate:

- navigation;
- retrieval;
- relationship discovery;
- longitudinal summarization;
- evidence explanation;
- cohort exploration;
- hypothesis generation.

It must not:

- silently merge identities;
- turn inference into a source fact;
- overwrite FHIR/source records;
- authorize an external action;
- make graph centrality equal clinical importance;
- make an LLM-generated edge authoritative.

## 3. Graphify concepts to adapt

Study Graphify for:

- heterogeneous objects in one graph;
- explicit explained edges;
- `EXTRACTED` versus `INFERRED` labels;
- path/query/explain operations;
- subgraph/community discovery;
- source references on graph facts;
- local-first computation where possible;
- durable machine-readable graph artifacts.

MedScale-specific extension:

```text
EXTRACTED
MAPPED
DERIVED
INFERRED
CLINICIAN_CONFIRMED
SUPPORTS
CONTRADICTS
AMENDS
SUPERSEDES
SAME_AS_PROPOSED
SAME_AS_CONFIRMED
```

Identity matching should use proposed/confirmed semantics rather than silent `same_as`.

## 4. Graph edge contract

A clinical graph edge should carry at minimum:

- edge ID;
- source node ID;
- target node ID;
- relation type;
- authority class;
- source artifact/resource ID;
- source revision;
- observed/effective time where relevant;
- created time;
- derivation method;
- model/tool/run ID if probabilistic;
- rationale/explanation;
- confidence if meaningful;
- review state;
- supersession/amendment state.

Confidence must never replace authority or review state.

## 5. Temporal graph

Healthcare relationships change over time.

Examples:

- medication active during one interval;
- diagnosis suspected before confirmation;
- allergy corrected;
- lab result amended;
- clinician relationship changed;
- guideline superseded.

Graph queries should support:

- valid/effective time;
- source-recorded time;
- ingestion time;
- knowledge-at-time queries.

A user should be able to answer:

> What did the record support at the time of this decision?

not only:

> What does the newest projection say now?

## 6. Patient graph views

Candidate graph lenses:

- longitudinal clinical graph;
- medication/condition relationship;
- result-to-assessment provenance;
- encounter/document source graph;
- evidence-to-claim graph;
- care-team graph;
- action/task graph;
- dataset/cohort membership graph.

Graph view remains optional. The same information must remain inspectable through conventional accessible lists/timelines/tables.

## 7. Research graph views

Candidate research nodes:

- paper;
- author;
- guideline;
- recommendation;
- clinical question;
- PICO element;
- claim;
- citation;
- dataset;
- variable;
- protocol;
- hypothesis;
- analysis;
- figure;
- manuscript section;
- decision.

This allows a project to connect:

```text
Clinical question
 -> evidence
 -> dataset
 -> analysis
 -> interpretation
 -> manuscript/report
```

with exact artifact provenance.

## 8. AFFiNE concepts to adapt

Study AFFiNE for:

- blocks as composable workspace units;
- docs and canvas in one workspace;
- tables/database views;
- backlinks/linked objects;
- reusable content in multiple contexts;
- self-hosting/local-first expectations;
- collaborative composition.

MedScale-specific rule:

A workspace block references a canonical object/artifact. It does not clone clinical truth into an independent note database.

Example:

```text
Patient Observation object #obs-42
  -> timeline card
  -> table row
  -> canvas block
  -> graph node
  -> evidence panel reference
```

All five views resolve to the same underlying identity.

## 9. Collaboration boundary

Collaborative note/canvas text can use CRDT/event-log approaches.

Canonical healthcare facts should not be blindly CRDT-merged.

Separate:

- user-authored workspace content;
- source clinical facts;
- derived AI proposals;
- approved canonical amendments/actions.

A simultaneous edit conflict in a research note is different from conflicting medication information.

## 10. Data Workbench

The Data Workbench is MedScale's Airtable-class surface for researchers, analysts, clinicians and operations teams.

It is not the canonical patient database.

Core entities:

```text
DataSource
DataSnapshot
Dataset
Table
Column
RowIdentity
View
Transformation
CohortDefinition
ValidationRule
AnalysisBinding
Export
```

## 11. Required views

Foundation views:

- Grid
- Form
- Gallery
- Kanban
- Calendar/time
- Summary/pivot-like aggregate view

Later/optional:

- geospatial where justified;
- network/graph lens;
- record detail form;
- dashboard composition.

Each saved view binds to a dataset snapshot or defined refresh policy.

## 12. Field types

Candidate types:

- text;
- rich text;
- integer/decimal;
- boolean;
- date/date-time/duration;
- category/single-select/multi-select;
- identifier;
- URL;
- file/artifact reference;
- person/practitioner reference;
- patient/subject reference;
- FHIR resource reference;
- linked record;
- formula/computed;
- unit-bearing measurement;
- code/terminology value;
- JSON;
- provenance/source reference.

Clinical value types need explicit units/codes rather than string-only storage.

## 13. Formula and transformation model

Do not build a second opaque spreadsheet execution engine that can silently change data.

Transformations should compile into explicit, versioned operations with:

- source snapshot;
- formula/query/code;
- parameters;
- engine/version;
- output snapshot;
- warnings;
- run receipt.

Ad-hoc visual formulas may exist for view-only display, but publication into a durable dataset should become a reproducible transformation.

## 14. Import surfaces

Candidate local import:

- CSV;
- TSV;
- XLSX;
- JSON;
- JSONL;
- Parquet;
- Arrow;
- FHIR bundles/resources;
- MedScale artifacts;
- local SQLite as external source;
- exported query results.

Governed connectors later:

- PostgreSQL;
- MySQL/MariaDB;
- SQL Server;
- approved warehouses/object stores;
- Hugging Face datasets;
- Kaggle;
- institutional data adapters.

Every import produces a source/snapshot receipt.

## 15. Provenance by cell/column/row where justified

Clinical/research data transformations need finer provenance than file-level lineage.

At minimum record:

- source dataset/snapshot;
- source column;
- transformation;
- output column;
- row identity strategy;
- filtering/cohort conditions.

When feasible, retain row-level origin IDs.

For expensive per-cell lineage, use an on-demand or compressed lineage strategy rather than making every table operation unusably heavy.

## 16. Cohort Builder

A researcher should be able to create cohorts using transparent criteria:

```text
Age >= 18
AND condition in (...)
AND encounter date between ...
AND lab X present
AND NOT unresolved identity
```

Each cohort must store:

- exact dataset snapshot;
- criteria;
- excluded/unknown handling;
- counts by filter stage;
- provenance;
- creation time;
- owner/review state.

Unknown values must not be silently treated as false.

## 17. De-identification views

Privacy Gate can create explicit derived views:

```text
Raw authorized dataset
 -> de-identification policy
 -> DeidReceipt
 -> pseudonymized/de-identified snapshot
 -> analytics/research workspace
```

The original raw dataset remains under stricter authority.

Never claim HIPAA Safe Harbor or another de-identification standard merely because a detector ran. Qualification and policy-specific evidence are required.

## 18. Grist and Baserow donor posture

### Grist Community

Preferred for study of:

- relational spreadsheet interaction;
- formulas;
- linked/reference data;
- local/self-managed patterns;
- table/form/card-style product thinking.

Current Grist Community documentation identifies its community source as Apache-2.0.

Adoption posture:
`REFERENCE / ADAPT / bounded COPY_SELECTIVE after exact qualification`.

### Baserow OSE

Preferred for study of:

- no-code relational database UX;
- field types;
- views;
- API-first design;
- generated table APIs;
- plugins/extensions;
- self-hosting.

Current Baserow OSE licensing places the open-source edition outside premium/enterprise directories under MIT.

Adoption posture:
`REFERENCE / ADAPT / bounded COPY_SELECTIVE after exact qualification`.

## 19. NocoDB and Teable posture

### NocoDB

Current main/develop licensing is Sustainable Use License with restrictions on commercial/product use.

Adoption posture:
`REFERENCE_ONLY` unless legal/licensing status changes or separate permission is documented.

### Teable

Core applications are AGPL-3.0 and include additional brand terms; some utility packages are MIT.

Adoption posture:
`REFERENCE_ONLY / exact MIT-package review`; no wholesale application absorption into Apache-2.0 MedScale without legal compatibility resolution.

## 20. Analytics handoff

Any Data Workbench view may become input to Analytics Gate only through an explicit snapshot binding.

```text
Saved View
 -> DatasetSnapshot
 -> AnalysisDefinition
 -> ComputeJob
 -> AnalysisRun
 -> Figures/Tables
 -> Evidence/Run Receipt
```

This prevents charts from drifting as source data changes.

## 21. AI in the Data Workbench

Allowed AI assistance may include:

- explain schema;
- propose column mapping;
- suggest validation rules;
- generate a transformation draft;
- explain a formula/query;
- propose cohort filters;
- summarize distributions;
- flag likely data-quality issues.

The model proposes. Deterministic engines execute after user approval.

AI must not silently mutate a clinical dataset.

## 22. Local-first UX

Users should be able to work against local admitted data with network disabled:

- inspect;
- filter;
- transform;
- join;
- calculate;
- build cohorts;
- visualize;
- export local artifacts;
- run admitted local analytics.

Remote connector refresh becomes unavailable, but cached/versioned snapshots remain usable.

## 23. Candidate Research OS ownership

- 075 Data Source Fabric — sources, snapshots, imports, table foundation;
- 076 Collaboration — shared blocks/review/activity;
- 079 Privacy Gate — data-class policies and de-identification views;
- 082 Analytics Gate — reproducible analytics;
- 083 Knowledge + Research Canvas — AFFiNE-inspired workspace + graph/table/canvas lenses;
- 085 Compute — heavier local transformations;
- 086 R Workspace — R analyses over bound snapshots;
- 090 Institutional Adapters — enterprise data sources;
- 091 Federation — only after local/site ownership and privacy are proven;
- 092 Whole-Platform Qualification — integrated data/graph/workspace proof.

No candidate is promoted by this plan.
