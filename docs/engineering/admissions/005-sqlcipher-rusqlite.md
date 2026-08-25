# Dependency admission: SQLCipher + rusqlite (Spec 005)

## SQLCipher v4.17.0 (acquisition target) / rusqlite binding

| Field | Value |
|---|---|
| Component | SQLCipher page encryption (preferred EncryptedVault metadata backend) |
| Acquisition pin | https://github.com/sqlcipher/sqlcipher tag **v4.17.0** |
| Binding candidate | rusqlite **0.37.0** features `bundled-sqlcipher-vendored-openssl` (bump allowance **0.40.2**) |
| Owning Spec | 005 |
| Purpose | Encrypted SQLite metadata DurableStore |
| License | SQLCipher BSD-style; rusqlite MIT/Apache-2.0 |
| Placement | `medscale-storage` feature `sqlcipher` (preferred) |
| Security | One SQLCipher copy per process; keys never in IPC |
| Tests required | Wrong-key fail-closed; round-trip; version string evidence |
| Update strategy | Pin minor; re-deny; re-verify embedded version |
| Exit strategy | `AesGcmSealedStore` EncryptedVault backend (research D4) |

## Spec 005.1 implement disposition (2026-08-25)

Workspace Spec 003 already pins rusqlite **0.37.0** `bundled` for unencrypted `SyntheticVault`. rusqlite cannot enable both `bundled` and `bundled-sqlcipher-*` in one dependency graph without splitting crates/processes.

**This PR ships EncryptedVault on the D4 AES-GCM sealed metadata + AES-GCM blob backend** so encryption-at-rest proofs land on Windows+Linux CI without weakening SyntheticVault migration source.

SQLCipher remains the **preferred re-entry** once SyntheticVault is retired or isolated behind a dedicated sqlcipher crate/process. Feature flag `sqlcipher` is reserved; do not claim SQLCipher page crypto until that backend is green and admission records the exact embedded SQLCipher revision.
