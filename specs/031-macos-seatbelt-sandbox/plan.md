# Implementation Plan — Spec 031 macOS Seatbelt sandbox READY_BASE

## Approach

1. Spec Kit package (specify → clarify → plan → checklist → tasks → analyze → implement → converge).
2. Contracts: `macos_seatbelt_ready_base()` + doctor `macos_measured`; keep `macos_seatbelt_scaffold()` for App Sandbox / XPC candidates.
3. `macos_seatbelt` module: `sandbox_init` SBPL `(allow default)(deny network*)` on `cfg(target_os = "macos")`.
4. Extend `medscale-os-sandbox-probe` for macOS measured network deny; off-macOS stubs `NotReadyOnThisHost`.
5. Doctor/CLI honesty; EXTERNAL_GATES remains OPEN.
6. Evidence + BUILD_QUEUE/roadmap/START_HERE → 031 CLOSED; deferred **032+**.

## Tech

- Rust `medscale-contracts` OS sandbox module
- Apple libSystem `sandbox_init` / `sandbox_free_error` (raw FFI; no new crate)
- Probe subprocess (Seatbelt is irreversible per process)

## Honesty

Doctor: macos_measured=true, windows_measured=true, linux_measured=true, platform_qualified=false, release_ready=false  
Gate WORKER_OS_SANDBOX_PLATFORM_QUALIFIED stays OPEN.
