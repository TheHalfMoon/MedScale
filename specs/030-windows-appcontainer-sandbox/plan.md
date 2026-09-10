# Plan: Spec 030 Windows Job Object / AppContainer READY_BASE

1. Spec Kit package (specify/clarify/plan/research/ADR/tasks/checklist/analyze/converge).
2. Contracts: `windows_job_object_ready_base()`, doctor `windows_measured`, keep AppContainer scaffold.
3. Windows-only `windows-sys` Job Object apply (`ACTIVE_PROCESS=1` + `KILL_ON_JOB_CLOSE`).
4. Probe binary + measured test proving child spawn denied; off-Windows `NotReadyOnThisHost`.
5. Doctor/CLI honesty; EXTERNAL_GATES row updated but remains OPEN.
6. Evidence SUMMARY / LIMITATIONS / WINDOWS_JOB_OBJECT_MEASURED; admission record.
7. BUILD_QUEUE / roadmap / START_HERE → 030 CLOSED; deferred **031+**.
8. Gates: fmt, clippy `-D warnings`, `cargo test --workspace --locked`.

## Architecture

```text
OsSandboxPlan(WindowsAppContainerJobObject, ReadyBaseMeasured)
                              |
              cfg(windows) CreateJobObject + ACTIVE_PROCESS=1
                              |
              AssignProcessToJobObject(current)
                              |
Doctor: windows_measured=true, linux_measured=true, platform_qualified=false
EXTERNAL_GATES WORKER_OS_SANDBOX_PLATFORM_QUALIFIED = OPEN
AppContainer scaffold: NotPlatformQualified (separate plan)
macOS: NotPlatformQualified scaffold
```
