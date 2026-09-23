# Tasks — Spec 081 AudioFlow Foundation

## T081-00 — Live truth
- [ ] Record base SHA, open PRs, CI state and schema version.

## T081-01 — Contracts
- [ ] `audio.rs` contracts, closed vocabularies, invariants, unit tests.

## T081-02 — Storage v10
- [ ] Tables, migration, capture chunk atomicity, transcript lineage, digest checks, consistency, backup/restore, tests; 078-080 rewind tests updated.

## T081-03 — Core authority
- [ ] WAV parsing, VAD/health, capture lifecycle and interruption recovery, route decisions and receipts, fixture engine seam, corrections, evidence, facade wiring, tests.

## T081-04 — CLI and Desktop
- [ ] `medscale audio ...` (human and JSON) and the Desktop Audio route over Core, tests.

## T081-05 — Qualification and closure
- [ ] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [ ] Deterministic exact-range scope record.
- [ ] Evidence files.
- [ ] Exact-head CI on the final head; merge; post-main CI.
- [ ] Closure PR; recompute the next unit.
