# Clarifications — Spec 030

| Question | Resolution |
|---|---|
| Set PlatformQualified on Windows? | No — ReadyBaseMeasured only; gate stays OPEN. |
| Require AppContainer for READY_BASE? | No — Job Object measured prove-deny is sufficient; AppContainer stays scaffold. |
| Require allow_paths on Windows? | No — unused for Job Object process-limit measurement. |
| windows_measured only on cfg(windows)? | Report true in doctor READY_BASE once evidence exists in-tree (same pattern as linux_measured). |
| Poison test runner with AssignProcessToJobObject? | No — use `medscale-os-sandbox-probe` child process. |
