# Spec 075 Promotion Candidate — Data Source Fabric + Data Workbench Foundation

**Status:** `DRAFT_PROMOTION_CANDIDATE_NOT_AUTHORIZED`  
**Prepared:** 2026-09-19  
**Live base at preparation:** `main@8db4ac089db2869da9ff6570ad7da91139987321`  
**Prerequisite:** Spec 074 `CLOSED_CANONICAL`  
**Candidate target branch:** `spec/075-data-source-fabric`

This document makes the first post-074 unit ready for canonical promotion. It does **not** grant implementation authority.

Before promotion, reverify live main, open PRs, current numbering, dependency gates, source/rights status and whether any newer founder authority supersedes this packet.

# 1. Why 075 is next

Research OS V2 Program Amendment 001 makes Data Source Fabric candidate 075.

The Clinical + Research Intelligence expansion increases the importance of this dependency because:

- Analytics needs immutable snapshots;
- Research Canvas needs stable datasets;
- cohort work needs explicit missingness/schema/provenance;
- R Workspace needs staged snapshots;
- Evidence/Graph work needs governed data artifacts;
- clinical/research datasets must not be fetched independently by every subsystem.

Spec 074 already provides Project + Artifact Graph organization.

Therefore 075 should establish one data-source/snapshot/workbench foundation before later analytics/R/research consumers expand.

# 2. Candidate authorized outcome

If promoted, 075 should deliver exactly this end-to-end user outcome:

> A user can create/open a Project, import a local tabular dataset or read/snapshot a supported external data source, inspect the immutable snapshot in a native MedScale Data Workbench, create a saved view and one deterministic transformation, close/reopen MedScale, and reproduce the source -> snapshot -> transformation lineage without a cloud dependency for local sources.

# 3. Candidate scope

## Data source contracts

- `DataSourceManifest`
- `DataSourceKind`
- `DataSourceCapability`
- `SourceHealth`
- `SourceSchema`
- `DataSnapshot`
- `SnapshotPart`
- `ImportReceipt`
- `RefreshReceipt`
- `SavedDataView`
- `DataTransformation`
- `TransformationReceipt`

Final exact fields must reuse existing IDs, object headers, scope, source, digest, time and evidence primitives.

## Local source support

Foundation:
- CSV/TSV;
- JSON/JSONL;
- Parquet;
- Arrow IPC/Feather if dependency qualification passes;
- XLSX only through a qualified hostile-input parser path.

## Database read-source support

Candidate foundation engines:
- PostgreSQL;
- MySQL/MariaDB;
- SQL Server;
- external SQLite.

Default is read-only source access.

The implementation may stage/qualify drivers in separate dependency slices rather than forcing all four into the first commit.

## Remote dataset source support

Candidate:
- Hugging Face datasets by exact repository/revision/files;
- Kaggle datasets by exact identifier/version/files.

Remote sources require existing network authority and opaque CredentialRef handling.

No trusted remote dataset scripts.

## Data Workbench foundation

- typed Grid;
- row detail;
- sort;
- filter;
- group;
- saved view;
- Form;
- Gallery;
- Kanban;
- Calendar/time;
- summary/aggregate view;
- deterministic transformation foundation.

The first implementation may stage view UI slices after the storage/Core vertical path, but the spec cannot close without the declared foundation set or an explicit canonical scope amendment.

# 4. Explicit non-goals

075 must not implement:

- MedAgent;
- model fleet;
- Privacy Gate expansion beyond using current data classification;
- web browser;
- clinical literature search;
- AudioFlow/scribe;
- analytics/statistical methods;
- Python/R execution;
- Clinical Graph;
- Hub/sync;
- extensions;
- institutional FHIR/EHR writes;
- real PHI;
- MESC;
- arbitrary formula scripting;
- a generic Airtable clone;
- a second database/vault/ID/authority system.

# 5. Current exact repository anchors

At the prepared baseline:

```text
crates/medscale-contracts/src/project_graph.rs
crates/medscale-contracts/src/objects/
crates/medscale-contracts/src/evidence/
crates/medscale-contracts/src/network/

crates/medscale-core/src/authority/facade.rs
crates/medscale-core/src/authority/project_graph.rs
crates/medscale-core/src/authority/source_ops.rs
crates/medscale-core/src/cli_session.rs

crates/medscale-storage/src/encrypted_vault.rs
crates/medscale-storage/src/sqlite_meta.rs
crates/medscale-storage/src/blob.rs
crates/medscale-storage/src/sealed_blob.rs
crates/medscale-storage/src/migrate.rs
crates/medscale-storage/src/backup.rs
crates/medscale-storage/src/project_graph.rs

crates/medscale-network/src/adapters.rs
crates/medscale-network/src/allowlist.rs
crates/medscale-network/src/transport.rs

crates/medscale-keys/src/keystore.rs
crates/medscale-keys/src/provider.rs

crates/medscale-cli/src/main.rs
crates/medscale-cli/src/project.rs

crates/medscale-desktop/src/project_workspace.rs
crates/medscale-desktop/ui/app.slint
crates/medscale-desktop/ui/components.slint
crates/medscale-desktop/ui/theme.slint
```

Default new module shape after promotion:

```text
medscale-contracts/src/data_sources/
medscale-core/src/data_sources/
medscale-cli/src/data_source.rs
medscale-cli/src/data_workbench.rs
medscale-desktop/src/data_sources.rs
medscale-desktop/src/data_workbench.rs
medscale-desktop/ui/data_sources.slint
medscale-desktop/ui/data_workbench.slint
```

Storage should extend existing storage modules/migrations first rather than create a new storage crate.

# 6. Candidate task graph

## T075-00 — Live truth / baseline

- verify main/head/worktree/PR/queue;
- verify 074 closure/post-main evidence;
- verify 075 number is unused and not already promoted;
- inspect exact live IDs/source/project/storage/network/secret/CLI/Desktop patterns;
- run current baseline required gates;
- record pre-existing failures separately;
- create `evidence/075-data-source-fabric/LIVE_TRUTH.md`.

**Gate:** no material product mutation before T075-00.

## T075-01 — Freeze contracts

Freeze:
- source kind/capability;
- schema/field types;
- snapshot identity;
- receipt types;
- saved view;
- transformation lineage;
- error/result states.

Required explicit states include:
`Denied`, `Unavailable`, `Unsupported`, `Stale`, `Partial`, `Corrupt`, `Conflict`, `Cancelled`.

Add serialization/strictness/invariant tests.

## T075-02 — Storage + migration

Add minimum additive encrypted metadata structures for:
- data sources;
- snapshots;
- snapshot parts;
- source/snapshot receipts;
- saved views;
- transformations.

Use existing blob/sealed-blob storage for larger data where semantics fit.

Prove:
- pre-075 vault migration;
- reopen;
- repeated migration;
- crash boundary;
- backup/restore;
- project artifact linkage;
- no old-ID rewrite.

## T075-03 — Local file vertical slice

Implement:
- candidate source;
- validation/quarantine;
- schema discovery;
- snapshot materialization;
- Project attachment;
- reopen;
- CLI inspect.

Start with the smallest safe deterministic format, likely CSV/TSV, then add JSON/Parquet/Arrow/XLSX only through qualified slices.

## T075-04 — Database read adapter

Implement one evidence-selected native read-only database adapter first, then qualify remaining candidate engines separately.

Must prove:
- opaque credential handling;
- no secret logging;
- read-only behavior;
- bounded discovery;
- explicit query;
- immutable snapshot;
- lost connection/cancel;
- schema change;
- restart.

## T075-05 — Remote dataset adapters

HF/Kaggle:
- exact identity/version/files;
- brokered network;
- opaque credentials;
- resumable download;
- hash/corruption detection;
- no trusted remote code;
- quarantine/admission;
- no upload/competition submission.

## T075-06 — Data Workbench grid

Native Desktop:
- dataset/snapshot selection;
- typed columns;
- virtualized/paged rows;
- row detail;
- filter/sort/group;
- empty/loading/error/corrupt/partial states;
- provenance/source inspector;
- light/dark + keyboard/focus.

All data arrives through Core.

## T075-07 — Saved views + alternate views

- saved Grid state;
- Form;
- Gallery;
- Kanban;
- Calendar/time;
- summary view.

Views are projections over the same dataset/snapshot; they do not duplicate authority.

## T075-08 — Transformation foundation

Support a minimal deterministic transformation set such as:
- select/drop/rename columns;
- typed cast with explicit failure;
- filter;
- simple deterministic computed expressions if safe;
- sort/materialize;
- join only if bounded contract is frozen.

Durable output becomes a new DataSnapshot with exact lineage.

Do not add arbitrary Python/JS/R formula execution.

## T075-09 — Scale/recovery/security

Test:
- large table;
- pagination/virtualization;
- disk full;
- malformed input;
- decompression/zip-bomb class if XLSX;
- invalid UTF-8/encoding policy;
- cancellation;
- database timeout;
- remote partial download;
- source deleted;
- schema changed;
- credential revoked;
- restart/reopen.

## T075-10 — Exact-head qualification

Run:
- fmt;
- dependency-direction;
- focused contract/storage/Core/CLI/Desktop tests;
- Clippy with current policy;
- workspace tests;
- cargo-deny/supply-chain gates;
- migrations/recovery;
- privacy/log/secret checks;
- rendered Desktop evidence;
- exact-range review.

Then:
- merge only when exact-head required gates pass;
- verify post-main CI;
- update BUILD_QUEUE to closed only with real post-main evidence.

# 7. Initial donor/dependency posture

Use:
- Arrow/Parquet as candidate local interchange;
- DataFusion belongs to 082 Analytics, not 075 unless 075 needs only a narrow schema/format library and proves it;
- Grist/Baserow are UX/behavior donors, not embedded application platforms;
- NocoDB is reference-only under current observed license;
- Teable core is reference-only by default;
- HF/Kaggle clients are adapter candidates, not authority.

Every new dependency requires exact version/license/security/exit review.

# 8. Acceptance criteria

075 can close only if all are proven on exact candidate head:

1. Local file -> immutable DataSnapshot -> Project works.
2. At least one qualified database read -> immutable snapshot path works.
3. Declared remote dataset adapter foundation works if retained in promoted scope; otherwise canonical promotion must explicitly defer it rather than silently omit it.
4. Data Workbench displays real Core-backed snapshot data.
5. Saved views survive restart.
6. Deterministic transformation creates a new snapshot with replayable lineage.
7. External source mutation does not rewrite old snapshots.
8. Secrets never appear in persisted/logged ordinary artifacts.
9. Malformed/partial/corrupt inputs produce explicit safe states.
10. Pre-075 vaults migrate/reopen/recover.
11. No alternate authority/storage/network path exists.
12. Local sources remain usable with network disabled.
13. No real PHI authorization is implied.
14. Exact-head required CI and post-main verification pass.

# 9. Promotion decision needed

Canonical promotion should create a normal `SPEC_075_PROMOTION.md` and a full `specs/075-data-source-fabric/` package only after live revalidation.

Until then:

```text
SPEC_075_IMPLEMENTATION_AUTHORITY = NO
SPEC_075_PROMOTION_PACKET = READY_FOR_REVALIDATION
```
