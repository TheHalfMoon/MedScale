# Tasks — Spec 084 MedScale Hub Foundation

## T084-00 — Live truth and promotion
- [ ] Record base SHA (Spec 083 closure main), open PRs, CI state and schema version.

## T084-01 — Contracts and keys
- [ ] `hub.rs` contracts, closed vocabularies, signing payloads, event chain; device keys; unit tests.

## T084-02 — Storage v13
- [ ] Tables, migration, event chain with sequence claims, links/outbox/mirror, consistency, backup/restore, tests; 078-083 rewind tests updated.

## T084-03 — Core authority
- [ ] Hub (init, invite, enroll, challenge, handshake, submit, pull, revoke, status) and client (join, queue, sign, record, mirror), facade wiring, sync orchestration, transports, tests.

## T084-04 — CLI
- [ ] `medscale hub ...` (human and JSON), `serve` over local IPC, tests.

## T084-05 — Qualification and closure
- [ ] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [ ] Deterministic exact-range scope record and security challenge.
- [ ] Evidence files.
- [ ] Exact-head CI on the final head; merge; post-main CI.
- [ ] Closure PR; recompute the next unit.
