# Clarifications — Spec 031

| Question | Resolution |
|---|---|
| App Sandbox entitlements in CI? | No — use `sandbox_init` SBPL; document App Sandbox as scaffold. |
| Measured capability? | Network deny (`deny network*`) with probe proving socket connect fails as PermissionDenied/EPERM (not ConnectionRefused). |
| macos_measured only on cfg(macos)? | Report true in doctor READY_BASE once evidence exists in-tree (same pattern as linux_measured / windows_measured). |
| Clear WORKER_OS_SANDBOX_PLATFORM_QUALIFIED? | No — AppContainer FS still scaffold; gate stays OPEN. |
| New crates? | Prefer none; libSystem FFI only. |
| RELEASE_READY / PRIVATE_DATA / MULTI_CLIENT? | Remain FALSE. |
| Deferred after close? | **032+**. |
