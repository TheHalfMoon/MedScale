# Tasks — Spec 084 MedScale Hub Foundation

## T084-00 — Live truth and promotion
- [x] Record base SHA (Spec 083 closure main), open PRs, CI state and schema version.

## T084-01 — Contracts and keys
- [x] `hub.rs` contracts, closed vocabularies, signing payloads, event chain; device keys; unit tests.

## T084-02 — Storage v13
- [x] Tables, migration, event chain with sequence claims, links/outbox/mirror, consistency, backup/restore, tests; 078-083 rewind tests updated.

## T084-03 — Core authority
- [x] Hub (init, invite, enroll, challenge, handshake, submit, pull, revoke, status) and client (join, queue, sign, record, mirror), facade wiring, sync orchestration, transports, tests.

## T084-04 — CLI
- [x] `medscale hub ...` (human and JSON), `serve` over local IPC, tests.

## T084-05 — Qualification and closure
- [x] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [x] Deterministic exact-range scope record and security challenge.
- [x] Evidence files.
- [x] Exact-head CI on the final head; merge; post-main CI.
- [x] Closure PR; recompute the next unit.

Reconciled 2026-09-24 against implementation, tests and CI: T084-01 to
T084-04 and the first three T084-05 items are complete on code head
`7858106` (run `36023022359`, 6/6; see `evidence/084-hub/QUALIFICATION.md`).
T084-00 is complete: promotion base `426bb34` (`evidence/084-hub/LIVE_TRUTH.md`).

Closed 2026-09-25: final head `1c98478` passed run `36033616030` (6/6);
PR #147 merged as `6caa698`; post-main run `36193194092` (6/6). See
`evidence/084-hub/CLOSURE.md` and `POST_MERGE_VERIFICATION.md`. Next unit
recomputed in `docs/planning/BUILD_QUEUE.md`.
