# Research — Spec 071

## Baseline truth

The pinned comparator was reverified from a local clone: OpenMed tag `v2.2.0`, commit `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837`, tree `1c949e35b2b8f2ea69da4284b370074fc4bf84ab`. This matches the canonical Spec 007 baseline.

## Claim-discipline truth

Repository search found no completed BenchmarkManifest, no parity-matrix row with `evidence_status=measured`, and no row with `surpass_allowed=true`. Therefore existing `PROVEN ADVANTAGE` product copy is not admissible as comparative superiority. Structural semantics can be described, but parity/surpass remains unmeasured.

## OpenMed baseline capability evidence

At the pinned commit, OpenMed documents multilingual PII/de-identification, clinical extraction/model families, an evaluation/release-evidence harness, a manifest-backed model registry, and Apple Silicon MLX/Swift MLX paths. `models.jsonl` contains 2266 rows at that exact baseline; this is a dated inventory fact only and remains an explicit anti-metric for MedScale parity.

## Runtime comparison boundary

Spec 069 measured the pinned Hugging Face NER Pack through MedScale tract CPU at warm p95 `596.100 ms`. The OpenMed baseline documents MLX acceleration, but the repository has no matched same-model/same-corpus/same-hardware BenchmarkManifest comparing that path with MedScale. Spec 071 therefore records `NO_RUNTIME_WINNER`.

## Safety and authority boundary

OpenMed remains comparator evidence, not MedScale authority. No Python runtime import, external weights, real PHI, ambient network acquisition, or MESC mutation is authorized by this spec.
