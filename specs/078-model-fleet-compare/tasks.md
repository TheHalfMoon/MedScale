# Tasks — Spec 078 Model Fleet + Compare

**Execution state:** `T078-00..T078-07 COMPLETE; T078-08 IN PROGRESS`

Check a task only when its implementation, tests and required evidence are
real on the branch. Do not pre-check future work.

## T078-00 — Live truth and baseline

- [x] Verify branch/base/main/PR state and repository cleanliness.
- [x] Read mandatory governance + Research OS V2 + Spec 078 authority chain.
- [x] Inspect current contracts/Core/storage/network/key/CLI/Desktop
      ownership paths, and the full Spec 077 contract/Core surface this
      spec drives (`AgentIdentity`, `AgentCapabilityManifest`,
      `ContextManifest`, `AgentRun`/`AgentRunState`/`AgentTurn`,
      `ToolInvocation`/`ToolReceipt`, `RunReceipt`, `AgentProposal`,
      `MedAgent::{create_agent_run, start_agent_run, execute_agent_run,
      cancel_agent_run, get_agent_run, list_agent_runs}`).
- [x] Confirm no live Spec 078 collision/superseding authority; prove
      numbering is free.
- [x] Verify Spec 077 closure and post-main evidence.
- [x] Record baseline focused/full gate state and any pre-existing
      failures.
- [x] Create initial evidence/live-truth record.

**Verified (this session):** `evidence/078-model-fleet-compare/LIVE_TRUTH.md`
records live-reverified Spec 077 closure (PR #133/#134 SHAs and CI run ids,
including a correction of one inaccurate run id an earlier self-generated
continuation prompt had cited), numbering-free proof, the live lane-plurality
inventory (exactly one real ONNX-backed Pack fixture), and the
promotion-commit's own exact-head CI qualification (run `35750689882`,
head `58c4cd8`, 6/6, re-verified via `gh run view ... --json
conclusion,headSha,jobs`).

**Gate:** no code mutation before T078-00 is complete.

## T078-01 — Freeze contracts/data model

- [x] Freeze exact field-level `AgentLane`/`LanePolicy`/`LaneTransform`/
      `AgentLaneStatus`/`FleetRun`/`FleetRunState`/`LaneRunRef`/
      `ComparisonRequest`/`ComparisonObservation`/
      `ComparisonObservationKind`/`ComparisonReport` contracts in
      `contracts.md`.
- [x] Reuse `OpaqueId`, `ObjectHeader`, `DigestSha256`, `ProjectRevision`,
      and Spec 077's frozen `medagent` contracts where semantically valid;
      zero modification to `crates/medscale-contracts/src/medagent.rs`.
- [x] Freeze the `FleetRunState` transition table explicitly; reject
      unsafe/unknown states via a closed-vocabulary parse pattern.
- [x] Freeze revision/precondition behavior for mutable rows.
- [x] Freeze `LaneTransform`'s exact variant set (zero variants, proven
      uninhabited at the type level) and resolve the classification-type
      question (no such primitive exists in this repository; dropped from
      `ComparisonReport`'s v1 shape, see `contracts.md` section 4).
- [x] Add serialization/validation/invariant tests.

**Verified (this session):** `crates/medscale-contracts/src/model_fleet.rs`
(771 lines) is import-clean (only `serde` + `crate::medagent::
{AgentCapabilityManifest, AgentRunState, ContextManifest, ToolKind}`
(Spec 077's own frozen contracts, reused unmodified) + internal `objects`/
`project_graph` -- no storage/network/UI/model-runtime dependency, no
`serde_json`), carries 12 `#[test]` cases (status/kind round-trips,
`FleetRunState` transition-table legality and terminal classification,
`FleetRun::aggregate_state` covering every lane-state combination,
`LanePolicy::validate_within` rejecting both a superset tool-kind grant and
an out-of-manifest artifact, bound checks, and `ComparisonReport`/
`ComparisonObservation` shape validation including the multi-lane-kind
enforcement and the participating/excluded-overlap rejection).
`crates/medscale-contracts/src/lib.rs` registers `pub mod model_fleet;`
alphabetically between `mobile` and `mesc`/`network`. `cargo fmt --check`
clean; local `cargo check`/`test` cannot link on this workstation (known
constraint) -- exact-head CI is the qualification path (T078-08).
`grep -rn "Sensitivity\|DataClass\|privacy\|Privacy" crates/
medscale-contracts/src` found no existing classification primitive,
confirming the `ComparisonReport.classification` field-drop decision was a
real finding, not an assumption.

**Acceptance:** contract tests pass; no storage/network/UI/model-runtime
dependency enters contracts; no Spec 077 contract file is modified.

## T078-02 — Storage schema + migration

- [x] Add Model Fleet storage using current encrypted vault/SQLite metadata
      architecture (v6 -> v7).
- [x] Add required indexes (`migration.md` section 3).
- [x] Implement atomic transactions (per-lane dispatch, fleet terminal
      state + dependent rows commit together) and stale-revision conflict
      behavior.
- [x] Wire every 078 table into `SqliteMetaStore::snapshot_bytes` (schema
      version bumped 6 -> 7) and add `restore_v7` in `backup.rs` -- **and
      actually re-verify** whatever fleet/lane/comparison consistency
      invariant this spec defines at restore time, not merely at insert
      time (the exact discipline Spec 076/077's own exact-range reviews
      established; do not repeat their original gap here).
- [x] Add forward migration test fixtures from representative pre-078
      vaults (including populated 074/075/076/077 vaults, with at least
      one real Spec 077 `AgentIdentity`/`ContextManifest`/completed
      `AgentRun` to prove the 078 layer binds to genuinely pre-existing
      objects).
- [x] Add repeated-open/repeated-migration safety tests.
- [x] Add crash/reopen and backup/restore recovery tests.
- [x] Prove no 078 operation cascades deletion into canonical or Spec 077
      target objects.
- [x] Fix forward any pre-existing test assertions elsewhere in the
      workspace that pin `finished_version == 6`, now stale at 7 (the same
      class of fix Spec 077 itself needed for the 5 -> 6 bump).

**Verified:** `crates/medscale-storage/src/model_fleet.rs` (v7 DDL: four
tables, seven indexes, zero foreign keys), the v6 -> v7 step in
`sqlite_meta.rs`, snapshot families + `restore_v7` in `backup.rs` with
restore-time `verify_model_fleet_consistency` (eight invariants). 14 tests in
`crates/medscale-storage/tests/model_fleet_078.rs` run against a genuinely
pre-078 v6 file built from populated 074/075/076/077 rows (including a
completed Spec 077 run bound to `pack-tiny-token-classifier-v0`). The
per-lane "atomic dispatch" wording was reconciled with the unmodified-Spec-077
rule in `migration.md` section 5 (create -> bind -> start). Stale v6 pins were
fixed forward, including a real weakening of the 074 interrupted-migration
test caught by CI run `35765069726`. Qualified by exact-head CI run `35768404980` on head `547fb41` (6/6 required jobs; see `evidence/078-model-fleet-compare/EXACT_HEAD_QUALIFICATION.md`). Details:
`evidence/078-model-fleet-compare/STORAGE_MIGRATION_RECOVERY.md`.

**Acceptance:** storage + migration suite green and pre-078 IDs/behavior
(including the full Spec 077 suite) preserved.

## T078-03 — AgentLane + LanePolicy

- [x] Implement agent-lane creation (bound to an existing `AgentIdentity` +
      `ContextManifest`), with `LanePolicy` subset-of-capability and
      subset-of-context validation (`security.md` T2), get/list, and
      retire.
- [x] CLI inspect for agent lane.

**Verified:** `authority::model_fleet` create/get/list/retire through
`CoreFacade::dispatch`, `AgentLane*` capabilities/envelopes/`CliSession`, and
CLI `lane-{create,show,list,retire}`. Eight Core integration tests (T2
refusals, binding refusals, T5, T7, scope, reopen durability) plus the CLI
end-to-end test. Qualified by exact-head CI run `35768404980` on head `547fb41` (6/6 required jobs; see `evidence/078-model-fleet-compare/EXACT_HEAD_QUALIFICATION.md`). Details:
`evidence/078-model-fleet-compare/CORE_AUTHORITY_QUALIFICATION.md`,
`CLI_QUALIFICATION.md`.

**Acceptance:** agent-lane vertical slice proven end to end with reopen
durability; a lane policy naming an ungranted tool kind or an
out-of-manifest artifact is refused at creation.

## T078-04 — FleetRun lifecycle

- [x] Implement `FleetRun` create (`Pending`), per-lane dispatch through
      Spec 077's unmodified `create_agent_run`/`start_agent_run`/
      `execute_agent_run`, and `FleetRunState` aggregation.
- [x] Implement fleet-level cancel (cancels every in-flight lane through
      Spec 077's own `cancel_agent_run`).
- [x] Implement `LaneRunRef` as the append-only binding record.
- [x] Prove structurally that no lane's in-flight run can read another
      lane's context/tools/results (`security.md` T3).
- [x] Prove that every `LaneRunRef` in a `FleetRun` references a distinct
      `AgentRun.header.id` (`security.md` T4) -- reject any attempt to bind
      the same `AgentRun` id twice.

**Verified:** create/dispatch/execute-lane/cancel, with Spec 077 driven only
through its public `MedAgent` entry points. Dispatch preflights every lane
(including Pack admission, a real gap fixed before first CI). The lane-policy
guard on Spec 077's direct `AgentToolInvoke` path closes a real T2 gap.
Cancel semantics follow `contracts.md` section 3. Seven Core tests over real
local ONNX execution cover Completed / PartiallyFailed / Failed, cancel and
its races, zero-write refusals, T3 isolation, the real-PHI gate, and reopen.
Qualified by exact-head CI run `35768404980` on head `547fb41` (6/6 required jobs; see `evidence/078-model-fleet-compare/EXACT_HEAD_QUALIFICATION.md`). Details: `SECURITY_ADVERSARIAL.md`.

**Acceptance:** full fleet state machine proven end to end, including
partial failure and fleet-level cancel mid-flight; no transition outside
the frozen table is reachable.

## T078-05 — Comparison engine

- [x] Implement `ComparisonRequest` handling (refuse for `Failed`/
      `Cancelled` fleets, accept for `Completed`/`PartiallyFailed`).
- [x] Implement the comparison computation over real `AgentProposal`/
      `RunReceipt`/`ToolReceipt` content: agreement/disagreement,
      contradiction candidates, evidence/citation overlap,
      unsupported-claim candidates, abstention, schema validity,
      resource/runtime facts.
- [x] Persist as an immutable `ComparisonReport` with explicit
      `excluded_lane_ids`.
- [x] Prove structurally: no numeric score/rank/winner field anywhere in
      the observation/report shape or its computation.
- [x] Record in `contracts.md`/`CLOSURE.md` exactly which admitted Pack(s)
      the closure-gate two-lane fixture uses and why (the lane-plurality
      scope decision), with real distinct `AgentRun` ids as evidence, not
      merely assumed.

**Verified:** `authority::model_fleet_compare` (pure; six unit tests,
including the structural no-score/rank/winner/confidence scan) and
`compute_comparison` (refused unless Completed/PartiallyFailed; failed or
cancelled lanes named in `excluded_lane_ids`; one immutable report row). The
closure two-lane fixture uses one admitted Pack, two identities, disjoint
contexts and different policies, with two distinct `AgentRun` ids and
receipts. Which kinds a real run can and cannot produce with the one
deterministic Pack is stated in
`evidence/078-model-fleet-compare/LANE_PLURALITY_QUALIFICATION.md`.
Qualified by exact-head CI run `35768404980` on head `547fb41` (6/6 required jobs; see `evidence/078-model-fleet-compare/EXACT_HEAD_QUALIFICATION.md`).

**Acceptance:** a real `ComparisonReport` computed over at least two
genuinely independent lane runs records factual observations across every
required kind with evidence references into real underlying rows; a
partial-failure fleet's report explicitly names its excluded lane(s).

## T078-06 — Fleet/comparison history

- [x] Implement bounded, filterable fleet-run history read (by Project, by
      state) and comparison-report history read (by fleet run).
- [x] Ensure recomputation creates a new report row, never mutates a prior
      one.

**Verified:** fleet-run list by Project and state (bounded), report history
per fleet run oldest-first, append-only recompute with lane runs and fleet
unchanged, and history across reopen with scope checks
(`recompute_appends_a_new_report_and_never_touches_lane_runs`). Qualified by
exact-head CI run `35768404980` on head `547fb41` (6/6 required jobs; see `evidence/078-model-fleet-compare/EXACT_HEAD_QUALIFICATION.md`).

**Acceptance:** every fleet run and comparison report is inspectable after
the fact with exact provenance back to the real Spec 077 rows it
references.

## T078-07 — Native Desktop and CLI parity

- [x] Add a Model Fleet navigation route through current native Slint
      composition (fleet list/detail, per-lane status, comparison-report
      view), backed exclusively by Core.
- [x] Preserve current design system, keyboard/focus/accessibility and
      light/dark parity conventions.
- [x] Complete CLI vertical slice with human + JSON output matching Spec
      074/075/076/077 conventions.

**Verified:** Desktop "Model Fleet" route built from existing components and
Theme tokens, with accessible roles and labels, backed only by
`model_fleet_workspace` over `CliSession`. A real-Core-session test drives
every view-model function with real ONNX execution. The CLI covers every
fleet/lane/comparison operation in human and JSON output. No rendered
Desktop evidence exists (same honest residual as Specs 075-077; see
`DESKTOP_QUALIFICATION.md`). Qualified by exact-head CI run `35768404980` on head `547fb41` (6/6 required jobs; see `evidence/078-model-fleet-compare/EXACT_HEAD_QUALIFICATION.md`).

**Acceptance:** CLI/Desktop tests prove real Core-backed fleet/lane/
comparison data (no fake product data); no direct storage/model-runtime
access from CLI/Desktop.

## T078-08 — Qualification, review and closure

- [x] Run format, dependency-direction, focused contract/storage/Core/CLI/
      Desktop tests.
- [x] Run Clippy under current policy, full workspace tests (Spec 077's own
      suite unmodified and still green), cargo-deny/supply-chain gates.
- [x] Run migration/reopen/recovery suite, including pre-078 fixtures and
      backup/restore.
- [x] Run malformed/hostile-input, cross-lane-leakage, fabricated-
      comparison, and structural no-effect suites.
- [x] Run credential/log secret and content-leakage scans.
- [x] Capture rendered Desktop evidence where the CI/toolchain allows it;
      otherwise record the same honest residual pattern Spec 075/076/077
      recorded.
- [x] Record the deterministic exact-range scope record of the full PR diff
      (`EXACT_RANGE_REVIEW.md`). Amended 2026-09-22 by
      `docs/planning/FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`: the
      earlier Alibaba Open Code Review engine-review requirement (and its
      `OCR_ENGINE_LLM_ENDPOINT` blocker) is revoked. No external semantic
      reviewer is required and none is substituted. The earlier wording is
      kept in git history; the OCR delegate preview log remains historical.
- [x] Run exact-head required CI (6/6).
- [x] Create/update `evidence/078-model-fleet-compare/` with exact
      commands, platform, SHAs, fixtures and results.
- [x] Open/update the Spec 078 PR with real evidence.
- [x] Merge only after exact-head required gates pass.
- [x] Verify post-merge `main` CI.
- [x] Update canonical status/queue to `CLOSED_CANONICAL` only after the
      post-main evidence exists (two-PR closure-bookkeeping pattern,
      mirroring Spec 074/075/076/077).
- [x] Recompute the next eligible unit; do not implement 079+ without
      separate promotion.

**Progress (this session):** the six checked items are proven on code
head `547fb41` by required CI run `35768404980` (6/6 success) plus the local
gates in `evidence/078-model-fleet-compare/EXACT_HEAD_QUALIFICATION.md`.
Secret/content leakage: no new logging or print call in Core/storage, no
secret markers in added lines, and a fixed, content-free failure reason
(`SECURITY_ADVERSARIAL.md` T9). Log-capture leakage is still proven by
inspection only, as in Specs 076/077. Rendered Desktop evidence: the
honest residual is in `DESKTOP_QUALIFICATION.md`. The deterministic
exact-range scope record is `EXACT_RANGE_REVIEW.md`, and the missing
`CONTRACT_QUALIFICATION.md` and `NO_NETWORK_LOCAL_PATH.md` now exist. The
earlier "OCR engine review" open item was removed by the 2026-09-22
review-policy amendment. Still open: fresh exact-head CI on the final
candidate head, merge, post-main verification, closure bookkeeping.

**Closure (2026-09-22):** final head `bd06f6d` passed exact-head run
`35777530037` (6/6); PR #135 merged as `bf400e8`; post-merge main run
`35793707525` passed 6/6. See `evidence/078-model-fleet-compare/CLOSURE.md`
and `POST_MERGE_VERIFICATION.md`. Next eligible unit recomputed: Spec 079
Privacy Gate (hard dependency 074 + 077), promoted separately.

**Acceptance:** all Spec 078 acceptance criteria in
`docs/planning/SPEC_078_PROMOTION.md` proven on the exact candidate head,
required CI green, PR merged, post-merge main verification recorded.
