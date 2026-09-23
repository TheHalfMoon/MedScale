# Tasks — Spec 082 Analytics Gate

## T082-00 — Live truth
- [ ] Record base SHA, open PRs, CI state and schema version.

## T082-01 — Contracts
- [ ] `analytics.rs` contracts, closed vocabularies, invariants, unit tests.

## T082-02 — Engine
- [ ] Read-only SQLite engine: screen, single statement, read-only, query_only, bounds, timeout, unit tests.

## T082-03 — Storage v11
- [ ] Tables, migration, atomic commit, digest checks, consistency, backup/restore, tests; 078-081 rewind tests updated.

## T082-04 — Core authority
- [ ] Bindings through the 075 authority, receipts, derived tables, replay, cohorts, statistics, facade wiring, tests.

## T082-05 — CLI and Desktop
- [ ] `medscale analytics ...` (human and JSON) and the Desktop Analytics route over Core, tests.

## T082-06 — Qualification and closure
- [ ] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [ ] Deterministic exact-range scope record.
- [ ] Evidence files.
- [ ] Exact-head CI on the final head; merge; post-main CI.
- [ ] Closure PR; recompute the next unit.
