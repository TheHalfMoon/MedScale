# Implementation Plan: Local Private Vault + Encryption + Recovery

**Branch**: `spec/005-local-private-vault-encryption-recovery` | **Date**: 2026-08-25 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/005-local-private-vault-encryption-recovery/spec.md`

## Summary

Qualify pre-PHI local private vault encryption: `EncryptedVault` with SQLCipher metadata + AES-GCM blobs, `medscale-keys` KeyProvider (passphrase + recovery codes + keyring wraps), app-data defaults, sync/remote refusal, single-writer key custody, encrypted backup/restore, retention/destruction, one-way migration from Spec 003 SyntheticVault, and log hygiene—synthetic-only. REAL_PHI remains unauthorized.

## Technical Context

**Language/Version**: Rust stable per workspace `rust-toolchain.toml`

**Primary Dependencies** (pins in research.md):

- rusqlite **0.37.0** (prefer) with `bundled-sqlcipher-vendored-openssl` → SQLCipher **v4.17.0** target; bump to rusqlite **0.40.2** if CI requires
- keyring-core **1.0.0** + windows-native-keyring-store **1.1.0** + linux-keyutils-keyring-store **1.0.0** (+ optional apple-native-keyring-store **1.0.2**)
- aes-gcm **0.11.1**, argon2 **0.5.3**, zeroize **1.9.0**, rand (lock at admission)
- Existing Spec 002/003/004 workspace crates

**Storage**: Encrypted DurableStore (SQLCipher) + sealed BlobStore; claim-scoped app-data defaults; sync roots refused

**Testing**: Unit/integration on Windows+Linux; key-loss; recovery; migration journal; backup/restore; log redaction; lease; wrong-key

**Target Platform**: Windows + Linux CI (macOS optional for Apple keyring)

**Project Type**: Rust workspace — new `medscale-keys`; extend `medscale-storage`, `medscale-contracts`, `medscale-core`

**Performance Goals**: Correctness and fail-closed crypto over throughput; bounded fixtures

**Constraints**: Synthetic-only; DEFAULT_DENY product network; no MESC mutation; no ambient keys to workers; one SQLCipher/SQLite per process; REAL_PHI unauthorized

**Scale/Scope**: Pre-PHI technical qualification of vault/encryption/recovery—not product UI, not legal counsel sign-off, not real PHI authorization

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Rust-owned trusted core: keys + EncryptedVault only in Core Host / storage/keys crates
- [x] Local-first / privacy-first: app-data default, sync refuse, no runtime egress, synthetic-only
- [x] Source/authority discipline: no key/DB handles in IPC; Projection/presentation untouched
- [x] Evidence-before-claims: key-loss, backup, migration, sync-root suites required
- [x] Fail-closed: wrong key, sync root, mid-migration, missing wraps
- [x] MESC/PHI/network anti-scope honored
- [x] Minimal reversible architecture: EncryptedVault trait; SQLCipher preferred; AES-GCM fallback documented exit only

## Project Structure

### Documentation (this feature)

```text
specs/005-local-private-vault-encryption-recovery/
├── spec.md
├── clarifications.md
├── research.md
├── plan.md
├── data-model.md
├── quickstart.md
├── analyze-notes.md
├── tasks.md
├── contracts/
│   ├── vault-api.md
│   └── key-lifecycle.md
└── checklists/
    └── requirements.md
```

### Source Code (repository root — implement phase only; not this planning package)

```text
crates/
  medscale-keys/          # NEW: KeyProvider, KDF, wrap/recovery, zeroize
  medscale-storage/       # EncryptedVault, SQLCipher meta, sealed blobs, migration, claim extend
  medscale-contracts/     # vault/key envelopes + errors
  medscale-core/          # facade capabilities for vault/key ops
docs/engineering/admissions/
  005-sqlcipher-rusqlite.md
  005-keyring.md
  005-crypto-aes-gcm-argon2.md
evidence/
  005-local-private-vault-encryption-recovery/
fixtures/
  synthetic/vault/encryption/   # migration source, recovery, key-loss markers
```

**Structure Decision**: Create `medscale-keys` now (roadmap + FFI/secret boundary). Keep EncryptedVault in `medscale-storage`. Do not implement Spec 006 UI/doctor productization in this unit.

## Implementation approach

1. Write admissions for SQLCipher/rusqlite, keyring, aes-gcm/argon2/zeroize **before** code.
2. Scaffold `medscale-keys` KeyProvider + key classes + passphrase/recovery wrap.
3. Feature `sqlcipher` on storage; EncryptedVault create/open; sealed blobs.
4. Extend claim/sync refusal + app-data location policy.
5. Wire single-writer lease + Core Host key custody tests.
6. Encrypted backup/restore + retention/destroy.
7. One-way migration journal from SyntheticVault.
8. Log redaction / secret non-emission tests.
9. Evidence archive; BUILD_QUEUE closeout only at converge.

## Complexity Tracking

No constitution violations. SQLCipher remains candidate (D2–D4); AES-GCM metadata backend is explicit exit only. REAL_PHI gate unchanged.
