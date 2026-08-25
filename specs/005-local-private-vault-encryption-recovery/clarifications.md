# Clarification Closeout: Spec 005

**Date**: 2026-08-25  
**Command**: `/speckit.clarify` equivalent (autonomous defaults)

No founder questions. Ambiguities resolved via `docs/planning/IMPLEMENTATION_DECISION_DEFAULTS.md`, constitution, MASTER_BUILD_PLAN_V2 Pre-PHI / Spec 005, SPECKIT_MASTER_ROADMAP_V2 Spec 005 row, GLM53 F-03/F-05/F-10/F-12/F-17, SOURCE_ACQUISITION_AND_COPY_PLAN (SQLCipher / keyring-core), Spec 003 claim/backup anti-scope, and EXTERNAL_GATES (REAL_PHI). Details live in `research.md`.

| ID | Ambiguity | Resolution |
|---|---|---|
| C1 | SQLCipher vs custom AES-GCM-only vault | **Prefer SQLCipher** for metadata via `EncryptedVault` + rusqlite `bundled-sqlcipher-vendored-openssl`; blobs AES-256-GCM under vault-scoped keys. AES-GCM metadata backend is **exit strategy only** if SQLCipher CI is non-viable—never plaintext production claim |
| C2 | Exact SQLCipher / rusqlite pin | Target **SQLCipher v4.17.0**; binding **rusqlite 0.37.0** (workspace continuity) with sqlcipher bundled features; allow bump to **rusqlite 0.40.2** if CI requires—freeze exact vendor SHA in admission |
| C3 | Keyring crates | **keyring-core 1.0.0** + **windows-native-keyring-store 1.1.0** + **linux-keyutils-keyring-store 1.0.0**; Apple store **1.0.2** optional for macOS CI; tests use in-memory/mock store—no ambient secret API |
| C4 | Recovery model | **Passphrase (Argon2id → KEK) + recovery codes** wrapping Vault DEK; OS keyring holds wrapped DEK convenience copy only |
| C5 | Default vault path | **OS local app-data** path class (not Documents/Desktop); refuse sync roots |
| C6 | Sync detection | Extend Spec 003 markers; refuse OneDrive/Dropbox/iCloud/Google Drive/remote markers; fail closed |
| C7 | Migration | **One-way** SyntheticVault → EncryptedVault with journal; no dual-authority after success |
| C8 | REAL_PHI | Spec 005 proves encrypted vault **tech with synthetic data only**; REAL_PHI remains EXTERNAL_GATES NOT_AUTHORIZED |
| C9 | Crate split | Introduce **`medscale-keys`**; EncryptedVault in `medscale-storage`; facade in `medscale-core` |
| C10 | Single-writer | Core Host sole owner of writable connection + active keys; lease refuse second writer |
| C11 | Legal PDPL/SFDA mapping | Technical qualification only; counsel sign-off is external gate—not implement blocker |
| C12 | Spec 006 doctor / PRIVACY_PROOF | Spec 005 provides vault/sync/key hygiene foundations; full `medscale doctor` + PRIVACY_PROOF productization is Spec 006 |

**Outstanding NEEDS CLARIFICATION markers in spec.md**: none

**External gates**: REAL_PHI remains NOT_AUTHORIZED. MESC mutation NO. Product runtime network DEFAULT_DENY. Implementation gate: Spec 004 `CLOSED_CANONICAL` (satisfied per BUILD_QUEUE).
