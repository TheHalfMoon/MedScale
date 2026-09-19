# Tasks — Spec 075 Data Source Fabric + Data Workbench Foundation

**Execution state:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`

Check a task only when its implementation, tests and required evidence are real on the branch. Do not pre-check future work.

## T075-00 — Live truth and baseline

- [ ] Verify branch/base/main/PR state and repository cleanliness.
- [ ] Read mandatory governance + Clinical/Research Intelligence + Spec 075 authority chain.
- [ ] Inspect current contracts/Core/storage/network/key/CLI/Desktop ownership paths.
- [ ] Confirm no live Spec 075 collision/superseding authority; prove numbering is free.
- [ ] Verify Spec 074 closure and post-main evidence.
- [ ] Record baseline focused/full gate state and any pre-existing failures.
- [ ] Create initial evidence/live-truth record.

**Gate:** no code mutation before T075-00 is complete.

## T075-01 — Freeze contracts/data model

- [ ] Freeze exact field-level source/snapshot/receipt/view/transformation contracts in `contracts.md`.
- [ ] Reuse `OpaqueId`, `ObjectHeader`, `DigestSha256` and existing provenance/audit primitives where semantically valid.
- [ ] Freeze DataSourceKind/Capability/Health vocabulary for 075; reject unsafe unknown authority kinds.
- [ ] Freeze SourceSchema/field-type system, snapshot identity, receipt lineage, saved-view and transformation semantics.
- [ ] Freeze revision/precondition and idempotency behavior for mutable source/view state.
- [ ] Add serialization/validation/invariant tests.

**Acceptance:** contract tests pass; no storage/network/UI dependency enters contracts.

## T075-02 — Storage schema + migration

- [ ] Add data-source storage using current encrypted vault/SQLite metadata architecture (v3 → v4).
- [ ] Add snapshot and snapshot-part storage (metadata rows; large bytes via existing blob/sealed-blob stores).
- [ ] Add import/refresh/transformation receipt and lineage storage.
- [ ] Add saved-view and transformation storage with required indexes.
- [ ] Implement atomic transactions and stale-revision conflict behavior.
- [ ] Add forward migration from representative pre-075 vaults (including populated 074 vaults).
- [ ] Add repeated-open/repeated-migration safety tests.
- [ ] Add crash/reopen and backup/restore recovery tests.
- [ ] Prove no source/snapshot operation cascades deletion into canonical target objects.
- [ ] Prove Project attachment compatibility with Spec 074 graph.

**Acceptance:** storage + migration suite green and pre-075 IDs/behavior preserved.

## T075-03 — Local file vertical slice

- [ ] Implement CSV/TSV candidate source, validation/quarantine, schema discovery, snapshot materialization.
- [ ] Implement Project attachment, reopen durability, and CLI inspect for the slice.
- [ ] Freeze malformed/partial/corrupt/encoding/resource policies.
- [ ] Qualify JSON/JSONL, Parquet, Arrow IPC/Feather, XLSX only in separate slices with exact dependency review.

**Acceptance:** CSV/TSV file → immutable snapshot → Project → CLI inspect → restart/reopen proven end to end.

## T075-04 — Database read adapter

- [ ] Implement one evidence-selected native read-only database adapter.
- [ ] Prove opaque credential handling and absence of secrets in logs/persisted artifacts.
- [ ] Prove read-only behavior, bounded discovery, explicit query identity, immutable snapshot binding.
- [ ] Prove lost-connection/cancel, schema-change state, and restart durability.
- [ ] Qualify remaining candidate engines separately or record explicit deferral.

**Acceptance:** at least one database engine path qualified end to end.

## T075-05 — Remote dataset adapters

- [ ] Implement Hugging Face / Kaggle acquisition through the existing Network Broker.
- [ ] Prove exact identity/version/files, resumable download, hash/corruption detection, quarantine/admission.
- [ ] Prove opaque credentials, no trusted remote code, no upload/competition submission.
- [ ] Or record an explicit canonical deferral amendment in this package.

**Acceptance:** remote foundation qualified, or deferral recorded with local-first closure intact.

## T075-06 — Native Data Workbench

- [ ] Implement Data Source / Data Workbench navigation through current Slint composition, backed exclusively by Core.
- [ ] Implement dataset/snapshot selection, typed columns, virtualized/paged rows, row detail, filter/sort/group.
- [ ] Implement empty/loading/error/corrupt/partial states, provenance/source inspector, light/dark parity, keyboard/focus.

**Acceptance:** Workbench tests/rendered evidence prove real Core-backed snapshot data.

## T075-07 — Saved and alternate views

- [ ] Implement saved Grid state, Form, Gallery, Kanban, Calendar/time, summary/aggregate views as projections.
- [ ] Prove saved views survive restart and views never duplicate authority data.

**Acceptance:** projection semantics and restart durability proven by tests.

## T075-08 — Deterministic transformations

- [ ] Implement the frozen minimal deterministic transformation set.
- [ ] Prove durable output is a new immutable DataSnapshot with exact replayable lineage.
- [ ] Prove explicit typed-cast failure semantics; no arbitrary code execution path.

**Acceptance:** new-snapshot creation, lineage replay, and failure semantics proven by tests.

## T075-09 — Dataset release / annotation foundation

- [ ] Only if retained at contract freeze: versioned Dataset Card, immutable release manifest, split/group metadata, annotation schema identity/version, rights/privacy state, Project linkage.
- [ ] Or record removal at freeze with rationale.

**Acceptance:** release-manifest immutability and Project linkage proven, or removal recorded.

## T075-10 — Qualification and closure

- [ ] Run format, dependency-direction, focused contract/storage/Core/CLI/Desktop tests.
- [ ] Run Clippy under current policy, workspace tests, cargo-deny/supply-chain gates.
- [ ] Run migration/reopen/recovery, malformed/corrupt, disk-full/cancel/timeout suites.
- [ ] Run credential/log secret scans and the no-network local path proof.
- [ ] Capture rendered Desktop evidence and perform exact-range review.
- [ ] Run exact-head required CI; merge only when green and governance permits.
- [ ] Verify post-merge main CI; update queue/status to `CLOSED_CANONICAL` only with real post-main evidence.
- [ ] Recompute the next eligible unit; do not implement 076+ without separate promotion.

**Acceptance:** all fourteen frozen acceptance requirements mapped to exact-head proof.
