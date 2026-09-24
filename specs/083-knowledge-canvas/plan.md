# Plan — Spec 083 Knowledge + Research Canvas

| Layer | Path |
|---|---|
| Contracts | `crates/medscale-contracts/src/knowledge.rs` (+ `Ord` on `OpaqueId`/`DigestSha256` for sorted source sets) |
| Storage v12 | `crates/medscale-storage/src/knowledge.rs` (+ `sqlite_meta.rs`, `backup.rs`) |
| Core | `crates/medscale-core/src/authority/knowledge.rs` (on the Spec 075 `DataSources` authority; reads Spec 080/081/082 rows through the metadata store with scope and Project checks), facade wiring, `CliSession` methods |
| CLI | `crates/medscale-cli/src/knowledge.rs` (`medscale knowledge ...`) |
| Desktop | `crates/medscale-desktop/src/knowledge_workspace.rs` + "Knowledge" route |
| Tests | contract and Core unit tests; `crates/medscale-storage/tests/knowledge_083.rs`; `crates/medscale-core/tests/knowledge_083.rs`; CLI and Desktop tests; the Spec 078-082 storage rewind tests drop the v12 tables |

No new crate and no new dependency. Tokenizing and scoring are a few dozen
lines of integer Rust; no search engine or vector library is admitted.

## Qualification

fmt, dependency direction, Clippy `-D warnings`, workspace tests,
cargo-deny/supply chain, the deterministic exact-range scope record and
security challenge, and exact-head plus post-main CI, per
`FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## Evidence (`evidence/083-knowledge-canvas/`)

README, LIVE_TRUTH, QUALIFICATION, SECURITY, EXACT_RANGE_REVIEW,
EXACT_HEAD_QUALIFICATION, and after merge POST_MERGE_VERIFICATION and
CLOSURE.
