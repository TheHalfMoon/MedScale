# Quickstart: Spec 007 OpenMed Capability Absorption / Parity Research

## Prerequisites

- Spec 007 is **RESEARCH_ELIGIBLE** — no models, REAL_PHI, or OpenMed runtime required to work this package
- Read: [spec.md](./spec.md), [plan.md](./plan.md), [research.md](./research.md), [data-model.md](./data-model.md)
- Planning inputs:
  - `docs/planning/OPENMED_PARITY_SURPASS_MATRIX_V2.md`
  - `docs/planning/SOURCE_ACQUISITION_AND_COPY_PLAN.md`
  - `docs/planning/OSS_CODE_ABSORPTION_MATRIX_V2.md`
- Optional: public git clone of OpenMed at commit `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837` for pin verification

## Scope reminder

- Research / matrices / evidence / corpora **design** only
- **No** Rust AI fabric, **no** OpenMed Python import into MedScale, **no** wholesale copy
- **No** REAL_PHI; synthetic corpora only
- Product network remains DEFAULT_DENY; MESC mutation unauthorized
- Spec **008** owns runtime measurement after this unit qualifies

## After research implement (expected outcomes)

```text
evidence/007-openmed-capability-absorption-parity/
  BASELINE.md
  PIN_VERIFICATION.md          # or documented_pin_only note
  CORPUS_REGISTRY.md
  SAUDI_ARABIC_PROGRAM.md

docs/matrices/
  openmed-parity-matrix-v2.2.0.json   # or .md
  openmed-absorption-dispositions.md
  terminology-rights-track.md

third_party/provenance/
  _TEMPLATE-openmed-component.md
```

Optional pin check (research tooling; not product runtime):

```powershell
# Illustrative — only if a disposable clone path is used
git -C <openmed-clone> rev-parse HEAD
# expect: 59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837
```

## Validation checklist (docs)

1. Baseline fields match SOURCE_ACQUISITION exactly
2. Parity matrix covers all V2 capability rows
3. Absorption table has zero wholesale-fork authorizations
4. Terminology track lists minimum systems; `copy_from_openmed=false`
5. Saudi/Arabic program has ≥4 trap classes; surpass gated
6. Workspace Cargo.toml has no new OpenMed runtime dependency from this unit

## Design docs map

| Need | Doc |
|---|---|
| Requirements | [spec.md](./spec.md) |
| Decisions | [research.md](./research.md) / [clarifications.md](./clarifications.md) |
| Schemas | [data-model.md](./data-model.md) |
| Baseline | [contracts/openmed-baseline-pin.md](./contracts/openmed-baseline-pin.md) |
| Corpora | [contracts/parity-corpus.md](./contracts/parity-corpus.md) |
| Absorption | [contracts/donor-absorption.md](./contracts/donor-absorption.md) |
| Terminology | [contracts/terminology-licensing-track.md](./contracts/terminology-licensing-track.md) |
| Saudi/Arabic | [contracts/saudi-arabic-benchmark.md](./contracts/saudi-arabic-benchmark.md) |
| Claims | [contracts/benchmark-manifest.md](./contracts/benchmark-manifest.md) |
| Tasks | [tasks.md](./tasks.md) |
| Analyze | [analyze-notes.md](./analyze-notes.md) |

## Anti-commands

Do **not** treat these as Spec 007 success:

```text
cargo add openmed          # forbidden interpretation
pip install openmed        # not MedScale trusted core
# claiming PARITY/SURPASS without BenchmarkManifest
```
