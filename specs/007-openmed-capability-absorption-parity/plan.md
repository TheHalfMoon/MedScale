# Implementation Plan: OpenMed Capability Absorption / Parity Research

**Branch**: `spec/007-openmed-capability-absorption-parity` | **Date**: 2026-08-25 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/007-openmed-capability-absorption-parity/spec.md`

## Summary

Deliver a **research-only** Spec Kit package that freezes OpenMed **v2.2.0** / `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837` as the competitive floor; materializes phase-scoped parity matrices and corpus descriptors; dispositions donor absorption operations; starts terminology/licensing EXTERNAL_GATES track; designs Saudi/Arabic benchmark program. **No Rust AI runtime, no OpenMed package import, no REAL_PHI, no wholesale copy.** Qualification target: research implementation (docs/matrices/evidence)—not product runtime.

## Technical Context

**Language/Version**: Documentation / schema (Markdown + JSON Schema or typed Markdown tables). No new Rust crates required by this unit.

**Primary Dependencies**: None for product runtime. Optional research tooling: git clone of OpenMed at pinned commit for verification; hash utilities; markdown lint as repo-standard.

**Storage**: Evidence under `evidence/007-openmed-capability-absorption-parity/`; matrices under `docs/matrices/` (or mirrored from this package). No canonical clinical DB changes.

**Testing**: Document consistency checks; schema validation of matrix/corpus JSON (when emitted); baseline pin field match; anti-scope greps (no OpenMed runtime dep in workspace from this unit).

**Target Platform**: N/A (research artifacts). Verification may run on Windows/Linux CI for doc/schema tests only.

**Project Type**: Spec Kit research package + planning matrices/evidence

**Performance Goals**: N/A for inference. Research completeness and claim-discipline (fail closed on PARITY/SURPASS without manifest).

**Constraints**: RESEARCH_ELIGIBLE only; pin exact OpenMed baseline; no moving-branch floor; no OpenMed Python trusted core; no terminology table copy from OpenMed; no REAL_PHI; DEFAULT_DENY product network unchanged; MESC mutation unauthorized; Spec 008 owns runtime.

**Scale/Scope**: Full V2 parity matrix coverage + OpenMed SOURCE rows + terminology track start + Saudi/Arabic program design. Not: NER runtime, pack install, FHIR Network Broker, MESC, mobile packs.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Rust-owned trusted core: no OpenMed Python authority path; future PORT_TO_RUST only in owning specs
- [x] Local-first / privacy-first: synthetic corpora; no hidden egress; REAL_PHI gated
- [x] Source/authority discipline: SOURCE_ACQUISITION operations mandatory; provenance before copy
- [x] Evidence-before-claims: BenchmarkManifest required for PARITY/SURPASS; no raw model-count metric
- [x] Fail-closed: unknown rights → EXTERNAL_GATES; DEFER rows explicit
- [x] MESC/PHI/network anti-scope honored
- [x] Minimal reversible architecture: research contracts only; Spec 008+ implements

## Project Structure

### Documentation (this feature)

```text
specs/007-openmed-capability-absorption-parity/
├── spec.md
├── clarifications.md
├── research.md
├── plan.md
├── data-model.md
├── quickstart.md
├── analyze-notes.md
├── tasks.md
├── contracts/
│   ├── openmed-baseline-pin.md
│   ├── parity-corpus.md
│   ├── donor-absorption.md
│   ├── terminology-licensing-track.md
│   ├── saudi-arabic-benchmark.md
│   └── benchmark-manifest.md
└── checklists/
    └── requirements.md
```

### Research outputs (implement phase — research only; not Rust runtime)

```text
docs/matrices/
  openmed-parity-matrix-v2.2.0.json   # or .md mirror of data-model schema
  openmed-absorption-dispositions.md
  terminology-rights-track.md
evidence/007-openmed-capability-absorption-parity/
  BASELINE.md
  PIN_VERIFICATION.md
  CORPUS_REGISTRY.md
  SAUDI_ARABIC_PROGRAM.md
third_party/provenance/               # templates only in 007; first real records in later COPY specs
  _TEMPLATE-openmed-component.md
```

**Structure Decision**: Keep Spec 007 package self-contained; emit durable matrices/evidence during research implement. Do not add `openmed` crate/path dependency to Cargo workspace.

## Implementation approach (research)

1. Freeze and verify OpenMed baseline pin (commit/tree/tag).
2. Materialize parity matrix schema instances from OPENMED_PARITY_SURPASS_MATRIX_V2.
3. Define corpus descriptors + metric suites (synthetic).
4. Write absorption dispositions for SOURCE OpenMed + relevant OSS donors.
5. Start terminology rights track + EXTERNAL_GATES rows for gated licenses.
6. Design Saudi/Arabic benchmark program (traps, metrics, surpass gate).
7. Document BenchmarkManifest contract for Spec 008+ claims.
8. Archive evidence; mark Spec 007 research QUALIFIED → CLOSED_CANONICAL when tasks done (converge).

## Complexity Tracking

No constitution violations. Research-only scope is intentional: Spec 007 must not become a backdoor OpenMed runtime. Deferred measurement corpora population is expected to continue into Spec 008 evidence without blocking 007 research close if designs and registries are complete.
