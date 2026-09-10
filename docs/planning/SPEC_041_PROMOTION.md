# Spec 041 promotion — macOS App Sandbox entitlements ReadyBaseMeasured

**Date:** 2026-09-10  
**Classification:** `EXISTING_Q09_RESIDUAL_ELIGIBLE_FOR_PROMOTION`  
**Not:** `DEFERRED_PRODUCT_SCOPE` · **Not:** `AUTHORITY_CONFLICT`

## Authority basis (post Spec 040 main `e4458fb…`)

| Source | Evidence |
|---|---|
| Spec 031 FR / LIMITATIONS | App Sandbox entitlements left scaffold after Seatbelt ReadyBaseMeasured |
| `EXTERNAL_GATES` WORKER_OS_SANDBOX | Residual: macOS App Sandbox entitlements + multi-OS composition |
| Spec 040 promotion pattern | Number ≥040 may be Q09 residual; advanced product remains **042+** |

## Separation (mandatory honesty)

| Layer | Spec 041 claim |
|---|---|
| Seatbelt `sandbox_init` | Already Spec 031 — not re-claimed as App Sandbox |
| Entitlements artifact + detection probe | ReadyBaseMeasured |
| Runtime entitlement enforcement | Requires signing — `enforcement_measured=false`; SIGNING_ACTION external |
| PLATFORM_QUALIFIED | Remains false |

## Deferred advanced product

Plugins/GraphRAG/replicas/imaging/CUDA remain **042+**.
