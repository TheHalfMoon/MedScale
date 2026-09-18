# Plan — Spec 074 Project + Artifact Graph Foundation

## Execution rule

Implement only the promoted Spec 074 scope. Follow `docs/planning/SPEC_074_PROMOTION.md`, the Research OS master contracts, repository implementation map, decision register and verification matrix. Live repository truth wins over stale assumptions.

## Phase 0 — Reverify and freeze

Before material code changes:

1. Fetch/update local `main` and verify it contains merge `a80c33307afc4577790282652e5b20911beb4bbe` or a proven later canonical descendant.
2. Confirm branch is `spec/074-project-artifact-graph-foundation` and no unexpected local changes exist.
3. Re-read `AGENTS.md`, `CURSOR.md`, `IMPLEMENTATION_AUTHORITY.md`, `START_HERE.md`, `BUILD_QUEUE.md`, the Research OS planning index/master contract/repository map/decision register/verification matrix, and this Spec 074 package.
4. Verify no Spec 074 collision or newer founder authority supersedes this promotion.
5. Inventory exact live types/modules for `OpaqueId`, `ObjectHeader`, audit/evidence, authority/session, storage migration, backup, writer locking, CLI routing and Desktop composition.
6. Record baseline required CI/test state. Pre-existing failures are evidence, not permission to suppress tests.
7. Freeze the field-level contract/data model in `contracts.md` before 074-B.

## 074-A — Contracts and invariants

### Work

- Add only the contract module(s) required by Spec 074.
- Reuse current identifier/header/provenance primitives.
- Define Project/Experiment lifecycles, ArtifactDescriptor/ProjectArtifactRef, typed ProjectGraphEdge/Predicate, ProjectContext/Summary and graph query contracts.
- Define strict validation and explicit revision/precondition behavior.
- Define typed results/errors without collapsing denied/conflict/corrupt states.

### Gate

Focused contract tests pass, dependency direction remains valid, and no storage/UI/network dependency leaks into contracts.

## 074-B — Storage and migration

### Work

- Extend current encrypted metadata/vault migration mechanism; do not create a second DB.
- Add bounded tables/indexes for Projects, Experiments, artifact refs and graph edges.
- Implement transactional repository/storage functions required by Core; storage does not decide authority.
- Add migration from representative pre-074 vaults.
- Add reopen, crash-boundary and backup/restore recovery tests.
- Preserve all existing IDs and pre-074 behavior.

### Gate

Migration is repeat-safe under current framework, crash semantics are unambiguous, old workflows still pass, and no cascade can delete referenced canonical objects.

## 074-C — Core authority

### Work

- Add typed Project/Experiment/reference/graph commands and queries using the current Core pattern.
- Enforce actor/session/scope, validation and expected-revision checks before storage mutation.
- Validate referenced canonical object identity/scope before attachment.
- Add bounded graph-neighbor query semantics; no unbounded recursive traversal.
- Integrate audit/receipts using existing mechanisms where applicable.
- Ensure ProjectContextResolve exposes only authorized references/summaries.

### Gate

Core-focused normal/denied/conflict/cross-scope/idempotent/corrupt tests pass. No direct Desktop/CLI storage path exists.

## 074-D — CLI vertical slice

### Work

- Add project and experiment command groups using Core.
- Add attach/detach and graph-neighbor inspection.
- Preserve current machine/human output conventions and exit semantics.
- Refactor the current large CLI entry file only if needed and only behavior-preservingly.

### Gate

CLI integration tests prove Core parity, JSON stability for new commands, and explicit error-state handling.

## 074-E — Native Desktop Projects workspace

### Work

- Add Projects navigation/workspace through current native Slint composition.
- Implement create/list/open/update/archive/restore-if-admitted.
- Show active Project context.
- Implement Experiment list/create/open/status.
- Show artifact references and typed relationship inspection with missing/stale states.
- Implement real empty/loading/error/denied/conflict/corrupt states.
- Preserve current design system, keyboard/focus/accessibility and minimum-window behavior.
- Do not build a decorative force-directed graph as a prerequisite.

### Gate

Desktop tests/rendered evidence prove real Core-backed state and no fake product data.

## 074-F — Qualification, review and closure

### Work

1. Run focused contract/storage/Core/CLI/Desktop tests.
2. Run migration/reopen/recovery suite.
3. Run security/adversarial reference and graph-bounds suite.
4. Run Personal and Lab synthetic scale fixtures and record measured results without unsupported budget claims.
5. Run all live required repository gates.
6. Perform exact-range review against canonical base/merge ancestry and ensure only authorized scope is present.
7. Create/update `evidence/074-project-artifact-graph-foundation/` with exact commands, platform, SHAs, fixtures and results.
8. Open/update the Spec 074 PR with real evidence.
9. Merge only after exact-head required gates pass and governance allows it.
10. Verify post-merge `main` CI.
11. Update canonical status/queue to `CLOSED_CANONICAL` only after the post-main evidence exists.

## Parallelism

Default is sequential 074-A -> B -> C -> D -> E -> F because contracts/storage/Core are shared boundaries. Parallelize only test/doc work that cannot race the same contract/schema/authority files.

## Stop conditions

Stop the current mutation, record evidence, and resolve before continuing if:

- live main contradicts the promoted base in a material way;
- another actor has already claimed Spec 074 with conflicting work;
- a required migration would rewrite existing canonical object IDs;
- an implementation requires a new authority/ID/provenance system;
- the proposed change requires scope from Spec 075+;
- a test exposes data loss/cross-scope leakage/authority bypass;
- required CI fails for the current change;
- real PHI or gated credentials/terms would be required.

Do not stop for ordinary implementation decisions already resolved by canonical planning; use the decision register, tests and safest minimal design.