# Plan — Spec 040 Windows AppContainer LPAC

## Approach

Extend Spec 033/038 AppContainer launch path: attribute list size **2**, add
`PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY` =
`PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT`, zero capability SIDs.

Child (`appcontainer-lpac-child`) verifies LPAC identity + FS deny + network deny.
Parent (`appcontainer-lpac`) CreateProcess-waits for exit 0.

## Files

- `crates/medscale-contracts/src/os_sandbox/windows_appcontainer.rs` — LPAC measure
- `crates/medscale-contracts/src/os_sandbox/mod.rs` — target/plan/doctor flag
- `crates/medscale-contracts/src/bin/os_sandbox_probe.rs` — argv modes
- Doctor/CLI axes + tests + `evidence/040-windows-appcontainer-lpac/`

## Non-goals

PLATFORM_QUALIFIED, macOS App Sandbox entitlements, plugin architecture, cloud runtime.
