# MedScale Research OS Product Map

**Status:** Planning candidate — not implementation authority

## Primary navigation candidate

The future product should avoid exposing every subsystem as a top-level route. Projects are the primary context; global routes support cross-project work.

```text
Home
Projects
MedAgent
Analytics
Audio
Models
Evidence
Library
Tasks
Messages
Integrations
Audit Trail
Settings
```

Within a Project:

```text
Overview
MedAgent
Data
Documents
Audio
Analytics
Evidence
Experiments
Tasks
Notes
Rooms
Team
Runs
Audit
```

Exact navigation remains subject to UX qualification and current MedScale design authority.

## Home

Home remains work-oriented rather than a KPI dashboard. Candidate contents:

- continue recent project/work;
- pending review/approval;
- agent or compute runs needing attention;
- tasks due;
- privacy/runtime warnings;
- recent evidence changes;
- explicit local/Hub/worker posture.

## Projects

Projects become the stable container for research context. A Project may contain multiple Experiments and all associated artifacts, members, rooms, workflows, and policies.

Candidate project templates:

- Clinical Study;
- Data Analysis;
- Model Evaluation;
- Systematic Review;
- Imaging Study;
- Omics Study;
- Wet Lab Study;
- Quality Improvement;
- General Research.

Templates are Packs/configuration, not separate products.

## MedAgent

Agent IDE with:

- prompt/command surface;
- context selector;
- model/agent lane selector;
- split-pane outputs;
- tool/browser traces;
- evidence pane;
- compare view;
- run timeline;
- capability/privacy posture;
- interrupt/steer/cancel;
- voice entry through AudioFlow.

## Analytics

Distinct analytical workspace with:

- Data catalog;
- Cohorts;
- SQL/Query;
- Explore;
- Notebook/run history;
- Figures;
- Models/experiments where relevant;
- Reports;
- Exports;
- governed RAG/evidence support.

## Audio

User-facing name for AudioFlow:

- Live;
- Sessions;
- Imports;
- Transcripts;
- Huddles;
- Audio Models;
- Voices (later, after explicit permission/provenance design).

## Evidence

Cross-project and project-local evidence explorer:

- source documents;
- exact spans/pages/timestamps;
- findings;
- claims/proposals;
- contradictions;
- provenance;
- validation/review state;
- model/run source relationships.

## Library

Local/user-controlled research library for:

- papers;
- documents;
- guidelines;
- datasets available for reuse;
- media;
- Packs;
- project templates.

Importing to a Project creates explicit references/versions rather than hidden copies where feasible.

## Rooms and Messages

Collaboration should attach conversation to work. Rooms may be bound to:

- Project;
- Experiment;
- Dataset;
- Analysis Run;
- MedAgent Run;
- Manuscript;
- Protocol;
- Audio Session.

Messages can reference artifacts/evidence by immutable ID/revision and support timestamp/frame/page anchors.

## Tasks

Tasks may be human- or agent-owned, but agent execution always follows capabilities. Candidate links:

```text
Task -> Artifact
Task -> Evidence
Task -> Run
Task -> Room
Task -> Approval
```

## Experiments

Candidate Experiment model:

```text
Hypothesis
Protocol
Inputs
Samples/Datasets
Environment
Equipment/Runtime
Procedure
Observations
Runs
Results
Deviations
Analysis
Evidence
Conclusion
```

This model must remain extensible enough for computational and wet-lab work through Packs.

## Research Canvas

A visual evidence/work composition surface. Canvas nodes remain live references to canonical artifacts:

```text
Paper -> quoted EvidenceSpan
Dataset -> Figure
Audio -> Clip/AudioEvidenceRef
Model Run -> Result
Note -> Hypothesis
Protocol -> Experiment
```

MedAgent may explain, critique, or identify missing evidence in a Canvas but does not silently mutate its authoritative artifacts.

## Team

Candidate membership layers:

```text
Organization (optional institutional scope)
  -> Team
      -> Project
          -> Artifact / Room / Experiment
```

Authorization should support relationship-aware delegation without flattening healthcare/research roles into generic `admin/editor/viewer` only.

## Model Center extension

Model Center should become a unified governed runtime registry:

```text
Language Models
Medical Models
Embedding Models
Vision Models
Audio Models
NER / Privacy Models
Diarization
TTS
VAD / Enhancement
```

Every executable model/runtime combination retains immutable provenance, rights, device/runtime requirements, benchmark state, and promotion/rollback state.

## Product promise shown to users

The UI should make the trust posture inspectable at all times:

- Local / Hub / external route;
- data classification;
- model/runtime identity;
- evidence/provenance;
- agent capability scope;
- pending approvals;
- offline/network state.

The promise is not that every operation is always offline. The promise is that local operation is the default, user-controlled sharing is explicit, and leaving a trust boundary is visible and governed.
