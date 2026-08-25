# Research: Spec 005 Local Private Vault + Encryption + Recovery

**Date**: 2026-08-25  
**Spec**: `005-local-private-vault-encryption-recovery`

## Decisions

### D1 — EncryptedVault trait (fail-closed architecture)

- **Decision**: Introduce a durable `EncryptedVault` trait (logical name; Rust trait in `medscale-storage`) that is the **only** production vault open path after Spec 005. All create/open/migrate/backup/restore go through it. Plaintext `SyntheticVault` remains a **migration source / test fixture** only—never a post-005 production durability claim.
- **Alternatives**: (a) Encrypt only blobs, leave SQLite plaintext; (b) Keep SyntheticVault as default.
- **Rationale**: Fail closed (decision default 3); F-10 production encryption before PHI; one authority path.

### D2 — Preferred backend: SQLCipher (metadata) + AES-GCM (blobs)

- **Decision**:
  1. **Metadata / DurableStore**: SQLCipher page encryption via rusqlite.
  2. **Blobs**: AES-256-GCM sealed files under claim-scoped blob store, keyed by vault-scoped BlobEncryption material derived/wrapped from Vault DEK (no cross-vault plaintext dedup assumption—GLM storage correction).
  3. Feature flag: `medscale-storage` feature `sqlcipher` enables the SQLCipher backend (default for Spec 005 product path).
- **Alternatives**: SQLCipher for everything including blobs-as-BLOBs only; whole-vault AES container; age/minisign wrappers.
- **Rationale**: SOURCE_ACQUISITION SQLCipher candidate; Spec 003 already FS blobs; separate blob crypto preserves GC/quarantine semantics without forcing all blobs through DB pages.

### D3 — SQLCipher / rusqlite exact pin (Windows + Linux CI)

| Component | Pin | Notes |
|---|---|---|
| SQLCipher (acquisition target) | **v4.17.0** | `https://github.com/sqlcipher/sqlcipher` tag/release **v4.17.0**; BSD-style license; DEPENDENCY/FFI |
| rusqlite (binding) | **0.37.0** preferred (workspace Spec 003 continuity) | Features: **`bundled-sqlcipher-vendored-openssl`** (not plain `bundled`) for EncryptedVault builds so Windows+Linux CI compile without system SQLCipher/OpenSSL |
| rusqlite bump allowance | **0.40.2** | If 0.37.0 + bundled-sqlcipher fails CI (MSVC/openssl), bump workspace rusqlite to **0.40.2** with the same feature set; amend admissions; keep **one SQLite/SQLCipher copy per process** |
| libsqlite3-sys | whatever rusqlite pin pulls | Record exact crate version + Cargo.lock digests in `docs/engineering/admissions/005-sqlcipher-rusqlite.md` **before** first encrypted-open code lands |

**Implement-time verification (mandatory)**:

1. Confirm embedded SQLCipher version string / vendor CHANGELOG equals **4.17.0**, OR
2. If rusqlite’s vendored tree is a different SQLCipher 4.x, either:
   - patch/vendor **exact** sqlcipher **v4.17.0** sources into the build with provenance, **or**
   - amend the admission pin to the **exact tested** SQLCipher revision/SHA shipped by that rusqlite release (still SQLCipher 4.x page crypto)—document delta from acquisition row without asking founder.

**CI posture**: Prefer vendored OpenSSL feature to avoid fragile system library discovery on Windows runners. Linux uses the same feature for homogeneity.

### D4 — Exit strategy if SQLCipher bundling is non-viable in one PR

- **Decision**: Architecture stays fail-closed:
  1. `EncryptedVault` trait remains mandatory.
  2. Primary backend = SQLCipher (D2/D3).
  3. **Fallback backend** (only if SQLCipher cannot green on Windows+Linux CI after reasonable fix attempts): `AesGcmSealedStore` — SQLite metadata file remains possible **only if** all sensitive payloads are AES-256-GCM sealed blobs/pages under KeyProvider; **or** replace SQLite metadata with an encrypted sealed document store behind the same trait.
  4. Fallback MUST NOT ship as “encrypted” if any durable clinical/synthetic content remains plaintext at rest.
  5. Record fallback activation in research amendment + EXTERNAL_GATES/BUILD_QUEUE note; keep SQLCipher as preferred re-entry path.
- **Rationale**: User-authorized contingency; SQLCipher remains preferred per acquisition plan.

### D5 — Keyring pins (keyring-core + selected stores)

| Crate | Version | Platform |
|---|---|---|
| `keyring-core` | **1.0.0** | API |
| `windows-native-keyring-store` | **1.1.0** | Windows CI/desktop |
| `linux-keyutils-keyring-store` | **1.0.0** | Linux CI (keyutils) |
| `apple-native-keyring-store` | **1.0.2** | Optional macOS CI; mobile depth → Spec 009 |
| Test store | in-process mock / `keyring-core` sample feature if present | Unit tests without OS secret service |

- **Decision**: Select **only** needed platform stores; set default store explicitly at Core Host startup (`set_default_store`); **no ambient secret API** exposing unrelated credentials. Service/user/account labels namespaced `medscale.vault.<vault_id>`.
- **Alternatives**: Monolithic `keyring` 3.x; roll custom DPAPI-only.
- **Rationale**: SOURCE_ACQUISITION + GLM F-12; Spec 005/009 split.

### D6 — Crypto companion crates (planning pins)

| Crate | Version | Purpose |
|---|---|---|
| `aes-gcm` | **0.11.1** | Blob / wrap payloads |
| `argon2` | **0.5.3** | Passphrase → KEK (Argon2id) |
| `zeroize` | **1.8+ / 1.9.0** | Best-effort key zeroization |
| `rand` | **0.8.x or 0.10.2** | Prefer workspace-compatible; freeze exact at admission |

Admit via `docs/engineering/admissions/005-*.md` before first use. Prefer RustCrypto; no custom cryptography.

### D7 — Key classes and lifecycle

- **Decision**:
  - **VaultDEK**: Data encryption key for SQLCipher key material and/or blob KEK root.
  - **WrapKEK**: Derived from passphrase (Argon2id with vault-unique salt).
  - **RecoveryWrap**: Per recovery code high-entropy secret wrapping VaultDEK.
  - **BlobEncryption**: Derived or wrapped key for AES-GCM blob sealing (vault-scoped).
  - **OS keyring**: Stores **wrapped** VaultDEK (or unlock token), never “convenience plaintext DEK beside DB.”
- Lifecycle: create → wrap → store → unlock → use in Core Host memory → lock/zeroize on close; rotate = re-wrap DEK (SQLCipher rekey if supported) with tests.
- **Rationale**: Key class separation (master plan); minimum disclosure.

### D8 — Recovery model

- **Decision**: Passphrase **and** recovery codes (recommend generate **10** single-use or multi-use codes—implement pins **multi-use wrapped codes** with optional revoke list for MVP simplicity; document). Both unwrap VaultDEK. Key-loss proof: remove keyring + forget passphrase + destroy recovery wraps → ciphertext unreadable.
- **Alternatives**: Shamir; hardware token only; passphrase-only.
- **Rationale**: Usable recovery without cloud; F-10.

### D9 — Vault location + sync/remote refusal

- **Decision**: Default root = OS local app-data (e.g. Windows `%LOCALAPPDATA%\MedScale\vaults\<id>`, Linux `$XDG_DATA_HOME/medscale/vaults/<id>` or `~/.local/share/medscale/vaults/<id>`). Extend Spec 003 `assert_claim_path` markers: `onedrive`, `dropbox`, `google drive`, `icloud`, `\\remote\\`, `/mnt/sync`, plus documented additions (`box sync`, `mega`, OneDrive localized folder names where detectable). Refuse on create/open/migrate/backup dest. Do not claim OneDrive-specific corruption without proof—refusal is for locking/sync risk and claim scope (F-03).
- **Rationale**: Roadmap; Spec 003 extension.

### D10 — Single-writer + Core Host key custody

- **Decision**: Per Spec 002/roadmap: one Core Host per open vault owns writable SQLCipher connection and active DEK in memory. Lease file/lock under vault root. Clients are IPC-only (006 wires product IPC; 005 tests lease in-process). No DB/key handles in messages.
- **Rationale**: F-05; SPECKIT topology.

### D11 — Backup / snapshot / retention

- **Decision**: Encrypted backup artifact = manifest + ciphertext metadata snapshot + sealed blobs + **public** wrap metadata (salts, KDF params, wrapped DEK blobs)—**never** raw DEK. Restore requires unwrap. Retention/destroy: shred/overwrite best-effort + delete keyring entries + invalidate wraps; audit without secrets. Snapshot implications: snapshots are ciphertext; cloning vault directory without wraps ≠ readable vault.
- **Rationale**: Extends Spec 003 BackupRestore into encrypted posture.

### D12 — Migration SyntheticVault → EncryptedVault

- **Decision**: One-way `MigrateToEncryptedVault`:
  1. Validate claim path; refuse sync roots.
  2. Provision keys (passphrase + recovery codes + keyring wrap).
  3. Journal states: `Started` → `CopiedSealed` → `Switched` → `TombstonedPlain` → `Done`.
  4. On success, open path is EncryptedVault only; plaintext DB/files tombstoned or removed per journal.
  5. Crash: resume journal; never PASS half-migrated as encrypted.
- **Rationale**: Founder-required upgrade path; fail closed.

### D13 — Crate placement

- **Decision**: Create **`crates/medscale-keys`** (KeyProvider, KDF, wrap/unwrap, recovery, zeroize). Extend **`medscale-storage`** (EncryptedVault, SQLCipher meta, sealed blobs, migration, claim). Extend **`medscale-contracts`** (vault/key envelopes, errors). Orchestrate in **`medscale-core`**. Optional thin CLI debug hooks only—not Spec 006 product shell.
- **Rationale**: Roadmap crate topology; unsafe/FFI boundary for SQLCipher + keyring.

### D14 — Admission / provenance pattern

- **Decision**: Before first donor/dependency line for SQLCipher/keyring/crypto, create:
  - `docs/engineering/admissions/005-sqlcipher-rusqlite.md`
  - `docs/engineering/admissions/005-keyring.md`
  - `docs/engineering/admissions/005-crypto-aes-gcm-argon2.md`
  Binding exact URL/version/features/license/placement/tests/update/exit/SBOM per SOURCE_ACQUISITION.
- **Rationale**: Spec 003 rusqlite admission precedent; SQLCipher not yet admitted.

### D15 — REAL_PHI / anti-scope

- **Decision**: All proofs use synthetic markers/fixtures. Completing Spec 005 **does not** flip REAL_PHI. No MESC mutation. No product runtime network. No OpenMed runtime. Log/crash hygiene tests are technical only; full PRIVACY_PROOF artifact productization is Spec 006.
- **Rationale**: EXTERNAL_GATES; roadmap row.

### D16 — Relationship to Spec 003 / 004

- **Decision**: Consume Spec 003 DurableStore/BlobStore/Backup concepts; encrypt and harden. Do not reopen Spec 004 presentation semantics. Presentation continues via Core Host over encrypted store after migration.
- **Rationale**: One material unit; BUILD_QUEUE 004 CLOSED → 005 READY.

## Clarifications closed (no founder ask)

See `clarifications.md` C1–C12.

## Anti-scope confirmation

| Deferred to | Not in Spec 005 |
|---|---|
| 003 | Unencrypted SyntheticVault as long-term production store (superseded after migrate) |
| 004 | Timeline/Brief/extractors (consume only) |
| 006 | Desktop/CLI product shell, v0 UI, full `medscale doctor`, PRIVACY_PROOF packaging |
| 007+ | OpenMed/MESC runtime, terminology packs |
| 008+ | Models, packs, workers with ambient keys |
| 009 | Mobile Keychain/Keystore depth, 16KB, silent key sync bans on device |
| 013 | Network Broker / any online egress |
| — | Real PHI, counsel PDPL/SFDA legal opinion as code gate, custom cryptography |

## Evidence expectations at implement time

- `cargo test` suites: encrypt round-trip, wrong-key, sync-root refuse, lease, recovery, key-loss, encrypted backup/restore, migration journal crash, log redaction
- Windows + Linux CI build with `sqlcipher` feature
- Admissions + Cargo.lock digests + SQLCipher version string
- Archive under `evidence/005-local-private-vault-encryption-recovery/` with explicit limitations (synthetic-only; REAL_PHI unauthorized; no product network; SQLCipher candidate claim scope)
