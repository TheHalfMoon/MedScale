# Perf harness methodology (Spec 027)

## Delivery-plan targets (NOT claimed met)

From `docs/planning/TRUSTED_V1_DELIVERY_PLAN.md`:

| Path | Target |
|---|---|
| Timeline for 10,000 events | p95 ≤ 250 ms |
| Lexical search over 10,000 local records | p95 ≤ 300 ms |
| 1 MiB bounded FHIR ingest (excl. external validation) | p95 ≤ 500 ms |

## Harness procedure

1. Synthetic fixtures only; REAL_PHI unauthorized.
2. Explicit warmup runs (default 3), then timed runs (default 30).
3. Sort sample durations ascending.
4. p50 = sample at index `floor(n * 50 / 100)`; p95 = sample at index `floor(n * 95 / 100)` (clamped).
5. Report milliseconds + hardware/toolchain note + measured scale.
6. Set `budgets_claimed_met=false` and `release_ready=false` in JSON evidence.

## CI policy

Assert only that the harness executes and emits finite p50/p95 numbers.
**Do not** fail CI when measured times exceed delivery-plan budgets.

## Regenerating evidence

```powershell
$env:CARGO_TARGET_DIR = "D:\medscale-target"
# Optional: nearer to delivery-plan 1 MiB ingest (still not a budget claim)
# $env:MEDSCALE_027_FHIR_BYTES = "1000000"
cargo test -p medscale-core --test perf_harness_027 --locked -- --nocapture
```

Output: `evidence/027-perf-sbom-release-evidence/perf_harness_latest.json`
