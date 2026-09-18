# Tasks — Spec 074 Project + Artifact Graph Foundation

**Execution state:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`

Check a task only when its implementation, tests and required evidence are real on the branch. Do not pre-check future work.

## T074-00 — Live truth and baseline

- [ ] Verify branch/base/main/PR state and repository cleanliness.
- [ ] Read mandatory governance + Research OS + Spec 074 authority chain.
- [ ] Inspect current contracts/Core/storage/CLI/Desktop ownership paths.
- [ ] Confirm no live Spec 074 collision/superseding authority.
- [ ] Record baseline focused/full gate state and any pre-existing failures.
- [ ] Create initial evidence/live-truth record.

**Gate:** no code mutation before T074-00 is complete.

## T074-01 — Freeze contracts/data model

- [ ] Freeze exact field-level Project/Experiment/artifact-ref/edge/context/summary contracts in `contracts.md`.
- [ ] Reuse `OpaqueId`, `ObjectHeader` and existing provenance/audit primitives where semantically valid.
- [ ] Freeze Project/Experiment lifecycle and archive semantics.
- [ ] Freeze typed graph predicate vocabulary for 074; reject unsafe unknown authority predicates.
- [ ] Freeze revision/precondition and idempotency behavior.
- [ ] Add serialization/validation/invariant tests.

**Acceptance:** contract tests pass; no storage/network/UI dependency enters contracts.

## T074-02 — Storage schema + migration

- [ ] Add Projects storage using current encrypted vault/SQLite metadata architecture.
- [ ] Add Experiments storage.
- [ ] Add project artifact-reference storage.
- [ ] Add typed graph-edge storage and required indexes.
- [ ] Implement atomic transactions and stale-revision conflict behavior.
- [ ] Add forward migration from pre-074 representative vaults.
- [ ] Add repeated-open/repeated-migration safety tests.
- [ ] Add crash/reopen and backup/restore recovery tests.
- [ ] Prove no reference/Project operation cascades deletion into canonical target objects.

**Acceptance:** storage + migration suite green and pre-074 IDs/behavior preserved.

## T074-03 — Core Project lifecycle

- [ ] Implement Project create/get/list/update/archive (+ restore only if frozen contract includes it).
- [ ] Enforce actor/session/scope and expected revision/preconditions.
- [ ] Integrate existing audit/receipt patterns as applicable.
- [ ] Add normal/invalid/denied/conflict/corrupt/idempotent tests.

**Acceptance:** all Project mutations flow through Core; no alternate authority path.

## T074-04 — Core Experiment lifecycle

- [ ] Implement Experiment create/get/list/update/archive according to frozen state machine.
- [ ] Enforce owning Project scope and revision/preconditions.
- [ ] Prevent mutation through archived Project where prohibited.
- [ ] Add state-transition/conflict/reopen tests.

**Acceptance:** Experiment lifecycle is deterministic and Project-scoped.

## T074-05 — Artifact references

- [ ] Implement attach/detach reference commands.
- [ ] Validate target canonical object identity and allowed scope before attach.
- [ ] Never copy target authority payload merely to attach it.
- [ ] Handle a target that later becomes missing/tombstoned with an explicit stale/missing reference state.
- [ ] Add duplicate/idempotency/cross-scope/missing-target tests.

**Acceptance:** attachments organize existing objects without identity or authority duplication.

## T074-06 — Typed Project Graph

- [ ] Implement edge create/remove/tombstone according to frozen contract.
- [ ] Validate edge subject/object/project scope and predicate.
- [ ] Add bounded graph-neighbor query with pagination/limit.
- [ ] Ensure inferred/search/model/vector relationships cannot enter canonical graph implicitly.
- [ ] Add duplicate edge, cross-project, pathological traversal and stale-write tests.

**Acceptance:** graph is explicit, typed, bounded and restart-durable.

## T074-07 — Project context + summaries

- [ ] Implement `ProjectContextResolve` using authorized references only.
- [ ] Implement Project summary/count query without leaking inaccessible content.
- [ ] Add missing/stale/corrupt reference behavior.
- [ ] Add authorization/scope tests.

**Acceptance:** future consumers can resolve a bounded Project context without ambient vault access.

## T074-08 — CLI vertical slice

- [ ] Add Project create/list/show/update/archive/context commands.
- [ ] Add Experiment commands.
- [ ] Add attach/detach commands.
- [ ] Add graph-neighbor inspection.
- [ ] Preserve human/JSON conventions and stable error/exit behavior.
- [ ] Refactor CLI routing only if required and behavior-preserving.
- [ ] Add integration tests.

**Acceptance:** CLI exposes no policy/storage logic outside Core.

## T074-09 — Native Desktop Projects workspace

- [ ] Add Projects navigation/surface using current Slint design authority.
- [ ] Add real create/list/open/update/archive flows.
- [ ] Show active Project context visibly.
- [ ] Add Experiment surface.
- [ ] Add artifact reference list and typed relationship inspector.
- [ ] Show missing/stale/denied/conflict/corrupt/empty/loading states honestly.
- [ ] Preserve keyboard/focus/accessibility/minimum-window behavior.
- [ ] Add state/adapter/accessibility tests and rendered evidence if current UI qualification practice requires it.

**Acceptance:** Desktop is Core-backed and contains no fake Project data.

## T074-10 — Compatibility + adversarial qualification

- [ ] Run pre-074 workflow regression suite.
- [ ] Test stale revisions, duplicate requests, cross-scope refs, invalid predicates and corrupt rows.
- [ ] Test graph query limits/resource-abuse cases.
- [ ] Test crash boundaries and recovery.
- [ ] Confirm no new runtime network path.
- [ ] Confirm no real-PHI fixtures/logging.

**Acceptance:** no authority/privacy/compatibility regression owned by 074 remains open.

## T074-11 — Scale evidence

- [ ] Create Personal fixture: >=10 Projects and >=1,000 artifact refs total.
- [ ] Create Lab fixture: >=100 Projects OR >=100,000 refs/edges in one Project.
- [ ] Measure Project list/open, mutation, bounded neighbor query, reopen/migration and storage growth.
- [ ] Record host/workload/method/results; make no unsupported universal budget claim.

**Acceptance:** evidence exists and no correctness failure appears at declared fixture scale.

## T074-12 — Exact-head closure

- [ ] Run all focused tests.
- [ ] Run current full workspace/required gates.
- [ ] Run formatting, dependency-direction, Clippy and cargo-deny according to live CI.
- [ ] Review exact diff against authorized 074 scope.
- [ ] Update `evidence/074-project-artifact-graph-foundation/` with exact SHA/commands/results/limitations.
- [ ] Ensure PR body maps acceptance criteria to evidence.
- [ ] Merge normally only when exact-head required checks pass.
- [ ] Verify post-merge main CI.
- [ ] Update canonical queue/status to `CLOSED_CANONICAL` only after post-main proof.
- [ ] Recompute next eligible Research OS unit; do not implement it without promotion.

**Terminal truth:**

```text
SPEC_074_CLOSED_CANONICAL = TRUE only after every T074-00..12 applicable item is proven.
SPEC_075_IMPLEMENTATION_AUTHORIZED = FALSE unless separately promoted.
RESEARCH_OS_COMPLETE = FALSE.
REAL_PHI_AUTHORIZED = FALSE.
```