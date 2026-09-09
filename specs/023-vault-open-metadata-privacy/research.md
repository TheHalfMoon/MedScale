# Research: Spec 023

## Q03 residual (from Spec 017 LIMITATIONS)

EncryptedVault materializes plaintext `meta.work.sqlite3` while unlocked. SQLCipher was deferred pending `bundled` vs `bundled-sqlcipher` isolation (ADR-017-001 + admission 005).

## Collision resolution options

| Option | Pros | Cons | Disposition |
|---|---|---|---|
| A. Crate/process isolation | Keeps SyntheticVault on plain `bundled` | Two SQLite builds; process boundary cost; still cannot unify features in one graph without care | Rejected for READY_BASE (heavier) |
| B. Workspace-wide SQLCipher | One dependency graph; EncryptedVault page crypto; SyntheticVault opens without key (SQLCipher plaintext-compatible mode) | Larger crypto surface for engineering path; OpenSSL vendored build | **Selected** |
| C. Retire SyntheticVault first | Removes collision | Blocks migration tests / H0 paths | Out of scope |

## Key derivation

Use VaultDek bytes as SQLCipher key material via `PRAGMA key` (hex-encoded passphrase string) applied immediately after `Connection::open`, before any schema touch. Fail-closed on wrong key (`file is not a database` / migrate failure). Never log key.

## Legacy sealed plaintext work files

Pre-023 AES-GCM seals may contain plaintext SQLite. Open path: try SQLCipher key first; on failure, open without key then `PRAGMA rekey` to encrypt under VaultDek-derived material, then continue. Spec 017 wipe still applies on close.

## Doctor honesty

Page encryption removes known-marker disk scan risk for the open work file. OS swap/hibernate/snapshots and MemoryMock keyring still block `PRIVATE_DATA_READY`. Keep `open_work_plaintext_risk=true` for residual OS risk; add `open_work_page_encrypted=true`.

## Pins

- rusqlite **0.37.0** feature `bundled-sqlcipher-vendored-openssl` (bump ≤0.40.2 only if required)
- SQLCipher acquisition target **v4.17.0**; record **embedded** `PRAGMA cipher_version` from tests (may differ from acquisition tag if libsqlite3-sys vendors a different tested revision — amend admission with measured string)
