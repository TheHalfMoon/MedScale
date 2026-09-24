# Tasks — Spec 083 Knowledge + Research Canvas

## T083-00 — Live truth
- [x] Record base SHA, open PRs, CI state and schema version.

## T083-01 — Contracts
- [x] `knowledge.rs` contracts, closed vocabularies, invariants, unit tests.

## T083-02 — Storage v12
- [x] Tables, migration, atomic index versions, digest checks, canvas revisions, consistency, backup/restore, tests; 078-082 rewind tests updated.

## T083-03 — Core authority
- [x] Current sources, freshness, index build/status, lexical search with receipts, canvas create/edit/view with inspection, facade wiring, tests.

## T083-04 — CLI and Desktop
- [x] `medscale knowledge ...` (human and JSON) and the Desktop Knowledge route over Core, tests.

## T083-05 — Qualification and closure
- [x] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [x] Deterministic exact-range scope record and security challenge.
- [x] Evidence files.
- [x] Exact-head CI on the final head; merge; post-main CI.
- [x] Closure PR; recompute the next unit.

Reconciled 2026-09-24 against implementation, tests and CI: T083-00 to
T083-05 are complete. Code head `8a05ccf` passed run `35997730588` (6/6);
evidence head `194840c` passed run `36004914365` (6/6); PR #145 merged as
`2892860` with post-main run `36014350193 (6/6)`; the closure PR records it.
The next dependency-ready unit is 084.
