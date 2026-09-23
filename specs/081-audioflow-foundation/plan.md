# Plan — Spec 081 AudioFlow Foundation

| Layer | Path |
|---|---|
| Contracts | `crates/medscale-contracts/src/audio.rs` |
| Storage v10 | `crates/medscale-storage/src/audio.rs` (+ `sqlite_meta.rs`, `backup.rs`) |
| Core | `crates/medscale-core/src/authority/audio.rs` (WAV, VAD, engines, authority), facade wiring, `CliSession` methods |
| CLI | `crates/medscale-cli/src/audio.rs` (`medscale audio ...`) |
| Desktop | `crates/medscale-desktop/src/audio_workspace.rs` + "Audio" route |
| Tests | contract and Core unit tests; `crates/medscale-storage/tests/audio_081.rs`; `crates/medscale-core/tests/audio_081.rs`; CLI and Desktop tests |

No new crate and no new dependency. Signal processing is small, pure Rust in
Core, and there is no audio device code. All fixtures are synthetic tones and
silence generated in code.

## Qualification

fmt, dependency direction, Clippy `-D warnings`, workspace tests,
cargo-deny/supply chain, the deterministic exact-range scope record, and
exact-head plus post-main CI, per
`FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## Evidence (`evidence/081-audioflow-foundation/`)

README, LIVE_TRUTH, QUALIFICATION, SECURITY_PRIVACY, EXACT_RANGE_REVIEW,
EXACT_HEAD_QUALIFICATION, and after merge POST_MERGE_VERIFICATION and
CLOSURE.
