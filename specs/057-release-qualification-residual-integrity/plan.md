# Plan: Spec 057 Release Qualification Residual Integrity

## Architecture

This is a bounded Q05 release-evidence repair with no trusted-core authority change.

- `medscale-desktop`: add an engineering-only bounded `--perf-idle-ms` probe path that initializes the normal model-free report, emits readiness, retains state, sleeps, and exits.
- `medscale-desktop/tests/runtime_perf_057.rs`: measure scaffold cold process launch and idle RSS using read-only OS tools (`ps` on Unix, `tasklist` on Windows); write untracked JSON under `target/medscale-evidence/057-*`.
- `PERF_METHODOLOGY.md` / Spec 055 dossier: enumerate all six Trusted V1 targets and distinguish measured scaffold paths from final-v0-gated paths.
- `run-host-perf-measurement.ps1`: run both existing core harnesses and the new runtime harness with explicit runtime knobs.
- `.github/workflows/ci.yml`: run the runtime harness in the existing required perf job, upload its target evidence, and pin `actions/upload-artifact` to an immutable SHA.
- regression test: require immutable SHA pins for every `uses:` line and honest performance coverage text.

## Evidence

Static evidence lives under `evidence/057-release-qualification-residual-integrity/`. Runtime JSON is deliberately untracked and uploaded by CI/operator runs so stale host figures are not mistaken for canonical attainment.

## Test strategy

Run fmt, clippy, targeted desktop/runtime tests, Spec 057 honesty tests, workspace tests, cargo-deny, workflow pin scan, and exact-head GitHub CI. `budgets_claimed_met` and `RELEASE_READY` remain false.
