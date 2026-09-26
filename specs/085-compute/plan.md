# Plan — Spec 085 MedScale Compute

| Layer | Path |
|---|---|
| Contracts | `crates/medscale-contracts/src/compute.rs` (types, job kinds, worker `respond`, output validation, worker self-confinement, sibling executable lookup); Compute capabilities, requests and responses in `envelopes/mod.rs` |
| Worker | `crates/medscale-contracts/src/bin/compute_worker.rs` (`medscale-compute-worker`; links only the contracts crate) |
| Qualification harness | `crates/medscale-contracts/src/bin/compute_fault_worker.rs` (`medscale-compute-fault-worker`; named only by tests) |
| Storage v14 | `crates/medscale-storage/src/compute.rs` (+ `sqlite_meta.rs`, `backup.rs`); the Spec 078-084 storage rewind tests drop the v14 tables |
| Core | `crates/medscale-core/src/compute_supervisor.rs` (process launch and supervision), `crates/medscale-core/src/authority/compute.rs` (admission, run, cancel, recover, reads), facade wiring, `CliSession` methods |
| CLI | `crates/medscale-cli/src/compute.rs` (`medscale compute ...`) |
| Tests | contract unit tests; `crates/medscale-storage/tests/compute_085.rs`; `crates/medscale-core/tests/compute_085.rs`; CLI test |

No new crate and no new dependency. The worker executable is found only
next to the running executable (or, for test binaries, in the parent of
`target/<profile>/deps`); there is no environment or request override.

## Qualification

fmt, dependency direction, Clippy `-D warnings`, workspace tests,
cargo-deny/supply chain, the deterministic exact-range scope record and
security challenge, and exact-head plus post-main CI, per
`FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## Evidence (`evidence/085-compute/`)

README, LIVE_TRUTH, QUALIFICATION, SECURITY, EXACT_RANGE_REVIEW,
EXACT_HEAD_QUALIFICATION, and after merge POST_MERGE_VERIFICATION and
CLOSURE.
