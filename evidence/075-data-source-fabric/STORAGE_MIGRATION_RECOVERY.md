# STORAGE_MIGRATION_RECOVERY — Spec 075

## Migration

```text
MIGRATION = v3 -> v4, additive (crates/medscale-storage/src/data_sources.rs: V4_DDL)
JOURNAL = existing begin/finish_migration framework (interrupted -> MigrationIncomplete)
REPEAT_OPEN = safe (IF NOT EXISTS everywhere;-open-twice covered by migration_v3_to_v4_preserves_old_rows)
```

## Fixtures and sequence

Covered by `crates/medscale-storage/tests/data_sources_075.rs`:

- fresh vault migrates to v4 (`journal.finished_version == 4`), reopen preserves rows;
- source CRUD with CAS conflicts; unknown id is `NotFound`;
- snapshot insert/list/parts/latest with duplicate-conflict backstop;
- receipts (import insert-once, refresh upsert), views (CAS), transformations, releases (snapshot-scoped inheritance, missing snapshot is `NotFound`);
- tampered status fails closed on read;
- backup/restore round-trip preserves fabric rows (manifest v4).

```text
RESULT = PENDING (exact-head CI on PR #129 branch spec/075-data-source-fabric)
```

## Pre-075 regression

Full workspace suite on the branch proves pre-075 workflows (074 graph,
vault, CLI, Desktop) still pass with v4 present. Encrypted sealed backup
carries v4 rows with no code change (same DB file).

## Rollback

Restore verified pre-migration backup. No destructive down-migration exists.
Interrupted migration fails closed with `MigrationIncomplete`.
