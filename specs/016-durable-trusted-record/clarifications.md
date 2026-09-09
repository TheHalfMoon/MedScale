# Clarifications: Spec 016

Resolved from inventory of `CoreFacade` / `InMemoryAuthorityStore` on main `e969833` and Trusted V1 review package. No founder clarification required.

| ID | Question | Decision |
|---|---|---|
| C1 | Numbered spec id? | **016** `durable-trusted-record`. Advanced deferred work moves to **017+**. |
| C2 | Persist projections? | **Yes** for restart equality of last rebuilt projection bodies; remain `authoritative=false` and rebuildable. |
| C3 | Persist source bytes in SQLite JSON? | **No** for large payloads. Persist object envelope + digest; hydrate bytes from `FsBlobStore` / sealed blob store. In-memory `SourceRecord.bytes` remains API-compatible after hydrate. |
| C4 | Encryption for Q02? | **Defer private-data claim to Q03**. Extend existing SQLite; EncryptedVault continues seal-at-close of the whole DB. Document plaintext-while-open limitation. Do not add SQLCipher in 016 without Q03 ADR. |
| C5 | Second authority service? | **Forbidden**. Durable store backs the same `CoreFacade`. |
| C6 | ID strategy? | Persist `next_seq` in `store_state` table; IDs remain opaque strings `{prefix}-{seq}` as today. |
| C7 | Writer ownership? | OS exclusive advisory/mandatory lock on `{vault}/writer.lock` held for open lifetime; crash recovery: lock release on process death. Lock file bytes are not proof of ownership. |
| C8 | Action intents? | Persist as `ActionAuditRecord` rows (existing class); outbox remains a derived scan. |
| C9 | Leases / packs / allowlist? | **Not** part of durable clinical record in 016. Process leases stay memory; packs stay PackStore; allowlist memory. |
| C10 | Backup format? | Extend manifest to `schema_version: 2` with `objects` snapshot + blob digests; restore validates full closure into fresh destination. |

## Remaining non-blocking notes

- Platform-specific fsync/rename nuances documented in `research.md`; tests assert logical protocol outcomes.
- Q04 authenticated sessions explicitly out of scope.
