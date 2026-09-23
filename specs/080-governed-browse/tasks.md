# Tasks — Spec 080 Governed Browse

## T080-00 — Live truth
- [ ] Record base SHA, open PRs, CI state and schema version.

## T080-01 — Contracts
- [ ] `browse.rs` contracts, closed vocabularies, invariants, unit tests.

## T080-02 — Network transport
- [ ] `validate_url`, `is_forbidden_ip`, `PublicOnlyResolver`, live and scripted transports, unit tests.

## T080-03 — Storage v9
- [ ] Tables, migration, atomic commit, digest checks, consistency, backup/restore, tests.

## T080-04 — Core authority
- [ ] Allowlist, policy order, fetch loop with per-hop re-validation, evidence/downloads, takeover, cancel, envelope and facade wiring, tests.

## T080-05 — CLI and Desktop
- [ ] `medscale browse ...` (human and JSON) and the Desktop Browse route over Core, tests.

## T080-06 — Qualification and closure
- [ ] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [ ] Deterministic exact-range scope record.
- [ ] Evidence files.
- [ ] Exact-head CI on the final head; merge; post-main CI.
- [ ] Closure PR; recompute the next unit.
