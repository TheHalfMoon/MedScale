# Quickstart: Spec 005 Local Private Vault + Encryption + Recovery

## Prerequisites

- Spec 004 `CLOSED_CANONICAL`
- Spec 003 SyntheticVault + claim paths available for migration tests
- Rust toolchain from `rust-toolchain.toml`
- Read: [spec.md](./spec.md), [plan.md](./plan.md), [data-model.md](./data-model.md), [research.md](./research.md)
- Windows and/or Linux environment for sqlcipher CI claims

## Scope reminder

This unit is **pre-PHI encryption / key / recovery / backup / migration / log hygiene** only:

- Synthetic data only — REAL_PHI remains EXTERNAL_GATES NOT_AUTHORIZED
- No product runtime network
- No MESC mutation / OpenMed runtime
- No Spec 006 Desktop/CLI product shell / v0 UI / full `medscale doctor` packaging
- No models/packs/OCR/ASR

## After implement (expected commands)

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p medscale-keys -p medscale-storage -p medscale-core -p medscale-contracts
cargo test --workspace
```

Feature-focused (names illustrative until implement):

```powershell
cargo test -p medscale-storage --features sqlcipher encrypted_vault_roundtrip
cargo test -p medscale-storage --features sqlcipher vault_sync_root_and_lease
cargo test -p medscale-keys vault_recovery_key_loss
cargo test -p medscale-storage --features sqlcipher encrypted_backup_restore_retention
cargo test -p medscale-storage --features sqlcipher migrate_synthetic_to_encrypted
cargo test -p medscale-core vault_log_redaction
```

## Evidence

Archive under `evidence/005-local-private-vault-encryption-recovery/`:

- `git rev-parse HEAD`
- `rustc -V` / toolchain file
- hash of `Cargo.lock`
- SQLCipher version string / admission digests
- keyring crate pins
- fixture corpus hashes
- test command transcripts (Windows + Linux as available)
- explicit limitations: synthetic-only; REAL_PHI unauthorized; no product network; sync-root claim scope; SQLCipher candidate

## Design docs map

| Need | Doc |
|---|---|
| Requirements | [spec.md](./spec.md) |
| Decisions | [research.md](./research.md) / [clarifications.md](./clarifications.md) |
| Types | [data-model.md](./data-model.md) |
| Vault API | [contracts/vault-api.md](./contracts/vault-api.md) |
| Key lifecycle | [contracts/key-lifecycle.md](./contracts/key-lifecycle.md) |
| Tasks | [tasks.md](./tasks.md) |
| Analyze | [analyze-notes.md](./analyze-notes.md) |

## Implement gate

Package is **QUALIFIED**. Start `/speckit.implement` when BUILD_QUEUE shows Spec 004 `CLOSED_CANONICAL` and Spec 005 `READY` (current queue meets this).
