# Plan: Spec 053

- [x] Contracts: `linux_seccomp` module (confined unsafe FFI, hand-built classic BPF).
- [x] `OsSandboxTarget::LinuxSeccompComposition` + plan + try_apply arm (child-measured).
- [x] Doctor axis + inventory (9 axes) + CLI print + required-keys test.
- [x] Probe `seccomp-child` / `seccomp-composition` modes.
- [x] contracts + core tests; evidence/053; promotion record.
- [x] MESC/PHI/network anti-scope honored.

## Constraints
Synthetic-only; DEFAULT_DENY product network; no MESC mutation; workspace
`unsafe_code=deny` with confined `allow(unsafe_code)` module only; no new
dependencies (libc already Linux-pinned).
