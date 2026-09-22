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

- [ ] Implement agent identity registration (bound to one admitted local
      model Pack), get/list, revoke.
- [ ] Implement `AgentCapabilityManifest` as immutable-once-set.
- [ ] Wire 077 `Capability`/`RequestBody`/`ResponseBody` variants into
      `envelopes/mod.rs`, plus a `medagent()` facade helper and dispatch
      arms in `facade.rs` (mirrors the `pg`/`ds`/`collab` pattern exactly).
- [ ] CLI inspect for agent identity.

**Acceptance:** identity vertical slice complete and CI-green; reopen
durability proven; registration against a non-admitted Pack is refused.

## T077-04 — ContextManifest

- [ ] Implement context-manifest create/get from an explicit artifact
      list.
- [ ] Implement the single, consulted-everywhere read-boundary check.

**Acceptance:** an agent run cannot resolve an artifact outside its bound
`ContextManifest` through any exposed Core path.

## T077-05 — AgentRun lifecycle

- [ ] Implement `AgentRun` create/get/list and the full frozen state
      machine including cancel/interrupt.
- [ ] Implement `AgentTurn` as the append-only per-step record.

**Acceptance:** full lifecycle proven end to end including a cancellation
race test; no transition outside the frozen table is reachable.

## T077-06 — Tool invocation

- [ ] Implement `ToolManifest`/`ToolInvocation`/`ToolReceipt`, policy-
      checked against `AgentCapabilityManifest` and `ContextManifest`
      before dispatch, executed entirely by Core.

**Acceptance:** an ungranted tool kind is refused before execution and
recorded as a refusal.

## T077-07 — Model Pack lane + AgentProposal

- [ ] Wire a real text-prompt-in/proposal-out run against the existing
      `medscale-pack` runtime; record exactly which admitted
      `PackRuntimeAdapter` this spec uses and why.
- [ ] Persist the run's output as an `AgentProposal`, submitted through
      the existing `Proposal`/`CreateProposal` path.
- [ ] Structurally prove no call path from this module into
      `authority::promote`, `authority::amend`, or `contracts::actions`.

**Acceptance:** a real local Project-grounded run (zero network) completes
and produces a persisted, evidence-only `AgentProposal`.

## T077-08 — RunReceipt + run history

- [ ] Implement `RunReceipt` committed atomically with a run's terminal
      transition.
- [ ] Implement bounded, filterable run-history read.

**Acceptance:** every terminal run has exactly one `RunReceipt`; no
orphaned receipt, no receipt-less terminal run.

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
