# Implementation Plan: H0-B Trusted Presentation + Coverage

**Branch**: `spec/004-h0b-trusted-presentation-coverage` | **Date**: 2026-08-25 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/004-h0b-trusted-presentation-coverage/spec.md`

## Summary

Build deterministic, LLM-free presentation over Spec 003 synthetic vault data: typed per-resource extractors → Timeline / narrow Brief / Coverage Projections, honest unknown/absence/conflict/time-precision/unit semantics, source drill-down with tagged spans, and golden deterministic rebuild. No general FHIRPath engine, no models, no real PHI, no product UI shell, no Spec 005 encryption.

## Technical Context

**Language/Version**: Rust stable per workspace `rust-toolchain.toml` (from Spec 001+)

**Primary Dependencies**: Spec 002 contracts/core; Spec 003 storage/fhir ingest + visibility + RebuildProjection; bounded UCUM/unit helper (pin at implement via STANDARD/dependency admission). No FHIRPath engine, no model crates, no network, no SQLCipher.

**Storage**: Read promoted ClinicalAssertions + SourceRecords via Core Host / DurableStore; write only non-authoritative Projection bodies + AuditRecords. No new canonical DB ownership.

**Testing**: Unit extractor tests; golden rebuild; negative absence/conflict/unit/unsupported-type suites; synthetic FHIR fixtures extending Spec 003 corpus

**Target Platform**: Same as Spec 001–003 CI/dev matrix (Windows + Linux as available)

**Project Type**: Rust workspace libraries — extend `medscale-fhir` (extractors), `medscale-contracts` (presentation envelopes), `medscale-core` (facade orchestration); optional thin `presentation` module

**Performance Goals**: Determinism and honesty over throughput; bounded extractor set; documented max events for H0-B fixtures

**Constraints**: Synthetic-only; DEFAULT_DENY product network; LLM-free; Projection ≠ truth; Proposal ≠ Assertion; no general FHIRPath; UCUM subset only

**Scale/Scope**: H0-B presentation + coverage proofs; not Desktop/CLI product shell; not encryption; not terminology server

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Rust-owned trusted core: presentation via Core Host / facade only
- [x] Local-first / privacy-first: no telemetry, no runtime egress, synthetic-only, LLM-free
- [x] Source/authority discipline: extractors cite SourceRecords; Projection non-authoritative; Proposal/validator ≠ Assertion
- [x] Evidence-before-claims: golden rebuild + negative suites required
- [x] Fail-closed: quarantine drill-down, unrecognized units, unsupported types
- [x] MESC/PHI/network/OpenMed anti-scope honored
- [x] Minimal reversible architecture: closed extractor set; no FHIRPath engine; UCUM subset behind narrow interface

## Project Structure

### Documentation (this feature)

```text
specs/004-h0b-trusted-presentation-coverage/
├── spec.md
├── clarifications.md
├── research.md
├── plan.md
├── data-model.md
├── quickstart.md
├── analyze-notes.md
├── tasks.md
├── contracts/
│   ├── presentation-api.md
│   └── extractor-coverage.md
└── checklists/
    └── requirements.md
```

### Source Code (repository root — implement phase only; not this planning PR)

```text
crates/
  medscale-contracts/   # Timeline/Brief/Coverage envelopes, CoverageStatus, DrillDown types
  medscale-core/        # facade: GetTimeline, GetBrief, GetCoverage, DrillDown, RebuildProjection kinds
  medscale-fhir/        # typed extractors (patient, observation, condition) + UCUM subset helper
  medscale-storage/     # read-path only as needed via existing DurableStore (no 004 ownership rewrite)
evidence/
  004-h0b-trusted-presentation-coverage/
fixtures/
  synthetic/fhir/r4/presentation/   # golden + absence + conflict + unit fixtures
```

**Structure Decision**: Prefer extractors in `medscale-fhir` and orchestration in `medscale-core`. Do not create a speculative `medscale-presentation` crate unless module boundaries force it. Do not mutate Spec 002/003 planning packages during 004 planning/implement beyond necessary contract extensions in crates.

## Implementation approach

1. Extend contracts: CoverageStatus, TimelineEvent, Brief body schema, DrillDownResult, presentation Projection kinds.
2. Implement closed typed extractors (Patient, Observation, Condition) with evidence spans.
3. Pin admitted UCUM subset + unit compare/normalize rules behind narrow trait.
4. Wire facade: GetTimeline / GetBrief / GetCoverage / DrillDown + RebuildProjection for presentation kinds.
5. Coverage accounting: Present / Absent / Unknown / Conflict / unit honesty.
6. Golden rebuild + negative suites; archive evidence.
7. Dependency gate: assert no FHIRPath engine / model crates.

## Complexity Tracking

No constitution violations. FHIRPath deferral (D2) and UCUM-subset narrowing (D6) are intentional and documented.
