# Research: open-vault plaintext surfaces (Q03)

| Surface | When present | Risk | Spec 017 action |
|---|---|---|---|
| `meta.work.sqlite3` | EncryptedVault while open | Full metadata plaintext | Seal+remove on close; detect leftover on open |
| `meta.work.sqlite3-wal` / `-shm` / `-journal` | SQLite journaling while open / crash | Partial plaintext | Remove with work file |
| `meta.sealed` | At rest | Ciphertext | Keep |
| `vault_header.json` | Always | Key wraps (not DEK plaintext) | Existing; no secrets in doctor |
| `sealed_blobs/*` | At rest | Ciphertext | Keep |
| `.writer.lock` | EncryptedVault lease marker | Presence ≠ ownership | Document; Q04 strengthens |
| `writer.lock.sqlite3` | SyntheticVault Spec 016 lock | Lock DB only | N/A privacy claim |
| SyntheticVault `meta.sqlite3` + `blobs/` | Always open engineering | Plaintext by design | No PRIVATE claim |
| OS swap/snapshots | Platform | Residual | Limitation until measured |

## Analyze

PASS for lifecycle wipe implementation. PRIVATE_DATA_READY not asserted by wipe alone.
