# Research: Spec 004 H0-B Trusted Presentation + Coverage

**Date**: 2026-08-25  
**Spec**: `004-h0b-trusted-presentation-coverage`

## Decisions

### D1 — Presentation inputs (promoted only by default)

- **Decision**: Default Timeline and Brief clinical content are derived only from **ClinicalAssertions** (promoted) with evidence links to **SourceRecords**. Unpromoted Proposals and validator EvaluationRecords are excluded from default clinical events/sections. Optional “candidates” view may list Proposals later but is **out of H0-B MVP** unless a task explicitly adds a non-default read.
- **Alternatives**: (a) Mix Proposals into Brief; (b) Treat validator Pass as clinical Present.
- **Rationale**: Constitution `Proposal != ClinicalAssertion`; validators are evidence only (Spec 003 D5); master plan H0-B “promoted canonical resources.”

### D2 — No general FHIRPath engine

- **Decision**: Closed set of **typed per-resource extractors** with fixed structural paths in Rust. **Do not admit** a general FHIRPath library, expression evaluator, or user-supplied path language in H0-B. Revisit only in Spec 011/013 with real requirement (GLM F-14).
- **Alternatives**: fhirpath crate now; JSONPath; OpenMed path helpers wholesale.
- **Rationale**: MASTER_BUILD_PLAN §16; roadmap Spec 004 exit; decision defaults 7/9 (bounded, reversible).

### D3 — Admitted extractor set (H0-B)

- **Decision**: Admit extractors for:
  - `Patient` — demographics subset (id evidence, name, birthDate, gender as available)
  - `Observation` — code, value[x] quantity/string/codeable, effective/issued times, status
  - `Condition` — code, clinicalStatus, verificationStatus, onset/recorded times
  - Additional FHIR resource types → `UnsupportedResourceType` / coverage `Unknown` until research amendment
- **Alternatives**: Extract all R4 resources; copy OpenMed summary breadth.
- **Rationale**: Smallest reversible set that proves timeline/Brief/coverage/unit/conflict; OpenMed breadth is Spec 007 parity research, not H0-B load-bearing.

### D4 — Timeline ordering

- **Decision**: Total order for timeline events:
  1. `effective_time` if present (with precision-aware compare rules that do **not** invent false Instant ordering across coarse precisions—document as “sortable key = (precision_rank, truncated_value)”; incomparable coarse buckets keep secondary keys)
  2. else `recorded_time`
  3. `ClinicalAssertion.id` lexicographic as final tie-breaker
- Preserve `TimePrecision` and `approximate` on each event; never upgrade precision.
- **Alternatives**: Recorded-only order; “latest wins” collapse.
- **Rationale**: Spec 002 time semantics; OPENMED_PARITY time supersession row; fail-closed honesty.

### D5 — Coverage status vocabulary

- **Decision**:
  - `Present` — extractor found value in promoted inputs
  - `Absent` — admitted extractor ran; no matching value/resource
  - `Unknown` — concept not evaluated or extractor unsupported for resource/path
  - `Conflict` — ≥2 disagreeing Present values under typed claim key
  - `IncomparableUnits` — quantities cannot be compared under admitted UCUM subset
  - `UnhealthyEvidence` — cited source quarantined/missing (fail closed for drill-down)
- **Alternatives**: Binary found/missing; silent latest-wins.
- **Rationale**: Roadmap negative/absence/conflict tests; constitution honesty.

### D6 — UCUM / unit semantics (bounded)

- **Decision**: Pin a **small admitted UCUM subset** sufficient for H0-B vitals/quantity fixtures (e.g. kg/g, cm/m, Cel/[degF] only if conversion rules are pinned and tested—otherwise treat as incomparable). Implement behind `UnitSemantics` trait. Unrecognized unit → no silent conversion. **Do not** copy OpenMed/SNOMED/LOINC tables. Full terminology Packs remain Spec 007+.
- **Alternatives**: Full UCUM library now; ignore units; float-compare numbers only.
- **Rationale**: Roadmap “UCUM/unit semantics where required”; F-04 narrowing; SOURCE_ACQUISITION terminology rules.

### D7 — Brief shape (narrow)

- **Decision**: `SubjectBriefV1` fixed sections:
  - `identity` (Patient extractor)
  - `vitals` (Observation subset keyed by admitted code list in fixtures/research pin)
  - `conditions` (Condition extractor list, conflicts retained)
  - `coverage_summary` (counts by CoverageStatus)
- No narrative paragraph generation. No LLM. Section keys are closed enums/strings.
- **Alternatives**: Free-text summary; OpenMed-width document Brief.
- **Rationale**: “Narrow LLM-free Brief”; useful offline wedge before models.

### D8 — Projection kinds and rebuild

- **Decision**: Extend Spec 003 `RebuildProjection` with kinds:
  - `SubjectTimelineV1`
  - `SubjectBriefV1`
  - `SubjectCoverageV1`
  Stub kinds from 003 remain valid. Rebuild is deterministic given sorted `built_from` inputs + canonical JSON body encoding (sorted object keys, stable array order by documented keys).
- **Alternatives**: Ephemeral-only views without Projection persistence; single mega-Projection.
- **Rationale**: Projection ≠ truth; golden rebuild exit; Spec 003 hooks.

### D9 — Source drill-down / spans

- **Decision**: Prefer **raw-byte** `TextSpan` on SourceRecord bytes for string/decimal excerpts; structural fields may use `WholeResourceCitation { source_id, content_digest, json_pointer_like_path_note }` where byte span is impractical—path note is **documentation aid only**, not FHIRPath execution. Coordinate conversions follow Spec 002 text rules when derived text appears (H0-B primarily uses raw source).
- **Alternatives**: Unicode-only spans; no drill-down until Spec 006 UI.
- **Rationale**: GLM F-08; provenance constitutive of H0-B.

### D10 — Conflict non-resolution

- **Decision**: H0-B **detects and surfaces** conflicts; does **not** authorize clinical conflict resolution or auto-promote winners. Later Spec 011 may own richer conflict/freshness intelligence without changing H0-B honesty baseline.
- **Alternatives**: Latest-recorded wins; confidence-based pick.
- **Rationale**: Fail closed; relevance ≠ authority foreshadowing.

### D11 — Crate / module placement

- **Decision**: Extractors + UCUM helper in `medscale-fhir`; facade methods in `medscale-core`; types in `medscale-contracts`. No new crate unless compile-time ownership pressure appears.
- **Rationale**: Decision default 9; Spec 003 placement precedent.

### D12 — Fixtures and evidence

- **Decision**: Extend synthetic FHIR R4 fixtures under `fixtures/synthetic/fhir/r4/presentation/` with golden, absence, conflict, unit-incomparable, unsupported-resource, and quarantine drill-down cases. Evidence under `evidence/004-h0b-trusted-presentation-coverage/`.
- **Rationale**: Roadmap exit tests; synthetic-only H0.

### D13 — Dependencies (admission posture)

- **Decision**: Admit only:
  - Existing workspace crates
  - Optional small UCUM/unit helper **or** hand-pinned conversion table for admitted subset (prefer minimal; pin exact revision if external crate)
  - **Do not admit**: FHIRPath engines, model runtimes, networking, SQLCipher, OpenMed/MESC Python, OCR/ASR, licensed terminology dumps
- **Rationale**: SOURCE_ACQUISITION + decision defaults.

### D14 — Relationship to Spec 003

- **Decision**: Consume ingest/visibility/blob verify/RebuildProjection. Do not reopen lexical policy, GC, or backup interface design. Presentation reads must respect quarantine/visibility fail-closed.
- **Rationale**: One material spec per unit; BUILD_QUEUE 003 CLOSED → 004 READY.

## Clarifications closed (no founder ask)

See `clarifications.md` C1–C12.

## Anti-scope confirmation

| Deferred to | Not in Spec 004 |
|---|---|
| 003 | Lexical ingest, blob GC, synthetic backup (prerequisite; do not reimplement) |
| 005 | SQLCipher, KeyProvider, key-loss/recovery UX, production backup productization |
| 006 | Desktop/CLI product shell, OS IPC product wiring, v0 visual UI |
| 007+ | OpenMed absorption runtime, broad terminology Packs, MESC artifacts |
| 008+ | Models, packs, workers with ambient authority |
| 011/013 | General FHIRPath revisit, retrieval intelligence, network broker |
| — | Real PHI, product runtime egress, MESC mutation, LLM Brief |

## Evidence expectations at implement time

- `cargo test` suites: golden timeline/Brief/coverage rebuild, absence, conflict, units, unsupported type, drill-down quarantine
- Archive under `evidence/004-h0b-trusted-presentation-coverage/` with commit, toolchain, lock digest, fixture hashes, explicit “no FHIRPath / no models / synthetic-only” limitations
