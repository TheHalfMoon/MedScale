# Tasks — Spec 077 MedAgent Workbench

**Execution state:** `PROMOTED_IMPLEMENTATION_AUTHORIZED` (not yet implemented)

Check a task only when its implementation, tests and required evidence are
real on the branch. Do not pre-check future work.

## T077-00 — Live truth and baseline

- [x] Verify branch/base/main/PR state and repository cleanliness.
- [x] Read mandatory governance + Research OS V2 + Spec 077 authority chain.
- [x] Inspect current contracts/Core/storage/network/key/CLI/Desktop
      ownership paths, and the existing `medscale-pack` runtime surface
      (`PackRuntimeAdapter`, `FixtureRuntime`, `OnnxTokenClassifierRuntime`,
      `PackStore`, `PackManifestV0`).
- [x] Confirm no live Spec 077 collision/superseding authority; prove
      numbering is free.
- [x] Verify Spec 076 closure and post-main evidence.
- [x] Record baseline focused/full gate state and any pre-existing
      failures.
- [x] Create initial evidence/live-truth record.

**Gate:** no code mutation before T077-00 is complete.

## T077-01 — Freeze contracts/data model

- [x] Freeze exact field-level `AgentIdentity`/`AgentProfile`/
      `AgentCapabilityManifest`/`ContextManifest`/`AgentRun`/
      `AgentRunState`/`AgentTurn`/`ToolManifest`/`ToolInvocation`/
      `ToolReceipt`/`RunReceipt`/`AgentProposal` contracts in `contracts.md`.
- [x] Reuse `OpaqueId`, `ObjectHeader`, `DigestSha256`, `ProjectRevision`,
      `ArtifactDescriptor`/`ArtifactVersionBinding`, `Proposal`/
      `ProducerKind` where semantically valid.
- [x] Freeze the `AgentRunState` transition table explicitly; reject
      unsafe/unknown states via a closed-vocabulary parse pattern.
- [x] Freeze revision/precondition behavior for mutable rows.
- [x] Freeze the `AgentProposal` <-> `Proposal` relationship design
      decision (plan.md 077-A) and record it here.
- [x] Add serialization/validation/invariant tests.

**Verified (this session):** `crates/medscale-contracts/src/medagent.rs`
(799 lines) is import-clean (only `serde`/`serde_json`/internal `objects`/
`project_graph` -- no storage/network/UI/model-runtime dependency), carries
10 `#[test]` cases (status/kind round-trips, transition-table legality,
bound checks), and `contracts.md` section on `AgentProposal` records the
thin-linking-record design decision referenced from `plan.md` 077-A.
`ProducerKind::Agent(OpaqueId)` was added additively in
`crates/medscale-contracts/src/objects/authority_classes.rs`.

**Acceptance:** contract tests pass; no storage/network/UI/model-runtime
dependency enters contracts.

## T077-02 — Storage schema + migration

- [x] Add MedAgent storage using current encrypted vault/SQLite metadata
      architecture (v5 -> v6).
- [x] Add required indexes (`migration.md` section 3).
- [x] Implement atomic transactions (mutation + terminal-state/`RunReceipt`
      commit together) and stale-revision conflict behavior.
- [x] Wire every 077 table into `SqliteMetaStore::snapshot_bytes` (schema
      version bumped 5 -> 6) and add `restore_v6` in `backup.rs` -- **and
      actually re-verify** whatever run/receipt consistency invariant this
      spec defines at restore time, not merely at insert time (the exact
      gap Spec 076's exact-range review found and fixed in its own
      `restore_v5`; do not repeat it here).
- [x] Add forward migration test fixtures from representative pre-077
      vaults (including populated 074/075/076 vaults, with at least one
      076 `ParticipantKind::Agent` participant to prove the integration
      point).
- [x] Add repeated-open/repeated-migration safety tests.
- [x] Add crash/reopen and backup/restore recovery tests.
- [x] Prove no 077 operation cascades deletion into canonical target
      objects.
- [x] Fix forward any pre-existing test assertions in `project_graph_074.rs`/
      `data_sources_075.rs`/`collaboration_076.rs` that pin
      `finished_version == 5`, now stale at 6.

**Acceptance:** storage + migration suite green and pre-077 IDs/behavior
preserved.

**Reconciled (this session):** the prior commit (`a430d48`) implemented the
full v6 DDL, CRUD, and `restore_v6` re-verification logic, but shipped with
**zero tests** of its own (`grep -c '#\[test\]' crates/medscale-storage/src/medagent.rs`
== 0, and no `crates/medscale-storage/tests/medagent_077.rs` existed) --
the code existed but the acceptance bullets above were not actually met.
Closed by adding `crates/medscale-storage/tests/medagent_077.rs` (11 tests):
migration fixture with a 076 `ParticipantKind::Agent` participant +
repeated-open safety, a full CRUD/lifecycle roundtrip exercising every 077
table, CAS/stale-revision tests for identity revoke and run transitions, two
tests that directly tamper the DB to prove `verify_run_receipt_consistency`
actually detects both an orphaned receipt and a receipt-less terminal run
(not just a happy-path no-op), a transaction-atomicity test for
`insert_executed_tool_invocation_with_receipt`, backup/restore roundtrip
covering every 077 family, a hand-edited-backup test proving `restore_v6`'s
`verify_run_receipt_consistency` call fails closed on a tampered snapshot
whose outer digest was recomputed to match (mirroring Spec 076's own
`restore_rejects_hand_edited_backup_with_broken_activity_chain`), and a
no-cascade test proving the canonical `Project` row survives every 077
mutation path unchanged. See `evidence/077-medagent-workbench/T077-02_GAP_CLOSURE.md`.

## T077-03 — AgentIdentity + capability manifest

- [x] Implement agent identity registration (bound to one admitted local
      model Pack), get/list, revoke.
- [x] Implement `AgentCapabilityManifest` as immutable-once-set.
- [x] Wire 077 `Capability`/`RequestBody`/`ResponseBody` variants into
      `envelopes/mod.rs`, plus a `medagent()` facade helper and dispatch
      arms in `facade.rs` (mirrors the `pg`/`ds`/`collab` pattern exactly).
- [x] CLI inspect for agent identity.

**Acceptance:** identity vertical slice complete and CI-green; reopen
durability proven; registration against a non-admitted Pack is refused.

**Implemented (this session):** `Capability::AgentIdentityRegister/Read/
Revoke` + matching `RequestBody`/`ResponseBody` variants in
`envelopes/mod.rs`; `crates/medscale-core/src/authority/medagent.rs`
(`MedAgent` struct, mirrors `Collab`'s exact pattern, including the same
scope-level `ActionAuditRecord` audit-trail convention); a `medagent()`
facade helper + 4 dispatch arms + `capability_matches` pairs in
`facade.rs`; `CliSession::medagent_identity_{register,get,list,revoke}` in
`cli_session.rs`; a full `medagent identity {register,show,list,revoke}`
CLI vertical slice in `crates/medscale-cli/src/medagent.rs` wired into
`main.rs`. `pack_version` is Core-derived from the admitted
`PackManifestV0`, never caller-supplied (contracts.md T077-03 decision
note). Tests: `crates/medscale-core/tests/medagent_077.rs` (4 tests) --
full register/get/list/revoke roundtrip through real `CoreFacade::dispatch`
using the Spec 008 fixture pack, non-admitted-pack registration refusal
(`security.md` T5), capability-manifest immutability +
project-scoping proof, and vault-reopen durability. See
`evidence/077-medagent-workbench/T077-03_IMPLEMENTATION.md`.

## T077-04 — ContextManifest

- [x] Implement context-manifest create/get from an explicit artifact
      list.
- [x] Implement the single, consulted-everywhere read-boundary check.

**Acceptance:** an agent run cannot resolve an artifact outside its bound
`ContextManifest` through any exposed Core path.

**Implemented (this session):** `Capability::ContextManifestCreate/Read` +
`RequestBody::ContextManifestCreate/Get` + `ResponseBody::
MedAgentContextManifest{manifest, resolutions}` in `envelopes/mod.rs`;
`MedAgent::create_context_manifest`/`get_context_manifest` in
`medagent.rs` (Core); `MedAgent::require_artifact_in_context` -- **the**
single Core-internal boundary check every future tool-dispatch path
(T077-06) must call, returning `Unauthorized` for any object not named by
the bound `ContextManifest`, `WrongScope` for a cross-realm/scope context
id. `resolve_context_artifact` duplicates `project_graph::
ProjectGraph::resolve_descriptor`'s logic (per this spec's module-
independence convention) so every selected artifact's `ReferenceResolution`
is recomputed live on every read, never cached (`migration.md` section 4)
-- shape is validated at write time only (`ArtifactDescriptor::validate`),
so a manifest may legitimately name an artifact that does not exist yet or
has since gone stale; this is by design, not a gap.

Tests (inline `#[cfg(test)] mod tests` in `medagent.rs`, mirroring
`project_graph.rs`'s own precedent, since `MedAgent` is not exported for
external integration tests): resolution matrix (Current/Missing/
UnsupportedKind), create+get roundtrip proving live (not creation-time)
resolution including a not-yet-existing artifact resolving `Missing`,
`require_artifact_in_context` allowing the named artifact and refusing
both a real-but-unnamed artifact and a fabricated id (`security.md` T4's
core primitive), and cross-scope isolation for both `get_context_manifest`
and `require_artifact_in_context`. CLI: `medagent context-create`/
`context-show`. See `evidence/077-medagent-workbench/T077-04_IMPLEMENTATION.md`.

## T077-05 — AgentRun lifecycle

- [x] Implement `AgentRun` create/get/list and the full frozen state
      machine including cancel/interrupt.
- [x] Implement `AgentTurn` as the append-only per-step record.

**Acceptance:** full lifecycle proven end to end including a cancellation
race test; no transition outside the frozen table is reachable.

**Implemented (this session):** `Capability::AgentRunCreate/Read/Start/
Cancel` + matching `RequestBody`/`ResponseBody` variants; `MedAgent::
create_agent_run/get_agent_run/list_agent_runs/start_agent_run/
cancel_agent_run/list_agent_turns` in `medagent.rs` (Core). `AgentTurn`s
are never externally appendable -- `start_agent_run` auto-appends the
initial `PromptSubmitted` turn from the run's own already-known `prompt`
field (no caller-supplied turn content is ever trusted); `ToolRequested`/
`ToolResult` (T077-06) and `ModelOutput` (T077-07) turns will likewise be
Core-internal side effects of their own dispatch paths. `create_agent_run`
and `start_agent_run` both re-check the bound `AgentIdentity` is `Active`
and its captured `pack_version` still matches the currently admitted Pack
(`security.md` T5, never cached). `create_agent_run` also refuses an
identity or context manifest that does not belong to the run's own
`project_id` (cross-project cross-wiring). `cancel_agent_run` commits a
`RunReceipt` atomically with the terminal transition
(`tool_invocation_ids` is empty until T077-06 exists). CLI: `medagent
run-create/run-show/run-list/run-start/run-cancel/run-turns`.

Tests (`crates/medscale-core/tests/medagent_077.rs`, 6 new): full
start-then-cancel lifecycle through real `CoreFacade::dispatch` (proving
the auto-appended turn and its exact payload), cancelling a still-`Pending`
run directly, a cancellation race (two callers submit the same
`expected_revision`; exactly one wins, the other fails closed with
`Conflict`, and the run shows exactly one terminal transition -- not two),
illegal transitions (`Cancelled -> Running`, `Cancelled -> Cancelled`)
rejected, cross-project identity/context rejection, and a revoked identity
failing closed at both create and start. See
`evidence/077-medagent-workbench/T077-05_IMPLEMENTATION.md`.

## T077-06 — Tool invocation

- [x] Implement `ToolManifest`/`ToolInvocation`/`ToolReceipt`, policy-
      checked against `AgentCapabilityManifest` and `ContextManifest`
      before dispatch, executed entirely by Core.

**Acceptance:** an ungranted tool kind is refused before execution and
recorded as a refusal.

**Implemented (this session):** `Capability::AgentToolInvoke` + `RequestBody::
AgentToolInvoke{run_id,kind,arguments}` + `ResponseBody::
MedAgentToolInvocation{invocation, receipt: Option<...>}`; `MedAgent::
invoke_tool` in `medagent.rs` (Core) -- the sole tool-dispatch path and
the first production caller of `require_artifact_in_context` (its
`#[allow(dead_code)]` from T077-04 is now removed). Arguments are parsed
into closed, `deny_unknown_fields` typed structs
(`ReadContextArtifactArgs`/`SearchContextArtifactsArgs`) before any use
(`security.md` T3); a tool kind not in the run's `AgentCapabilityManifest`,
an oversized argument payload, a malformed argument shape, or an artifact
named outside the run's bound `ContextManifest` are all recorded as a
`Refused` `ToolInvocation` with a reason -- never a hard error, never
partial execution. Invoking a tool on a non-`Running` run (Pending or
terminal) IS a hard `AuthorityError::Conflict`, deliberately distinct from
a refusal: that is a caller/session-state problem, not "the model asked
for something disallowed." Every invocation appends `ToolRequested`
(before the grant/boundary check) and `ToolResult` (after, whichever way
it resolved) turns as side effects -- never a directly callable "append
arbitrary turn" capability. `ReadContextArtifact` returns the artifact's
raw bytes (`SourceRecord`/`DerivedSourceArtifact` only), lossily UTF-8
decoded and bounded; `SearchContextArtifacts` does a bounded,
case-insensitive substring search over only the run's bound context
artifacts (never a broader vault query), with UTF-8-char-boundary-safe
snippet extraction. CLI: `medagent tool-invoke`.

Tests (`crates/medscale-core/tests/medagent_077.rs`, 6 new, through real
`CoreFacade::dispatch` against real `SourceRecord`s): granted
`ReadContextArtifact` executes and returns exact content (plus proves the
3-turn sequence: `prompt_submitted`/`tool_requested`/`tool_result`), an
ungranted kind is refused with no receipt, a granted kind naming a
real-but-out-of-context artifact is refused (not executed), bounded
case-insensitive search finds the expected snippet, malformed arguments
(missing required field) are refused rather than panicking or partially
executing, and invoking a tool against a non-`Running` run fails closed
with `Conflict`. See `evidence/077-medagent-workbench/T077-06_IMPLEMENTATION.md`.

## T077-07 — Model Pack lane + AgentProposal

- [x] Wire a real text-prompt-in/proposal-out run against the existing
      `medscale-pack` runtime; record exactly which admitted
      `PackRuntimeAdapter` this spec uses and why.
- [x] Persist the run's output as an `AgentProposal`, submitted through
      the existing `Proposal`/`CreateProposal` path.
- [x] Structurally prove no call path from this module into
      `authority::promote`, `authority::amend`, or `contracts::actions`.

**Acceptance:** a real local Project-grounded run (zero network) completes
and produces a persisted, evidence-only `AgentProposal`.

**Implemented (this session):** `Capability::AgentRunExecute` +
`RequestBody::AgentRunExecute{run_id,local_path,max_tokens,synthetic_only}`
+ `ResponseBody::MedAgentRunExecuted{turn,proposal}`; `MedAgent::
execute_agent_run` in `medagent.rs` (Core). **Runtime used:
`medscale_pack::OnnxTokenClassifierRuntime`** (`tract_onnx_token_
classification_v1`), the only `PackRuntimeAdapter` in this repository
genuinely admitted for real local model execution -- `FixtureRuntime` is
explicitly documented in its own source as "no native model engine," a
stub, not a real model. Exercised against the Spec 069 fixture pack
(`pack-tiny-token-classifier-v0`); the Spec 008 `pack-fixture-ner-v0` used
throughout T077-03..T077-06 declares `runtime_requirements:
fixture_runtime_v0` and carries no ONNX artifacts at all, so it cannot be
used for T077-07's real-execution requirement. `local_path` is
caller-supplied on every call (mirrors `PacksEvaluateLocal` exactly --
`PackManifestV0` is purely content-addressed and never stores an on-disk
path). `execute_agent_run` verifies the supplied directory's manifest
matches BOTH the run's bound `AgentIdentity.pack_id`/`pack_version` AND
the currently admitted `PackStore` entry's digest/epoch/version (exact
model Pack identity + exact runtime identity), gates on
`synthetic_only: true` (mirrors `PackEvaluationRequest.synthetic_only` --
real PHI through this runtime requires a later, explicit gate this spec
does not grant), appends the `ModelOutput` turn, and constructs the
`Proposal` object directly with `producer: ProducerKind::Agent(
agent_identity_id)` (not through `RequestBody::CreateProposal`, whose
frozen shape always sets `ProducerKind::Rule` with no way to override it --
this mirrors that same dispatch arm's exact construction pattern instead).
`AgentProposal`'s `evidence_refs` link to every artifact named by the
run's bound `ContextManifest`. No prepared-model cache reuse (a deliberate
v1 simplification: a run's model execution happens once, not in a hot
loop, unlike interactive pack evaluation).

**Structural proof (T3):** `grep -n "authority::promote\|authority::amend\|contracts::actions\|super::promote\|super::amend\|::actions::" crates/medscale-core/src/authority/medagent.rs`
returns no matches -- zero references to any of those paths anywhere in
this spec's Core module.

Tests (`crates/medscale-core/tests/medagent_077.rs`, 4 new, through real
`CoreFacade::dispatch` against the real Spec 069 ONNX fixture pack): a
real local model execution produces a `ModelOutput` turn and an
`AgentProposal`, whose underlying `Proposal` object (read back via
`Capability::ReadObject`) carries `producer: agent(..)` and non-empty
`evidence_refs`; `synthetic_only: false` refuses with
`ExternalGateRequired` before touching the runtime; a `local_path`
pointing at a different, really-admitted pack than the one the identity
is bound to fails closed with `DigestMismatch`; execution against a
non-`Running` run fails closed with `Conflict`. See
`evidence/077-medagent-workbench/T077-07_IMPLEMENTATION.md`.

## T077-08 — RunReceipt + run history

- [x] Implement `RunReceipt` committed atomically with a run's terminal
      transition.
- [x] Implement bounded, filterable run-history read.

**Acceptance:** every terminal run has exactly one `RunReceipt`; no
orphaned receipt, no receipt-less terminal run.

**Implemented (this session):** `cancel_agent_run` (T077-05) was already
atomic with its `RunReceipt`, but always recorded an empty
`tool_invocation_ids: Vec::new()` even when tools had actually run before
cancellation -- a real provenance gap, not merely an unimplemented
feature. Refactored into a shared `MedAgent::commit_terminal_run` helper
(used by `cancel_agent_run` and two new methods,
`complete_agent_run`/`fail_agent_run`) that threads the run's *real*
tool-invocation history via `self.meta.list_tool_invocations(run_id)`
(executed and refused invocations alike, in `seq` order -- "exact
provenance" means the full record, not just the successes) into every
terminal `RunReceipt`, closing that gap for all three terminal states at
once. `Capability::AgentRunComplete`/`AgentRunFail` +
`RequestBody::AgentRunComplete`/`AgentRunFail` added (reusing
`ResponseBody::MedAgentRunTerminal`); `fail_agent_run` takes a bounded
`failure_reason: String`. "Resource/timing facts" from `plan.md`'s work
bullet are not separately tracked: `RunReceipt`'s frozen contract
(T077-01) has no timestamp/duration field, and this promotion did not
authorize amending that frozen shape -- turn/tool-invocation counts
already recoverable from `list_agent_turns`/`tool_invocation_ids.len()`
are the resource facts this spec's frozen contract actually supports; an
honest residual, not a fabricated claim of tracking that does not exist.

**Bounded, filterable run history:** `list_agent_runs` (both the Core
method and the underlying storage query) gained a `status:
Option<AgentRunState>` filter, using
`idx_medagent_runs_project(project_id, status)` directly via SQL `WHERE`
(not a post-fetch filter, which could silently under-return fewer than
`limit` matches when more exist beyond a pre-filter window). By Project
was already required (always present); by agent identity already existed
(T077-05); by state is the addition this task's own acceptance bullet
names. CLI: `medagent run-list --status`, `run-complete`, `run-fail`.

Tests (`crates/medscale-core/tests/medagent_077.rs`, 4 new, through real
`CoreFacade::dispatch`): a completed run's receipt threads the exact real
`ToolInvocation` id from a tool call made mid-run; a failed run's receipt
carries the exact failure reason; completing a still-`Pending` run
(`Pending -> Completed` is not in the frozen table) fails closed with
`Conflict`; run history correctly returns only the matching subset for
each of `Cancelled`/`Pending`/no-filter across two runs in different
states. See `evidence/077-medagent-workbench/T077-08_IMPLEMENTATION.md`.

## T077-09 — Native Desktop and CLI parity

- [ ] Add MedAgent navigation through current native Slint composition,
      backed exclusively by Core: split-pane run list/detail, context
      selection, prompt submission, cancel/interrupt controls.
- [ ] Preserve design system, keyboard/focus/accessibility, light/dark
      parity.
- [ ] Complete CLI vertical slice with human + JSON output.
- [ ] Add an in-module real-Core-session test for every new Desktop
      view-model function (mirroring Spec 075/076's precedent, and
      specifically checking any session-holder-identity assumption the way
      Spec 076's own such test caught a real bug).

**Acceptance:** CLI vertical slice complete and CI-green; Desktop panel
implemented and genuinely Core-backed.

## T077-10 — Qualification and closure

- [ ] Run format, dependency-direction, focused contract/storage/Core/CLI/
      Desktop tests.
- [ ] Run Clippy under current policy, workspace tests, cargo-deny/
      supply-chain gates.
- [ ] Run migration/reopen/recovery, malformed/corrupt, context-boundary-
      leakage suites.
- [ ] Run credential/log secret and content-leakage scans.
- [ ] Capture rendered Desktop evidence or record the honest residual if
      the toolchain/CI cannot produce it.
- [ ] Perform exact-range review of the full PR diff (OpenCodeReview
      delegation mode, matching Spec 076's discipline, unless the founder
      gives a different explicit instruction).
- [ ] Run exact-head required CI; merge only when green and governance
      permits.
- [ ] Verify post-merge main CI; update queue/status to `CLOSED_CANONICAL`
      only with real post-main evidence.
- [ ] Recompute the next eligible unit; do not implement 078+ without
      separate promotion.

**Acceptance:** all frozen acceptance requirements mapped to exact-head
proof.
