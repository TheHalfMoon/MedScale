# Spec 053 promotion - Linux seccomp-bpf composition

## Promotion decision

```text
UNIT = SPEC_053_LINUX_SECCOMP_COMPOSITION
CLASS = EXECUTABLE_WITH_CI_MEASUREMENT
AUTHORITY = TRUSTED_V1 residual (Q09 / Spec 052 LIMITATIONS + Spec 044 composition honesty)
PROMOTION = APPROVED_FOR_SPEC_KIT_PACKAGE
```

## Residual proof

`evidence/052-linux-landlock-composition/LIMITATIONS.md` explicitly leaves
seccomp composition open. This unit closes that in-repo residual at
READY_BASE depth (child-measured strict allowlist + SIGSYS deny).

## Bound

- Add `LinuxSeccompComposition` ReadyBaseMeasured plan + child-measured apply path.
- Measure on Linux (x86_64 + aarch64 filter tables; ubuntu CI runs x86_64):
  child installs `PR_SET_NO_NEW_PRIVS` + `SECCOMP_MODE_FILTER` strict
  allowlist (read/write/exit/exit_group/rt_sigreturn), proves an allowed
  write, then must die by SIGSYS on a denied `socket()` call.
- Doctor: `linux_seccomp_composition_measured=true`; keep
  `platform_qualified=false`; inventory grows to 9 axes.
- Non-Linux and unsupported arches: `NotReadyOnThisHost` (honest scaffold).

## Out of scope

- Claiming `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED` / `platform_qualified=true`.
- In-process worker confinement (filters are irreversible per thread).
- Parent-process seccomp confinement; worker-spawn confinement is later work.
- macOS App Sandbox signed enforcement; Windows brokered-handle composition.
- RELEASE_READY / PRIVATE_DATA_READY.

## Numbering

Spec **053** Trusted V1 residual; advanced deferred renumbered **054+**.
