# Quickstart: Spec 004 H0-B Trusted Presentation + Coverage

## Prerequisites

- Spec 003 `CLOSED_CANONICAL` (trusted ingest, blob visibility, RebuildProjection hooks)
- Spec 002 object/time/span/authority contracts available in workspace
- Rust toolchain from `rust-toolchain.toml`
- Read: [spec.md](./spec.md), [plan.md](./plan.md), [data-model.md](./data-model.md), [research.md](./research.md)

## Scope reminder

This unit is **deterministic LLM-free presentation + coverage** only:

- No real PHI
- No general FHIRPath engine
- No models / LLM Brief generation
- No SQLCipher / KeyProvider / production recovery UX (005)
- No Desktop/CLI product shell / v0 UI integration (006)
- No OpenMed/MESC runtime, OCR/ASR, product network

## After implement (expected commands)

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p medscale-fhir -p medscale-core -p medscale-contracts
cargo test --workspace
```

Focus suites (names illustrative until implement):

```powershell
cargo test -p medscale-fhir extractor_patient
cargo test -p medscale-fhir extractor_observation
cargo test -p medscale-fhir unit_semantics_ucum_subset
cargo test -p medscale-core coverage_absence_conflict
cargo test -p medscale-core timeline_golden_rebuild
cargo test -p medscale-core brief_golden
cargo test -p medscale-core presentation_drilldown
cargo test -p medscale-core presentation_golden_rebuild
```

## Evidence

Archive under `evidence/004-h0b-trusted-presentation-coverage/`:

- `git rev-parse HEAD`
- `rustc -V` / toolchain file
- hash of `Cargo.lock`
- fixture corpus hashes (presentation golden/absence/conflict/units)
- test command transcripts
- `PresentationRulesVersion` + `ucum_subset` pin/digest
- explicit limitations (synthetic-only; no FHIRPath; no models; LLM-free; Projections non-authoritative)

## Design docs map

| Need | Doc |
|---|---|
| Requirements | [spec.md](./spec.md) |
| Decisions | [research.md](./research.md) / [clarifications.md](./clarifications.md) |
| Types | [data-model.md](./data-model.md) |
| Presentation API | [contracts/presentation-api.md](./contracts/presentation-api.md) |
| Extractors / coverage / units | [contracts/extractor-coverage.md](./contracts/extractor-coverage.md) |
| Tasks | [tasks.md](./tasks.md) |
| Analyze | [analyze-notes.md](./analyze-notes.md) |

## Implement gate

Package is **QUALIFIED**. Start `/speckit.implement` when BUILD_QUEUE shows Spec 003 `CLOSED_CANONICAL` and Spec 004 `READY` (current queue meets this).
