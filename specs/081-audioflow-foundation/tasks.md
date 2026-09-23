# Tasks — Spec 081 AudioFlow Foundation

## T081-00 — Live truth
- [x] Record base SHA, open PRs, CI state and schema version.

## T081-01 — Contracts
- [x] `audio.rs` contracts, closed vocabularies, invariants, unit tests.

## T081-02 — Storage v10
- [x] Tables, migration, capture chunk atomicity, transcript lineage, digest checks, consistency, backup/restore, tests; 078-080 rewind tests updated.

## T081-03 — Core authority
- [x] WAV parsing, VAD/health, capture lifecycle and interruption recovery, route decisions and receipts, fixture engine seam, corrections, evidence, facade wiring, tests.

## T081-04 — CLI and Desktop
- [x] `medscale audio ...` (human and JSON) and the Desktop Audio route over Core, tests.

## T081-05 — Qualification and closure
- [x] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [x] Deterministic exact-range scope record.
- [x] Evidence files.
- [x] Exact-head CI on the final head; merge; post-main CI.
- [x] Closure PR; recompute the next unit.

Reconciled 2026-09-23 against implementation, tests and CI: T081-00 to
T081-04 are complete on code head `502b49a` (run `35889704517`, 6/6; see
`evidence/081-audioflow-foundation/QUALIFICATION.md`). The last two T081-05
items were completed by PR #141 (merged as `876b9fe`, post-main run
`35906389709` 6/6) and the closure PR; the next unit is 082.
