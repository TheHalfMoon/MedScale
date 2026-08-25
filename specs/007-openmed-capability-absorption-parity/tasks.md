# Tasks: OpenMed Capability Absorption / Parity Research

**Input**: Design documents from `/specs/007-openmed-capability-absorption-parity/`

**Prerequisites**: Spec 007 package (this directory); planning matrices cited in plan.md; BUILD_QUEUE `RESEARCH_ELIGIBLE`

**Tests**: Document/schema consistency; baseline pin field match; anti-scope (no OpenMed runtime dep; no REAL_PHI corpus; no Rust AI fabric in this unit)

**Note**: Package is **QUALIFIED for research implementation** (docs/matrices/evidence). **Do not** implement Rust NER/PII/de-ID runtime. **Do not** import OpenMed into MedScale. **Do not** wholesale-copy OpenMed. **Do not** authorize REAL_PHI.

## Phase 1: Setup + Baseline Pin

**Purpose**: Freeze identity and evidence root

- [x] T001 Create `evidence/007-openmed-capability-absorption-parity/` with README limitations (research-only; synthetic; no runtime)
- [x] T002 [P] Write `evidence/.../BASELINE.md` binding URL, tag `v2.2.0`, commit `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837`, tree `1c949e35b2b8f2ea69da4284b370074fc4bf84ab`
- [x] T003 [P] Attempt pin verification (`git_clone` or archive); write `PIN_VERIFICATION.md` or record `documented_pin_only`
- [x] T004 [P] Add `third_party/provenance/_TEMPLATE-openmed-component.md` (fields from SOURCE acquisition rule)
- [x] T005 Confirm anti-scope in evidence: no Cargo OpenMed dep; REAL_PHI NOT_AUTHORIZED; MESC mutation NO

**Checkpoint**: Baseline pin frozen and cited everywhere

---

## Phase 2: Foundational Matrix Schema Materialization

**Purpose**: Parity matrix instance covering all V2 rows

- [x] T006 Create `docs/matrices/` output (if absent) and `openmed-parity-matrix-v2.2.0.json` (or `.md`) per [data-model.md](./data-model.md)
- [x] T007 Populate **all** OPENMED_PARITY_SURPASS_MATRIX_V2 capability rows with classification, owning_specs, evidence_status=`designed` or `waived`
- [x] T008 Ensure every row’s `baseline_commit` equals the frozen pin
- [x] T009 Mark DEFER/USEFUL rows with explicit waiver objects (non-blocking)
- [x] T010 [P] Consistency check: capability_id catalog in data-model matches matrix instance

**Checkpoint**: 100% V2 row coverage

---

## Phase 3: User Story 2 — Parity Corpora Registry (P1)

**Goal**: Synthetic corpus descriptors + metrics for 007-owned families

**Independent Test**: CORPUS_REGISTRY lists minimum families from `contracts/parity-corpus.md`

- [x] T011 [US2] Write `evidence/.../CORPUS_REGISTRY.md` with descriptors for NER, PII, de-ID, multilingual, Saudi/Arabic design, terminology-contract, unicode-offset handoff
- [x] T012 [US2] For each corpus: `phi_class=synthetic_only`, metrics, anti_metrics including `raw_model_count`
- [x] T013 [US2] Link `corpus_ids` into parity matrix rows
- [x] T014 [US2] Optional: seed minimal synthetic fixture stubs under `evidence/.../fixtures/` (no REAL_PHI)
- [x] T015 [US2] Document harness_owner_spec=`008` for measurement execution

**Checkpoint**: Corpora designed; measurement deferred to 008 without blocking design complete

---

## Phase 4: User Story 3 — Donor Absorption Dispositions (P1)

**Goal**: Disposition every SOURCE OpenMed family + 007-relevant OSS donors

**Independent Test**: dispositions table; zero wholesale-fork ops

- [x] T016 [US3] Emit `docs/matrices/openmed-absorption-dispositions.md` from `contracts/donor-absorption.md`
- [x] T017 [US3] Cover all SOURCE_ACQUISITION OpenMed capability-family rows with operation + owning_spec + forbidden list
- [x] T018 [US3] Disposition Presidio, medSpaCy, scispaCy/QuickUMLS, Snowstorm, terminology assets per OSS matrix
- [x] T019 [US3] Explicit `DO_NOT_COPY` block: wholesale fork, Python trusted core, terminology tables, service plane as broker
- [x] T020 [US3] Record placement handoff (P1 for complex hostile/model paths) for Spec 008/010

**Checkpoint**: Absorption map complete; no runtime import authorized

---

## Phase 5: User Story 4 — Terminology / Licensing Track (P1)

**Goal**: Start rights track + EXTERNAL_GATES hooks

**Independent Test**: terminology-rights-track lists minimum systems; copy_from_openmed=false

- [x] T021 [US4] Write `docs/matrices/terminology-rights-track.md` with SNOMED, LOINC, UCUM, ICD, ATC, UMLS/Athena
- [x] T022 [US4] Define Pack metadata field list (version, checksum, rights_uri, NOTICE, expiry/revocation)
- [x] T023 [US4] Update `docs/planning/EXTERNAL_GATES.md` with terminology license gate rows (OPEN; non-blocking for 007 research close)
- [x] T024 [US4] Affirm never-copy-from-OpenMed rule in track + evidence limitations
- [x] T025 [US4] Note Snowstorm REFERENCE-only; no local terminology server requirement

**Checkpoint**: Track started; counsel gates recorded without stopping research

---

## Phase 6: User Story 5 — Saudi / Arabic Program Design (P1)

**Goal**: Trap taxonomy + metrics + surpass gate

**Independent Test**: SAUDI_ARABIC_PROGRAM.md has ≥4 trap classes; surpass forbidden until exact-head

- [x] T026 [US5] Write `evidence/.../SAUDI_ARABIC_PROGRAM.md` per `contracts/saudi-arabic-benchmark.md`
- [x] T027 [US5] Define ≥4 trap classes (MSA/dialect, code-switch, synthetic national-ID, critical-number; plus entity/RTL as applicable)
- [x] T028 [US5] Register metrics and link corpus `om-saudi-arabic-design-v0`
- [x] T029 [US5] Encode surpass_policy=`forbidden_until_exact_head_benchmark` against pinned OpenMed commit
- [x] T030 [US5] Optional seed synthetic Arabic/PII trap examples (synthetic only)

**Checkpoint**: Program designed for Spec 008 measurement

---

## Phase 7: Benchmark Manifest + Claim Discipline

**Goal**: Claim envelope ready for 008+

- [x] T031 Publish BenchmarkManifest field checklist under evidence (mirror `contracts/benchmark-manifest.md`)
- [x] T032 Document forbidden claim shortcuts (raw model count; unpinned tip floor; surpass without evidence)
- [x] T033 [P] Cross-link PRIVACY_PROOF / offline rows as SURPASS_PENDING where matrix requires

**Checkpoint**: No PARITY/SURPASS claim possible without manifest schema awareness

---

## Phase 8: Polish + Research Closeout Prep

- [x] T034 Validate quickstart paths and matrix/evidence file presence
- [x] T035 Grep/confirm workspace has no OpenMed product runtime dependency introduced by this unit
- [x] T036 Re-read analyze-notes gates; ensure SC-001…SC-007 satisfied by artifacts
- [ ] T037 Update `docs/planning/BUILD_QUEUE.md` on converge only: Spec 007 research `CLOSED_CANONICAL` / unlock Spec 008 dependency as appropriate—**not during planning package authoring**
- [x] T038 Do **not** start Spec 008 Rust fabric in this unit

---

## Dependencies & Execution Order

```text
Phase 1 (baseline)
  -> Phase 2 (matrix)
  -> Phase 3 (corpora) || Phase 4 (absorption) || Phase 5 (terminology)  [parallel after matrix]
  -> Phase 6 (Saudi/Arabic) depends on Phase 3 corpus slots
  -> Phase 7 (manifest) after matrix+corpora
  -> Phase 8 closeout
```

### Parallel opportunities

- T002–T005 parallel in Phase 1
- T011–T015 after T006–T010
- T016–T020 || T021–T025 after Phase 2
- T026–T030 after T011 corpus id exists

### User story mapping

| Story | Tasks |
|---|---|
| US1 Baseline | T001–T005 |
| US2 Corpora | T006–T015 |
| US3 Absorption | T016–T020 |
| US4 Terminology | T021–T025 |
| US5 Saudi/Arabic | T026–T030 |
| Claims / closeout | T031–T038 |

## Implementation notes

- Prefer Markdown + JSON matrices over new Rust crates.
- If OpenMed clone is unavailable, proceed with `documented_pin_only` and continue all other tasks.
- First real `COPY_BOUNDED` bytes belong to later owning specs with provenance—not required to close 007 research if dispositions and templates exist.
