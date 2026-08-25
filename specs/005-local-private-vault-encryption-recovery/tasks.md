# Tasks: Local Private Vault + Encryption + Recovery

**Input**: Design documents from `/specs/005-local-private-vault-encryption-recovery/`

**Prerequisites**: Spec 004 `CLOSED_CANONICAL`; plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Required by roadmap exit — encryption round-trip, sync-root refuse, single-writer/key lifecycle, snapshot/retention/key-loss/restore, migration, log hygiene; synthetic-only.

**Note**: Planning package is QUALIFIED. Implement on `spec/005-local-private-vault-encryption-recovery`. Do not authorize REAL_PHI. Do not implement Spec 006 product UI/doctor packaging in this unit.

## Phase 1: Setup + Admissions

**Purpose**: Provenance before first encrypted/dependency line; crate scaffolds

- [ ] T001 Confirm Spec 004 closed and workspace builds; note baseline commit/toolchain in `evidence/005-local-private-vault-encryption-recovery/BASELINE.md`
- [ ] T002 [P] Create admission `docs/engineering/admissions/005-sqlcipher-rusqlite.md` binding SQLCipher **v4.17.0** target + rusqlite **0.37.0** / `bundled-sqlcipher-vendored-openssl` (bump allowance **0.40.2**) per research D3
- [ ] T003 [P] Create admission `docs/engineering/admissions/005-keyring.md` for keyring-core **1.0.0**, windows-native-keyring-store **1.1.0**, linux-keyutils-keyring-store **1.0.0** (+ optional apple **1.0.2**)
- [ ] T004 [P] Create admission `docs/engineering/admissions/005-crypto-aes-gcm-argon2.md` for aes-gcm **0.11.1**, argon2 **0.5.3**, zeroize **1.9.0**, rand pin
- [ ] T005 Scaffold `crates/medscale-keys` in workspace; wire empty KeyProvider module + lints
- [ ] T006 [P] Add `medscale-storage` feature `sqlcipher` and EncryptedVault module stubs; keep SyntheticVault for migration source
- [ ] T007 [P] Extend `medscale-contracts` with vault/key envelopes, SyncRootRefused, MissingKeyMaterial, EncryptionProfile types
- [ ] T008 [P] Add synthetic encryption fixtures under `fixtures/synthetic/vault/encryption/` (markers, migration source vault recipe)

**Checkpoint**: Admissions present; workspace compiles with stubs; no plaintext “production encrypted” claim yet

---

## Phase 2: Foundational Keys + Claim Path

**Purpose**: KeyProvider + location/sync policy before vault IO

- [ ] T009 Implement key classes + WrappedKeyRecord + Argon2id passphrase wrap/unwrap in `medscale-keys`
- [ ] T010 [P] Implement recovery code generate/wrap/unlock + revoke
- [ ] T011 [P] Integrate keyring-core + platform stores behind KeyProvider; mock store for unit tests
- [ ] T012 Extend Spec 003 claim markers + `ResolveDefaultVaultPath` LocalAppData policy; refuse sync/remote roots
- [ ] T013 [P] Tests: passphrase/recovery round-trip; keyring mock; sync-root refuse; app-data default resolution

**Checkpoint**: Keys wrap/unwrap; sync roots refused; no founder ask

---

## Phase 3: User Story 1 — EncryptedVault SQLCipher + Sealed Blobs (P1) 🎯 MVP

**Goal**: Create/open encrypted vault; wrong key fails; markers round-trip

**Independent Test**: `cargo test encrypted_vault_roundtrip`

- [ ] T014 [US1] Wire rusqlite sqlcipher features; verify/record embedded SQLCipher version vs **4.17.0** (amend admission if needed)
- [ ] T015 [US1] Implement EncryptedVault create/open/close with SQLCipher metadata DurableStore
- [ ] T016 [US1] Implement SealedBlobStore AES-256-GCM put/get/verify
- [ ] T017 [US1] Tests: write marker → reopen with keys → read; wrong/missing keys → fail closed; marker absent as plaintext on disk

**Checkpoint**: US1 — encryption at rest proof

---

## Phase 4: User Stories 2–3 — Sync Refusal + Single-Writer Key Custody (P1)

**Goal**: Claim scope + lease + no keys in IPC

**Independent Test**: `cargo test vault_sync_root_and_lease`

- [ ] T018 [US2] Enforce sync/remote refusal on create/open/migrate/backup dest (extend markers)
- [ ] T019 [US3] Enforce single-writer lease for EncryptedVault; second writer → LeaseHeld
- [ ] T020 [US3] Ensure Authority envelopes / debug dumps contain no DEK/passphrase/SQLCipher secrets (negative tests)
- [ ] T021 [US2/US3] Tests: OneDrive/Dropbox/iCloud/Google Drive paths refused; lease conflict; secret non-emission

**Checkpoint**: US2–US3 green

---

## Phase 5: User Story 4 — Recovery + Key-Loss Proof (P1)

**Goal**: Passphrase + recovery codes; full key-loss unreadable

**Independent Test**: `cargo test vault_recovery_key_loss`

- [ ] T022 [US4] Unlock paths: OS keyring → passphrase → recovery code
- [ ] T023 [US4] Simulate keyring wipe; unlock via passphrase and via recovery code
- [ ] T024 [US4] Key-loss: destroy all wraps → open fails + plaintext marker scan fails on DB/blobs
- [ ] T025 [US4] Tests archive recovery evidence notes (synthetic-only)

**Checkpoint**: US4 — recovery/key-loss exit

---

## Phase 6: User Story 5 — Encrypted Backup / Retention (P1)

**Goal**: Backup/restore with keys; retention destroys usability

**Independent Test**: `cargo test encrypted_backup_restore_retention`

- [ ] T026 [US5] Implement BackupEncryptedVault / RestoreEncryptedVault with wrap metadata in manifest
- [ ] T027 [US5] Restore without unlock material fails closed; with keys matches digests/closure
- [ ] T028 [US5] Implement DestroyVaultKeys scopes + RetentionDestructionRecord (no secrets in audit)
- [ ] T029 [US5] Tests: backup/restore; restore-without-keys; destroy → unreadable

**Checkpoint**: US5 — snapshot/restore/retention proofs

---

## Phase 7: User Story 6 — One-Way Migration from Spec 003 (P1)

**Goal**: SyntheticVault → EncryptedVault journaled upgrade

**Independent Test**: `cargo test migrate_synthetic_to_encrypted`

- [ ] T030 [US6] Implement MigrationJournal state machine + crash checkpoints
- [ ] T031 [US6] Migrate fixture Spec 003 vault; preserve ids/digests; switch authority
- [ ] T032 [US6] Post-migration SyntheticVault open on upgraded root fails closed / superseded
- [ ] T033 [US6] Crash-restart tests: no half-encrypted PASS
- [ ] T034 [US6] Re-migrate already-encrypted → typed AlreadyEncrypted / idempotent Done

**Checkpoint**: US6 — one-way upgrade

---

## Phase 8: User Story 7 — Log / Privacy Hygiene (P1)

**Goal**: Secrets never logged

**Independent Test**: `cargo test vault_log_redaction`

- [ ] T035 [US7] Audit error paths for typed non-secret messages
- [ ] T036 [US7] Capture logs during intentional failures; assert passphrase/recovery/key fixtures absent
- [ ] T037 [US7] Document Spec 005 limitations for later PRIVACY_PROOF (006); do not claim system-wide zero packets

**Checkpoint**: US7 — log hygiene

---

## Phase 9: Facade Wiring + Polish

- [ ] T038 Wire vault/key capabilities into `medscale-core` facade envelopes
- [ ] T039 Integration: encrypted vault → ingest synthetic fixture (003 path) → optional 004 presentation smoke (if cheap) via Core Host
- [ ] T040 Run `cargo test --workspace` + fmt/clippy with `sqlcipher` feature on Windows+Linux CI matrix as available
- [ ] T041 If SQLCipher CI blocked after fixes, activate documented AES-GCM EncryptedVault exit backend (research D4) **without** plaintext claim; amend evidence
- [ ] T042 Archive evidence under `evidence/005-local-private-vault-encryption-recovery/` per quickstart
- [ ] T043 Validate `quickstart.md` commands
- [ ] T044 Update `docs/planning/BUILD_QUEUE.md` on closeout: Spec 005 `CLOSED_CANONICAL`, Spec 006 unblocked (only at converge/merge)
- [ ] T045 Ensure REAL_PHI EXTERNAL_GATES remains NOT_AUTHORIZED; no MESC mutation; no product network clients

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 → Phase 2 → Phase 3 (EncryptedVault needs keys + claim)
- Phase 4 after Phase 3 (lease/open exist)
- Phase 5 after Phase 2–3 (unlock paths)
- Phase 6 after Phase 3–5 (backup needs encrypted vault + keys)
- Phase 7 after Phase 3 (migration needs EncryptedVault)
- Phase 8 after Phase 3–5 (operations to log)
- Phase 9 after Phase 3–8

### User Story Dependencies

- **US1**: Foundation Phase 2–3
- **US2/US3**: After US1 open path
- **US4**: After KeyProvider + EncryptedVault
- **US5**: After US1 + US4 unlock
- **US6**: After US1 (+ keys from US4 provision)
- **US7**: Cross-cutting late tests

### Parallel Opportunities

- T002/T003/T004 admissions; T005/T006/T007 scaffolds
- T009/T010/T011 key paths; T012 claim extend
- US5 backup tests parallel with US6 migration once create/open stable

---

## Implementation Strategy

### MVP

1. Phases 1–2 (admissions + keys + claim)  
2. Phase 3 EncryptedVault round-trip  
3. Phase 5 recovery/key-loss  
4. Phase 7 migration  
5. Phases 4/6/8 exit proofs  

### Notes

- Do **not** flip REAL_PHI authorization
- Do **not** implement Spec 006 Desktop/CLI product shell or full PRIVACY_PROOF packaging
- Do **not** mutate MESC or add product network egress
- Prefer SQLCipher; AES-GCM metadata backend only as documented exit
- Create admissions **before** first dependency line
