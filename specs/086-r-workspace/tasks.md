# Tasks — Spec 086 R Workspace

## T086-00 — Live truth and promotion
- [ ] Record base SHA (Spec 085 closure main), open PRs, CI state, schema version and sandbox qualification state.

## T086-01 — Contracts
- [x] `r_workspace.rs` contracts, closed vocabularies, manifest validation, launch allowlist, renderers; unit tests; fake-IDE harness binary.

## T086-02 — Storage v15
- [x] Tables, migration, receipt binding, atomic publication, consistency, backup/restore, tests; 078-085 rewind tests updated.

## T086-03 — Core authority
- [x] Host configuration and launcher; stage, inspect, launch, run refusal, publish, reads; facade wiring; tests including adversarial outputs, tampering, stale input and restart.

## T086-04 — CLI
- [x] `medscale r ...` (human and JSON), tests.

## T086-05 — Qualification and closure
- [ ] fmt, dependency direction, Clippy, workspace tests, cargo-deny (exact-head CI).
- [ ] Deterministic exact-range scope record and security challenge.
- [ ] Evidence files.
- [ ] Exact-head CI on the final head; merge; post-main CI.
- [ ] Closure PR; recompute the next unit.

Items T086-01 to T086-04 are written; they are complete only once CI
compiles and passes them (no local compile on this workstation).
