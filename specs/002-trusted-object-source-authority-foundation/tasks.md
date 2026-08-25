# Tasks: Trusted Object / Source / Authority + Process/Text Foundation

**Input**: Design documents from `/specs/002-trusted-object-source-authority-foundation/`

**Prerequisites**: Spec 001 CLOSED_CANONICAL (or equivalent workspace with `medscale-contracts` + `medscale-core`); plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Required by roadmap exit gate — property/serialization and invariant tests included below.

## Phase 1: Setup

**Purpose**: Confirm Spec 001 baseline and module layout

- [ ] T001 Confirm workspace builds and note baseline commit/toolchain in `evidence/002-trusted-object-foundation/BASELINE.md`
- [ ] T002 [P] Create module skeletons under `crates/medscale-contracts/src/{objects,text,envelopes,ffi_policy,worker_policy}/`
- [ ] T003 [P] Create module skeletons under `crates/medscale-core/src/{authority,process,effects,text,validate}/`
- [ ] T004 Admit `serde`/`serde_json` via dependency admission template if not already present; update lockfile

---

## Phase 2: Foundational Types (Blocks Stories)

**Purpose**: Shared IDs, headers, enums before facade logic

- [ ] T005 Define `OpaqueId`, `VaultId`, `DigestSha256`, `ObjectHeader`, `ObjectClass` in `crates/medscale-contracts/src/objects/mod.rs`
- [ ] T006 [P] Define `Realm` fields usage + `authority_scope_id` newtype in `crates/medscale-contracts/src/objects/scope.rs`
- [ ] T007 [P] Define `MedicalTime`, `TimePrecision`, `EffectState`, `PlacementClass` in `crates/medscale-contracts/src/objects/time_effect.rs`
- [ ] T008 Define versioned `AuthorityRequest`/`AuthorityResponse` envelopes in `crates/medscale-contracts/src/envelopes/mod.rs`
- [ ] T009 [P] Add serde round-trip unit tests for headers/enums in `crates/medscale-contracts/src/objects/serde_tests.rs`

**Checkpoint**: Contracts compile; foundational enums serialize

---

## Phase 3: User Story 1 — Durable Object Classes (P1) 🎯 MVP

**Goal**: Distinct durable classes with serialization and no silent coercion

**Independent Test**: `cargo test -p medscale-contracts object_serde` and coercion-fail tests

- [ ] T010 [P] [US1] Implement `SourceRecord` and `DerivedSourceArtifact` types in `crates/medscale-contracts/src/objects/source.rs`
- [ ] T011 [P] [US1] Implement `Proposal` and `ClinicalAssertion` types in `crates/medscale-contracts/src/objects/authority_classes.rs`
- [ ] T012 [P] [US1] Implement `EvaluationRecord`, `ActionAuditRecord`, `Projection` in `crates/medscale-contracts/src/objects/eval_proj_audit.rs`
- [ ] T013 [P] [US1] Implement `IdentityAssertion` and `IdentityMergeDecision` in `crates/medscale-contracts/src/objects/identity.rs`
- [ ] T014 [US1] Add property/serde round-trip tests (≥1 fixture per class) in `crates/medscale-contracts/tests/object_roundtrip.rs`
- [ ] T015 [US1] Add tests that untagged/wrong-class decode fails in `crates/medscale-contracts/tests/object_no_coercion.rs`

**Checkpoint**: US1 complete — classes distinct and round-trippable

---

## Phase 4: User Story 2 — Source vs Derived + Tagged Text (P1)

**Goal**: Immutable source identity ≠ hash; tagged spans + RawByte↔UnicodeScalar

**Independent Test**: `cargo test -p medscale-core text_span_coords`

- [ ] T016 [P] [US2] Implement `CoordinateSystem`, `TextSpan`, representation tags in `crates/medscale-contracts/src/text/mod.rs`
- [ ] T017 [US2] Implement RawByte↔UnicodeScalar conversion + bounds checks in `crates/medscale-core/src/text/convert.rs`
- [ ] T018 [US2] Enforce source immutability helpers (digest verify; no overwrite API) in `crates/medscale-core/src/authority/source_ops.rs`
- [ ] T019 [US2] Tests: happy-path conversion + ≥3 out-of-bounds/fail-closed cases in `crates/medscale-core/tests/text_span_coords.rs`
- [ ] T020 [US2] Test: derived transform cannot mutate source bytes in `crates/medscale-core/tests/source_immutability.rs`

**Checkpoint**: US2 complete — tagged coordinates and source discipline proven

---

## Phase 5: User Story 3 — Single-Writer Core Host + Facade (P1)

**Goal**: In-process Core Host lease + versioned facade without DB/key handles

**Independent Test**: `cargo test -p medscale-core lease_single_writer`

- [ ] T021 [US3] Implement `CoreHostLease` registry/simulator in `crates/medscale-core/src/process/lease.rs`
- [ ] T022 [US3] Implement `CoreFacade` dispatch for acquire/release/ping/read in `crates/medscale-core/src/authority/facade.rs`
- [ ] T023 [US3] Ensure response types never include DB/key handle fields (compile-time review + unit assert) in `crates/medscale-core/src/authority/handles.rs`
- [ ] T024 [US3] Tests: exclusive lease, second acquire `AlreadyHeld`, release, transient-owner path in `crates/medscale-core/tests/lease_single_writer.rs`
- [ ] T025 [P] [US3] Document OS IPC intent mapping in crate rustdoc linking `contracts/authority-facade.md`

**Checkpoint**: US3 complete — single-writer decision enforced in simulator

---

## Phase 6: User Story 4 — Identity, Realm/Scope, Promotion (P1)

**Goal**: Explicit merge only; scope checks; authorized Proposal→Assertion

**Independent Test**: `cargo test -p medscale-core promote_proposal`

- [ ] T026 [US4] Implement in-memory object store scoped by realm/`authority_scope_id` in `crates/medscale-core/src/authority/store.rs`
- [ ] T027 [US4] Implement `CreateIdentityAssertion` + deny silent merge in `crates/medscale-core/src/authority/identity.rs`
- [ ] T028 [US4] Implement `DecideIdentityMerge` requiring authorization in `crates/medscale-core/src/authority/identity.rs`
- [ ] T029 [US4] Implement `PromoteProposal` → `ClinicalAssertion` + `AuditRecord` in `crates/medscale-core/src/authority/promote.rs`
- [ ] T030 [US4] Tests: unauthorized promotion deny; authorized promotion provenance; cross-scope deny; no silent merge in `crates/medscale-core/tests/promote_and_identity.rs`

**Checkpoint**: US4 complete — authority class promotion and identity rules hold

---

## Phase 7: User Story 5 — Effect/Retry Vocabulary (P2)

**Goal**: Effect state machine with UNKNOWN fail-closed

**Independent Test**: `cargo test -p medscale-core effect_unknown_no_retry`

- [ ] T031 [US5] Implement transition table in `crates/medscale-core/src/effects/machine.rs`
- [ ] T032 [US5] Wire `TransitionEffect` on facade in `crates/medscale-core/src/authority/facade.rs`
- [ ] T033 [US5] Tests: legal paths + illegal transitions + UNKNOWN without reconcile denied in `crates/medscale-core/tests/effect_unknown_no_retry.rs`

**Checkpoint**: US5 complete — retry vocabulary frozen

---

## Phase 8: User Story 6 — FFI Hardening + Worker Supervision Stubs (P2)

**Goal**: Validators for FFI admission checklist and worker deny-by-default policy

**Independent Test**: `cargo test -p medscale-core ffi_admission_validate`

- [ ] T034 [P] [US6] Define `FfiAdmissionRecord` fields in `crates/medscale-contracts/src/ffi_policy/mod.rs`
- [ ] T035 [P] [US6] Define `WorkerSupervisionPolicy` + empty grants default in `crates/medscale-contracts/src/worker_policy/mod.rs`
- [ ] T036 [US6] Implement completeness validator + ambient-grant rejection in `crates/medscale-core/src/validate/mod.rs`
- [ ] T037 [US6] Tests: incomplete FFI record fails; P1 cannot be waived by “has FFI”; worker ambient caps denied in `crates/medscale-core/tests/ffi_and_worker_policy.rs`
- [ ] T038 [P] [US6] Add `docs/engineering/FFI_ADMISSION_CHECKLIST.md` pointer to contract (short, references OSS matrix §3)

**Checkpoint**: US6 complete — policy stubs enforceable in types/tests

---

## Phase 9: Polish & Evidence

- [ ] T039 Run full `cargo test --workspace` + fmt/clippy; fix regressions
- [ ] T040 Archive evidence under `evidence/002-trusted-object-foundation/` per quickstart
- [ ] T041 Validate `quickstart.md` commands
- [ ] T042 Update `docs/planning/BUILD_QUEUE.md` on closeout: Spec 002 `CLOSED_CANONICAL`, Spec 003 `READY` (only at converge/merge)
- [ ] T043 Ensure Spec 003 package planning can start; do not implement 003 in this branch

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 → Phase 2 → Phases 3–8 (US1 types before US2–US6 logic)
- US1 (Phase 3) blocks US2–US4 practical wiring (types needed)
- US3 facade useful before US4–US5 facade methods (can stub store early)
- US6 mostly parallel after Phase 2 types

### User Story Dependencies

- **US1**: After Phase 2
- **US2**: After US1 source types
- **US3**: After Phase 2 envelopes; independent of US2
- **US4**: After US1 + US3
- **US5**: After US1 ActionAuditRecord + US3 facade
- **US6**: After Phase 2; parallel with US3–US5

### Parallel Opportunities

- T002/T003; T006/T007; T010–T013; T034/T035 after their prerequisites
- US3 and US6 can proceed in parallel once Phase 2 done
- US2 text work parallel with US3 lease work after US1 source/span types land

---

## Implementation Strategy

### MVP

1. Phases 1–3 (types + serde/property tests)  
2. Phase 5 lease/facade skeleton  
3. Then US2/US4/US5/US6 to satisfy full exit gate

### Notes

- Do **not** implement Spec 001 bootstrap tasks here
- Do **not** add FHIR ingest, vault crypto, or UI
- Prefer modules over new crates
