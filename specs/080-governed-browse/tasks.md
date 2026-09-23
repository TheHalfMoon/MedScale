# Tasks — Spec 080 Governed Browse

## T080-00 — Live truth
- [x] Record base SHA, open PRs, CI state and schema version.

## T080-01 — Contracts
- [x] `browse.rs` contracts, closed vocabularies, invariants, unit tests.

## T080-02 — Network transport
- [x] `validate_url`, `is_forbidden_ip`, `PublicOnlyResolver`, live and scripted transports, unit tests.

## T080-03 — Storage v9
- [x] Tables, migration, atomic commit, digest checks, consistency, backup/restore, tests.

## T080-04 — Core authority
- [x] Allowlist, policy order, fetch loop with per-hop re-validation, evidence/downloads, takeover, cancel, envelope and facade wiring, tests.

## T080-05 — CLI and Desktop
- [x] `medscale browse ...` (human and JSON) and the Desktop Browse route over Core, tests.

## T080-06 — Qualification and closure
- [x] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [x] Deterministic exact-range scope record.
- [x] Evidence files.
- [ ] Exact-head CI on the final head; merge; post-main CI.
- [ ] Closure PR; recompute the next unit.

Reconciled 2026-09-23 against implementation, tests and CI (not against the
presence of code alone): T080-00 to T080-05 are complete on code head
`5daec97` (run `35861492981`, 6/6; see
`evidence/080-governed-browse/QUALIFICATION.md`). T080-02/-03/-04 were
reopened during the T080-06 challenge and closed with the fixes recorded in
`SECURITY_ADVERSARIAL.md` C1-C6. The last two T080-06 items stay open until
merge, post-main CI and the closure PR.
