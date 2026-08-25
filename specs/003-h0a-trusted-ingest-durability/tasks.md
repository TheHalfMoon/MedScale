# Tasks: H0-A Trusted Ingest + Durability

**Input**: Design documents from `/specs/003-h0a-trusted-ingest-durability/`

**Prerequisites**: Spec 002 `CLOSED_CANONICAL`; plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Required by roadmap exit gate — lexical, identity, blob integrity, GC race, crash/fault, backup/restore, migration interrupt included below.

**Note**: Planning-only until Spec 002 closes. Do not implement on a branch that mutates Spec 001/002 packages.

## Phase 1: Setup

**Purpose**: Confirm Spec 002 baseline and storage/fhir module layout

- [x] T001 Confirm Spec 002 closed and workspace builds; note baseline commit/toolchain in `evidence/003-h0a-trusted-ingest-durability/BASELINE.md`
- [x] T002 [P] Create `medscale-storage` crate or modules (`durable`, `blob`, `gc`, `backup`, `migrate`) under chosen layout per research D1/D4
- [x] T003 [P] Create `medscale-fhir` crate or modules (`lexical`, `ingest`, `identity_extract`) for structural parse boundary
- [x] T004 Admit SQLite binding (exact pin) via dependency admission template; enforce one SQLite copy per process; update lockfile — **do not admit SQLCipher**
- [x] T005 [P] Add synthetic FHIR R4 fixtures + adversarial lexical corpus under `fixtures/synthetic/fhir/r4/`

**Checkpoint**: Workspace compiles with storage/fhir skeletons; fixtures present

---

## Phase 2: Foundational Storage Contracts

**Purpose**: Traits, claim paths, blob records before ingest

- [x] T006 Define `BlobRef`, `BlobRecord`, `FilesystemClaimScope`, `IngestReceipt` types in `medscale-contracts`
- [x] T007 [P] Define `DurableStore` + `BlobStore` traits in `medscale-storage`
- [x] T008 Implement claim-scoped path validation (no `..` escape; sync-root refuse/non-claim policy hooks)
- [x] T009 Implement FS blob put (temp → fsync → rename) + get/verify/quarantine
- [x] T010 Implement SQLite (or chosen) metadata DurableStore open under Core Host lease only
- [x] T011 [P] Unit tests: digest verify success/fail → quarantine in storage tests

**Checkpoint**: Blobs verify; paths claim-scoped; store opens only with lease

---

## Phase 3: User Story 1 — Synthetic FHIR Trusted Ingest (P1) 🎯 MVP

**Goal**: Exact-byte SourceRecord custody via facade

**Independent Test**: `cargo test lexical_fhir_ingest` / ingest accept path

- [x] T012 [US1] Implement `IngestFhirSynthetic` facade method wiring Core Host → storage
- [x] T013 [US1] Persist SourceRecord + BlobRef + visibility + IngestReceipt + AuditRecord
- [x] T014 [US1] Idempotent duplicate digest → `Duplicate` without byte mutation
- [x] T015 [US1] Tests: Patient/Observation/Bundle accept with exact-byte round-trip

**Checkpoint**: US1 — trusted custody of synthetic FHIR bytes

---

## Phase 4: User Story 2 — Lexical Safety (P1)

**Goal**: Duplicate-key / decimal / version / truncate fail-closed

**Independent Test**: `cargo test duplicate_key_reject` (+ decimal/version suites)

- [x] T016 [US2] Duplicate JSON key detection → reject/quarantine
- [x] T017 [US2] Decimal/number policy (no silent f64 clinical coerce)
- [x] T018 [US2] FHIR version gate admit only R4 4.0.1
- [x] T019 [US2] Size/depth limits + truncated JSON reject
- [x] T020 [US2] Adversarial fixture suite (≥3 lexical fails) all fail-closed

**Checkpoint**: US2 — lexical exit tests green

---

## Phase 5: User Story 3 — Identity + Validator Evidence (P1)

**Goal**: Evidence-only validator + no silent identity merge

**Independent Test**: `cargo test ingest_identity_no_silent_merge`

- [x] T021 [US3] Emit IdentityAssertion/Proposal from Patient identifiers with evidence_refs
- [x] T022 [US3] `AttachValidatorEvidence` → EvaluationRecord `evidence_only`
- [x] T023 [US3] Fixture oracle path for validator evidence without network
- [x] T024 [US3] Tests: shared identifier ≠ merge; validator never creates ClinicalAssertion

**Checkpoint**: US3 — identity/validator evidence wired

---

## Phase 6: User Story 4 — Visibility + Projection Rebuild (P1)

**Goal**: Blob-first visibility; rebuildable stub Projection

**Independent Test**: blob visibility + rebuild tests

- [x] T025 [US4] `ReadCanonicalVisibility` + read-time verify
- [x] T026 [US4] Corruption injection → quarantine + evidence
- [x] T027 [US4] `RebuildProjection` for `SubjectCustodyStub` / `IngestIndexStub`
- [x] T028 [US4] Deterministic rebuild after Projection delete

**Checkpoint**: US4 — visibility + hooks ready for Spec 004

---

## Phase 7: User Story 5 — GC, Races, Crash/Fault (P1)

**Goal**: Mark/tombstone/sweep + fault injection invariants

**Independent Test**: `cargo test gc_promotion_race` / `crash_fault_ingest`

- [x] T029 [US5] Implement GC mark/tombstone/sweep with epoch state
- [x] T030 [US5] Promotion/ingest pin or re-mark against GC
- [x] T031 [US5] Fault points + crash recovery tests for ingest write path
- [x] T032 [US5] GC↔promotion race suite (zero live-ref deletions)
- [x] T033 [US5] Crash mid-GC recovery tests

**Checkpoint**: US5 — durability race/crash proofs

---

## Phase 8: User Story 6–7 — Migration, FS Claim, Backup/Restore (P1)

**Goal**: Migration interrupt safety; FS claim evidence; synthetic backup interface proof

**Independent Test**: `cargo test backup_restore_closure` / `migration_interrupt`

- [x] T034 [US6] MigrationJournal + idempotent resume on open
- [x] T035 [US6] Kill mid-migration test → safe reopen
- [x] T036 [US6] Record FilesystemClaimScope in evidence template
- [x] T037 [US7] Implement `BackupVault` manifest + artifact layout
- [x] T038 [US7] Implement `RestoreVault` with restore closure check
- [x] T039 [US7] Tampered digest restore fail-closed test
- [x] T040 [US7] Multi-blob end-to-end backup → wipe → restore proof

**Checkpoint**: US6–US7 — synthetic backup interface proven; migration/FS claim documented

---

## Phase 9: Polish & Evidence

- [x] T041 Run full `cargo test --workspace` + fmt/clippy; fix regressions
- [x] T042 Archive evidence under `evidence/003-h0a-trusted-ingest-durability/` per quickstart (include FS claim scope)
- [x] T043 Validate `quickstart.md` commands
- [x] T044 Update `docs/planning/BUILD_QUEUE.md` on closeout: Spec 003 `CLOSED_CANONICAL`, Spec 004 `READY` (only at converge/merge)
- [x] T045 Ensure Spec 004 planning can proceed; do not implement 004 presentation in this branch

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 → Phase 2 → Phase 3 (ingest) → Phase 4 (lexical hardens ingest)
- Phase 5 after Phase 3 (needs SourceRecord)
- Phase 6 after Phase 2–3 (visibility/blobs)
- Phase 7 after Phase 2–3 (GC needs blobs + promotion path)
- Phase 8 after Phase 2–3 (backup needs durable vault)
- Phase 4 can overlap late Phase 3 once accept path exists

### User Story Dependencies

- **US1**: After Phase 2
- **US2**: After US1 ingest entrypoint
- **US3**: After US1
- **US4**: After US1 + blob verify
- **US5**: After US1 + visibility
- **US6/US7**: After durable store + blobs

### Parallel Opportunities

- T002/T003/T005; T006/T007; fixture authoring parallel with trait skeletons
- US3 validator fixture work parallel with US2 lexical once receipts exist
- US7 backup implementation parallel with US5 GC once blob store stable

---

## Implementation Strategy

### MVP

1. Phases 1–3 (custody accept path)  
2. Phase 4 lexical suite  
3. Phase 7–8 durability + backup proofs  
4. Then US3/US4 polish for full exit gate

### Notes

- Do **not** implement Spec 002 foundation tasks here
- Do **not** admit SQLCipher or real PHI
- Do **not** build H0-B Brief/timeline UI
- Prefer traits + smallest reversible SQLite/FS backend
