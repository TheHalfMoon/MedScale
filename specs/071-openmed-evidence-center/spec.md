# Spec 071 — OpenMed Evidence Center

**Status**: IN_PROGRESS
**Base**: `129b63d0d93e8fc14fa0dcd786d95b1ac4697e68`

## Goal

Turn the Evidence route into a fail-closed comparative evidence center bound to the pinned OpenMed v2.2.0 baseline. Every parity, surpass, or equivalent superiority claim must either carry a complete BenchmarkManifest or be explicitly refused.

## Required outcomes

1. Reverify OpenMed `v2.2.0` at commit `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837` and tree `1c949e35b2b8f2ea69da4284b370074fc4bf84ab`; record reproducible source hashes for the comparator documents used by the product surface.
2. Materialize a complete 39-row claim ledger from the canonical parity matrix so no capability silently disappears from comparative accounting.
3. `PARITY`, `SURPASS`, `PRIVATE`, `PROVEN ADVANTAGE`, or equivalent superiority labels are forbidden unless a complete BenchmarkManifest exists for the capability.
4. Structural MedScale differences may be shown as `STRUCTURAL DIFFERENTIATION` only; they must not imply measured superiority.
5. Waived/deferred/useful rows remain explicit and non-blocking; raw model count remains an anti-metric even when a dated OpenMed inventory count is displayed as context.
6. The Evidence UI exposes pinned baseline identity, claim state, MedScale evidence, OpenMed baseline evidence, limitations/missing measurement, and evidence references in text-accessible form.
7. Clinical NER, PII/de-identification, multilingual coverage, and Apple Silicon acceleration must show the actual gap state until matched-task evidence exists.
8. Runtime comparison records the Spec 069 tract CPU measurement and the pinned OpenMed MLX feature presence, but refuses a runtime winner because no same-model/same-corpus/same-hardware BenchmarkManifest exists.
9. No OpenMed Python runtime is imported into MedScale, no external weights are vendored, no real PHI is authorized, no online acquisition is added, and MESC remains untouched.
10. Local qualification, exact-range review, exact-head CI, protected merge, and post-main verification are required before closure.

## Explicit non-goals

- No attempt to manufacture parity by running unmatched models/corpora.
- No production clinical model promotion or real-PHI evaluation.
- No MLX/ORT/Core ML runtime promotion without matched evidence and authority review.
- No mobile implementation; mobile remains deferred.
- No release-ready claim; Spec 072 owns rebuilt-product requalification.
