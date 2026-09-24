# Plan — Spec 082 Analytics Gate

| Layer | Path |
|---|---|
| Contracts | `crates/medscale-contracts/src/analytics.rs` |
| Engine | `crates/medscale-storage/src/analytics_engine.rs` (read-only in-memory SQLite) |
| Storage v11 | `crates/medscale-storage/src/analytics.rs` (+ `sqlite_meta.rs`, `backup.rs`) |
| Core | `crates/medscale-core/src/authority/analytics.rs` (on the Spec 075 `DataSources` authority), facade wiring, `CliSession` methods |
| CLI | `crates/medscale-cli/src/analytics.rs` (`medscale analytics ...`) |
| Desktop | `crates/medscale-desktop/src/analytics_workspace.rs` + "Analytics" route |
| Tests | contract, engine and Core unit tests; `crates/medscale-storage/tests/analytics_082.rs`; `crates/medscale-core/tests/analytics_082.rs`; CLI and Desktop tests |

No new crate and no new dependency (SQLite through the admitted `rusqlite`).
The engine lives in `medscale-storage` because that crate owns SQLite; Core
owns policy, bindings, receipts and replay. Snapshots are read only through
the Spec 075 scope checks and digest-verified loader. Five 075 helpers
become `pub(super)` inside `authority` for this.

## Qualification

fmt, dependency direction, Clippy `-D warnings`, workspace tests,
cargo-deny/supply chain, the deterministic exact-range scope record, and
exact-head plus post-main CI, per
`FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## Evidence (`evidence/082-analytics-gate/`)

README, LIVE_TRUTH, QUALIFICATION, SECURITY, EXACT_RANGE_REVIEW,
EXACT_HEAD_QUALIFICATION, and after merge POST_MERGE_VERIFICATION and
CLOSURE.
