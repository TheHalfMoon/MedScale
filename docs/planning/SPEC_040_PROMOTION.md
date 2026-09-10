# Spec 040 promotion — Windows AppContainer LPAC ReadyBaseMeasured

**Date:** 2026-09-10  
**Classification:** `EXISTING_Q09_RESIDUAL_ELIGIBLE_FOR_PROMOTION`  
**Not:** `DEFERRED_PRODUCT_SCOPE` · **Not:** `AUTHORITY_CONFLICT`

## Authority basis (live main `7ffadb25…`)

| Source | Evidence |
|---|---|
| `EXTERNAL_GATES.md` `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED` | Explicit residual: LPAC still scaffold after Spec 038 network ReadyBaseMeasured |
| Spec 038 FR-003 / LIMITATIONS | LPAC capability matrix not measured; scaffold retained deliberately |
| `BUILD_QUEUE.md` next-eligible | Lists “LPAC + macOS App Sandbox entitlements + multi-OS PLATFORM_QUALIFIED” as Trusted V1 residual |
| Specs 030 / 033 / 038 | Same Q09 Windows AppContainer ladder; LPAC is next bounded step |
| Roadmap `040+` row | Deferred **advanced product** (plugins/GraphRAG/replicas/imaging/CUDA/etc.), not every integer ≥040 |

## Distinction from deferred 040+

| Promoted Spec 040 | Remains deferred as 041+ advanced |
|---|---|
| Windows LPAC ReadyBaseMeasured for worker sandbox evidence | Plugins, GraphRAG, replicas, imaging, genomics, CUDA, broad integrations |
| Builds on Spec 030/033/038 AppContainer path | New product surfaces / research tracks |

## Honesty after promotion

- `platform_qualified` remains **false**
- `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED` remains **OPEN** until multi-OS composition + macOS App Sandbox entitlements (and any further measured residuals) close it
- ReadyBaseMeasured ≠ PLATFORM_QUALIFIED
- Numbering: Spec **040** is this residual; advanced deferred work renumbered as **041+**
