# Spec 052 promotion — Linux Landlock composition

## Promotion decision

```text
UNIT = SPEC_052_LINUX_LANDLOCK_COMPOSITION
CLASS = EXECUTABLE_WITH_CI_MEASUREMENT
AUTHORITY = TRUSTED_V1 residual (Q09 / Spec 026 LIMITATIONS + Spec 044 composition honesty)
PROMOTION = APPROVED_FOR_SPEC_KIT_PACKAGE
```

## Residual proof

`evidence/026-pack-signer-os-sandbox/LIMITATIONS.md` explicitly leaves:

```text
No seccomp / network deny / rlimit composition in this unit.
```

Spec 044 inventories `linux_landlock` as ReadyBaseMeasured FS-only. Multi-OS
`platform_qualified` remains false; this unit deepens **Linux-side composition**
only (Landlock FS allowlist + Landlock TCP network deny + RLIMIT_NOFILE).

## Bound

- Add `LinuxLandlockComposition` ReadyBaseMeasured plan + apply path.
- Measure FS deny outside allowlist, TCP connect deny, and lowered RLIMIT_NOFILE
  on Linux (CI ubuntu + cfg(linux) tests / probe).
- Doctor: `linux_landlock_composition_measured=true`; keep `platform_qualified=false`.
- Seccomp filter composition remains a later residual / honest limitation.

## Out of scope

- Claiming `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED` / `platform_qualified=true`.
- macOS App Sandbox signed enforcement.
- Windows brokered-handle composition.
- Full seccomp-bpf policy.
- RELEASE_READY / PRIVATE_DATA_READY.
