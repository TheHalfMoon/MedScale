# Contract: Benchmark Manifest

**Spec**: 007 (schema); Spec 008+ (population for runtime claims)  
**Status**: Canonical claim envelope  
**Related**: OPENMED_PARITY_SURPASS_MATRIX_V2 §3, DEFINITION_OF_DONE culture

## Rule

No MedScale evidence artifact may assert `PARITY`, `SURPASS`, `PRIVATE` (privacy superiority), or equivalent against OpenMed without a completed **BenchmarkManifest** binding the fields below.

## Required fields

| Field | Rule |
|---|---|
| `openmed_commit` | For floor comparisons: exactly `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837` |
| `medscale_commit` | Exact measured tree |
| `artifact_hashes` | Models/packs/tools used |
| `corpus_hash` | Fixture corpus identity |
| `rights` | Corpus + artifact rights notes |
| `hardware_os_runtime` | Machine class + OS + runtime ids |
| `quality_metrics` | Pre-registered metrics only |
| `latency_cold_start_throughput` | When performance claimed |
| `memory` | RAM/VRAM when claimed |
| `network_privacy_observation` | DEFAULT_DENY / attributable notes |
| `failure_behavior` | Unsupported / abstain / error |
| `limitations` | Explicit non-claims |
| `harness_revision` | Eval harness id/version |

## Forbidden claim shortcuts

- Raw model catalogue count as parity
- “Looks competitive” without metrics
- Comparing to unpinned OpenMed tip as the floor
- Surpass language on Saudi/Arabic before exact-head evidence

## Waiver claims

`claim_type: WAIVER` still requires `limitations`, `scope`, and pointer to matrix DEFER/USEFUL row—never silent pass.
