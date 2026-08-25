# Feature Specification: OpenMed Capability Absorption / Parity Research

**Feature Branch**: `spec/007-openmed-capability-absorption-parity`

**Created**: 2026-08-25

**Status**: Package complete; `QUALIFIED` for **research implementation** (docs / matrices / evidence / corpora design). **Not** qualified for product runtime, OpenMed import, models, or REAL_PHI.

**Input**: Pin OpenMed **v2.2.0** commit `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837` as the only competitive baseline; define phase-scoped parity corpora and benchmark manifests; disposition all OpenMed/OSS donor absorption rows for MedScale; start the terminology/licensing rights track; design the Saudi/Arabic PII + clinical NER benchmark program. **No runtime OpenMed import into MedScale. Never wholesale-copy OpenMed.**

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Pinned OpenMed Baseline (Priority: P1)

A researcher or implementer can cite a single frozen OpenMed baseline (tag, commit, tree) for all Spec 007 parity and absorption work. Comparisons never drift to `main`, `latest`, or an unpinned release.

**Why this priority**: SOURCE_ACQUISITION / AGENTS.md / OPENMED_PARITY_SURPASS_MATRIX_V2 require exact baseline; moving-branch comparison is forbidden.

**Independent Test**: Baseline pin document records URL, tag `v2.2.0`, commit `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837`, tree `1c949e35b2b8f2ea69da4284b370074fc4bf84ab`; every parity row references that pin.

**Acceptance Scenarios**:

1. **Given** Spec 007 artifacts, **When** any parity or absorption claim is written, **Then** it binds OpenMed commit `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837` (or explicitly waives with documented DEFER).
2. **Given** a temptation to compare against a newer OpenMed tip, **When** research is reviewed, **Then** the tip is recorded only as informational delta—not as the parity floor.
3. **Given** SOURCE_ACQUISITION OpenMed section, **When** baseline is verified, **Then** tree hash and tag match the frozen pin.

---

### User Story 2 - Phase-Scoped Parity Matrix & Corpora (Priority: P1)

MedScale maintains a complete, phase-scoped parity matrix (from OPENMED_PARITY_SURPASS_MATRIX_V2) plus defined corpora/fixture families for REQUIRED_PARITY rows owned by 007/008/010/011, without claiming SURPASS until exact-head evidence exists.

**Why this priority**: Roadmap Spec 007 exit; MASTER_BUILD_PLAN §8; parity is phase-scoped, not “clone everything before H0.”

**Independent Test**: Parity matrix schema instance covers all V2 rows; each row has classification, owning spec, corpus id or waiver; corpus registry lists synthetic-only sources and hash placeholders.

**Acceptance Scenarios**:

1. **Given** OPENMED_PARITY_SURPASS_MATRIX_V2, **When** Spec 007 matrix is published, **Then** every V2 capability row appears with classification and owning spec.
2. **Given** a REQUIRED_PARITY row for 007/008, **When** corpora are defined, **Then** synthetic fixture families, metrics, and anti-metrics (e.g. raw model count) are specified.
3. **Given** a DEFER or USEFUL_PARITY row, **When** matrix is reviewed, **Then** it does not block Spec 007 closeout or invent product runtime.

---

### User Story 3 - Donor Absorption Dispositions (Priority: P1)

Every OpenMed capability family in SOURCE_ACQUISITION and every relevant OSS donor in OSS_CODE_ABSORPTION_MATRIX_V2 has an explicit disposition (`COPY_BOUNDED`, `PORT_TO_RUST`, `REFERENCE_ONLY`, `ARTIFACT_IMPORT`, `DO_NOT_COPY`, etc.) with placement class and owning later spec—without admitting OpenMed Python as trusted runtime.

**Why this priority**: Prevents silent fork / wholesale copy; binds provenance rules before Spec 008+.

**Independent Test**: Absorption disposition table covers SOURCE OpenMed rows + OSS matrix OpenMed/Presidio/medSpaCy/terminology donors; zero rows imply wholesale fork.

**Acceptance Scenarios**:

1. **Given** SOURCE_ACQUISITION OpenMed table, **When** dispositions are recorded, **Then** each family maps to operation + owning spec + MedScale target intent.
2. **Given** “import OpenMed package into MedScale,” **When** disposition is checked, **Then** answer is DO_NOT_COPY / REFERENCE_ONLY for runtime—never DEPENDENCY of OpenMed Python in trusted core.
3. **Given** a future COPY_BOUNDED action, **When** Spec 007 closes, **Then** provenance template and freeze procedure exist before first donor-derived line (actual copy deferred to owning implement specs).

---

### User Story 4 - Terminology / Licensing Track Start (Priority: P1)

Spec 007 starts the terminology rights track: identify SNOMED/LOINC/UCUM/ICD/ATC/UMLS (and related) as Pack inputs with rights/version/checksum gates; record that licensed content is never assumed bundled and never copied from OpenMed; open EXTERNAL_GATES rows for counsel/license acceptance where needed—without blocking research closeout.

**Why this priority**: SOURCE_ACQUISITION §5; OSS matrix terminology rows; Spec 011/013 consume packs later.

**Independent Test**: Terminology track document lists systems, rights posture, pack metadata fields, and external gate IDs; no terminology tables copied from OpenMed.

**Acceptance Scenarios**:

1. **Given** terminology standards list, **When** track starts, **Then** each system has rights class (`open` \| `caller_supplied` \| `gated_license` \| `unknown_pending_counsel`).
2. **Given** OpenMed terminology snapshots in baseline, **When** absorption is considered, **Then** operation is REFERENCE_ONLY / contract PORT—not table COPY.
3. **Given** missing counsel sign-off, **When** Spec 007 closes research, **Then** EXTERNAL_GATES records the blocker; Spec 007 research remains closable.

---

### User Story 5 - Saudi / Arabic Benchmark Program Design (Priority: P1)

Spec 007 designs (does not necessarily fully populate) a Saudi/Arabic PII + clinical NER benchmark program: dialect/code-switch traps, national-ID/entity/number traps, evaluation metrics, corpus rights rules, and surpass-pending-evidence posture—synthetic-first; no REAL_PHI.

**Why this priority**: OPENMED_PARITY row “Saudi/Arabic PII + clinical NER” is REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE; MedScale differentiator.

**Independent Test**: Benchmark program design doc defines trap classes, metric suite, corpus admission rules, and linkage to Spec 008 measurement; no real PHI corpus admitted.

**Acceptance Scenarios**:

1. **Given** OpenMed multilingual baseline at pinned commit, **When** Saudi/Arabic program is designed, **Then** it specifies code-switch, dialect, entity, and critical-number trap families.
2. **Given** surpass claims, **When** design is reviewed, **Then** surpass is forbidden until exact-head MedScale vs pinned OpenMed evidence exists.
3. **Given** REAL_PHI gate, **When** corpus candidates are listed, **Then** only synthetic / rights-cleared public fixtures are eligible in Spec 007; real clinical Arabic remains gated.

### Edge Cases

- OpenMed tip ahead of v2.2.0: informational only; does not change baseline pin.
- Missing local clone of OpenMed: research may proceed from published matrices + documented pin; verification task records clone-or-archive evidence when available.
- Restricted terminology present in OpenMed trees: never copy; rights track only.
- Donor row with ambiguous placement: default to REFERENCE_ONLY + P1 isolation for hostile/complex paths (OSS matrix placement rule).
- Claiming PARITY or SURPASS without benchmark manifest fields: fail closed—claim invalid.
- Spec 008 starting before 007 qualification: blocked by BUILD_QUEUE; 007 research must close contracts first.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Spec 007 MUST pin OpenMed baseline as tag `v2.2.0`, commit `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837`, tree `1c949e35b2b8f2ea69da4284b370074fc4bf84ab`, URL `https://github.com/maziyarpanahi/openmed`.
- **FR-002**: All Spec 007 parity comparisons MUST use that pin; MUST NOT use a moving branch as the floor.
- **FR-003**: Spec 007 MUST publish a phase-scoped parity matrix covering OPENMED_PARITY_SURPASS_MATRIX_V2 rows with classification, owning spec, corpus/waiver, and evidence status.
- **FR-004**: Spec 007 MUST define parity corpora (synthetic fixture families, metrics, harness binding fields) for 007-owned and 007-handoff rows without requiring model runtime in this unit.
- **FR-005**: Spec 007 MUST disposition OpenMed SOURCE_ACQUISITION families and relevant OSS donors with explicit operations and DO_NOT_COPY for wholesale fork / OpenMed Python trusted runtime.
- **FR-006**: Spec 007 MUST start the terminology/licensing track (systems, rights classes, pack metadata schema, EXTERNAL_GATES hooks) without bundling licensed tables.
- **FR-007**: Spec 007 MUST design the Saudi/Arabic PII + clinical NER benchmark program (trap classes, metrics, corpus rules, surpass gate).
- **FR-008**: Spec 007 MUST NOT import OpenMed as MedScale product runtime, MUST NOT wholesale-copy OpenMed, MUST NOT authorize REAL_PHI, MUST NOT introduce product network egress, MUST NOT mutate MESC, MUST NOT implement Rust AI/NER/PII runtime (that is Spec 008+).
- **FR-009**: Spec 007 research deliverables MUST live under `specs/007-...`, `docs/matrices/` (or evidence mirrors), and `evidence/007-openmed-capability-absorption-parity/` as applicable—not under trusted-core crates as runtime.
- **FR-010**: Benchmark / parity claim manifests MUST bind fields from OPENMED_PARITY_SURPASS_MATRIX_V2 §3 (commits, hashes, hardware, metrics, limitations).
- **FR-011**: Provenance procedure for any future COPY_BOUNDED fragment MUST be documented before first donor-derived line in a later owning spec.
- **FR-012**: Spec 007 MAY record informational deltas vs post-v2.2 OpenMed tips but MUST label them non-authoritative for parity floor.

### Key Entities

- **OpenMedBaselinePin**: Frozen URL/tag/commit/tree identity.
- **ParityCapabilityRow**: One matrix row (capability, classification, owning spec, corpus, status).
- **ParityCorpusDescriptor**: Synthetic corpus/fixture family with rights and hash slots.
- **DonorAbsorptionDisposition**: Operation + placement + owning spec + forbidden interpretations.
- **TerminologyRightsTrackEntry**: Terminology system + rights class + pack metadata requirements.
- **SaudiArabicBenchmarkProgram**: Trap taxonomy + metrics + corpus admission + surpass gate.
- **BenchmarkManifest**: Exact-head evidence envelope for any future PARITY/SURPASS claim.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Baseline pin document exists and matches SOURCE_ACQUISITION commit/tree exactly (100% field match).
- **SC-002**: Parity matrix includes 100% of OPENMED_PARITY_SURPASS_MATRIX_V2 V2 rows with classification + owning spec.
- **SC-003**: ≥1 corpus descriptor defined for each REQUIRED_PARITY family owned or co-owned by 007 (NER, PII, de-ID, multilingual, Saudi/Arabic design, terminology contracts, Unicode/offset handoff notes)—synthetic-only.
- **SC-004**: Absorption disposition table covers all SOURCE_ACQUISITION OpenMed rows; 0 rows authorize wholesale OpenMed fork or Python trusted-core runtime.
- **SC-005**: Terminology rights track lists SNOMED, LOINC, UCUM, ICD, ATC, UMLS/Athena (minimum) with rights class + EXTERNAL_GATES linkage where gated.
- **SC-006**: Saudi/Arabic program design names ≥4 trap classes and forbids SURPASS without exact-head evidence.
- **SC-007**: Deliverable introduces no OpenMed runtime dependency, no REAL_PHI corpus, no Rust AI fabric implementation, no MESC mutation, no product network client.

## Assumptions

- Spec 004 (or later) having closed enables RESEARCH_ELIGIBLE work without models/PHI/runtime per BUILD_QUEUE / SPECKIT_MASTER_ROADMAP.
- Spec 008 implements runtimes consuming 007 contracts; 007 does not ship inference.
- OPENMED_PARITY_SURPASS_MATRIX_V2, SOURCE_ACQUISITION_AND_COPY_PLAN, OSS_CODE_ABSORPTION_MATRIX_V2 remain canonical planning inputs.
- Ordinary research choices follow IMPLEMENTATION_DECISION_DEFAULTS; legal/license acceptance is EXTERNAL_GATES only.
- Synthetic fixtures and publicly documented trap patterns are sufficient for Spec 007 design; populating large measured corpora may continue into 008 evidence archives.
- Presidio / medSpaCy / scispaCy remain REFERENCE / selective absorb per OSS matrix—not Spec 007 runtime.
