# Research: Spec 008

## Baseline

- Spec 007 OpenMed pin + parity matrix designed; measurement deferred  
- Spec 013 Network Broker exists; workers must not bypass  
- worker_policy deny_by_default already in contracts  

## Engines

SOURCE pins ONNX/llama/Candle as candidates. Spec 008 does **not** admit them—FixtureRuntime only so fabric + Pack admission ship without FFI risk.

## Pack format

JSON PackManifestV0 + artifact file digests under local directory. Fail-closed on forbidden artifact kinds.

## Confinement

Policy + architecture tests for ambient deny. Landlock/AppContainer/Seatbelt = PLATFORM_QUALIFIED follow-on (documented).
