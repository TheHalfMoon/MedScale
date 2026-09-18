# Tasks — Spec 074 Project + Artifact Graph Foundation

**Execution state:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`

Check a task only when its implementation, tests and required evidence are real on the branch. Do not pre-check future work.

## T074-00 — Live truth and baseline

- [x] Verify branch/base/main/PR state and repository cleanliness.
- [x] Read mandatory governance + Research OS + Spec 074 authority chain.
- [x] Inspect current contracts/Core/storage/CLI/Desktop ownership paths.
- [x] Confirm no live Spec 074 collision/superseding authority.
- [x] Record baseline focused/full gate state and any pre-existing failures.
- [x] Create initial evidence/live-truth record.

**Gate:** no code mutation before T074-00 is complete.

## T074-01 — Freeze contracts/data model

- [x] Freeze exact field-level Project/Experiment/artifact-ref/edge/context/summary contracts in `contracts.md`.
- [x] Reuse `OpaqueId`, `ObjectHeader` and existing provenance/audit primitives where semantically valid.
- [x] Freeze Project/Experiment lifecycle and archive semantics.
- [x] Freeze typed graph predicate vocabulary for 074; reject unsafe unknown authority predicates.
- [x] Freeze revision/precondition and idempotency behavior.
- [x] Add serialization/validation/invariant tests.

**Acceptance:** contract tests pass; no storage/network/UI dependency enters contracts.

## T074-02 — Storage schema + migration

- [x] Add Projects storage using current encrypted vault/SQLite metadata architecture.
- [x] Add Experiments storage.
- [x] Add project artifact-reference storage.
- [x] Add typed graph-edge storage and required indexes.
- [x] Implement atomic transactions and stale-revision conflict behavior.
- [x] Add forward migration from pre-074 representative vaults.
- [x] Add repeated-open/repeated-migration safety tests.
- [x] Add crash/reopen and backup/restore recovery tests.
- [x] Prove no reference/Project operation cascades deletion into canonical target objects.

**Acceptance:** storage + migration suite green and pre-074 IDs/behavior preserved.

## T074-03 — Core Project lifecycle

- [x] Implement Project create/get/list/update/archive (+ restore only if frozen contract includes it).
- [x] Enforce actor/session/scope and expected revision/preconditions.
- [x] Integrate existing audit/receipt patterns as applicable.
- [x] Add normal/invalid/denied/conflict/corrupt/idempotent tests.

**Acceptance:** all Project mutations flow through Core; no alternate authority path.

## T074-04 — Core Experiment lifecycle

- [x] Implement Experiment create/get/list/update/archive according to frozen state machine.
- [x] Enforce owning Project scope and revision/preconditions.
- [x] Prevent mutation through archived Project where prohibited.
- [x] Add state-transition/conflict/reopen tests.

**Acceptance:** Experiment lifecycle is deterministic and Project-scoped.

## T074-05 — Artifact references

- [x] Implement attach/detach reference commands.
- [x] Validate target canonical object identity and allowed scope before attach.
- [x] Never copy target authority payload merely to attach it.
- [x] Handle a target that later becomes missing/tombstoned with an explicit stale/missing reference state.
- [x] Add duplicate/idempotency/cross-scope/missing-target tests.

**Acceptance:** attachments organize existing objects without identity or authority duplication.

## T074-06 — Typed Project Graph

- [x] Implement edge create/remove/tombstone according to frozen contract.
- [x] Validate edge subject/object/project scope and predicate.
- [x] Add bounded graph-neighbor query with pagination/limit.
- [x] Ensure inferred/search/model/vector relationships cannot enter canonical graph implicitly.
- [x] Add duplicate edge, cross-project, pathological traversal and stale-write tests.

**Acceptance:** graph is explicit, typed, bounded and restart-durable.

## T074-07 — Project context + summaries

- [x] Implement `ProjectContextResolve` using authorized references only.
- [x] Implement Project summary/count query without leaking inaccessible content.
- [x] Add missing/stale/corrupt reference behavior.
- [x] Add authorization/scope tests.

**Acceptance:** future consumers can resolve a bounded Project context without ambient vault access.

## T074-08 — CLI vertical slice

- [x] Add Project create/list/show/update/archive/context commands.
- [x] Add Experiment commands.
- [x] Add attach/detach commands.
- [x] Add graph-neighbor inspection.
- [x] Preserve human/JSON conventions and stable error/exit behavior.
- [x] Refactor CLI routing only if required and behavior-preserving.
- [x] Add integration tests.

**Acceptance:** CLI exposes no policy/storage logic outside Core.

## T074-09 — Native Desktop Projects workspace

- [x] Add Projects navigation/surface using current Slint design authority.
- [x] Add real create/list/open/update/archive flows.
- [x] Show active Project context visibly.
- [x] Add Experiment surface.
- [x] Add artifact reference list and typed relationship inspector.
- [x] Show missing/stale/denied/conflict/corrupt/empty/loading states honestly.
- [x] Preserve keyboard/focus/accessibility/minimum-window behavior.
- [x] Add state/adapter/accessibility tests and rendered evidence if current UI qualification practice requires it.

**Acceptance:** Desktop is Core-backed and contains no fake Project data.

## T074-10 — Compatibility + adversarial qualification

- [x] Run pre-074 workflow regression suite.
- [x] Test stale revisions, duplicate requests, cross-scope refs, invalid predicates and corrupt rows.
- [x] Test graph query limits/resource-abuse cases.
- [x] Test crash boundaries and recovery.
- [x] Confirm no new runtime network path.
- [x] Confirm no real-PHI fixtures/logging.

**Acceptance:** no authority/privacy/compatibility regression owned by 074 remains open.

## T074-11 — Scale evidence

- [x] Create Personal fixture: >=10 Projects and >=1,000 artifact refs total.
- [x] Create Lab fixture: >=100 Projects OR >=100,000 refs/edges in one Project.
- [x] Measure Project list/open, mutation, bounded neighbor query, reopen/migration and storage growth.
- [ ] Record host/workload/method/results; make no unsupported universal budget claim.

**Status 2026-09-18:** harness + both fixtures exist and are env-gated
(`MEDSCALE_074_FULL_SCALE=1`; smoke shapes green in every CI run);
lab-full correctness passed; reopen measured (1.31 ms in CI). Full-fixture
TIMINGS cannot fit bounded CI time (residual O(n) persist per source op is
frozen pre-074 behavior; a CI perf-step attempt timed out with zero
failures and was reverted). Full timings remain local-only evidence for
after the workstation toolchain recovery. No budget claimed at any point.

**Acceptance:** evidence exists and no correctness failure appears at declared fixture scale.

## T074-12 — Exact-head closure

- [x] Run all focused tests.
- [x] Run current full workspace/required gates.
- [x] Run formatting, dependency-direction, Clippy and cargo-deny according to live CI.
- [x] Review exact diff against authorized 074 scope.
- [x] Update `evidence/074-project-artifact-graph-foundation/` with exact SHA/commands/results/limitations.
- [x] Ensure PR body maps acceptance criteria to evidence.
- [x] Merge normally only when exact-head required checks pass.
- [x] Verify post-merge main CI.
- [ ] Update canonical queue/status to `CLOSED_CANONICAL` only after post-main proof.
- [ ] Recompute next eligible Research OS unit; do not implement it without promotion.

**Terminal truth:**

```text
SPEC_074_CLOSED_CANONICAL = TRUE only after every T074-00..12 applicable item is proven.
SPEC_075_IMPLEMENTATION_AUTHORIZED = FALSE unless separately promoted.
RESEARCH_OS_COMPLETE = FALSE.
REAL_PHI_AUTHORIZED = FALSE.
```