# MedScale Research OS Architecture

**Status:** Planning candidate — not implementation authority

## 1. Architectural goal

MedScale must scale from one offline researcher to a lab, research center, institution, and controlled federation without changing its fundamental trust model.

The architecture therefore separates:

1. **authority** — MedScale Core;
2. **interaction** — Desktop and CLI;
3. **intelligence** — models, agents, retrieval, analytics, speech;
4. **collaboration** — projects, rooms, tasks, notes, activity;
5. **compute** — local and remote governed workers;
6. **sync** — optional user-controlled MedScale Hub;
7. **domain extension** — Research Packs.

## 2. Invariants

1. Core authority remains local-first and explicit.
2. No model confidence grants clinical or research authority.
3. No agent receives ambient access to the complete vault.
4. Every cross-boundary data movement is classified and policy-checked.
5. Derived outputs preserve provenance to exact inputs and runtime identity.
6. Unknown/unobserved facts remain unknown; no fabricated parity or telemetry.
7. Network access is a capability, not an assumption.
8. Self-hosted collaboration may exist without a MedScale-hosted cloud.
9. Heavy runtimes may be isolated workers; Desktop must not absorb every donor stack.
10. Failure must be visible: no silent local-to-cloud, GPU-to-CPU, or authority escalation.

## 3. Planes

### Authority Plane

MedScale Core owns:

- vault lifecycle;
- identity and session authority;
- artifact identity/revision/digest;
- FHIR and canonical health-data semantics;
- Project and Experiment ownership;
- capability and policy decisions;
- privacy classifications;
- Pack admission;
- audit and evidence contracts;
- approval/irreversible-action boundaries.

### Intelligence Plane

Contains untrusted or bounded producers:

- MedAgent;
- local model runtimes;
- model fleet/compare;
- browser/search tools;
- retrieval;
- document intelligence;
- audio/speech runtimes;
- external adapters when explicitly enabled.

Outputs enter the authority plane only through typed proposal/artifact contracts.

### Analytics Plane

Native default path:

```text
Vault / Project Artifacts
        -> Governed Views
        -> Arrow / Parquet
        -> DataFusion
        -> SQL / cohort / statistics
        -> table / figure / derived dataset
        -> QueryReceipt + provenance
```

Superset or similar BI systems are optional self-hosted adapters over approved materialized views; they are not Desktop dependencies.

### Collaboration Plane

Uses a MedScale-owned event model inspired by Buzz without making collaboration events the regulated record itself.

Candidate event classes:

- project/room creation;
- message/thread/comment;
- task state;
- note/canvas revision;
- agent participation;
- workflow trace;
- approval/refusal;
- presence/typing (ephemeral);
- media annotation;
- artifact reference;
- run/result notification;
- audit checkpoint.

Events reference canonical artifacts through immutable identifiers and revisions.

### Audio Intelligence Plane — AudioFlow

AudioFlow is not a recorder feature. It is the multimodal audio intelligence subsystem.

```text
Capture Source
  -> Capture Health
  -> Conditioning (AEC/NS/AGC/resample where justified)
  -> VAD / segmentation
  -> Voice Runtime Router
       -> live general
       -> multilingual / Arabic / code-switch
       -> medical-specialized
       -> long-form multi-speaker
       -> low-resource/mobile
       -> offline quality/import
  -> transcription / diarization / alignment
  -> non-destructive Transcript Revision Ledger
  -> AudioEvidence time mapping
  -> MedAgent / RAG / notes / tasks / analytics
```

TTS, voice design/cloning, dubbing, and duplex agent voice are later leaves built on the same model/runtime governance.

### Compute Plane

MedScale Compute executes bounded jobs without ambient vault access.

A `ComputeJobManifest` should bind:

- actor and project;
- exact input artifacts/revisions;
- data classification;
- allowed model/runtime;
- CPU/RAM/GPU/time budgets;
- filesystem scope;
- network/egress policy;
- secrets/capabilities;
- expected output contract;
- expiry/cancellation;
- audit/provenance destination.

Execution targets may include local CPU/GPU, lab workstations, self-hosted GPU nodes, institutional clusters, or future HPC adapters.

### Sync Plane — MedScale Hub

The Hub exists only when sharing/sync is enabled.

Responsibilities:

- project membership;
- collaboration events;
- room/thread state;
- tasks/notes/activity;
- artifact metadata and transfer coordination;
- search indexes permitted by policy;
- workflows;
- audit checkpoints;
- worker coordination;
- conflict handling;
- optional object-storage integration.

The Hub does not automatically gain plaintext access to every local vault artifact.

## 4. Capability model

Humans, services, and agents receive explicit capabilities.

Examples:

```text
read_project_documents
read_local_phi
read_deidentified_dataset
run_local_model
run_compute_job
search_public_web
create_draft_note
create_task
post_room_message
request_export
approve_export
modify_canonical_record
```

A MedAgent can be a first-class participant while still being denied authority-changing capabilities.

## 5. MedAgent model

Each lane binds:

```text
agent identity
role/persona
model Pack
runtime/device
project context selector
tool capability set
data-class policy
network policy
budget
output schema
evidence policy
```

A `FleetRun` dispatches the same or transformed task to multiple lanes. Comparison must report observed properties rather than a synthetic winner score:

- agreement/disagreement;
- evidence coverage;
- citation overlap;
- unsupported claims;
- abstentions;
- structured-output validity;
- tool use;
- runtime/latency/resource facts;
- privacy exposure;
- unknown/unobservable dimensions.

Model agreement is never equivalent to evidence agreement or clinical authority.

## 6. Governed browsing

Browser/search access follows:

```text
MedAgent request
  -> capability check
  -> privacy/data-class gate
  -> sandbox / egress policy
  -> deterministic search/HTTP/Playwright first
  -> agentic browser fallback only when justified
  -> captured source/evidence
  -> BrowseReceipt
```

Sensitive project content cannot be sent into a public browser/tool route unless the policy explicitly permits the transformed/deidentified material.

## 7. Privacy Gate

The Privacy Gate applies to model, browser, export, collaboration, connector, analytics-adapter, and compute boundaries.

Candidate modes:

- `LOCAL_PHI` — authorized local processing only;
- `TEAM_PROTECTED` — may enter an authorized self-hosted project boundary;
- `EXTERNAL_DEIDENTIFIED` — must pass the approved de-identification pipeline before external processing;
- `PUBLIC` — no sensitive-data restriction, still provenance-governed.

A de-identification workflow should produce a `DeidReceipt`; automated detection must never be presented as proof that all PHI/PII has been removed.

## 8. Audio privacy

Audio requires additional rules:

- capture is visible and user-authorized;
- raw audio retention is explicit;
- speaker labels (`SPEAKER_01`) are distinct from persistent speaker identity;
- persistent voice embeddings/identity are opt-in, encrypted, and deletable;
- model and transcript revisions never overwrite source evidence silently;
- medical terms, doses, numbers, negation, and names receive specialized evaluation;
- no hidden wake-word/always-listening path in the default product.

## 9. Event and artifact separation

A collaboration message may reference `Dataset@v4`; it does not own that dataset. A task completion event may reference `Run#184`; it does not establish the run's evidence. A MedAgent message may propose a Finding; Core must create or admit the typed Finding artifact separately.

This keeps collaboration flexible while preserving regulated and scientific provenance.

## 10. Storage strategy

Candidate logical stores, all behind MedScale-owned interfaces:

- encrypted local vault for sensitive/canonical data;
- relational metadata store for projects/artifacts/permissions;
- content-addressed object store for large media/datasets where needed;
- Arrow/Parquet derived analytical artifacts;
- searchable text/evidence index;
- optional vector index;
- graph projection derived from canonical artifact relationships;
- append-only/tamper-evident audit/event checkpoints.

No single donor storage model should dictate all of these concerns.

## 11. Scale principle

The same artifact/capability/provenance contracts must work at all deployment levels. Scale is achieved by moving execution and synchronization behind contracts, not by creating a separate enterprise product with different semantics.
