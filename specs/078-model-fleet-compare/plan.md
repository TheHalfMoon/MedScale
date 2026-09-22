# Plan — Spec 078 Model Fleet + Compare

## Execution rule

Implement only the promoted Spec 078 scope. Follow
`docs/planning/SPEC_078_PROMOTION.md`, the Research OS V2 master contracts/
repository map/decision register/verification matrix, and this Spec 078
package. Live repository truth wins over stale assumptions.

## Phase 0 — Reverify and freeze

Before material code changes:

1. Fetch/update local `main` and verify it contains merge
   `ff2e677294b1ea8704bbeefa605d129c1a99e8f2` or a proven later canonical
   descendant.
2. Confirm branch is `spec/078-model-fleet-compare` and no unexpected local
   changes exist.
3. Re-read `AGENTS.md`, `CURSOR.md`, `IMPLEMENTATION_AUTHORITY.md`,
   `SPEC_078_PROMOTION.md`, `START_HERE.md`, `BUILD_QUEUE.md`, the Research
   OS V2 execution roadmap/spec-implementation-contracts/decisions, and
   this Spec 078 package.
4. Verify no Spec 078 collision or newer founder authority supersedes this
   promotion.
5. Inventory exact live types/modules for `OpaqueId`, `ObjectHeader`,
   `DigestSha256`, `ProjectRevision`/`check_revision`/`initial_revision`,
   and every Spec 077 contract/Core surface this spec drives:
   `AgentIdentity`, `AgentCapabilityManifest`, `ContextManifest`,
   `AgentRun`/`AgentRunState`/`AgentTurn`, `ToolInvocation`/`ToolReceipt`,
   `RunReceipt`, `AgentProposal`, and Core's `MedAgent::{
   create_agent_run, start_agent_run, execute_agent_run, cancel_agent_run,
   get_agent_run, list_agent_runs}`.
6. Record baseline required CI/test state. Pre-existing failures are
   evidence, not permission to suppress tests.
7. Freeze the field-level contract/data model in `contracts.md` before
   078-B.

## 078-A — Contracts and invariants (T078-01)

### Work

- Add the contract module(s) required by Spec 078
  (`medscale-contracts/src/model_fleet.rs` or the repository-conventional
  shape).
- Reuse `OpaqueId`, `ObjectHeader`, `DigestSha256`, `ProjectRevision`, and
  Spec 077's frozen `medagent` contracts as-is (import them, never copy/
  redefine them).
- Define `AgentLane`/`LanePolicy`/`LaneTransform`/`AgentLaneStatus`,
  `FleetRun`/`FleetRunState`/`LaneRunRef`, `ComparisonRequest`/
  `ComparisonObservation`/`ComparisonObservationKind`/`ComparisonReport`.
- Define strict validation and explicit revision/precondition behavior for
  mutable rows; define the `FleetRunState` transition table explicitly
  (`Pending -> Running -> {Completed, PartiallyFailed, Failed}`, plus
  `Pending -> Cancelled` and `Running -> Cancelled`, no other edge).
- Reuse `AuthorityError` as-is; add no new variant unless T078-01 proves a
  genuine gap, recorded in `contracts.md`.
- Decide and record (frozen at T078-01) `LaneTransform`'s exact variant set
  (expected: zero variants, a closed-empty enum, unless a genuine minimal
  need appears) and whether an existing classification/data-class type
  exists to reuse for `ComparisonReport`. **Resolved at T078-01:** no such
  primitive exists anywhere in this repository; `classification` is dropped
  from `ComparisonReport`'s v1 shape rather than fabricated (`contracts.md`
  section 4 reconciliation note, `SPEC_078_PROMOTION.md` constraint 6
  amendment).

### Gate

Focused contract tests pass, dependency direction remains valid, no
storage/UI/network/model-runtime dependency leaks into contracts, no
modification to any Spec 077 contract file, and the freeze record in
`contracts.md` is filled in with exact Rust paths.

## 078-B — Storage and migration (T078-02)

### Work

- Extend the current encrypted metadata/vault migration mechanism
  (v6 -> v7); do not create a second DB.
- Add bounded tables/indexes for agent lanes (+ lane policy), fleet runs,
  lane-run-refs, comparison reports (`migration.md` section 2).
- Implement transactional repository/storage functions required by Core;
  storage does not decide authority.
- Add migration from representative pre-078 vaults (including populated
  074/075/076/077 vaults, with at least one real Spec 077
  `AgentIdentity`/`ContextManifest`/completed `AgentRun` to prove the
  078 layer binds to genuinely pre-existing objects).
- Add reopen, crash-boundary and backup/restore recovery tests.
- Preserve all existing IDs and pre-078 behavior; no modification to any
  Spec 077 (or earlier) table.

### Gate

Migration is repeat-safe under current framework, crash semantics are
unambiguous (a fleet's terminal state and its dependent rows commit
together or not at all), old workflows (including full Spec 077 workflows)
still pass, and no cascade can delete referenced canonical or Spec 077
objects.

## 078-C — AgentLane + LanePolicy (T078-03)

### Work

- Implement agent-lane creation bound to an existing `AgentIdentity` +
  `ContextManifest`, with explicit `LanePolicy` (subset-of-capability and
  subset-of-context validation, `security.md` T2), get/list, and retire.
- CLI inspect for agent lane.

### Gate

Agent-lane vertical slice proven end to end with reopen durability; a lane
policy naming an ungranted tool kind or an out-of-manifest artifact is
refused at creation.

## 078-D — FleetRun lifecycle (T078-04)

### Work

- Implement `FleetRun` create (`Pending`), per-lane dispatch (each lane's
  own real `AgentRun` created and started through Spec 077's unmodified
  `create_agent_run`/`start_agent_run`/`execute_agent_run`), and
  `FleetRunState` aggregation (`Running` -> `{Completed, PartiallyFailed,
  Failed}`).
- Implement fleet-level cancel: cancels every still-in-flight lane through
  Spec 077's own `cancel_agent_run`, never a second cancellation mechanism.
- Implement `LaneRunRef` as the append-only binding record between a
  `FleetRun`'s lane and the real `AgentRun` it dispatched.

### Gate

Full fleet state machine proven end to end, including partial failure (one
lane fails/is cancelled, another completes) and fleet-level cancel
mid-flight; no transition outside the frozen table is reachable; no lane's
in-flight run can read another lane's context/tools/results
(`security.md` T3).

## 078-E — Comparison engine (T078-05)

### Work

- Implement `ComparisonRequest` handling: refuse for a `Failed`/`Cancelled`
  `FleetRun` (no usable output), accept for `Completed`/`PartiallyFailed`.
- Implement the comparison computation over the fleet's completed lanes'
  real `AgentProposal`/`RunReceipt`/`ToolReceipt` content: agreement/
  disagreement, contradiction candidates, evidence/citation overlap,
  unsupported-claim candidates, abstention, schema validity, resource/
  runtime facts.
- Persist the result as an immutable `ComparisonReport`, with
  `excluded_lane_ids` explicitly naming any failed/cancelled lane left out.
- Enforce structurally: no numeric score/rank/winner field anywhere in the
  `ComparisonObservation`/`ComparisonReport` shape or its computation.

### Gate

A real `ComparisonReport` computed over at least two genuinely independent
lane runs (distinct `AgentRun.header.id`s, `security.md` T4) records
factual observations across every required kind with evidence references
into the real underlying `AgentProposal`/`RunReceipt`/`ToolReceipt` rows;
a partial-failure fleet's report explicitly names its excluded lane(s).

## 078-F — Fleet/comparison history (T078-06)

### Work

- Implement bounded, filterable fleet-run history read (by Project, by
  state) and comparison-report history read (by fleet run).
- Ensure `ComparisonReport` recomputation (if ever triggered) creates a new
  report row, never mutates a prior one.

### Gate

Every fleet run and comparison report is inspectable after the fact with
exact provenance back to the real Spec 077 rows it references.

## 078-G — Native Desktop and CLI parity (T078-07)

### Work

- Add a Model Fleet navigation route through current native Slint
  composition, backed exclusively by Core: at minimum a fleet list/detail
  view showing per-lane status and a comparison-report view.
- Preserve current design system, keyboard/focus/accessibility and
  light/dark parity conventions.
- Complete CLI vertical slice with human + JSON output matching Spec
  074/075/076/077 conventions.

### Gate

CLI/Desktop tests prove real Core-backed fleet/lane/comparison data (no
fake product data); no direct storage/model-runtime access from CLI/
Desktop.

## 078-H — Qualification, review and closure (T078-08)

### Work

1. Run format, dependency-direction, focused contract/storage/Core/CLI/
   Desktop tests.
2. Run Clippy under current policy, full workspace tests (including Spec
   077's own suite, unmodified and still green), cargo-deny/supply-chain
   gates.
3. Run migration/reopen/recovery suite, including pre-078 fixtures and
   backup/restore.
4. Run malformed/hostile-input, cross-lane-leakage, fabricated-comparison
   (`security.md` T4), and structural no-effect (fleet/comparison output ->
   clinical authority) suites.
5. Run credential/log secret and content-leakage scans.
6. Capture rendered Desktop evidence where the CI/toolchain allows it; if
   not, record the same honest residual pattern Spec 075/076/077 recorded
   rather than fabricating a render.
7. Perform exact-range review of the full PR diff using OpenCodeReview
   (delegation mode), matching the discipline established in Spec 076/077,
   unless the founder gives a different explicit instruction for this
   spec. No other review tool/method is accepted (Cubic/CodeRabbit/GitHub
   Copilot review/Claude self-review remain non-authoritative if they
   auto-post).
8. Run exact-head required CI.
9. Create/update `evidence/078-model-fleet-compare/` with exact commands,
   platform, SHAs, fixtures and results.
10. Open/update the Spec 078 PR with real evidence.
11. Merge only after exact-head required gates pass and governance allows
    it.
12. Verify post-merge `main` CI.
13. Update canonical status/queue to `CLOSED_CANONICAL` only after the
    post-main evidence exists.
14. Recompute the next eligible unit; do not implement 079+ without
    separate promotion.

### Required evidence set under `evidence/078-model-fleet-compare/`

```text
README.md (index + per-file rules)
LIVE_TRUTH.md (T078-00 branch/base/main/PR/CI state)
CONTRACT_QUALIFICATION.md (frozen fields to Rust paths + tests)
STORAGE_MIGRATION_RECOVERY.md (fixtures, before/after, backup/migrate/reopen/restore)
CORE_AUTHORITY_QUALIFICATION.md (mutation/query matrix + denial/conflict/scope)
CLI_QUALIFICATION.md (human + JSON, exits, Core parity)
DESKTOP_QUALIFICATION.md (native state/adapter/a11y + renders or honest residual)
SECURITY_ADVERSARIAL.md (threat-gate results from security.md, including the
  structural no-effect proof and the fabricated-comparison/T4 proof)
NO_NETWORK_LOCAL_PATH.md (egress-disabled proof)
LANE_PLURALITY_QUALIFICATION.md (exactly how the two-lane closure fixture
  proves genuine independence given the repository's real model inventory,
  with real run evidence -- distinct AgentRun ids, distinct RunReceipts)
EXACT_RANGE_REVIEW.md (authorized scope only, unexpected files, new dependencies)
EXACT_HEAD_QUALIFICATION.md (candidate head + live CI)
POST_MERGE_VERIFICATION.md (merge SHA + main checks)
CLOSURE.md (terminal truth block)
logs/ (CLI vertical-slice log, any UI transcript)
```

## Parallelism

Default is sequential T078-00 -> T078-08 slices because contracts/storage/
Core are shared boundaries. 078-F/G touch mostly disjoint concerns once
078-A/B/C/D/E land, so focused test/evidence writing may proceed in
parallel once the shared contract/storage/fleet-lifecycle foundation lands
-- but no slice may widen a frozen contract without amending `contracts.md`
first.

## Stop conditions

Stop the current mutation, record evidence, and resolve before continuing
if:

- live main contradicts the promoted base in a material way;
- another actor has already claimed Spec 078 with conflicting work;
- a required migration would rewrite existing canonical or Spec 077 object
  IDs;
- an implementation requires a new authority/ID/provenance/model-inference
  foundation, or any modification to a Spec 077 contract/storage table/Core
  function;
- the proposed change requires scope from Spec 079+ (privacy-gate
  transforms, browser/external delegate adapters, Hub/Compute);
- a test exposes a fleet/comparison path reaching `ClinicalAssertion`/
  `PromoteProposal`/`TransitionEffect`, a cross-lane permission-union/
  leakage, a fabricated/duplicated comparison, an authority bypass, or
  secret/content leakage;
- the repository's real model inventory genuinely cannot support even the
  same-Pack-different-policy two-lane closure fixture the promotion's
  lane-plurality decision describes -- this would be a blocking finding to
  resolve explicitly (scope amendment or founder decision), not a reason
  to silently build a second inference engine to manufacture diversity;
- required CI fails for the current change;
- real PHI or gated credentials/terms would be required.

Do not stop for ordinary implementation decisions already resolved by
canonical planning; use the decision register, tests and safest minimal
design.
