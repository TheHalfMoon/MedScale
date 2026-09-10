# Research — Spec 030 Windows sandbox READY_BASE

## Decision: Job Object measured; AppContainer scaffold

- Full AppContainer profiles typically need profile creation APIs and capability SID wiring; heavier for non-admin CI and not equivalent to a one-shot `restrict_self` without brokered launches.
- Job Objects are available without elevation: `CreateJobObjectW` + `SetInformationJobObject` + `AssignProcessToJobObject`.
- Measurable deny without admin: `JOB_OBJECT_LIMIT_ACTIVE_PROCESS = 1` denies further `CreateProcess` (ERROR_NOT_ENOUGH_QUOTA / spawn Err).
- Honesty: Job Objects do **not** provide Landlock-style filesystem allowlists or AppContainer network/FS isolation. Spec 030 measures process-creation confinement only.

## Alternatives rejected for READY_BASE

| Approach | Why not now |
|---|---|
| Full AppContainer + LPAC | Profile lifecycle + ACL complexity; not required to unlock ReadyBaseMeasured complementary to Spec 026 |
| Network rate Job Object controls | Rate-limit ≠ hard deny; weaker proof |
| Claim PlatformQualified | EXTERNAL_GATES and macOS residual forbid |

## Dependency

- `windows-sys` 0.61.2 (MIT OR Apache-2.0), Windows-only features: Foundation, JobObjects, Threading.
- Workspace `unsafe_code` lint is `deny` (not `forbid`) so `windows_job` may `#![allow(unsafe_code)]` for Win32 FFI.
