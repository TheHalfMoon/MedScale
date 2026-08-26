# OS sandbox scaffold (Spec 008 follow-on)

**Date:** 2026-08-26  
**Main context:** post Spec 008 `CLOSED_CANONICAL`  
**Gate:** `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED` = `OPEN`

## Claim scope

This artifact documents **plans only** for:

- Linux Landlock (+ candidate seccomp/rlimit)
- Windows AppContainer + Job Object (+ brokered handles)
- macOS Seatbelt / App Sandbox / XPC pattern

**Claim:** `OsSandboxQualification::NotPlatformQualified`  
**Not claimed:** `PLATFORM_QUALIFIED`, measured confinement, production worker isolation.

## Code

- `medscale_contracts::os_sandbox::{OsSandboxPlan, try_apply_os_sandbox}`
- `try_apply_os_sandbox` always returns `NotPlatformQualified` while the gate is OPEN.

## Next evidence required to close gate

Per-OS measured apply tests with exact SHA/toolchain/OS, corpus of denied ambient capabilities, and limitations recorded before flipping any plan to `PlatformQualified`.
