# Plan — Spec 086 R Workspace

| Layer | Path |
|---|---|
| Contracts | `crates/medscale-contracts/src/r_workspace.rs` (types, vocabularies, manifest validation, launch allowlist, deterministic CSV/schema/README/`.Rproj` renderers); R Workspace capabilities, requests and responses in `envelopes/mod.rs` |
| Qualification harness | `crates/medscale-contracts/src/bin/r_fake_ide.rs` (`medscale-r-fake-ide`; stands in for an IDE in tests; not shipped) |
| Storage v15 | `crates/medscale-storage/src/r_workspace.rs` (+ `sqlite_meta.rs`, `backup.rs`); the Spec 078-085 storage rewind tests drop the v15 tables |
| Core | `crates/medscale-core/src/r_workspace_host.rs` (host configuration, launcher), `crates/medscale-core/src/authority/r_workspace.rs` (stage, inspect, launch, run refusal, publish, reads), facade wiring, `CliSession` methods |
| CLI | `crates/medscale-cli/src/r_workspace.rs` (`medscale r ...`) |
| Tests | contract unit tests; `crates/medscale-storage/tests/r_workspace_086.rs`; `crates/medscale-core/tests/r_workspace_086.rs`; CLI test |

No new crate and no new dependency. Publication reuses the Spec 075 CSV
parser (`parse_delimited`) so published tables are typed exactly as
imported data.

## Qualification

fmt, dependency direction, Clippy `-D warnings`, workspace tests,
cargo-deny/supply chain, the deterministic exact-range scope record and
security challenge, and exact-head plus post-main CI, per
`FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## Evidence (`evidence/086-r-workspace/`)

README, LIVE_TRUTH, QUALIFICATION, SECURITY, EXACT_RANGE_REVIEW,
EXACT_HEAD_QUALIFICATION, and after merge POST_MERGE_VERIFICATION and
CLOSURE.
