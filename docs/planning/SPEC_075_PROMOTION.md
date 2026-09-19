# Spec 075 Promotion — Data Source Fabric + Data Workbench Foundation

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`
**Founder promotion date:** 2026-09-19
**Canonical base:** `ae0441918296c2d1510a71061249c7e55e65d760`
**Target branch:** `spec/075-data-source-fabric`

## Authority

The founder explicitly approved the MedScale Clinical + Research Intelligence OS planning direction represented by planning PR #128, subject to exact live verification, successful required CI, canonical repository governance, and reconciliation with newer repository authority. PR #128 was qualified exact-head (head `3deeb5aeaee4bf6a28186b8721a3d4582ec27f1b`, 19 planning-only files under `docs/planning/`, exact-head CI run `35411823551` success) and merged normally into `main` as `ae0441918296c2d1510a71061249c7e55e65d760`. Post-merge main CI run `35413163411` completed successfully across all six required jobs.

This document promotes **only Spec 075 — Data Source Fabric + Data Workbench Foundation** for implementation.

Predecessor closure proof: Spec 074 is `CLOSED_CANONICAL` (exact-head `660ca54…`, run `35308381108`, PR #122 merged as `3d59255…`, post-merge main run `35309949710`; see `evidence/074-project-artifact-graph-foundation/CLOSURE.md`).

Numbering proof: no `specs/075-*` package, no `SPEC_075_PROMOTION.md`, and no Spec 075 implementation exists on the canonical base. The live `BUILD_QUEUE.md` 075+ row holds only deferred V2 candidates requiring fresh promotion. Amendment 001 (`RESEARCH_OS_PROGRAM_AMENDMENT_001_DATA_EXTENSIONS.md`) assigns Data Source Fabric to candidate number 075, superseding older candidate aliases.

Standing implementation authority in `IMPLEMENTATION_AUTHORITY.md` remains active. Normal branch/commit/push/PR/merge authority applies only within the promoted scope and only after real required gates pass.

## Authorized scope

Spec 075 may implement only the minimum governed data-source/snapshot/workbench foundation required so that later Analytics, R Workspace, Evidence/Graph, MedAgent, and research consumers share one source model instead of each fetching independently:

- DataSourceManifest, DataSourceKind, capability, and source-health contracts;
- SourceSchema with explicit field types, missingness, and schema fingerprint;
- immutable DataSnapshot with snapshot parts and content digests;
- import/refresh receipts and transformation receipts with exact lineage;
- saved data views (Grid state plus approved alternate views);
- bounded deterministic transformation set producing new immutable snapshots;
- local tabular file vertical slice (CSV/TSV first, then JSON/JSONL, Parquet, Arrow IPC/Feather only if dependency qualification passes, XLSX only through a qualified hostile-input parser path);
- evidence-selected read-only database adapter first (PostgreSQL, MySQL/MariaDB, SQL Server, external SQLite candidates; stage drivers in separate slices, do not force all providers into one patch);
- remote dataset adapter foundation (Hugging Face datasets by exact repository/revision/files; Kaggle datasets by exact identifier/version/files) only through the existing Network Broker with opaque credentials, resume, corruption detection, quarantine/admission, no trusted remote code, and no upload/competition submission — or explicitly defer remote adapters in a canonical scope amendment rather than silently omitting them;
- native Core-backed Slint Data Workbench (typed Grid, row detail, filter/sort/group, paging/virtualization, provenance inspector, loading/empty/error/partial/corrupt states, keyboard/focus/accessibility, light/dark parity);
- Project attachment of sources/snapshots reusing the Spec 074 Project + Artifact Graph;
- CLI inspect/mutation paths through Core with stable machine-readable output;
- encrypted local persistence and migration (storage schema v3 → v4, additive only);
- versioned Dataset Card plus immutable release manifest only if retained in the frozen contract; full annotation workflow stays out;
- migration, recovery, compatibility, scale, security, accessibility, and exact-head evidence required for closure.

Staging rule: the first implementation may land storage/Core vertical slices before workbench view slices, and may qualify database drivers and remote adapters in separate dependency slices. The spec cannot close without the declared foundation set or an explicit canonical scope amendment recorded in the owning package.

## Explicitly not authorized

This promotion does **not** authorize implementation of:

- Spec 076 Collaboration Substrate;
- Spec 077 MedAgent + Evidence Copilot Foundation;
- Spec 078 Model Fleet + Compare;
- Spec 079 Privacy Gate (beyond using current data classification);
- Spec 080 Governed Browse + Medical Literature Acquisition;
- Spec 081 AudioFlow Foundation + Local Scribe MVP;
- Spec 082 Analytics Gate + Cohort Builder (DataFusion ownership stays with 082; 075 uses only a narrow schema/format library if proven necessary);
- Spec 083 Clinical Graph + Knowledge/Research Canvas;
- Spec 084 MedScale Hub;
- Spec 085 MedScale Compute;
- Spec 086 R Workspace;
- Spec 087 Community Extensions;
- Specs 088–092 (AudioFlow Advanced, Research/Evidence Packs, Institutional Adapters, Federation, Whole-Platform Qualification);
- arbitrary Python/JavaScript/shell/R formula execution;
- a generic Airtable clone or a second database/vault/ID/authority system;
- real PHI;
- MESC work;
- new cloud/runtime network authority beyond the existing fail-closed broker;
- donor wholesale copying (Grist/Baserow are UX/behavior donors, not embedded platforms; NocoDB and Teable core are reference-only);
- a new ID, provenance, audit, or authority foundation when current MedScale primitives are sufficient.

## Mandatory architecture constraints

1. Reuse current MedScale `OpaqueId`, `ObjectHeader`, `DigestSha256`, realm/scope, digest/provenance, audit, vault, migration, Network Broker, and Core patterns where applicable.
2. Existing patient/FHIR/source/document/evidence/model/Pack objects remain canonical in their owning systems. Data-source attachment references them by stable identity; snapshots carry dataset bytes, never rewritten patient truth.
3. The Clinical/Research Graph remains a rebuildable derived projection; 075 dataset lineage does not become graph authority.
4. Workspace views (Grid, Form, Gallery, Kanban, Calendar/time, summary) are projections over the same canonical dataset/snapshot; they never clone patient facts into view-specific truth stores.
5. Consequential derived values keep Linked Evidence (source span, snapshot row/column, tool run, review state) where available.
6. CLI and Desktop must use the same Core command/query semantics. No direct storage/data-provider access from UI.
7. Local/offline operation is mandatory for local sources: with network egress disabled, import/snapshot/inspect/transform/reopen of local sources must remain useful. Unavailable local engines report `UNAVAILABLE`/`DENIED`, never silently call cloud.
8. Database adapters are read-only, use opaque credential references, never log secrets, bound schema discovery, and honor timeout/cancel.
9. Migration must preserve all pre-075 workflows and object identities (no existing object ID rewrite; Project attachment compatibility).
10. Unknown/unavailable/stale/partial/conflicting/denied/corrupt/unsupported states stay distinct; missing input is not zero.
11. No later Research OS unit may be started merely because its planning document exists.

## Implementation order

```text
T075-00 Live truth and baseline (no material product mutation before it closes)
T075-01 Contracts freeze + invariant tests
T075-02 Storage schema v4, migration, crash/reopen/recovery tests
T075-03 Local file vertical slice (CSV/TSV first)
T075-04 Read-only database adapter (one evidence-selected engine first)
T075-05 Remote dataset adapters (or explicit canonical deferral)
T075-06 Native Data Workbench grid
T075-07 Saved and alternate views
T075-08 Deterministic transformations
T075-09 Dataset release / annotation foundation (only if retained in frozen scope)
T075-10 Exact-head qualification, review, and closure
```

A later slice may not paper over a failed earlier invariant. Parallelize only test/document/evaluation work that cannot race shared contracts, schema, authority paths, or canonical planning artifacts.

## Frozen acceptance requirements

Spec 075 can close only when all of the following are proven on the exact reviewed head:

1. Local file → immutable DataSnapshot → Project attachment works and survives close/reopen.
2. At least one qualified database read → immutable snapshot path works with opaque credentials and no secret logging.
3. Declared remote dataset adapter foundation works, or canonical promotion explicitly deferred it.
4. Data Workbench displays real Core-backed snapshot data (no fake product data).
5. Saved views survive restart.
6. Deterministic transformation creates a new snapshot with replayable lineage.
7. External source mutation does not rewrite old snapshots.
8. Secrets never appear in persisted or logged ordinary artifacts.
9. Malformed/partial/corrupt inputs produce explicit safe states.
10. Pre-075 vaults migrate (v3 → v4), reopen, and recover from backup.
11. No alternate authority/storage/network path exists (dependency-direction gate holds).
12. Local sources remain usable with network disabled.
13. No real-PHI authorization is implied; fixtures are synthetic/permitted non-PHI.
14. Exact-head required CI and post-main verification pass.

## Evidence paths

All qualification evidence lives under `evidence/075-data-source-fabric/` (see `specs/075-data-source-fabric/plan.md` for the required evidence set, mirroring the Spec 074 evidence structure).

## Completion rule

Spec 075 becomes `CLOSED_CANONICAL` only when all acceptance criteria in the owning Spec 075 package are proven on the exact reviewed head, required CI is green, the PR is merged normally, and post-merge main verification is recorded.

Closure of 075 does not itself authorize 076. After closure, live governance must recompute the next eligible unit and explicitly promote it.
