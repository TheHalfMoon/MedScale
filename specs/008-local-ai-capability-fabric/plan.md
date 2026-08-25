# Plan: Spec 008 Local AI Capability Fabric

**Branch**: `spec/008-local-ai-capability-fabric`  
**Approach**: Contracts + `medscale-pack` offline store + Core facade + CLI; FixtureRuntime only; no native AI deps.

## Architecture

```text
CLI packs install/list/promote
  -> CoreFacade (PacksInstallLocal / PacksList / PacksPromote)
    -> medscale-pack (admit, format gate, PackStore)
WorkerSupervisionPolicy.deny_by_default + MockWorkerHost tests
DoctorReport.packs_runtime
```

## Decisions

| ID | Decision |
|---|---|
| D1 | Zero ONNX/llama/Candle DEPENDENCY in Spec 008 close |
| D2 | Fixture pack under evidence/fixtures; content-addressed by SHA-256 |
| D3 | 008S OS PLATFORM_QUALIFIED recorded as limitations + EXTERNAL_GATES row; policy confinement is exit for this unit |
| D4 | Promotion: candidate→current requires explicit promote; canary optional |

## Admissions

None (no new native runtime crates).
