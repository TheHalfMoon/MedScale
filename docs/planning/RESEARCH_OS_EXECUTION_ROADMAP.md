# MedScale Research OS Execution Roadmap

**Status:** Candidate roadmap only. This document does not promote or authorize a specification.

## Sequencing principle

The Research OS expansion must be dependency-ordered. Do not implement collaboration, audio, analytics, or federation as isolated feature islands. The Project/Artifact/Capability substrate must exist first.

The candidate numbering below assumes the current active canonical sequence retains ownership of all work through Spec 073. Before any unit is promoted, reverify `specs/CURRENT.md` and canonical governance. If the active frontier changes, renumber/rebase this roadmap rather than overriding it.

## Program map

```text
074 Project + Artifact Graph Foundation
  -> 075 Collaboration Substrate
  -> 076 MedAgent Workbench
       -> 077 Model Fleet + Compare
       -> 078 Privacy Gate
            -> 079 AudioFlow Foundation
            -> 080 Analytics Gate
                 -> 081 Knowledge + Research Canvas
                      -> 082 MedScale Hub
                           -> 083 AudioFlow Advanced
                           -> 084 MedScale Compute
                                -> 085 Research Packs
                                     -> 086 Institutional Adapters
                                          -> 087 Federation
                                               -> 088 Whole-Platform Qualification
```

Parallelism is allowed only where contracts and evidence prove independence.

---

## 074 — Project + Artifact Graph Foundation

### Goal
Create the durable product ontology on which all new work depends.

### Required outcomes
- `Project`, `Experiment`, `Artifact`, `Run`, `EvidenceRef`, `Dataset`, `Document`, `Finding` contracts;
- stable artifact IDs, revisions, digests, provenance, creator, project ownership, classification;
- explicit relationships forming the Project Graph;
- local project workspace UX without requiring Hub/network;
- migration strategy for current MedScale artifacts/routes;
- activity references cannot mutate canonical artifacts indirectly;
- CLI parity for project/artifact inspection.

### Acceptance focus
Determinism, migration safety, provenance, offline lifecycle, large-project scale, no duplicate authority model.

---

## 075 — Collaboration Substrate

### Goal
Introduce rooms, threads, tasks, notes, activity, workflow/approval events, and agent identities while keeping canonical data separate from collaboration state.

### Required outcomes
- MedScale-owned event envelope;
- `Room`, thread, task, note/canvas, approval/refusal contracts;
- member and agent identities;
- ephemeral presence/typing separated from durable history;
- immutable artifact references in events;
- local single-user mode remains fully functional;
- selective Buzz source-adoption record and exact copied/adapted paths if used;
- searchable activity history;
- tamper-evident audit checkpoints.

### Non-goal
No mandatory server or real-time multi-device sync yet.

---

## 076 — MedAgent Workbench

### Goal
Make MedAgent the primary governed intelligence workspace inside a Project.

### Required outcomes
- split-pane agent IDE UX;
- project/context selector;
- one admitted local model lane;
- tools exposed only through capability manifests;
- structured RunManifest;
- evidence/source panel;
- proposal-only output semantics;
- cancel/interrupt/steer lifecycle;
- run history as project artifacts;
- browser and external providers still denied unless separately gated.

---

## 077 — Model Fleet + Compare

### Goal
Run multiple model/agent lanes under one task and compare observable results without manufacturing a winner.

### Required outcomes
- `AgentLane` and `FleetRun` contracts;
- multiple admitted local models;
- optional delegate adapters for approved external coding/research agents on non-sensitive data;
- side-by-side result panes;
- agreement/disagreement and contradiction extraction;
- evidence/citation comparison;
- structured-output validity;
- latency/resource/runtime facts;
- abstention/unknown semantics;
- no consensus-equals-truth behavior.

---

## 078 — Privacy Gate

### Goal
Make sensitive-data classification and transformation a cross-platform boundary rather than a one-off PII feature.

### Required outcomes
- data-class model (`LOCAL_PHI`, `TEAM_PROTECTED`, `EXTERNAL_DEIDENTIFIED`, `PUBLIC` or canonically refined equivalent);
- deterministic + model-assisted PII/PHI recognition;
- redact/tokenize/generalize/pseudonymize transformations;
- residual scan and explicit uncertainty;
- `DeidReceipt`;
- reversible pseudonymization only under scoped local/institutional key authority;
- enforcement on model, browser, export, Hub, adapter, compute, and connector boundaries;
- no UI claim that automated de-identification proves absence of sensitive data.

---

## 079 — AudioFlow Foundation

### Goal
Add local-first audio capture, transcription, diarization, dictation, and voice control as evidence-bearing Project capabilities.

### Required outcomes
- `AudioSession`, `AudioSource`, `TranscriptRevision`, `AudioEvidenceRef`;
- native capture/control contract;
- imported audio/video path;
- visible capture state;
- capture health and conditioning boundary;
- VAD/segmentation ownership;
- `VoiceRuntimeRouter`;
- at least one streaming local STT route and one higher-quality offline route;
- diarization/alignment path;
- medical terminology/numeric safety evaluation corpus;
- Arabic + Arabic-English code-switch evaluation dimension;
- non-destructive transcript revision ledger;
- command/context/dictation voice semantics for MedAgent;
- interruption/cancellation;
- VoiceStudio/Himsat/Wispral donor qualification records;
- no wake word as default.

---

## 080 — Analytics Gate

### Goal
Provide reproducible native analysis without making a BI server a Desktop dependency.

### Required outcomes
- governed analytical views over project data;
- Arrow/Parquet interchange where appropriate;
- DataFusion qualification or justified alternative;
- SQL editor and natural-language query proposal path;
- cohort/query builder;
- `QueryReceipt` with exact SQL/inputs/revisions/runtime;
- derived Dataset/Figure artifacts;
- statistical validation hooks and explicit not-run states;
- native table/chart surfaces;
- read-only default for AI-generated queries;
- privacy enforcement before materialization/export.

---

## 081 — Knowledge + Research Canvas

### Goal
Join documents, data, audio, evidence, retrieval, and visual reasoning without generic vector-only RAG.

### Required outcomes
- lexical + structured + vector + graph retrieval plan;
- permission-filtered retrieval;
- exact source spans/pages/timestamps/revisions;
- `RetrievalReceipt`;
- Project/Evidence Graph projection;
- Research Canvas containing live references to artifacts, not pasted copies;
- literature/paper workflow;
- MedAgent grounding over Project Graph;
- contradiction and missing-evidence inspection.

---

## 082 — MedScale Hub

### Goal
Enable user-controlled lab/team collaboration without a mandatory MedScale cloud.

### Required outcomes
- self-hostable Hub;
- team/project membership and invitations;
- room/thread/task/note synchronization;
- artifact metadata and transfer coordination;
- policy-aware search;
- workflow coordination;
- audit checkpoints;
- offline-first reconnect/conflict behavior;
- encrypted transport and explicit server trust posture;
- single-laptop mode remains valid;
- backup/recovery and upgrade/rollback strategy.

Candidate deployment targets: workstation, LAN server, on-prem server, private VPC.

---

## 083 — AudioFlow Advanced

### Goal
Turn AudioFlow into the full audio intelligence workspace after capture/transcript foundations are proven.

### Required outcomes
- project audio huddles;
- humans + scoped agents in audio sessions;
- live transcript/evidence/task extraction;
- TTS/listen-back;
- optional duplex agent voice;
- voice design/cloning under explicit permission and provenance;
- synthetic-audio watermark/provenance strategy;
- media time/frame annotations;
- batch audio workflow;
- optional remote audio workers;
- persistent speaker identity remains opt-in/encrypted/deletable.

---

## 084 — MedScale Compute

### Goal
Scale jobs from local hardware to lab/institutional compute without widening data authority.

### Required outcomes
- `ComputeJobManifest`;
- local CPU/GPU worker;
- self-hosted remote worker;
- bounded input artifact access;
- network/filesystem/secret policy;
- quotas, cancellation, expiry, crash containment;
- signed/hashed outputs and logs;
- reproducible environment/runtime identity;
- OpenSandbox or alternative isolation qualification;
- no worker receives ambient vault access.

---

## 085 — Research Packs

### Goal
Expand from health-data workflows to broader lab/research workflows without bloating Core.

### Initial candidates
- Clinical Research;
- AI Research;
- Imaging;
- Omics;
- Wet Lab;
- Systematic Review.

Each Pack must define schemas, workflows, validation, tool/model needs, UI extensions, evidence semantics, and export/import boundaries while remaining subordinate to Core authority.

---

## 086 — Institutional Adapters

### Goal
Connect MedScale to real organizational infrastructure through explicit adapters.

Candidate adapters:
- institutional identity/SSO;
- object storage;
- LIMS/ELN;
- EHR/FHIR systems;
- HPC/Slurm/Kubernetes;
- institutional model registry;
- Superset/BI;
- OpenRAG/OpenSearch or equivalent large retrieval service;
- notification/communication systems.

No adapter becomes required for personal/lab local-first operation.

---

## 087 — Federation

### Goal
Enable controlled collaboration across institutions without requiring a central MedScale data cloud.

Research areas:
- signed project/artifact bundles;
- cross-institution identity/trust;
- federated queries/analytics;
- data stays at site where required;
- policy negotiation;
- provenance across institutions;
- consent/data-use constraints;
- revocation and expiry;
- result aggregation without hidden source loss.

Federation must remain research/qualification work until the trust model is proven.

---

## 088 — Whole-Platform Qualification

### Goal
Qualify the integrated Research OS rather than declaring completion from feature presence.

Required evidence families:
- privacy/PHI boundary testing;
- model/agent authority testing;
- browser/egress abuse testing;
- Hub tenancy and authorization;
- audio capture/privacy and long-session stability;
- ASR/diarization quality on declared populations/tasks;
- analytics correctness and reproducibility;
- collaboration concurrency/conflict recovery;
- compute isolation;
- backup/restore/migration;
- accessibility;
- performance and resource envelopes;
- provenance/attribution closure;
- failure/recovery drills;
- platform-specific packaging/signing where applicable.

## Promotion rule

This roadmap is not self-authorizing. A candidate unit becomes executable only when canonical governance promotes a bounded specification with dependencies, acceptance criteria, recovery rules, and evidence requirements. Live repository truth always overrides this document.
