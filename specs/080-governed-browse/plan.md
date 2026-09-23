# Plan — Spec 080 Governed Browse

| Layer | Path |
|---|---|
| Contracts | `crates/medscale-contracts/src/browse.rs` |
| Network (sole egress) | `crates/medscale-network/src/browse.rs` |
| Storage v9 | `crates/medscale-storage/src/browse.rs` (+ `sqlite_meta.rs`, `backup.rs`) |
| Core | `crates/medscale-core/src/authority/browse.rs`, facade wiring, `CliSession` methods |
| CLI | `crates/medscale-cli/src/browse.rs` (`medscale browse ...`) |
| Desktop | `crates/medscale-desktop/src/browse_workspace.rs` + "Browse" route |
| Tests | contracts, network and Core unit tests; `crates/medscale-storage/tests/browse_080.rs`; `crates/medscale-core/tests/browse_080.rs`; CLI and Desktop tests |

No new crate and no new dependency (`ureq` 3.4 is already admitted in
`medscale-network`). Hermetic tests use `ScriptedBrowseTransport`; no test
opens a socket.

## Qualification

fmt, dependency direction, Clippy `-D warnings`, workspace tests,
cargo-deny/supply chain, the deterministic exact-range scope record, and
exact-head plus post-main CI, per
`FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## Evidence (`evidence/080-governed-browse/`)

README, LIVE_TRUTH, CONTRACT_QUALIFICATION, NETWORK_SSRF_QUALIFICATION,
STORAGE_MIGRATION_RECOVERY, CORE_AUTHORITY_QUALIFICATION, CLI_QUALIFICATION,
DESKTOP_QUALIFICATION, SECURITY_ADVERSARIAL, EXACT_RANGE_REVIEW,
EXACT_HEAD_QUALIFICATION, POST_MERGE_VERIFICATION, CLOSURE.
