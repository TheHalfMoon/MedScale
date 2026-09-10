# Research — Spec 033 Windows AppContainer FS

## Decision: measured child FS deny via AppContainer profile

- Spec 030 deferred full AppContainer due to profile/capability complexity; Job Object covered process-spawn deny.
- Spec 033 implements the residual FS path without claiming PLATFORM_QUALIFIED:
  - `CreateAppContainerProfile` / derive on ALREADY_EXISTS
  - `PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES`
  - Child (`medscale-os-sandbox-probe appcontainer-fs-child`) cannot read a host-temp marker
- Parent process is **not** Landlock-restricted; measurement is child isolation — documented honestly.

## Alternatives rejected

| Approach | Why not |
|---|---|
| Claim PlatformQualified | macOS App Sandbox entitlements + multi-OS composition still open |
| LPAC / network capability matrix | Separate residual; not required for FS ReadyBaseMeasured |
| Restrict current process in-place | AppContainer is launch-time, not Job-Object-style restrict_self |

## Dependency

- `windows-sys` 0.61.2 features: Foundation, Security, Security_Authorization, Security_Isolation, JobObjects, Threading.
