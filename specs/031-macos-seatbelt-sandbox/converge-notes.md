# Converge notes — Spec 031

## Closed as READY_BASE

- macOS Seatbelt `sandbox_init` ReadyBaseMeasured with measured network deny
- Doctor `macos_measured=true`; `platform_qualified=false`; `release_ready=false`
- EXTERNAL_GATES `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED` remains OPEN
- Evidence under `evidence/031-macos-seatbelt-sandbox/`
- BUILD_QUEUE Spec 031 `CLOSED_CANONICAL`; deferred **032+**

## Non-claims preserved

- Not multi-OS PLATFORM_QUALIFIED
- Not App Sandbox entitlements / container FS isolation
- Not Windows AppContainer FS/network measured
- Not RELEASE_READY / PRIVATE_DATA_READY / MULTI_CLIENT_RELEASE_READY
