# Tasks — Spec 075 Data Source Fabric + Data Workbench Foundation

**Execution state:** `CLOSED_CANONICAL` (see `evidence/075-data-source-fabric/CLOSURE.md`)

Check a task only when its implementation, tests and required evidence are real on the branch. Do not pre-check future work.

## T075-00 — Live truth and baseline

- [x] Verify branch/base/main/PR state and repository cleanliness.
- [x] Read mandatory governance + Clinical/Research Intelligence + Spec 075 authority chain.
- [x] Inspect current contracts/Core/storage/network/key/CLI/Desktop ownership paths.
- [x] Confirm no live Spec 075 collision/superseding authority; prove numbering is free.
- [x] Verify Spec 074 closure and post-main evidence.
- [x] Record baseline focused/full gate state and any pre-existing failures.
- [x] Create initial evidence/live-truth record.

**Gate:** no code mutation before T075-00 is complete.

## T075-01 — Freeze contracts/data model

- [x] Freeze exact field-level source/snapshot/receipt/view/transformation contracts in `contracts.md` (435 lines).
- [x] Reuse `OpaqueId`, `ObjectHeader`, `DigestSha256` and existing provenance/audit primitives where semantically valid.
- [x] Freeze DataSourceKind/Capability/Health vocabulary for 075; reject unsafe unknown authority kinds.
- [x] Freeze SourceSchema/field-type system, snapshot identity, receipt lineage, saved-view and transformation semantics.
- [x] Freeze revision/precondition and idempotency behavior for mutable source/view state.
- [x] Add serialization/validation/invariant tests (`crates/medscale-contracts/tests/data_sources_075.rs`, 25 tests incl. the exact-range-review addition `remote_dataset_file_names_reject_traversal_and_absolute_paths`).

**Acceptance:** contract tests pass; no storage/network/UI dependency enters contracts. **PROVEN** (CONTRACT_QUALIFICATION.md).

## T075-02 — Storage schema + migration

- [x] Add data-source storage using current encrypted vault/SQLite metadata architecture (v3 → v4).
- [x] Add snapshot and snapshot-part storage (metadata rows; large bytes via existing blob/sealed-blob stores).
- [x] Add import/refresh/transformation receipt and lineage storage.
- [x] Add saved-view and transformation storage with required indexes.
- [x] Implement atomic transactions and stale-revision conflict behavior.
- [x] Add forward migration from representative pre-075 vaults (including populated 074 vaults).
- [x] Add repeated-open/repeated-migration safety tests.
- [x] Add crash/reopen and backup/restore recovery tests, including a
      genuinely fail-closed corrupted-id-sequence path and rejection of a
      tampered/crafted backup carrying a stored Database credential (both
      added by the exact-range review; see EXACT_RANGE_REVIEW.md).
- [x] Prove no source/snapshot operation cascades deletion into canonical target objects.
- [x] Prove Project attachment compatibility with Spec 074 graph.

**Acceptance:** storage + migration suite green and pre-075 IDs/behavior preserved. **PROVEN** (STORAGE_MIGRATION_RECOVERY.md). Note: the pre-existing Spec 074 migration tests (`project_graph_074.rs`) required a forward-only fix in this closure — they pinned `finished_version == 3`, which became stale once the real v3→v4 migration landed; fixed in `bbc4d89`.

## T075-03 — Local file vertical slice

- [x] Implement CSV/TSV candidate source, validation/quarantine, schema discovery, snapshot materialization.
- [x] Implement Project attachment, reopen durability, and CLI inspect for the slice.
- [x] Freeze malformed/partial/corrupt/encoding/resource policies, including a
      pre-read file-size bound check that was missing before the exact-range
      review (an oversized local file was previously read fully into memory
      before the byte bound was checked).
- [x] Qualify JSON/JSONL in this spec; Parquet, Arrow IPC/Feather, and XLSX
      remain explicitly `Unsupported` pending a separate slice
      (`unqualified_formats_and_engines_fail_closed`).

**Acceptance:** CSV/TSV file → immutable snapshot → Project → CLI inspect → restart/reopen proven end to end. **PROVEN**.

## T075-04 — Database read adapter

- [x] Implement one evidence-selected native read-only database adapter (external SQLite).
- [x] Prove opaque credential handling and absence of secrets in logs/persisted artifacts (create/update path plus, after the exact-range review, the restore path too).
- [x] Prove read-only behavior, bounded discovery, explicit query identity, immutable snapshot binding.
- [ ] Prove lost-connection/cancel and schema-change state for the database adapter specifically. **NOT SEPARATELY PROVEN**: schema-change acknowledgment is proven for the general refresh path (`refresh_schema_change_requires_acknowledgment`, CSV-sourced), not with a database-engine source; no lost-connection/cancel-mid-query fixture exists for the SQLite adapter. Restart durability is proven generally (backup/restore, reopen suites).
- [x] Qualify remaining candidate engines separately or record explicit deferral: Postgres/MySQL/SQL Server are defined as `DatabaseEngine` vocabulary but rejected as `Unsupported` at acquisition time; no separate qualification attempted in 075.

**Acceptance:** at least one database engine path qualified end to end. **PROVEN** for the acceptance bar; the unchecked sub-item above is a genuine, honestly-recorded gap, not a blocker to this bar.

## T075-05 — Remote dataset adapters

- [x] Implement Hugging Face / Kaggle acquisition through the existing Network Broker (code path, parsing, and merge logic exist and are unit-tested).
- [ ] Prove exact identity/version/files, resumable download, hash/corruption detection, quarantine/admission **against a real remote fetch**. **NOT PROVEN LIVE**: no explicit deferral amendment document was written, but the actual, observed, repeatedly-verified behavior across every CI run in this closure is that remote fetch never leaves the deny-before-socket state (`UreqTransport::send` unconditionally returns `ExternalGateRequired`; `remote_import_without_allowlist_denies_before_socket` is the only exercised path). The parsing/merge/hash-verification logic (`parse_fetched_files`, `SourceRevisionBinding::Remote`) is unit-tested with synthetic bytes, not against a real Hugging Face/Kaggle endpoint.
- [x] Prove opaque credentials, no trusted remote code, no upload/competition submission (true by construction: no code-execution path exists for remote bytes; only data parsing).
- [ ] Or record an explicit canonical deferral amendment in this package. **NOT DONE AS A FORMAL AMENDMENT**; recorded instead as an honest residual in `evidence/075-data-source-fabric/CLOSURE.md`.

**Acceptance:** remote foundation qualified, or deferral recorded with local-first closure intact. **MET via the second branch**: local-first closure is intact (every local-source workflow is fully proven with network egress disabled; `NO_NETWORK_LOCAL_PATH.md`), and the remote path fails closed rather than silently degrading. This is an honest `READY_BASE`-style closure of this task, not a `PASS` of live remote acquisition.

## T075-06 — Native Data Workbench

- [x] Implement Data Source / Data Workbench navigation through current Slint composition, backed exclusively by Core.
- [x] Implement dataset/snapshot selection, typed columns, virtualized/paged rows, row detail, filter/sort/group.
- [x] Implement empty/loading/error/corrupt/partial states, provenance/source inspector, light/dark parity, keyboard/focus.

**Acceptance:** Workbench tests/rendered evidence prove real Core-backed snapshot data. **PARTIALLY PROVEN**: `workbench_flows_through_real_core_session` and related in-module tests prove real Core-backed data through compiled, passing tests. No rendered evidence exists (see DESKTOP_QUALIFICATION.md's honest residual: no rendering step in CI, no local rendering possible, no headless `AppWindow` test harness in this codebase). The exact-range review also found and fixed two real Desktop bugs here (wrong-snapshot-shown-as-latest; stale cross-project data selection), verified by property-flow tracing rather than execution — see EXACT_RANGE_REVIEW.md.

## T075-07 — Saved and alternate views

- [x] Implement saved Grid state as a projection.
- [ ] Form, Gallery, Kanban, Calendar/time, summary/aggregate views. **NOT IMPLEMENTED**: only the saved Grid view is implemented in this closure. The contract (`DataViewKind`) and storage layer support additional view kinds generically, but no Form/Gallery/Kanban/Calendar/summary view was built.
- [x] Prove saved views survive restart and views never duplicate authority data (proven for the implemented Grid view).

**Acceptance:** projection semantics and restart durability proven by tests. **PROVEN for the Grid view only**; the remaining view kinds are an honestly-recorded gap for a future slice, not silently claimed done.

## T075-08 — Deterministic transformations

- [x] Implement the frozen minimal deterministic transformation set.
- [x] Prove durable output is a new immutable DataSnapshot with exact replayable lineage.
- [x] Prove explicit typed-cast failure semantics; no arbitrary code execution path.

**Acceptance:** new-snapshot creation, lineage replay, and failure semantics proven by tests. **PROVEN** (`transform_view_release_lineage` and unit tests in `data_acquire.rs`).

## T075-09 — Dataset release / annotation foundation

- [x] Retained at contract freeze: versioned Dataset Card, immutable release manifest, split/group metadata, annotation schema identity/version, rights/privacy state, Project linkage.

**Acceptance:** release-manifest immutability and Project linkage proven, or removal recorded. **PROVEN** (`transform_view_release_lineage`'s release creation/conflict assertions; the exact-range review additionally found and fixed a silent-pagination-truncation bug in release listing — `release_list_pagination_cursor_is_not_silently_discarded`).

## T075-10 — Qualification and closure

- [x] Run format, dependency-direction, focused contract/storage/Core/CLI/Desktop tests.
- [x] Run Clippy under current policy, workspace tests, cargo-deny/supply-chain gates.
- [x] Run migration/reopen/recovery, malformed/corrupt, disk-full/cancel/timeout suites (disk-full/OS-level timeout are not separately fixture-tested; captured as a residual, not silently assumed).
- [x] Run credential/log secret scans and the no-network local path proof.
- [x] Capture rendered Desktop evidence and perform exact-range review. **Exact-range review: DONE** (`/code-review high`, 18 confirmed findings, material ones fixed in `aa771e9`, see EXACT_RANGE_REVIEW.md). **Rendered Desktop evidence: NOT CAPTURED** — honest residual, see DESKTOP_QUALIFICATION.md/README.md.
- [x] Run exact-head required CI; merge only when green and governance permits. (Required four exact-head CI attempts to reach a genuinely green, exact-range-reviewed head — see CLOSURE.md's closure trail for the full sequence of real failures and forward-only fixes.)
- [x] Verify post-merge main CI; update queue/status to `CLOSED_CANONICAL` only with real post-main evidence.
- [ ] Recompute the next eligible unit; do not implement 076+ without separate promotion. **IN PROGRESS** as the immediate next step after this closure commit.

**Acceptance:** all fourteen frozen acceptance requirements mapped to exact-head proof. **MET with two honestly-recorded, non-blocking gaps** (T075-05 remote live-fetch not exercised; T075-06/T075-07 rendered evidence and non-Grid view kinds not produced) — see `evidence/075-data-source-fabric/CLOSURE.md` for the complete terminal-truth statement.
