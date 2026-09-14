# Clarifications: Spec 057

- Fresh audit found two repository-owned Q05 defects after Specs 055-056: missing runtime launch/RSS measurement coverage and one mutable CI action reference.
- `cold model-free launch p95 <= 2 s` and `idle memory <= 250 MiB` are targets only; this unit measures without asserting attainment.
- The current desktop executable is a scaffold. Its process launch/RSS measurements are engineering baselines, not final-shell qualification.
- `UI interaction response <= 100 ms` remains `BLOCKED_BY_FINAL_V0_UI`; no synthetic substitute may clear it.
- `actions/upload-artifact` is pinned to `ea165f8d65b6e75b540449e92b4886f43607fa02` (`v4.6.2`, verified from upstream tag refs on 2026-09-14).
- No new production dependency is admitted; `sha2` is reused as a dev dependency from the locked workspace.
- MESC is not involved.
