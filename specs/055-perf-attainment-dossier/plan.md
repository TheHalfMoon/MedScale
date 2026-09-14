# Plan: Spec 055 Perf Attainment Dossier

## Architecture
Docs + evidence only; no trusted-core behavior change. New files:

- `evidence/055-perf-attainment/PERFORMANCE_DOSSIER.md`: verdict, bindings,
  harness lineage, scales, observed CI-scale figures with provenance, missing
  bar, flip criteria.
- `evidence/055-perf-attainment/BINDING.md`: exact source/tree/lock SHAs,
  toolchain, target, profile, host disclosure method.
- `crates/medscale-core/tests/perf_attainment_dossier_055.rs`: honesty test.
- `docs/planning/SPEC_055_PROMOTION.md`, BUILD_QUEUE row.

## Files
See above. No doctor contract change: existing
`perf_budgets_attained_on_qualified_hardware` missing class already expresses
the gap; dossier references it.

## Test strategy
New test checks file presence, required binding keys, NON_ATTAINMENT verdict,
and absence of attainment/release-ready claims. Existing workspace gates
(fmt/clippy/test/deny) must pass.

