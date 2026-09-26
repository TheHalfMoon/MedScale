# Tasks — Spec 085 MedScale Compute

## T085-00 — Live truth and promotion
- [x] Record base SHA (Spec 084 closure main), open PRs, CI state, schema version and sandbox qualification state.

## T085-01 — Contracts and worker
- [ ] `compute.rs` contracts, closed vocabularies, job kinds, worker protocol, output validation; worker and fault-harness binaries; unit tests.

## T085-02 — Storage v14
- [ ] Tables, migration, single-claim transitions, atomic terminal commits, consistency, backup/restore, tests; 078-084 rewind tests updated.

## T085-03 — Core authority
- [ ] Supervisor (cleared environment, scratch directory, bounded pipes, timeout, cancel); admission, run, cancel, recover, reads; facade wiring; tests including fault injection.

## T085-04 — CLI
- [ ] `medscale compute ...` (human and JSON), tests.

## T085-05 — Qualification and closure
- [ ] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [ ] Deterministic exact-range scope record and security challenge.
- [ ] Evidence files.
- [ ] Exact-head CI on the final head; merge; post-main CI.
- [ ] Closure PR; recompute the next unit.
