# MedScale Research OS Repository Implementation Map

**Status:** Planning map — not implementation authority.

This document maps the Research OS program onto the current MedScale repository so a future implementer does not invent a second architecture. It is based on the canonical repository shape at `main` SHA `c795796132ce633b23b693d55bccc0521d0d26aa` (Spec 073 closed). Before implementation, reverify live `main`; if paths/ownership changed, reconcile under canonical governance rather than following stale paths blindly.

## 1. Current workspace anchors

The current Rust workspace already provides the primary ownership boundaries required by Research OS:

```text
crates/medscale-contracts   serializable contracts, no authority side effects
crates/medscale-core        authority, validation, state machines, effect intent
crates/medscale-storage     SQLCipher/vault persistence, migrations, blob storage
crates/medscale-fhir        FHIR-specific semantics
crates/medscale-keys        credentials/key boundaries
crates/medscale-network     brokered network transport and allowlists
crates/medscale-pack        Pack identity, provenance, store and model runtime
crates/medscale-cli         CLI adapter
crates/medscale-desktop     native Slint product UI adapter
```

**Default rule:** extend these crates first. New crates are exceptions that require a promoted spec to prove cohesion, dependency direction and operational value.

## 2. Existing primitives that MUST be reused

Before adding Research OS types, inspect and reuse current contracts around:

- `OpaqueId`;
- `ObjectHeader`;
- `DigestSha256`;
- `MedicalTime`;
- `EffectState`;
- `EvaluationRecord`;
- `Projection`;
- `ActionAuditRecord`;
- current evidence/document/network/Pack envelopes and IDs.

`EvaluationRecord` remains evidence-only; `Projection` remains rebuildable/non-authoritative; `ActionAuditRecord` remains the basis for durable audit/external-action intent semantics. Do not invent `ResearchId`, `AgentAuditId`, `AudioDigest`, `BrowseAudit`, `AnalyticsAudit` or equivalent parallel foundations if an existing primitive can express the same semantics.

## 3. Dependency direction

Research OS implementation MUST preserve the existing dependency-direction gate.

Conceptual target:

```text
contracts <- storage/fhir/keys/network/pack <- core <- cli/desktop
```

Exact existing dependency direction remains governed by repository checks; this diagram is not permission to add a new dependency edge.

Rules:

1. `medscale-contracts` has no DB, network, UI or policy effects.
2. `medscale-storage` persists contracts but does not decide authority.
3. `medscale-network` transports Core-approved requests; it does not infer permission.
4. `medscale-pack` verifies/adopts executable/model artifacts; it does not grant project/user authority.
5. `medscale-core` is the only product authority orchestrator.
6. CLI/Desktop call Core paths and never reproduce policy logic.
7. Hub/browser/workers introduced later remain bounded clients/services of Core-owned contracts, not alternate authority planes.

## 4. Contract module placement defaults

Use domain modules inside `crates/medscale-contracts/src/` rather than one giant Research OS file.

Candidate default modules when their owning spec is promoted:

```text
projects/        074 Project, Experiment, ProjectGraphEdge, ArtifactDescriptor
collaboration/   075 Room, Message, Task, NoteDocument, Approval contracts
agents/          076 AgentIdentity, ContextManifest, ToolManifest, Run contracts
privacy/         078 DataClass, PrivacyTransform, EgressDecision, DeidReceipt
browse/          079 BrowseRequest/Route/Session/Evidence/Receipt contracts if not cleaner under network/evidence
audio/           080 AudioSource, AudioSession, TranscriptRevision, AudioEvidenceRef
analytics/       081 AnalysisPlan, QueryReceipt, CohortDefinition, Figure/Table contracts
knowledge/       082 IndexManifest, RetrievalPlan/Receipt, Canvas contracts
hub/             083 sync/protocol envelopes if they remain pure contracts
compute/         085 ComputeJobManifest, leases, worker/output contracts
research_packs/  086 ResearchPackManifest and declarative extension contracts
institutional/   087 adapter-neutral identity/mapping/effect envelopes only where generic
federation/      088 only after research authorizes a protocol
```

Do not create all modules up front. A module appears only with its promoted spec.

If a contract is clearly a specialization of an existing module (`evidence`, `documents`, `network`, `objects`, `actions`, `envelopes`), extend that module instead. The promoted spec must document the final choice.

## 5. Core module placement defaults

Current Core already contains `authority`, `effects`, `ipc`, `process`, `text`, `validate`, `workflow`, `doctor` and session surfaces. Research OS extends these patterns rather than creating an independent application service.

Candidate domain modules after promotion:

```text
core/src/projects/        074 lifecycle + graph authority
core/src/collaboration/   075 local room/task/note authority
core/src/agents/          076 run/context/tool orchestration
core/src/privacy/         078 classification/transformation/egress decisions
core/src/browse/          079 browse policy/route/session/evidence admission, not browser engine internals
core/src/audio/           080 route/capture/transcript authority, not raw DSP if separable
core/src/analytics/       081 governed analysis orchestration
core/src/knowledge/       082 indexing/retrieval orchestration
core/src/hub/             083 sync admission/reconciliation client-side authority
core/src/compute/         085 job admission/lease/output admission
core/src/research_packs/  086 Pack extension validation
```

The names are defaults, not blanket authorization. If existing modules already own behavior, extend them instead.

### Core command location

A promoted spec must choose one consistent typed command/query pattern compatible with current Core. Do not add ad-hoc methods directly to Desktop/CLI.

Every mutation or sensitive action path must be traceable to:

```text
request -> actor/session -> relationship/capability -> privacy/data class
        -> revision/precondition -> deterministic validation
        -> storage transaction/effect intent -> receipt/audit -> result
```

## 6. Storage map

Current storage already contains encrypted vault, SQLite metadata, blob, migration, backup, GC, privacy probes and writer-lock facilities.

Default use:

- metadata/relations/index metadata: evolve current SQLCipher/SQLite metadata paths;
- large datasets/media/audio/download/model-adjacent artifacts: existing blob/content-addressed storage patterns where semantics fit;
- schema migration: `medscale-storage` migration mechanism;
- backup/recovery: extend existing backup contract before adding a second backup system;
- concurrency: preserve current transaction/writer assumptions until a promoted spec explicitly changes them.

### Storage creation rule

Before adding a table, the active spec MUST name:

```text
owner contract
primary key / stable OpaqueId mapping
schema version
project scope
classification
revision strategy
unique constraints
indexes + expected queries
transaction owner
foreign/reference validation
archive/tombstone/delete behavior
backup behavior
migration forward path
rollback/recovery path
```

### Suggested schema families

Do not pre-create them; these are ownership guides:

```text
074: projects, experiments, project_artifact_refs, project_graph_edges
075: rooms, room_memberships, collab_events, tasks, note_revisions, approvals
076: agent_runs, agent_turns, tool_receipts, context_manifests or sealed refs
078: privacy_transform metadata, deid_receipts, pseudonym-map references only
079: browse_sessions/receipts/evidence metadata and quarantined download refs only when persistence is required
080: audio_source metadata, audio_sessions, transcript_revisions, diarization revisions
081: analysis/query receipts, governed view metadata, cohort definitions
082: index manifests/chunks/embedding metadata/canvas revisions
083: sync cursors/outbox/inbox/conflict metadata; Hub server schema separately qualified
085: compute jobs/leases/receipts/output candidates
086: installed research-pack metadata/migration state
```

Sensitive payloads SHOULD remain sealed/blob-backed where the existing vault pattern requires it; metadata tables must not become accidental plaintext prompt/transcript/audio/browser credential stores.

## 7. Migration rule

Each Research OS spec that persists state MUST provide a storage migration and reopen test.

Required flow:

```text
pre-spec fixture vault
 -> backup/recovery point
 -> migrate once
 -> validate schema + old data
 -> exercise old behavior
 -> exercise new behavior
 -> close/reopen
 -> verify exact objects/revisions/digests
 -> re-run migration safely/idempotently according to current framework
```

If rollback of schema bytes is unsafe, define restore-from-pre-migration-backup rather than pretending down-migration is safe.

No spec may destructively rewrite current canonical object IDs merely to fit Project Graph or later Research OS views.

## 8. CLI map

Current CLI is concentrated in `crates/medscale-cli/src/main.rs`. Do not grow this indefinitely without an explicit refactor task.

For 074 or the first Research OS CLI expansion, the promoted spec MUST evaluate splitting command groups into modules while preserving current command behavior. A refactor may be included only when needed for bounded maintainability and must remain behavior-preserving.

Candidate command families:

```text
medscale project ...
medscale experiment ...
medscale agent ...
medscale privacy ...
medscale browse ...
medscale audio ...
medscale analytics ...
medscale knowledge ...
medscale hub ...
medscale compute ...
medscale pack ...
```

Every command with state-changing/sensitive behavior calls Core; JSON output must use typed Core results and preserve `Denied`, `Conflict`, `Unknown`, `Partial`, `Stale`, etc.

## 9. Desktop map

Current Desktop is native Slint with Rust adapter modules including patient workspace, population insights, product intelligence, utility surfaces and workflow studio.

Research OS UX MUST remain native and must not import Buzz Tauri/React or VoiceStudio Electron as an app shell.

Default product surfaces:

```text
Projects         074
Project rooms    075
MedAgent         076
Fleet Compare    077
Privacy          078 cross-cutting + inspectable surface
Browse           079 integrated primarily into MedAgent/Project evidence, not a generic unsafe web browser
Audio            080 / AudioFlow foundation
Analytics        081
Knowledge/Canvas 082
Team/Hub status  083
Audio Huddles    084 advanced AudioFlow
Compute          085
Research Packs   086
Institution      087 adapter/admin surfaces where applicable
```

UI files/modules should be decomposed by product surface as complexity grows. `main.rs` remains composition/bootstrap, not the home for Research OS business logic.

Rules:

- no fake/mock product state presented as real capability;
- loading/empty/error/denied/unavailable/partial/stale/unknown states are explicit;
- authority/data/network/model/origin identity visible where trust depends on it;
- browser evidence is visually distinct from model summary;
- Core result states are not collapsed to generic success/error;
- keyboard/accessibility requirements remain part of acceptance.

## 10. Network and Browse map

All Research OS product egress extends `medscale-network` or a canonical successor broker.

Current anchors include adapter, allowlist and transport layers. New Browse, Hub, connector or institutional routes must use versioned request types and Core-approved destination/data policy.

No new crate may call arbitrary HTTP/WebSocket directly merely because it is a Hub/browser/audio dependency.

Candidate separation only after promoted-spec proof:

```text
network/adapters       deterministic HTTP/API adapters
network/browse         broker-facing browse transport/policy glue if existing network modules become too crowded
network/hub            Hub transport client
network/institutional  provider adapters
```

### Spec 079 Browse execution placement

Default flow:

```text
MedAgent/Desktop/CLI
 -> Core Browse request
 -> capability + Privacy Gate
 -> medscale-network route/destination policy
 -> HTTP/search OR bounded deterministic browser worker
 -> untrusted result parser/quarantine
 -> Core evidence validation/admission
 -> BrowseReceipt / Project evidence link
```

Browser rendering/automation libraries do not become Core dependencies. Raw credentials stay in `medscale-keys`/scoped external credential boundary and are referenced by handles only. Redirects are re-checked; private-network/loopback targets are denied by default for public Browse.

A headless Hub server introduced by 083 is a separate deployment component and must not weaken the Desktop/Core broker rule for outbound product actions.

## 11. Pack/runtime map

`medscale-pack` already contains format, store and runtime/ONNX support. Research OS model/audio/retrieval Pack work MUST extend this trust/provenance system.

080 AudioFlow must first prove whether audio model/runtime metadata fits an extension of the existing Pack format. Default is **extend**, not create an independent audio model store.

076/077 MedAgent/Fleet use admitted Packs; they do not load arbitrary Hugging Face repositories directly from UI.

086 Research Packs are a distinct domain-extension manifest concept and must not be confused with executable/model Pack admission. Contracts/UI must make that distinction explicit.

## 12. Worker boundary map

No worker framework is added before a consuming spec needs it.

Initial hierarchy:

```text
in-process trusted deterministic operation
  only if admitted library/runtime and risk allows

bounded local process/worker
  deterministic browser rendering, custom/heavy model, Python/R, arbitrary analysis code, risky parser

self-hosted remote worker
  lab GPU or institutional compute

container/OpenSandbox/HPC adapter
  only after 085/087 qualification
```

The worker receives staged exact inputs/capabilities, not the vault path. Spec 079 browser worker gets only approved navigation/request/credential handles; Spec 085 Compute formalizes the general worker lease/job contract.

## 13. Hub deployment map

083 may add new server crates/binaries only after local collaboration contracts are closed.

Candidate shape, subject to promoted-spec proof:

```text
crates/medscale-hub-contracts   only if existing contracts cannot cleanly hold wire types
crates/medscale-hub-server      evidence-selected server runtime
crates/medscale-hub-client      preferably transport/client logic integrated with network/Core boundaries
```

**Do not create these crates during 074-082.**

Buzz may donate isolated patterns/components only after exact revision/path provenance and behavior tests. Nostr/Buzz relay semantics are not automatically the Hub protocol.

## 14. AudioFlow implementation map

Foundation 080 order:

```text
1. audio contracts + Pack metadata extension design
2. OS capture abstraction contract
3. capture state machine + visible state
4. source digest/blob retention policy
5. conditioning/VAD interface contracts
6. runtime-router qualification harness
7. one selected live local STT route
8. one selected offline-quality route
9. transcript revision storage
10. diarization/alignment route
11. AudioEvidenceRef time mapping
12. COMMAND/CONTEXT/DICTATION integration with MedAgent
13. CLI
14. Desktop Audio workspace
15. soak/benchmark/security evidence
```

Do not start by copying VoiceStudio UI/backend. Use VoiceStudio/Himsat/Wispral as qualified donors behind MedScale contracts.

084 later adds huddles/TTS/duplex/cloning. These MUST NOT block 080 closure.

## 15. Analytics implementation map

081 order:

```text
1. AnalyticalView / Query contracts
2. governed read-only data projection from exact artifact revisions
3. DataFusion qualification harness
4. parser/planner write denial
5. execution/cancel/time/resource limits
6. QueryReceipt
7. derived dataset/table/figure artifacts
8. cohort definitions/materialization semantics
9. independent statistics fixtures
10. CLI
11. native Desktop Analytics workspace
12. optional adapter contract only; no Superset dependency in default Desktop
```

`population_insights.rs` is existing product behavior to inspect/reconcile; 081 must not create a competing analytics truth without migration/compatibility analysis.

## 16. Knowledge/RAG implementation map

082 order:

```text
source parser evidence -> IndexManifest -> lexical index -> permission filtering
 -> optional vector benchmark -> RetrievalPlan/Receipt -> EvidenceSpan
 -> Project Graph grounding -> Canvas refs -> MedAgent integration
```

Indexes are projections. Existing evidence/document modules are primary sources of truth.

Integrations are conditional:
- 079 Browse closes before web-source receipt/span indexing is accepted;
- 080 AudioFlow closes before audio-timestamp evidence indexing is accepted;
- 081 Analytics closes before derived analytical artifacts receive specialized knowledge integration.

No vector DB service is mandatory for Personal/Lab.

## 17. Collaboration/Buzz adaptation map

075 must first implement MedScale contracts and local semantics. Donor code transfer is optional.

For each Buzz candidate component, record:

```text
Buzz commit
exact source paths
feature/pattern being transferred
transitive dependencies
license/NOTICE + founder permission basis
security assumptions
MedScale contract it implements
behavior tests independent of Buzz
maintenance/update strategy
```

If adapting the component imports Buzz identity/protocol/storage assumptions that MedScale does not need, reimplement the smaller pattern instead.

## 18. VoiceStudio adaptation map

080/084 must separately qualify:

- model/engine registry patterns;
- streaming transcription protocol patterns;
- dictation/output-session safety;
- diagnostics/runtime doctor/error journal;
- diarization/batch concepts;
- later TTS/cloning/dubbing features.

Do not import Electron as MedScale shell. Do not put Python into Core. If Python engines win quality benchmarks, run them as pinned optional workers.

## 19. Test placement default

Follow existing crate conventions. The promoted spec must name exact files after inspecting the live tree.

Expected layers:

```text
contract serialization/strictness tests
storage migration/reopen/transaction tests
Core authority/state-machine tests
CLI integration tests
Desktop state/adapter/accessibility tests
network/Pack/browser/worker integration tests where applicable
adversarial privacy/security tests
performance/soak/benchmark harnesses
full workspace gates
```

Evidence goes under the repository's existing `evidence/<spec>-.../` convention, not inside chat logs.

## 20. CI and dependency gates

Current main CI runs formatting, dependency-direction checking, clippy, workspace tests, portable release qualification, performance evidence, cargo-deny and supply-chain policy checks.

Every promoted spec must identify which existing gates remain applicable and which new focused/platform/benchmark jobs are needed. Do not weaken current gates to land Research OS work.

Any new dependency must pass supply-chain/license/security review and be pinned according to current repository policy.

## 21. Creation of a new crate — hard gate

A new crate is allowed only when the active spec records all of:

1. functionality has a cohesive responsibility not owned cleanly by an existing crate;
2. placing it in the existing crate would create a forbidden dependency or materially damage testability/packaging;
3. public API/dependency direction is explicit;
4. lifecycle/ownership is stable enough to justify workspace cost;
5. CI/build/package impact is qualified;
6. it does not create alternate Core authority.

Otherwise implement as an existing-crate module.

## 22. First implementation target after planning promotion

The first Research OS unit remains 074 (or the renumbered equivalent if live main advances): Project + Artifact Graph Foundation.

Recommended dependency-ordered slices:

```text
074-A contracts + serialization/invariant tests
074-B storage schema + migration/reopen/recovery tests
074-C Core project lifecycle + graph authority
074-D CLI project vertical slice
074-E Desktop Projects surface
074-F compatibility/scale/failure evidence + closure
```

Do not start 075 until 074 is canonically closed.

## 23. Required reconciliation at promotion time

Before the first line of Research OS implementation code:

- verify current main SHA and active spec;
- compare current workspace/crate/module structure to this map;
- inspect all current object/provenance/audit/policy/effect types;
- update the promoted spec with exact paths/types after live inspection;
- run current baseline tests/gates on the base revision;
- record any pre-existing failures separately;
- freeze the bounded 074 acceptance/evidence packet.

If this map and live repository disagree, live repository wins and the plan is amended explicitly. Silent divergence is prohibited.
