# MedScale Research OS Master Implementation Contract

**Status:** Planning contract. Not executable authority until a bounded canonical specification adopts the relevant clauses.

## 1. Purpose

This document removes implementation ambiguity from the Research OS proposal. Future specifications MUST inherit these contracts unless they explicitly supersede a clause with stronger evidence and an ADR/canonical decision.

The implementation objective is one product and one authority model that can scale from a single offline researcher to a laboratory, research center, institution, and later controlled federation.

North star:

> One workspace. Many engines. One authority. User-owned data. Local by default. Evidence everywhere.

## 2. Existing repository contracts are primary

Research OS MUST extend the current MedScale architecture, not create a parallel platform.

Existing ownership remains:

- `medscale-contracts`: pure serializable domain/wire contracts; no I/O and no policy side effects.
- `medscale-core`: authority orchestration, state machines, capability decisions, validation and effect intent creation.
- `medscale-storage`: durable persistence, migrations and transaction implementation.
- `medscale-fhir`: FHIR-specific semantics and validation.
- `medscale-keys`: local key and credential-store boundaries.
- `medscale-network`: network broker and all admitted network transport.
- `medscale-pack`: Pack identity, provenance, admission and model/runtime packaging.
- `medscale-desktop`: native Slint presentation/adaptation only; never independent authority.
- `medscale-cli`: CLI presentation/adaptation only; never independent authority.

Existing `OpaqueId`, `ObjectHeader`, `DigestSha256`, evaluation/projection and audit concepts MUST be reused. A future specification may refine them but MUST NOT invent a second identifier/provenance/audit system merely for Projects, agents, analytics or audio.

## 3. Authority rule

Only MedScale Core may authorize creation or mutation of canonical MedScale state.

No model, agent, browser, collaboration event, analytics engine, Hub, worker, Pack, BI adapter, RAG service, speech engine or donor-derived subsystem may directly mutate canonical state.

Every authority-changing path follows:

```text
Desktop / CLI / Hub request / agent proposal
    -> typed Core command
    -> session + actor resolution
    -> capability evaluation
    -> data-class/privacy evaluation
    -> precondition/revision validation
    -> deterministic domain validation
    -> durable transaction
    -> canonical object/revision
    -> audit/receipt/event projection
```

External effects use a separate effect lifecycle and MUST NOT be confused with the canonical transaction.

## 4. Command contract

Every mutating Research OS operation MUST have a typed command envelope containing at minimum:

```text
command_id: OpaqueId
actor_id: OpaqueId
session_id: OpaqueId
project_id: Option<OpaqueId>
operation: typed enum or versioned command name
idempotency_key: Option<String>
expected_revision: Option<u64>
submitted_at: MedicalTime
payload: typed contract
```

Rules:

1. Commands are requests, not facts.
2. `expected_revision` is required for mutation of an existing canonical object unless the object contract proves a stronger conflict mechanism.
3. Replaying a command with the same idempotency key MUST NOT duplicate an effect or canonical artifact.
4. Core returns typed outcomes, never relies on UI string parsing.
5. CLI and Desktop use the same Core command path.

## 5. Canonical object and revision contract

Every Research OS canonical object MUST expose or reference:

- opaque stable identity;
- object/schema kind;
- schema/contract version;
- monotonically controlled revision or equivalent immutable revision identity;
- content digest where content-addressing/provenance matters;
- creator/actor identity;
- owning Project when project-scoped;
- creation/update medical time;
- data classification where sensitive content may exist;
- provenance/source references;
- lifecycle state where applicable.

A mutable object update MUST produce a new revision identity/state. Historical provenance MUST remain inspectable where scientific or safety interpretation depends on the previous revision.

## 6. Artifact model

`Artifact` is an organizing abstraction, not permission to flatten all MedScale domain types into JSON blobs.

Specialized domain objects remain specialized when they carry validation or authority semantics. Project Graph edges may point to them through `OpaqueId` references.

Candidate generic artifact families:

- dataset;
- document;
- audio/media source;
- transcript revision;
- analysis result;
- figure/table;
- note/canvas document;
- model/agent run;
- retrieval result;
- experiment record;
- exported bundle.

FHIR resources, consent/authorization records, Pack manifests, keys, policy decisions and other authority-bearing objects MUST NOT be downgraded into generic artifact bodies.

## 7. Project Graph contract

The Project Graph is a typed relationship layer over canonical identities.

Each edge MUST contain:

```text
edge_id
project_id
subject_ref
predicate
object_ref
created_by
created_at
source/provenance
edge_revision or immutable identity
```

Predicates MUST come from a versioned registry. Free-form strings may be displayed as labels but MUST NOT define authority behavior.

Canonical edge examples:

- `contains`;
- `derived_from`;
- `analyzed_by`;
- `produced`;
- `supports`;
- `contradicts`;
- `discussed_in`;
- `references`;
- `supersedes`.

Derived search/vector/knowledge-graph projections are rebuildable and non-authoritative.

## 8. Transaction boundaries

### 8.1 Local canonical mutation

A canonical mutation is one storage transaction containing all state required to make that mutation unambiguous. Audit linkage required for the mutation MUST be durable in the same transaction or through a proven transactional outbox.

### 8.2 External effects

Browser writes, connector writes, Hub-delivered effects, notifications and institutional actions follow:

```text
Intent -> Authorized -> Dispatching -> Confirmed | Failed | Unknown
```

If the remote system times out after dispatch, state MUST become `Unknown`, not `Failed` and not `Confirmed`.

Retries require idempotency evidence or an explicit user decision.

### 8.3 Long-running compute/model/audio work

Long-running work follows:

```text
Created -> Admitted -> Queued -> Running ->
    Succeeded | Failed | Cancelled | TimedOut | Lost | Partial
```

`Succeeded` means the declared output contract was validated. Process exit code zero alone is insufficient.

## 9. Result/error taxonomy

Research OS APIs MUST distinguish at least:

- `Invalid` — request/contract failed validation;
- `Denied` — policy/capability refused;
- `Conflict` — revision/concurrent-state conflict;
- `NeedsApproval` — valid but requires a separate authority decision;
- `Unavailable` — capability/runtime/source not currently available;
- `Failed` — attempted and known to have failed;
- `Cancelled` — explicit cancellation completed;
- `TimedOut` — deadline exceeded and final remote effect state is known not confirmed;
- `Unknown` — final state cannot be established;
- `Partial` — bounded subset succeeded and exact subset is recorded;
- `Stale` — result/projection was built from superseded inputs;
- `Succeeded` — declared contract satisfied.

UI copy may simplify these states but may not collapse `Unknown`, `Failed` and `Succeeded`.

## 10. Capability evaluation order

Every sensitive operation MUST evaluate in this order:

1. valid authenticated/local session;
2. actor identity active/not revoked;
3. Project membership/relationship;
4. requested capability grant;
5. target object scope;
6. data classification/privacy rule;
7. network/worker/tool policy;
8. approval requirement;
9. revision/precondition;
10. execution.

Agents and services use the same evaluator as humans with narrower grants by default.

Revocation is checked at action time. In-flight jobs MUST also receive cancellation/revocation signals where technically possible; any result arriving after revocation is quarantined until Core reevaluates admission.

## 11. Data classification contract

Initial product semantics are:

- `LOCAL_PHI`: only admitted local/private execution; no Hub/external egress by default.
- `TEAM_PROTECTED`: may enter a specifically authorized user-controlled team boundary.
- `EXTERNAL_DEIDENTIFIED`: external use requires an approved de-identification transformation and receipt.
- `PUBLIC`: may use admitted network/external capabilities but remains provenance governed.

Institutions may define stricter policy profiles; they MUST NOT redefine these labels to weaker semantics.

Classification MUST propagate to derived artifacts using the most restrictive source classification unless a typed transformation (for example approved de-identification) establishes a new class with a receipt.

## 12. Privacy transformation contract

A privacy transform is immutable and produces:

```text
source refs/revisions
policy profile
recognizers + exact versions
model Pack identities where used
transformation operations
pseudonym mapping authority reference, if any
residual scan result
known limitations
output digest/ref
DeidReceipt
```

The transformed artifact is distinct from the source. Source data is never overwritten by a de-identification pass.

Re-identification requires a separate capability and audit event.

## 13. Network rule

No future Research OS crate may open arbitrary outbound network connections directly.

All product network egress MUST flow through `medscale-network` or a future canonical successor broker that preserves:

- destination policy;
- data-class policy;
- credential isolation;
- request limits/timeouts;
- audit/receipt linkage;
- explicit offline denial.

Sandboxed workers may have their own network namespace but their egress policy MUST be derived from a Core-approved manifest.

## 14. Model and agent context rule

A model/agent never receives "the Project" as ambient context.

Core constructs a `ContextManifest` listing exact permitted inputs:

```text
context_manifest_id
project_id
artifact refs + revisions
retrieval refs/receipts
system/instruction identity
model Pack identity
allowed tools
allowed data classes
network policy
context budget
cache policy
```

Context caches MUST be keyed by immutable input/model/instruction identity and MUST NOT silently retain raw sensitive prompts beyond declared retention.

## 15. Tool contract

Every MedAgent tool has a manifest containing:

- stable tool ID/version;
- input/output schema;
- required capability;
- accepted data classes;
- network requirement;
- side-effect classification (`read_only`, `proposal`, `external_effect`, `canonical_mutation_request`);
- timeout/resource limit;
- audit/receipt policy.

Tool output is untrusted until parsed and validated.

A model cannot elevate itself by generating a different tool name, URL, shell command or hidden prompt.

## 16. Browser contract

Order of use:

1. deterministic/local source lookup where possible;
2. HTTP/search through admitted broker;
3. Playwright-style deterministic browser automation;
4. agentic browser only when deterministic navigation is insufficient.

Public browsing never receives `LOCAL_PHI` or `TEAM_PROTECTED` source text. External-deidentified data requires a valid transformation receipt.

Every retrieved evidence item records URL/source identity, retrieval time, content hash where obtainable, selected spans and tool route.

Browser content is hostile input and may not directly alter capabilities, instructions or authority.

## 17. Collaboration event contract

Initial collaboration is NOT an event-sourced replacement for canonical state.

Durable `CollabEvent` MUST include:

```text
event_id
schema_version
project_id
room_id optional
actor_id
event_kind
created_at
body or body_ref
artifact refs with exact revisions
causal/base revision where mutation-like
integrity/audit reference
```

Initial conflict rules:

- messages/comments: append-only; edits create explicit replacement/edit events;
- tasks: optimistic concurrency on task revision; conflicting offline edits require deterministic merge for independent fields or explicit user resolution;
- notes/canvases: first implementation uses revisioned documents with conflict copies/user resolution; no CRDT is admitted until a spec proves the operational need and security model;
- canonical artifact metadata: Core revision conflict rules, never last-write-wins;
- presence/typing: ephemeral, not audit authority.

## 18. Hub contract

Hub is optional and untrusted relative to local Core authority.

Hub MAY own collaboration delivery/synchronization state and permitted server-side indexes. Hub MUST NOT be assumed to possess every local vault key or plaintext artifact.

Hub protocol requirements:

- version negotiation;
- tenant/community/project scoping before data lookup;
- authenticated actor/device identity;
- monotonic sync cursor per stream;
- idempotent event/artifact metadata submission;
- resumable large-object transfer;
- revocation checks;
- explicit conflict response;
- client-verifiable hashes for transferred artifacts;
- backup/restore and schema upgrade/rollback procedure.

## 19. Analytics contract

AI never executes unrestricted SQL directly.

Flow:

```text
Natural-language request
 -> proposed AnalysisPlan/SQL
 -> schema + capability validation
 -> read-only parser/planner guard
 -> governed DataFusion view
 -> bounded execution
 -> result validation
 -> immutable QueryReceipt
 -> derived Dataset/Table/Figure artifact
```

Mutable database statements are denied in the default analytics path.

Every result binds exact source revisions, SQL/plan, engine/version, parameters, locale/timezone where relevant, and deterministic/uncertainty semantics.

Statistical operations MUST expose whether assumptions/checks were run. `not_run` is a valid explicit state.

## 20. Notebook/code execution rule

The trusted Desktop MUST NOT embed an unrestricted Python/R shell.

Notebook-like UX stores cells as typed artifacts. SQL/native analytical cells may execute in-process only through admitted engines. Python/R/arbitrary code executes only through MedScale Compute with an explicit job manifest and sandbox policy.

## 21. Retrieval/index contract

Indexes are projections, never source authority.

Every index generation records:

- source refs/revisions/digests;
- chunking/parser version;
- embedding model Pack/revision where applicable;
- index schema/version;
- build time;
- permission scope;
- tombstone/removal state.

Permission filtering MUST occur before result disclosure and cache reuse. A cache built for one actor/scope cannot be reused as evidence of authorization for another.

Stale index results MUST be labeled or excluded when source revisions change.

## 22. AudioFlow contract

Raw audio is source evidence. Transcripts are derived revisions.

Required lineage:

```text
AudioSource@digest
 -> Capture/ImportReceipt
 -> TranscriptRevision#1
 -> optional TranscriptRevision#2 quality pass
 -> diarization/alignment revision
 -> AudioEvidenceRef(start,end,source_digest,transcript_revision)
```

No quality pass may silently rewrite the previous transcript.

`COMMAND`, `CONTEXT`, and `DICTATION` are explicit input modes. Speech recognition never authorizes an action by confidence alone; command execution still passes normal capability/approval checks.

Persistent speaker identity is separate from diarization labels and requires opt-in encrypted identity data.

## 23. Voice Runtime Router contract

A route decision MUST be explainable from explicit metadata:

```text
requested_task
language set
streaming requirement
diarization/timestamp requirement
medical/domain profile
device/accelerator
RAM/resource budget
offline/network policy
engine/model Pack identity
runtime health
rights/admission state
```

No hidden local-to-cloud or GPU-to-CPU fallback. Fallback creates an explicit route decision and receipt.

## 24. Compute contract

A worker receives only a `ComputeJobManifest` and staged inputs.

Manifest includes:

```text
job_id
actor/project
input artifact refs/revisions/digests
classification
runtime/environment identity
command/entrypoint contract
CPU/RAM/GPU/time budget
filesystem mounts
network/egress policy
secret handles, never ambient secret store
expected output schema
log redaction policy
expiry/cancellation
```

Workers MUST NOT mount or enumerate the entire vault.

Output admission requires digest verification, schema validation and policy reevaluation before creation of canonical derived artifacts.

## 25. Research Pack contract

A Pack may extend schemas/workflows/tools/validators/UI descriptors but may not execute arbitrary trusted-process UI/native code by default.

Initial Pack extension points are declarative/versioned:

- artifact kinds and fields;
- validators;
- workflow templates;
- tool/model requirements;
- Project views/navigation descriptors;
- import/export adapters;
- evidence semantics.

Any executable extension runs behind existing worker/tool boundaries.

Pack install/upgrade/uninstall MUST define migration and orphan-data behavior.

## 26. Schema/versioning rules

1. Persisted/wire contracts carry explicit schema version where evolution is expected.
2. Existing `serde(deny_unknown_fields)` posture remains preferred for authority-bearing inputs.
3. Breaking semantic change requires migration, not reinterpretation of old bytes.
4. Migrations are forward tested and recovery/rollback behavior is documented before promotion.
5. A Hub/client protocol handshake refuses unsupported major versions instead of guessing compatibility.
6. Pack and worker protocols use exact version negotiation.

## 27. Audit and receipts

Receipts are evidence artifacts, not authority by themselves.

Every receipt MUST bind:

- exact operation/job/run identity;
- actor/service identity;
- relevant project;
- immutable input identities;
- tool/runtime/model identity;
- policy decision reference where relevant;
- start/end state and timestamps;
- output identity/digest;
- error/unknown/partial state without collapsing semantics.

Sensitive raw content SHOULD NOT be duplicated into audit logs when stable refs/digests suffice.

## 28. Cache rules

Caches are rebuildable and non-authoritative.

Cache keys MUST include every input that can change semantic output: content revision/digest, model/runtime version, parser/chunker/instruction version, locale/config and policy scope as relevant.

Revocation or source deletion invalidates permission-bearing cache entries.

## 29. Deletion and retention

Each promoted subsystem MUST specify:

- canonical deletion/tombstone semantics;
- derived index/cache cleanup;
- Hub replicas/sync behavior;
- worker staging cleanup;
- logs/receipts retention;
- backup implications;
- legal/institutional retention override boundary.

Deletion MUST NOT silently falsify historical provenance. Where history must remain, retain a non-sensitive tombstone/digest/provenance reference rather than hidden plaintext.

## 30. Observability

Operational telemetry defaults to local. Telemetry may include timings, resource use, state transitions and error classes, but MUST avoid PHI/audio/transcript/prompt bodies by default.

`Unknown`, `not measured`, and `not observed` are first-class states.

## 31. Security defaults

- default deny for network, external providers and side-effect tools;
- no secret material in model prompts;
- project content is hostile input;
- browser pages cannot instruct Core to grant capabilities;
- model-generated code executes only in bounded workers;
- worker/browser logs are scrubbed;
- all external source code adoption records exact revision/path and security review;
- no hidden background microphone capture;
- no external PHI route without explicit canonical authorization.

## 32. Performance budgets

Every spec defines measurable budgets for its hot paths. Budgets MUST include hardware/workload assumptions and at least p50/p95 where latency is user-visible.

Performance optimization may not bypass validation, provenance or policy checks.

## 33. Implementation sequence rule

An implementer MUST NOT work ahead of the active promoted unit.

For each future spec:

1. reverify current `main` and active governance;
2. read this contract and all predecessor evidence;
3. refine only the next dependency-ordered unit;
4. identify exact existing types/modules to extend before creating new crates;
5. implement smallest end-to-end vertical slice;
6. run focused tests, full affected workspace gates and security/failure tests;
7. create evidence artifacts for the exact reviewed revision;
8. close only when canonical acceptance criteria are proven.

## 34. Prohibited shortcuts

Implementers MUST NOT:

- create a second Core/authority service;
- make Hub mandatory for local use;
- make Superset/OpenRAG/VoiceStudio/Buzz mandatory runtime dependencies merely because they are donors;
- bypass `medscale-network` for convenience;
- put unrestricted Python/R/shell execution inside Desktop/Core;
- infer successful external effects from timeout/exit status alone;
- treat model consensus as correctness;
- overwrite source audio/transcript/data during a derived transformation;
- implement CRDT/federation/microservices before measured need and canonical promotion;
- copy a donor wholesale without an adoption record;
- declare a capability complete from UI presence or compile success alone.

## 35. Definition of implementation-ready

A future Research OS spec is implementation-ready only when it has all of:

- exact base SHA and dependency closure;
- owned scope and explicit non-goals;
- contract/type changes;
- storage/migration changes;
- Core commands/state machines;
- CLI/Desktop surfaces;
- permission/data-class rules;
- failure/recovery behavior;
- donor/dependency decisions;
- test matrix;
- performance/security evidence requirements;
- rollback/migration plan;
- exact completion criteria.

If any item is absent, implementation must stop at specification refinement rather than fill the gap by preference.
