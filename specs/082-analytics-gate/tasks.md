# Tasks — Spec 082 Analytics Gate

## T082-00 — Live truth
- [x] Record base SHA, open PRs, CI state and schema version.

## T082-01 — Contracts
- [x] `analytics.rs` contracts, closed vocabularies, invariants, unit tests.

## T082-02 — Engine
- [x] Read-only SQLite engine: screen, single statement, read-only, query_only, bounds, timeout, unit tests.

## T082-03 — Storage v11
- [x] Tables, migration, atomic commit, digest checks, consistency, backup/restore, tests; 078-081 rewind tests updated.

## T082-04 — Core authority
- [x] Bindings through the 075 authority, receipts, derived tables, replay, cohorts, statistics, facade wiring, tests.

## T082-05 — CLI and Desktop
- [x] `medscale analytics ...` (human and JSON) and the Desktop Analytics route over Core, tests.

## T082-06 — Qualification and closure
- [x] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [x] Deterministic exact-range scope record.
- [x] Evidence files.
- [ ] Exact-head CI on the final head; merge; post-main CI.
- [ ] Closure PR; recompute the next unit.

Reconciled 2026-09-24 against implementation, tests and CI: T082-00 to
T082-05 and the first three T082-06 items are complete on code head
`11150b5` (run `35921559272`, 6/6; see
`evidence/082-analytics-gate/QUALIFICATION.md`). T082-02 to T082-05 were
reopened by the qualification challenge and closed with the fixes in
`SECURITY.md` F1-F4. The last two T082-06 items stay open until merge,
post-main CI and the closure PR.
