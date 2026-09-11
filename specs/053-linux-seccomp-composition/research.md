# Research: Spec 053

## Decision
Child-process seccomp-bpf measurement. Filters are irreversible per thread,
so in-process apply would permanently confine the caller (test runner, CLI).
The probe child installs `PR_SET_NO_NEW_PRIVS` + `SECCOMP_MODE_FILTER` with
a hand-built classic BPF allowlist (read/write/exit/exit_group/rt_sigreturn),
emits a marker via allowed write, then invokes denied `socket()` and must
die by SIGSYS (31). Parent asserts marker + signal.

## Alternatives rejected
- In-process apply: permanently confines caller; rejected as unsafe for tests/CLI.
- libseccomp dependency: new supply-chain surface for a 10-instruction
  filter; hand-built BPF with existing pinned libc is smaller and auditable.
- SECCOMP_RET_TRAP/ERRNO: KILL_PROCESS is the unambiguous deny signal for
  measurement; trap handlers add complexity without honesty gain.

## Arch coverage
x86_64 + aarch64 filter tables (AUDIT_ARCH + syscall numbers per arch).
Other arches honestly return NotReadyOnThisHost.
