# Tasks — Spec 086 R Workspace

## T086-00 — Live truth and promotion
- [x] Record base SHA (Spec 085 closure main), open PRs, CI state, schema version and sandbox qualification state (`evidence/086-r-workspace/LIVE_TRUTH.md`).

## T086-01 — Contracts
- [x] `r_workspace.rs` contracts, closed vocabularies, manifest validation, launch allowlist, renderers; unit tests; fake-IDE harness binary.

## T086-02 — Storage v15
- [x] Tables, migration, receipt binding, atomic publication, consistency, backup/restore, tests; 078-085 rewind tests updated.

## T086-03 — Core authority
- [x] Host configuration and launcher; stage, inspect, launch, run refusal, publish, reads; facade wiring; tests including adversarial outputs, tampering, stale input and restart.

## T086-04 — CLI
- [x] `medscale r ...` (human and JSON), tests.

## T086-05 — Qualification and closure
- [x] fmt, dependency direction, Clippy, workspace tests, cargo-deny (exact-head CI).
- [x] Deterministic exact-range scope record and security challenge.
- [x] Evidence files.
- [x] Exact-head CI on the final head; merge; post-main CI.
- [x] Closure PR; recompute the next unit.

Reconciled 2026-09-26: final head `c8e8551` passed exact-head run
`36241268247` (6/6); PR #152 merged as `16f2d1f`; post-main verification
in `evidence/086-r-workspace/POST_MERGE_VERIFICATION.md`; closure in
`CLOSURE.md`.
