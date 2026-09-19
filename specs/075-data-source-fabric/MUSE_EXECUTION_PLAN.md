# Muse Execution Plan — Spec 075 Data Source Fabric + Data Workbench Foundation

## Mission

Execute **only Spec 075** from exact live repository truth until it is either:

1. `CLOSED_CANONICAL` with exact-head and post-merge proof; or
2. blocked by a genuinely external/non-repository gate, with every independent 075 task completed and an exact blocker packet recorded.

Do not implement Spec 076 or any later Research OS unit without a separate canonical promotion.

## Language

Use **English only** for repository/GitHub/technical content, code, comments, commands, commit messages, PR bodies, reports, evidence and agent prompts.

## Authority chain

Read in this order before material mutation:

1. live GitHub/repository truth;
2. `/AGENTS.md` and `/CURSOR.md`;
3. `docs/planning/IMPLEMENTATION_AUTHORITY.md`;
4. `docs/planning/SPEC_075_PROMOTION.md`;
5. live `docs/planning/START_HERE.md` and `BUILD_QUEUE.md`;
6. `docs/planning/CLINICAL_RESEARCH_OS_PLAN_INDEX.md`;
7. Research OS Master Implementation Contract, Repository Implementation Map, Decision Register, Verification Matrix and Implementer Instructions (the Clinical + Research Intelligence packet merged via PR #128);
8. this Spec 075 package: `spec.md`, `plan.md`, `tasks.md`, `contracts.md`, `migration.md`, `security.md`;
9. current implementation files and tests.

`SPEC_075_PROMOTION.md` is explicit founder promotion authority for Spec 075. If `BUILD_QUEUE.md` or another status document still contains a historical statement that no repository unit is promoted, reconcile that bookkeeping to the explicit promotion as part of T075-00. Do not reinterpret stale status text as revoking the founder promotion. Conversely, if a **newer** explicit founder/canonical authority supersedes 075, follow the newer authority and document the conflict.

## Immutable constraints

- Never force-push or rebase shared history.
- Never bypass required gates or weaken tests/branch governance.
- Never fabricate CI, test, benchmark, review, migration, runtime or completion evidence.
- Never use real PHI; use synthetic/permitted non-PHI fixtures only.
- Never touch MESC.
- Never introduce a mandatory cloud service or hidden cloud fallback.
- Never create a second authority/ID/provenance/audit/database/blob/outbox model when existing MedScale primitives suffice.
- Never allow data-source organization to copy/replace canonical patient/FHIR/source/document/evidence/model/Pack/Project authority.
- Never add Spec 076+ implementation under the label of "preparation".
- No wholesale donor copy. Grist/Baserow are UX/behavior donors, not embedded platforms; NocoDB and Teable core are reference-only.
- No new product runtime network path beyond the existing fail-closed Network Broker.
- No destructive data-source erasure in 075.
- No arbitrary Python/JavaScript/shell/R execution in transformations.
- No free-SQL product path; database adapters are read-only with explicit recorded query identity.
- No trusted remote code execution during dataset acquisition.
- Credential plaintext never enters manifests, receipts, snapshots, logs, backups, or error strings.

## Required first actions — T075-00

Before writing production code:

1. Verify remote/default branch, current `origin/main`, this branch, HEAD, merge base and worktree cleanliness.
2. Verify PR #128 is merged and the branch descends from its merge commit `ae0441918296c2d1510a71061249c7e55e65d760`, unless live main has legitimately advanced; if advanced, inspect/reconcile rather than reset/rebase shared history.
3. Verify Spec 074 `CLOSED_CANONICAL` evidence and that no competing/open Spec 075 implementation PR/branch has become canonical.
4. Read all authority/planning/spec files listed above.
5. Inspect exact live paths/types for IDs, headers, scopes, audit/evidence, storage schema/migrations (expect v3), backup/recovery, writer lock/transactions, Core command patterns, Network Broker, keystores, CLI routing and Desktop composition.
6. Run/inspect the baseline tests and live required checks applicable before mutation. The founder-workstation local C toolchain is destroyed; GitHub CI is the authoritative qualification path and local `cargo test`/`cargo clippy` linkage is unavailable — record this honestly, do not claim local runs that did not happen.
7. Create `evidence/075-data-source-fabric/LIVE_TRUTH.md` with exact SHAs and baseline state.
8. Reconcile queue/status bookkeeping if it has not yet incorporated the founder promotion.
9. Update `contracts.md` freeze block with exact live Rust paths/types and set `CONTRACT_FREEZE=FROZEN_FOR_075` only after the field-level design is consistent with current code.

Do not start T075-02 until contract freeze is real.

## Execution sequence

### 075-A — Contracts (T075-01)

Implement/freeze the smallest contract surface required by 075.

Mandatory principles:

- reuse `OpaqueId`, `ObjectHeader`, `DigestSha256`, realm/scope and current serialization conventions;
- do not misuse `MedicalTime` for repository metadata timestamps;
- mutable 075 state has explicit monotonic revision/precondition semantics reusing the current canonical pattern;
- DataSourceKind/format vocabularies stay bounded; unknown authority-bearing kinds fail closed;
- snapshots are immutable with exact-source revision bindings; refresh never edits in place;
- views are projections storing parameters, never copied row payloads;
- transformations are bounded deterministic ops with replayable lineage; no eval/exec path;
- `SourceLocator` is a validated scoped reference, never an ambient path escape;
- credential references are opaque; plaintext never persists.

Run focused contract tests. Record contract evidence.

### 075-B — Storage + migration (T075-02)

Extend existing encrypted storage only (v3 → v4).

Implement the smallest schema for:

```text
data_sources
data_snapshots
data_snapshot_parts
data_receipts
data_saved_views
data_transformations
```

Exact table names may follow repository conventions. Snapshot bulk bytes reuse existing blob/sealed-blob stores under content-digest identity.

Requirements:

- additive migration;
- no canonical-object copying;
- no cascade deletion into canonical targets or snapshot history;
- atomic snapshot materialization (metadata + parts + blobs + receipt);
- stale-revision conflicts checked transactionally on mutable rows;
- indexes justified by actual queries;
- representative populated pre-075 migration fixtures (including 074 vaults and tabular fixtures);
- close/reopen, repeat-open, crash-boundary, backup/restore evidence.

Update the freeze block in `migration.md` with exact storage version/path/index details.

### 075-C — Local file slice (T075-03)

Implement CSV/TSV candidate → validation/quarantine → schema discovery → immutable snapshot → Project attachment → reopen → CLI inspect.

Add further formats only in separately qualified slices with exact dependency/license/security review. Freeze hostile-input policies before widening formats.

### 075-D — Database adapter (T075-04)

Implement one evidence-selected read-only adapter first with opaque credentials, read-only enforcement, bounded discovery, recorded query identity, immutable snapshots, timeout/cancel, schema-change state, and restart durability. Qualify remaining engines in separate slices or record explicit deferral.

### 075-E — Remote datasets (T075-05)

Implement Hugging Face / Kaggle acquisition through the existing Network Broker with exact identity/version/files, resume, corruption detection, quarantine/admission, and no trusted remote code. If unqualifiable in this unit, record an explicit canonical deferral instead of silent omission.

### 075-F — Data Workbench (T075-06)

Add a native Slint Data Source / Data Workbench surface using current design authority, backed exclusively by Core:

- source/snapshot selection;
- typed columns;
- virtualized/paged rows;
- row detail;
- filter/sort/group;
- provenance/source inspector;
- honest empty/loading/error/denied/conflict/corrupt/partial/unavailable states;
- keyboard/focus/accessibility;
- light/dark parity.

No fake product data. No direct storage/driver access from UI.

### 075-G — Views (T075-07)

Implement saved Grid state plus Form, Gallery, Kanban, Calendar/time, and summary/aggregate views as projections over the same snapshot. Prove restart durability and projection semantics.

### 075-H — Transformations (T075-08)

Implement the frozen bounded deterministic op set. Durable output is a new immutable snapshot with exact replayable lineage. Prove explicit cast-failure semantics and the absence of any code-execution path.

### 075-I — Release foundation (T075-09)

Only if retained at freeze: versioned Dataset Card, immutable release manifest, split/group metadata, annotation schema identity/version, rights/privacy state, Project linkage. Otherwise record removal with rationale.

### 075-J — Qualification and closure (T075-10)

Run:

- format, dependency-direction, focused contract/storage/Core/CLI/Desktop tests;
- Clippy under current policy, full workspace tests, cargo-deny/supply-chain gates;
- migration/reopen/recovery suite;
- malformed/corrupt/disk-full/cancel/timeout suites;
- credential/log secret scans;
- no-network local path proof;
- rendered Desktop evidence (light/dark);
- exact-range review;
- exact-head CI.

Build the evidence packet under `evidence/075-data-source-fabric/`.

Open/update one Spec 075 PR. Merge normally only after exact-head required checks are all green and live governance permits merge. Then verify post-merge `main` CI before setting `CLOSED_CANONICAL`.

## Commit/PR discipline

Use bounded, reviewable commits aligned with slices. Suggested commit families, not mandatory exact strings:

```text
docs: promote Spec 075 data source fabric
docs: add Spec 075 execution specification
feat(contracts): add data source fabric contracts
feat(storage): persist data source fabric state
feat(core): add data source authority paths
feat(cli): add data source commands
feat(desktop): add data workbench
 test/docs: qualify spec 075 closure
```

Do not create one giant opaque commit if the live workflow allows cleaner slice commits.

The PR description must state:

- exact base/head;
- authorized Spec 075 scope;
- acceptance/evidence mapping;
- migrations/recovery (v3 → v4);
- new dependencies (exact justified list, or none);
- security/adversarial results;
- scale results;
- no-network local path result;
- known limitations;
- explicit statement that Spec 076+ is not implemented/authorized.

## Decision policy

Do not ask the founder for routine implementation decisions already resolved by the spec/planning contracts. Choose the smallest design consistent with current code and evidence.

Ask/stop only for a real authority problem that cannot be resolved from repository truth, such as:

- a newer canonical decision directly contradicts this promotion;
- implementation necessarily rewrites canonical IDs/authority;
- required functionality is impossible without entering Spec 076+ scope;
- a legal/gated credential/real-PHI action is required;
- a destructive migration is the only apparent option.

For technical uncertainty, investigate current code, tests, primary docs/source and implement the safest minimal option. Record material deviations.

## No-stop rule within authorized work

Do not stop after one task, one commit, one green CI run, or one PR update. Continue dependency-ordered through all independently executable T075 tasks. If one path is externally blocked, record the blocker and continue every unaffected 075 task.

## Terminal report

Do not claim success early.

A successful terminal report must include exact:

```text
SPEC_075_CLOSED_CANONICAL=true
MERGE_SHA=<sha>
POST_MERGE_MAIN_CI=<run/result>
MIGRATION_RECOVERY=PASS (v3 -> v4)
CORE_CLI_DESKTOP_PARITY=PASS
SECURITY_ADVERSARIAL=PASS
SCALE_EVIDENCE=<paths/results>
NO_NETWORK_LOCAL_PATH=PASS
SPEC_076_IMPLEMENTATION_AUTHORIZED=false (unless a newer separate promotion exists)
RESEARCH_OS_COMPLETE=false
REAL_PHI_AUTHORIZED=false
```

If closure is blocked, report instead:

```text
SPEC_075_CLOSED_CANONICAL=false
REPOSITORY_OWNED_WORK_REMAINING=<exact items or ZERO>
EXTERNAL_BLOCKERS=<exact evidence-bound blockers>
```

Never replace an unknown/pending gate with PASS.
