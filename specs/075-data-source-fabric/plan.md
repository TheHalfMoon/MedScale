# Plan — Spec 075 Data Source Fabric + Data Workbench Foundation

## Execution rule

Implement only the promoted Spec 075 scope. Follow `docs/planning/SPEC_075_PROMOTION.md`, the Research OS master contracts, repository implementation map, decision register and verification matrix. Live repository truth wins over stale assumptions.

## Phase 0 — Reverify and freeze

Before material code changes:

1. Fetch/update local `main` and verify it contains merge `ae0441918296c2d1510a71061249c7e55e65d760` or a proven later canonical descendant.
2. Confirm branch is `spec/075-data-source-fabric` and no unexpected local changes exist.
3. Re-read `AGENTS.md`, `CURSOR.md`, `IMPLEMENTATION_AUTHORITY.md`, `SPEC_075_PROMOTION.md`, `START_HERE.md`, `BUILD_QUEUE.md`, the Clinical + Research Intelligence plan index/master plan/repository map/decision register, and this Spec 075 package.
4. Verify no Spec 075 collision or newer founder authority supersedes this promotion.
5. Inventory exact live types/modules for `OpaqueId`, `ObjectHeader`, digests, audit/evidence, authority/session, storage migration (current version 3), backup/recovery, writer locking, Network Broker, keystores, CLI routing and Desktop composition.
6. Record baseline required CI/test state. Pre-existing failures are evidence, not permission to suppress tests.
7. Freeze the field-level contract/data model in `contracts.md` before 075-B.

## 075-A — Contracts and invariants (T075-01)

### Work

- Add only the contract module(s) required by Spec 075 (`medscale-contracts/src/data_sources/` or the repository-conventional shape).
- Reuse `OpaqueId`, `ObjectHeader`, `DigestSha256`, realm/scope, and existing serialization/error conventions.
- Define DataSourceManifest/Kind/Capability/Health, SourceSchema/field types, DataSnapshot/SnapshotPart identity, import/refresh/transformation receipts, SavedDataView, DataTransformation lineage.
- Define strict validation and explicit revision/precondition behavior for mutable source/view state.
- Define typed results/errors: `Denied`, `Unavailable`, `Unsupported`, `Stale`, `Partial`, `Corrupt`, `Conflict`, `Cancelled` without collapsing states that affect user action or safety.

### Gate

Focused contract tests pass, dependency direction remains valid, and no storage/UI/network dependency leaks into contracts.

## 075-B — Storage and migration (T075-02)

### Work

- Extend current encrypted metadata/vault migration mechanism (v3 → v4); do not create a second DB.
- Add bounded tables/indexes for data sources, snapshots, snapshot parts, source/snapshot receipts, saved views, transformations.
- Reuse existing blob/sealed-blob storage for larger snapshot bytes where semantics fit.
- Implement transactional repository/storage functions required by Core; storage does not decide authority.
- Add migration from representative pre-075 vaults (including populated 074 vaults).
- Add reopen, crash-boundary and backup/restore recovery tests.
- Preserve all existing IDs and pre-075 behavior; prove Project attachment compatibility.

### Gate

Migration is repeat-safe under current framework, crash semantics are unambiguous, old workflows still pass, and no cascade can delete referenced canonical objects.

## 075-C — Local file vertical slice (T075-03)

### Work

- Implement candidate source → validation/quarantine → schema discovery → snapshot materialization → Project attachment → reopen → CLI inspect, starting with CSV/TSV.
- Add JSON/JSONL, Parquet, Arrow IPC/Feather, XLSX only through separately qualified slices with exact dependency/license/security review per new dependency.
- Freeze malformed/partial/corrupt/encoding policies (invalid UTF-8, decompression limits, resource bombs) before adding formats beyond CSV/TSV.

### Gate

CSV/TSV file → immutable snapshot → Project → CLI inspect → restart/reopen is proven end to end with quarantine and explicit failure states.

## 075-D — Database read adapter (T075-04)

### Work

- Implement one evidence-selected native read-only database adapter first; qualify remaining candidate engines in separate slices.
- Prove opaque credential handling, no secret logging, read-only behavior, bounded discovery, explicit query identity, immutable snapshot binding, lost-connection/cancel behavior, schema-change state, and restart durability.

### Gate

At least one database engine path is qualified end to end; other engines are either qualified in later slices or explicitly deferred without blocking closure of the selected engine.

## 075-E — Remote dataset adapters (T075-05)

### Work

- Implement Hugging Face / Kaggle dataset acquisition through the existing Network Broker: exact identity/version/files, resumable download, hash/corruption detection, quarantine/admission, opaque credentials, no trusted remote code, no upload/competition submission.
- If remote adapters cannot be qualified within this unit, record an explicit canonical deferral in this package instead of silently omitting them.

### Gate

Remote foundation is qualified, or the deferral amendment is recorded and all other acceptance criteria still hold (local-first closure remains valid).

## 075-F — Native Data Workbench (T075-06)

### Work

- Add Data Source / Data Workbench navigation through current native Slint composition, backed exclusively by Core.
- Implement dataset/snapshot selection, typed columns, virtualized/paged rows, row detail, filter/sort/group, empty/loading/error/corrupt/partial states, provenance/source inspector, light/dark parity, keyboard/focus.
- Preserve current design system, keyboard/focus/accessibility and minimum-window behavior.

### Gate

Workbench tests/rendered evidence prove real Core-backed snapshot data and no fake product data.

## 075-G — Saved and alternate views (T075-07)

### Work

- Implement saved Grid state plus Form, Gallery, Kanban, Calendar/time, and summary/aggregate views as projections over the same canonical snapshot.
- Prove saved views survive restart and alternate views never duplicate authority data.

### Gate

View tests prove projection semantics (one dataset, many views) and restart durability.

## 075-H — Deterministic transformations (T075-08)

### Work

- Implement the frozen minimal deterministic transformation set (select/drop/rename, typed cast with explicit failure, filter, sort/materialize; safe computed expressions and bounded join only if the frozen contract includes them).
- Durable output is a new immutable DataSnapshot with exact recorded lineage.
- No arbitrary Python/JS/shell/R execution path is added.

### Gate

Transformation tests prove new-snapshot creation, exact lineage replay, and explicit cast/failure semantics.

## 075-I — Dataset release foundation (T075-09)

### Work

- Only if retained at contract freeze: versioned Dataset Card, immutable release manifest, split/group metadata, annotation schema identity/version, rights/privacy state, Project linkage.
- Full annotation/review/adjudication workflow stays out (later owning specs).

### Gate

Release-manifest tests prove immutability and Project linkage, or the package records that T075-09 was removed at freeze with rationale.

## 075-J — Qualification, review and closure (T075-10)

### Work

1. Run format, dependency-direction, focused contract/storage/Core/CLI/Desktop tests.
2. Run Clippy under current policy, full workspace tests, cargo-deny/supply-chain gates.
3. Run migration/reopen/recovery suite, including pre-075 fixtures and backup/restore.
4. Run malformed/corrupt-input, disk-full/cancel/timeout suites where applicable.
5. Run credential/log secret scans and the no-network local path proof.
6. Capture rendered Desktop evidence (light/dark) and perform exact-range review.
7. Run exact-head required CI.
8. Create/update `evidence/075-data-source-fabric/` with exact commands, platform, SHAs, fixtures and results.
9. Open/update the Spec 075 PR with real evidence.
10. Merge only after exact-head required gates pass and governance allows it.
11. Verify post-merge `main` CI.
12. Update canonical status/queue to `CLOSED_CANONICAL` only after the post-main evidence exists.

### Required evidence set under `evidence/075-data-source-fabric/`

```text
README.md (index + per-file rules)
LIVE_TRUTH.md (T075-00 branch/base/main/PR/CI state)
CONTRACT_QUALIFICATION.md (frozen fields to Rust paths + tests)
STORAGE_MIGRATION_RECOVERY.md (fixtures, before/after, backup/migrate/reopen/restore)
CORE_AUTHORITY_QUALIFICATION.md (mutation/query matrix + denial/conflict/scope)
CLI_QUALIFICATION.md (human + JSON, exits, Core parity)
DESKTOP_QUALIFICATION.md (native state/adapter/a11y + renders)
SECURITY_ADVERSARIAL.md (threat-gate results from security.md)
SCALE_MEASUREMENTS.md (fixtures/generators/hardware, no marketing budgets)
NO_NETWORK_LOCAL_PATH.md (egress-disabled proof for local sources)
EXACT_RANGE_REVIEW.md (authorized scope only, unexpected files, new dependencies)
EXACT_HEAD_QUALIFICATION.md (candidate head + live CI)
POST_MERGE_VERIFICATION.md (merge SHA + main checks)
CLOSURE.md (terminal truth block)
DATA_WORKBENCH_LIGHT.png / DATA_WORKBENCH_DARK.png (deterministic renders)
logs/ (CLI vertical-slice log, workbench UI transcript, scale log)
```

## Parallelism

Default is sequential T075-00 → T075-10 slices because contracts/storage/Core are shared boundaries. Parallelize only test/doc work that cannot race the same contract/schema/authority files.

## Stop conditions

Stop the current mutation, record evidence, and resolve before continuing if:

- live main contradicts the promoted base in a material way;
- another actor has already claimed Spec 075 with conflicting work;
- a required migration would rewrite existing canonical object IDs;
- an implementation requires a new authority/ID/provenance system;
- the proposed change requires scope from Spec 076+;
- a test exposes data loss/cross-scope leakage/authority bypass/secret leakage;
- required CI fails for the current change;
- real PHI or gated credentials/terms would be required.

Do not stop for ordinary implementation decisions already resolved by canonical planning; use the decision register, tests and safest minimal design.
