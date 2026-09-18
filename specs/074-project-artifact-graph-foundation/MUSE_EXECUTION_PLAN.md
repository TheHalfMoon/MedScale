# Muse Execution Plan — Spec 074 Project + Artifact Graph Foundation

## Mission

Execute **only Spec 074** from exact live repository truth until it is either:

1. `CLOSED_CANONICAL` with exact-head and post-merge proof; or
2. blocked by a genuinely external/non-repository gate, with every independent 074 task completed and an exact blocker packet recorded.

Do not implement Spec 075 or any later Research OS unit without a separate canonical promotion.

## Language

Use **English only** for repository/GitHub/technical content, code, comments, commands, commit messages, PR bodies, reports, evidence and agent prompts.

## Authority chain

Read in this order before material mutation:

1. live GitHub/repository truth;
2. `/AGENTS.md` and `/CURSOR.md`;
3. `docs/planning/IMPLEMENTATION_AUTHORITY.md`;
4. `docs/planning/SPEC_074_PROMOTION.md`;
5. live `docs/planning/START_HERE.md` and `BUILD_QUEUE.md`;
6. `docs/planning/RESEARCH_OS_PLAN_INDEX.md`;
7. Research OS Master Implementation Contract, Repository Implementation Map, Decision Resolution Register, Verification Matrix and Implementer Instructions;
8. this Spec 074 package: `spec.md`, `plan.md`, `tasks.md`, `contracts.md`, `migration.md`, `security.md`;
9. current implementation files and tests.

`SPEC_074_PROMOTION.md` is explicit founder promotion authority for Spec 074. If `BUILD_QUEUE.md` or another status document still contains a historical statement that no repository unit is promoted, reconcile that bookkeeping to the explicit promotion as part of T074-00. Do not reinterpret stale status text as revoking the founder promotion. Conversely, if a **newer** explicit founder/canonical authority supersedes 074, follow the newer authority and document the conflict.

## Immutable constraints

- Never force-push or rebase shared history.
- Never bypass required gates or weaken tests/branch governance.
- Never fabricate CI, test, benchmark, review, migration, runtime or completion evidence.
- Never use real PHI.
- Never touch MESC.
- Never introduce a mandatory cloud service.
- Never create a second authority/ID/provenance/audit model when existing MedScale primitives suffice.
- Never allow Project organization to copy/replace canonical patient/FHIR/source/document/evidence/model/Pack authority.
- Never add Spec 075+ implementation under the label of “preparation”.
- No wholesale donor copy. Spec 074 needs no donor by default.
- No new product runtime network path.
- No destructive Project deletion in 074.

## Required first actions — T074-00

Before writing production code:

1. Verify remote/default branch, current `origin/main`, this branch, HEAD, merge base and worktree cleanliness.
2. Verify PR #121 is merged and the branch descends from its merge commit `a80c33307afc4577790282652e5b20911beb4bbe`, unless live main has legitimately advanced; if advanced, inspect/reconcile rather than reset/rebase shared history.
3. Verify no competing/open Spec 074 implementation PR/branch has become canonical.
4. Read all authority/planning/spec files listed above.
5. Inspect exact live paths/types for IDs, headers, scopes, audit/evidence, storage schema/migrations, backup/recovery, writer lock/transactions, Core command patterns, CLI routing and Desktop composition.
6. Run/inspect the baseline tests and live required checks applicable before mutation.
7. Create `evidence/074-project-artifact-graph-foundation/LIVE_TRUTH.md` with exact SHAs and baseline state.
8. Reconcile queue/status bookkeeping if it has not yet incorporated the founder promotion.
9. Update `contracts.md` freeze block with exact live Rust paths/types and set `CONTRACT_FREEZE=FROZEN_FOR_074` only after the field-level design is consistent with current code.

Do not start T074-02 until contract freeze is real.

## Execution sequence

### 074-A — Contracts

Implement/freeze the smallest contract surface required by 074.

Mandatory principles:

- reuse `OpaqueId`, `ObjectHeader`, `DigestSha256` and current scope/serialization conventions;
- do not misuse `MedicalTime` for repository metadata timestamps;
- mutable state has explicit monotonic revision/precondition semantics, reusing a current canonical type if present;
- ProjectGraph predicates remain a bounded organizational/workflow vocabulary;
- no clinical-truth/model-inferred predicate admission;
- Artifact references bind canonical identity and the strongest version/digest binding actually provided by the owner;
- `ProjectContext` is bounded/inspectable, never an ambient vault handle.

Run focused contract tests. Record contract evidence.

### 074-B — Storage + migration

Extend existing encrypted storage only.

Implement the smallest schema for:

```text
projects
experiments
project_artifact_refs
project_graph_edges
```

Exact table names may follow repository conventions.

Requirements:

- additive migration;
- no canonical-object copying;
- no cascade deletion into target objects;
- atomic mutations;
- stale-revision conflicts checked transactionally;
- indexes justified by actual queries;
- representative populated pre-074 migration fixtures;
- close/reopen, repeat-open, crash-boundary, backup/restore evidence.

Update the freeze block in `migration.md` with exact storage version/path/index details.

### 074-C — Core authority

Implement all Project/Experiment/ref/edge mutations and queries through Core.

Every mutation follows:

```text
request
 -> actor/session/scope
 -> capability/authority check
 -> input + expected revision validation
 -> canonical target/ref validation
 -> transaction/effect
 -> audit/receipt/result
```

No Desktop/CLI direct storage path.

Implement bounded graph-neighbor queries only. No arbitrary recursive graph engine in 074.

### 074-D — CLI

Expose the Spec 074 vertical slice through current CLI conventions.

Use Core only. Preserve typed error distinctions and JSON/human output conventions. Refactor the current large CLI entry point only if necessary for maintainability and without changing unrelated behavior.

### 074-E — Native Desktop

Add a native Slint Projects workspace using current design authority.

Must be real Core-backed behavior, including:

- Projects list/create/open/update/archive;
- active Project context;
- Experiments;
- artifact refs;
- typed relationship inspection;
- honest missing/stale/denied/conflict/corrupt/empty/loading states;
- keyboard/focus/accessibility.

Do not make a decorative graph visualization a closure dependency.

### 074-F — Qualification and closure

Run:

- all focused tests;
- migration/recovery suite;
- adversarial/security suite;
- Personal and Lab scale fixtures;
- current full workspace and required CI gates;
- exact-range review;
- exact-head CI.

Build the evidence packet under `evidence/074-project-artifact-graph-foundation/`.

Open/update one Spec 074 PR. Merge normally only after exact-head required checks are all green and live governance permits merge. Then verify post-merge `main` CI before setting `CLOSED_CANONICAL`.

## Commit/PR discipline

Use bounded, reviewable commits aligned with slices. Suggested commit families, not mandatory exact strings:

```text
feat(contracts): add project artifact graph contracts
feat(storage): persist project artifact graph state
feat(core): add project graph authority paths
feat(cli): add project workspace commands
feat(desktop): add projects workspace
 test/docs: qualify Spec 074 closure
```

Do not create one giant opaque commit if the live workflow allows cleaner slice commits.

The PR description must state:

- exact base/head;
- authorized Spec 074 scope;
- acceptance/evidence mapping;
- migrations/recovery;
- new dependencies (expected none, or justified exact list);
- security/adversarial results;
- scale results;
- known limitations;
- explicit statement that Spec 075+ is not implemented/authorized.

## Decision policy

Do not ask the founder for routine implementation decisions already resolved by the spec/planning contracts. Choose the smallest design consistent with current code and evidence.

Ask/stop only for a real authority problem that cannot be resolved from repository truth, such as:

- a newer canonical decision directly contradicts this promotion;
- implementation necessarily rewrites canonical IDs/authority;
- required functionality is impossible without entering Spec 075+ scope;
- a legal/gated credential/real-PHI action is required;
- a destructive migration is the only apparent option.

For technical uncertainty, investigate current code, tests, primary docs/source and implement the safest minimal option. Record material deviations.

## No-stop rule within authorized work

Do not stop after one task, one commit, one green local test, or one PR update. Continue dependency-ordered through all independently executable T074 tasks. If one path is externally blocked, record the blocker and continue every unaffected 074 task.

## Terminal report

Do not claim success early.

A successful terminal report must include exact:

```text
SPEC_074_CLOSED_CANONICAL=true
MERGE_SHA=<sha>
POST_MERGE_MAIN_CI=<run/result>
MIGRATION_RECOVERY=PASS
CORE_CLI_DESKTOP_PARITY=PASS
SECURITY_ADVERSARIAL=PASS
SCALE_EVIDENCE=<paths/results>
SPEC_075_IMPLEMENTATION_AUTHORIZED=false (unless a newer separate promotion exists)
RESEARCH_OS_COMPLETE=false
REAL_PHI_AUTHORIZED=false
```

If closure is blocked, report instead:

```text
SPEC_074_CLOSED_CANONICAL=false
REPOSITORY_OWNED_WORK_REMAINING=<exact items or ZERO>
EXTERNAL_BLOCKERS=<exact evidence-bound blockers>
```

Never replace an unknown/pending gate with PASS.