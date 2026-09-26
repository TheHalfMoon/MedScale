# Tasks — Spec 085 MedScale Compute

## T085-00 — Live truth and promotion
- [x] Record base SHA (Spec 084 closure main), open PRs, CI state, schema version and sandbox qualification state.

## T085-01 — Contracts and worker
- [x] `compute.rs` contracts, closed vocabularies, job kinds, worker protocol, output validation; worker and fault-harness binaries; unit tests.

## T085-02 — Storage v14
- [x] Tables, migration, single-claim transitions, atomic terminal commits, consistency, backup/restore, tests; 078-084 rewind tests updated.

## T085-03 — Core authority
- [x] Supervisor (cleared environment, scratch directory, bounded pipes, timeout, cancel); admission, run, cancel, recover, reads; facade wiring; tests including fault injection.

## T085-04 — CLI
- [x] `medscale compute ...` (human and JSON), tests.

## T085-05 — Qualification and closure
- [x] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [x] Deterministic exact-range scope record and security challenge.
- [x] Evidence files.
- [ ] Exact-head CI on the final head; merge; post-main CI.
- [ ] Closure PR; recompute the next unit.

Reconciled 2026-09-26 against implementation, tests and CI: T085-01 to
T085-04 and the first three T085-05 items are complete on code head
`3bd00d1` (run `36199048976`: ubuntu 874 passed / 0 failed / 1 ignored,
windows 871 / 0 / 1 with Clippy and Test green, macOS green; see
`evidence/085-compute/EXACT_HEAD_QUALIFICATION.md`). T085-00 is complete:
promotion base `434d1c7` (`evidence/085-compute/LIVE_TRUTH.md`).
