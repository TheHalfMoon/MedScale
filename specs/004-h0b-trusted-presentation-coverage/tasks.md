# Tasks: H0-B Trusted Presentation + Coverage

**Input**: Design documents from `/specs/004-h0b-trusted-presentation-coverage/`

**Prerequisites**: Spec 003 `CLOSED_CANONICAL`; plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Required by roadmap exit gate — bounded typed extractors (no FHIRPath), UCUM/unit where required, negative absence/conflict, golden rebuild, drill-down included below.

**Note**: Planning package is QUALIFIED. Implement on `spec/004-h0b-trusted-presentation-coverage`. Do not implement Spec 005 encryption or Spec 006 product UI in this unit.

## Phase 1: Setup

**Purpose**: Confirm Spec 003 baseline and presentation module layout

- [ ] T001 Confirm Spec 003 closed and workspace builds; note baseline commit/toolchain in `evidence/004-h0b-trusted-presentation-coverage/BASELINE.md`
- [ ] T002 [P] Add presentation module layout under `medscale-fhir` (`extractors/`, `units/`) and `medscale-core` presentation orchestration hooks
- [ ] T003 [P] Extend `medscale-contracts` with CoverageStatus, TimelineEvent, Brief/Coverage body types, DrillDownResult, PresentationRulesVersion
- [ ] T004 Admit UCUM/unit subset pin (hand table or narrow dependency) via dependency/STANDARD admission; record subset id+digest in research/evidence — **do not admit FHIRPath or model crates**
- [ ] T005 [P] Add synthetic presentation fixtures (golden, absence, conflict, units, unsupported resource, quarantine) under `fixtures/synthetic/fhir/r4/presentation/`

**Checkpoint**: Workspace compiles with presentation type skeletons; fixtures present

---

## Phase 2: Foundational Extractors + Units

**Purpose**: Closed typed extractors before facade reads

- [ ] T006 Implement `TypedResourceExtractor` trait + registry (Patient, Observation, Condition only)
- [ ] T007 [P] Implement `patient.v1` extractor with evidence refs + span/citation hooks
- [ ] T008 [P] Implement `observation.v1` extractor including valueQuantity path
- [ ] T009 [P] Implement `condition.v1` extractor
- [ ] T010 Implement `UnitSemantics` for admitted UCUM subset (comparable / incomparable / unrecognized)
- [ ] T011 [P] Unit tests: per-extractor happy paths; unsupported resource → Unsupported/Unknown

**Checkpoint**: Extractors deterministic; units fail closed on unrecognized

---

## Phase 3: User Story 3 — Coverage Accounting (P1)

**Goal**: Present / Absent / Unknown / Conflict / unit honesty

**Independent Test**: `cargo test coverage_absence_conflict`

- [ ] T012 [US3] Implement CoverageSlot builder from extractor outputs
- [ ] T013 [US3] Conflict detection retains ≥2 members; resolution = Unresolved
- [ ] T014 [US3] Map incomparable units → `IncomparableUnits`
- [ ] T015 [US3] Tests: absence fixture → Absent; unsupported → Unknown/Unsupported; conflict fixture → Conflict

**Checkpoint**: US3 — coverage exit tests green

---

## Phase 4: User Story 1 — Deterministic Timeline (P1) 🎯 MVP

**Goal**: Ordered timeline from promoted ClinicalAssertions

**Independent Test**: `cargo test timeline_golden_rebuild`

- [ ] T016 [US1] Implement timeline event mapping from ClinicalAssertions + extractors
- [ ] T017 [US1] Implement deterministic sort (effective → recorded → assertion id) with precision preservation
- [ ] T018 [US1] Exclude Proposals from default timeline events
- [ ] T019 [US1] Tests: ≥3-event golden order; missing effective uses recorded; Proposal-only → empty clinical timeline

**Checkpoint**: US1 — deterministic timeline

---

## Phase 5: User Story 2 — Narrow LLM-Free Brief (P1)

**Goal**: Structured Brief sections only

**Independent Test**: `cargo test brief_golden`

- [ ] T020 [US2] Implement SubjectBriefV1 section assembly (identity, vitals, conditions, coverage_summary)
- [ ] T021 [US2] Wire missing fields to Absent/Unknown per coverage rules
- [ ] T022 [US2] Tests: golden Brief JSON; assert no LLM/model API surface in presentation path

**Checkpoint**: US2 — narrow Brief

---

## Phase 6: User Story 4 — Source Drill-Down (P1)

**Goal**: Field → SourceRecord + TextSpan / citation

**Independent Test**: `cargo test presentation_drilldown`

- [ ] T023 [US4] Implement DrillDownPresentation facade method
- [ ] T024 [US4] Prefer raw-byte TextSpan; WholeResourceCitation for structural fields
- [ ] T025 [US4] Quarantined/missing blob → UnhealthyEvidence / fail closed
- [ ] T026 [US4] Tests: populated fields resolve; quarantine fixture fails closed

**Checkpoint**: US4 — provenance drill-down

---

## Phase 7: User Story 5 — Projection Rebuild (P1)

**Goal**: Rebuildable SubjectTimelineV1 / SubjectBriefV1 / SubjectCoverageV1

**Independent Test**: `cargo test presentation_golden_rebuild`

- [ ] T027 [US5] Extend RebuildProjection for three presentation kinds
- [ ] T028 [US5] Persist Projection with authoritative=false + built_from + rules_version
- [ ] T029 [US5] Canonical JSON body encoding for equality
- [ ] T030 [US5] Golden rebuild twice → equal; delta after new assertion

**Checkpoint**: US5 — golden rebuild

---

## Phase 8: User Stories 6–7 — FHIRPath Ban + UCUM Evidence (P1)

**Goal**: Roadmap exit proofs for extractor-only + units

**Independent Test**: dependency/evidence checks + unit suite

- [ ] T031 [US6] Document and test that no FHIRPath engine dependency is linked for H0-B presentation
- [ ] T032 [US7] Unit suite: incompatible units not equal; unrecognized no silent convert
- [ ] T033 [US6/US7] Archive extractor version + ucum_subset digest in evidence

**Checkpoint**: US6–US7 — exit gate proofs

---

## Phase 9: Facade Wiring + Polish

- [ ] T034 Wire GetTimeline / GetBrief / GetCoverage to facade envelopes in `medscale-contracts` + `medscale-core`
- [ ] T035 Integration test: ingest synthetic fixtures (003 path) → promote → presentation reads
- [ ] T036 Run full `cargo test --workspace` + fmt/clippy; fix regressions
- [ ] T037 Archive evidence under `evidence/004-h0b-trusted-presentation-coverage/` per quickstart
- [ ] T038 Validate `quickstart.md` commands
- [ ] T039 Update `docs/planning/BUILD_QUEUE.md` on closeout: Spec 004 `CLOSED_CANONICAL`, Spec 005 `READY` (only at converge/merge)
- [ ] T040 Ensure Spec 005/007 planning can proceed; do not implement encryption or product UI in this branch

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 → Phase 2 → Phase 3 (coverage uses extractors)
- Phase 4–5 after Phase 2–3 (timeline/Brief need extractors + coverage)
- Phase 6 after Phase 4–5 (drill-down needs fields)
- Phase 7 after Phase 4–5 (Projection bodies)
- Phase 8 can overlap late Phase 2–3 once units exist
- Phase 9 after Phase 4–7

### User Story Dependencies

- **US6/US7**: Foundation with Phase 2
- **US3**: After Phase 2
- **US1/US2**: After US3 helpers (coverage statuses)
- **US4**: After US1/US2 fields exist
- **US5**: After US1–US3 bodies defined

### Parallel Opportunities

- T002/T003/T005; T007/T008/T009 extractor impls in parallel
- US7 unit tests parallel with US3 conflict tests
- Fixture authoring parallel with trait skeletons

---

## Implementation Strategy

### MVP

1. Phases 1–2 (extractors + units)  
2. Phase 3 coverage + Phase 4 timeline  
3. Phase 5 Brief + Phase 7 rebuild  
4. Phase 6 drill-down + Phase 8 exit proofs  

### Notes

- Do **not** implement Spec 005 encryption or Spec 006 Desktop/CLI product shell
- Do **not** admit FHIRPath engine or LLM/model crates
- Do **not** auto-resolve clinical conflicts
- Prefer closed extractors + smallest UCUM subset
