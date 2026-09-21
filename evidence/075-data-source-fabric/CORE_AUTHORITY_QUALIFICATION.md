# CORE_AUTHORITY_QUALIFICATION — Spec 075

## Mutation/query matrix

Covered by `crates/medscale-core/tests/data_sources_075.rs` (facade
dispatch with Strict sessions) plus inline unit tests in `data_acquire.rs`:

| Path | Normal | Denied | Conflict | Cross-scope | Corrupt | Idempotent |
|---|---|---|---|---|---|---|
| source create/list/get/update/archive | yes | - | stale revision | WrongScope | - | - |
| preview | yes | archived source | - | WrongScope | - | read-only (no persist) |
| import | yes | archived source | duplicate digest tuple | WrongScope | digest/read-back verified | same bytes resolve same snapshot + original receipt |
| refresh | unchanged/changed/schema-ack | archived source | - | WrongScope | - | unchanged yields receipt only |
| rows | paging/filter/sort | - | - | WrongScope | digest + fingerprint + count verified | stable cursors |
| views create/list/get/update | schema-validated | archived view | stale revision | via snapshot | state re-validated on read | - |
| transform | lineage + cast counts | multi-input | - | via input | - | same ops are deterministic (replayable) |
| release create/get/list | digest-verified | - | duplicate version | via snapshot | bytes re-verified at release | - |
| sqlite adapter | import + preview | credential ref rejected | - | path claim | blob cells refused | - |
| remote adapter | brokered fixture path (unit) | empty allowlist denies | - | - | malformed quarantines | - |
| unqualified format/engine | UnsupportedSchema | - | - | - | - | - |

## Session/capability proof

- Mutations require sessions (Strict facade gate); reads are session-free
  per `is_read_or_health` (contract test asserts the exact set).
- Every op travels an explicit `capability_matches` pair (facade); mismatch
  is denied before authority.

```text
RESULT = PENDING (exact-head CI on PR #129 branch spec/075-data-source-fabric)
```
