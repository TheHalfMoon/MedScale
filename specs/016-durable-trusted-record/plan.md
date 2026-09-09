# Plan: Spec 016 Durable Trusted Record

## Goal

Make the trusted longitudinal record survive process restart by persisting the full `InMemoryAuthorityStore` object graph through `CoreFacade` and one SQLite+blob vault, with restart/failure/backup qualification.

## Decisions

| ID | Decision |
|---|---|
| D1 | Spec number **016**; advanced deferred work renumbered narrative as **017+** |
| D2 | ADR-016-001: extend rusqlite meta schema v2; no SQLCipher in this unit |
| D3 | Hydrate source/derived bytes from blob store; do not duplicate large payloads in SQLite |
| D4 | OS-exclusive `writer.lock` for Q02; Q04 sessions later |
| D5 | Backup manifest schema_version 2 includes objects |
| D6 | Privacy: document limitation; Q03 owns PRIVATE_DATA_READY |

## Implementation phases

1. **Storage crate**: migration v2, CRUD for authority_objects/store_state, writer lock module, backup extension.
2. **Core facade**: on vault open load graph; on mutating capabilities dual-write; promote atomic.
3. **Tests**: unit mapping; two-process restart; failure injection; isolation/backup.
4. **Evidence + queue closeout**.

## Dependency direction

```text
medscale-contracts ← medscale-storage ← medscale-core ← medscale-cli
```

No CLI→SQLite. No new crate required unless writer-lock helper stays inside `medscale-storage`.

## Test strategy

- Retain all existing workspace tests.
- Add `durable_restart_016` integration with `std::process::Command` of a small test binary or `cargo test` harness spawning helper.
- Fault tests: transaction rollback, missing blob, bad digest, migration started.
- Windows + Linux CI.

## Risks

| Risk | Mitigation |
|---|---|
| Dual-write drift memory vs SQLite | Load-from-disk is source of truth after open; writes go through one helper |
| Migration breaks old vaults | Journaled v1→v2; reconstruct Source envelopes from sources+blobs |
| False PASS via same-process reload | Mandate two OS processes in T05 |
| Privacy overclaim | Explicit FR-012 + evidence limitations |

## Qualification gate before T04

T01–T03 artifacts present; checklist PASS; analyze PASS; ADR accepted in package.
