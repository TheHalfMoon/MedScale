# Research: Spec 016 Durable Trusted Record

**Baseline**: MedScale main `e969833d56bcf735a737401743816b86578d62d5`  
**Inventory agent evidence**: `InMemoryAuthorityStore` + vault source meta/blobs only.

## Problem

`CoreFacade` keeps canonical authority objects in `Mutex<InMemoryAuthorityStore>` (`HashMap` + `next_seq`). Vault durability today persists only:

- content-addressed blobs under `{vault}/blobs/`
- `sources` rows in `meta.sqlite3` (Spec 003 schema v1)

After process exit, assertions, proposals, evaluations, identity, merges, audits/outbox intents, projections, and full `SourceRecord` envelopes in memory are lost. Backup/restore copies source meta + blobs only.

## Field inventory → durable ownership

| In-memory field / class | Durable owner in 016 | Notes |
|---|---|---|
| `InMemoryAuthorityStore.objects` Source | `authority_objects` + blob by digest | Bytes via blob store |
| Derived | `authority_objects` + blob by digest | Same |
| Proposal / Assertion / Evaluation / Identity / Merge / Projection / Audit | `authority_objects` JSON body | Typed class column |
| `next_seq` | `store_state.next_seq` | Atomic with inserts |
| `LeaseRegistry` | none (process) | Q04 later |
| egress allowlist | none | ephemeral policy |
| `PackStore` | none in 016 | Packs remain separate |
| `sources` table | retained + kept consistent with Source objects | digest uniqueness |
| External action intents | Audit rows in `authority_objects` | outbox scan |

## Storage options compared

| Option | Pros | Cons | Decision |
|---|---|---|---|
| A. Extend rusqlite meta + blob store | Same dependency family; transactional; migration journal exists; EncryptedVault already seals whole DB | Plaintext while open on synthetic path | **Select for 016** |
| B. SQLCipher / encrypted SQLite backend | Stronger at-rest for open files | Build complexity; not qualified; Q03 scope | Defer to Q03 ADR |
| C. Application-level encrypted record rows | Flexible | Query/migration harder; key handling risk | Evaluate in Q03 if B unfit |
| D. Second DB (sled/redb/etc.) | — | Second authority surface; violates smallest design | Reject |

## Blob / metadata commit protocol

```text
1. Write blob bytes to staging path (or write content-addressed temp)
2. fsync file (best-effort documented per OS)
3. Atomic rename into `blobs/{digest}`
4. Verify digest by re-read or hashing write buffer
5. BEGIN TRANSACTION
6. UPSERT authority_objects (+ sources row if Source)
7. Update store_state.next_seq if needed
8. COMMIT
9. Canonical visibility = committed row with visible=1 / object present
```

Interruption matrix:

| Stage | On reopen |
|---|---|
| After stage, before commit | Orphan blob eligible for GC; no object visibility |
| Commit succeeds, blob missing | Typed `ContentMissing`; refuse silent repair that invents bytes |
| Corrupt bytes vs digest | Typed `DigestMismatch` |
| Migration `started` unfinished | `MigrationIncomplete` recover-or-refuse per migrate module |

## OS assumptions

- **Atomic rename** within same volume for blob finalize (Windows `MoveFileEx`, POSIX `rename`).
- **fsync**: best-effort on blob file and DB; document that power-loss durability is platform-dependent; tests cover process-kill between stages, not forced power loss.
- **Locks**: exclusive file lock on `writer.lock` (`fs2` or std equivalent already used if present; otherwise add minimal proven crate or use `std`+platform APIs behind narrow module). Crash releases lock with process.
- **Directories**: claim-path checks from Spec 003 remain mandatory.

## Backup / restore

Extend Spec 003 backup:

- `manifest.schema_version = 2`
- Include serialized `authority_objects` + `store_state`
- Validate every referenced digest exists in backup blob set
- Restore only into empty/fresh destination; never clobber valid vault on failure

## Privacy honesty

Spec 005 sealed-at-close encrypts the working SQLite file when closed. While open, working metadata may be plaintext on disk. Spec 016 **must not** advertise `PRIVATE_DATA_READY`. Q03 qualifies open-vault WAL/temp/crash artifacts and metadata confidentiality.

## Exit / update strategy

- Schema versioned via `migration_journal`
- Object JSON uses `serde` of existing contract types; additive field evolution follows deny_unknown_fields discipline with explicit migrations
- Exit: export/backup format remains recoverable without MedScale binary if JSON+blobs preserved
