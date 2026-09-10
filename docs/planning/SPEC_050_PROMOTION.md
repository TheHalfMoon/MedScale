# Spec 050 promotion — host-bound delivery-plan perf measurement path (Q05 residual)

**Classification:** `EXISTING_Q05_RESIDUAL_ELIGIBLE_FOR_PROMOTION`  
**Not:** claiming budget attainment or RELEASE_READY.

## Authority

| Source | Residual |
|---|---|
| Doctor missing class `perf_budgets_attained_on_qualified_hardware` | Explicit Q05 gap |
| TRUSTED_V1_DELIVERY_PLAN performance section | Requires host-bound p50/p95 + hardware/toolchain/lock/fixture hashes |
| Spec 042/045 | Harness + 10k lexical exist; host binding procedure not packaged as operator script |

## Scope

- `scripts/run-host-perf-measurement.ps1`: run harness at configurable scale; capture OS/arch/CPU/RAM/rustc/lock/source/tree into evidence sidecar; never set `budgets_claimed_met=true`.
- Doctor `host_perf_measurement_path_present=true`.
- Keep `perf_budgets_attained_on_qualified_hardware` missing until measured budgets pass on qualified matrix.

## Numbering

Spec **050** residual; advanced deferred **051+**.
