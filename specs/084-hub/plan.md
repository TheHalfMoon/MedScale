# Plan — Spec 084 MedScale Hub Foundation

| Layer | Path |
|---|---|
| Contracts | `crates/medscale-contracts/src/hub.rs`; Hub capabilities, requests and responses in `envelopes/mod.rs` |
| Keys | `crates/medscale-keys/src/device_key.rs` (ed25519 device keys on the admitted `ed25519-dalek`; strict hex; `verify_strict`) |
| Storage v13 | `crates/medscale-storage/src/hub.rs` (+ `sqlite_meta.rs`, `backup.rs`); the Spec 078-083 storage rewind tests drop the v13 tables |
| Core | `crates/medscale-core/src/authority/hub.rs` (`Hub` builds a `Collab` under the device's session; `HubClient` for links, outbox and mirror), facade wiring, `SessionRegistry::revoke_holder`, `crates/medscale-core/src/hub_sync.rs` (join, sync, in-process and IPC transports, `serve_hub`), `CliSession` methods |
| CLI | `crates/medscale-cli/src/hub.rs` (`medscale hub ...`) |
| Tests | contract and key unit tests; `crates/medscale-storage/tests/hub_084.rs`; `crates/medscale-core/tests/hub_084.rs`; CLI test over the local IPC |

No new crate and no new dependency. Every Hub request is an ordinary
`AuthorityRequest`, carried in-process or by the Spec 024 local-socket IPC.

## Qualification

fmt, dependency direction, Clippy `-D warnings`, workspace tests,
cargo-deny/supply chain, the deterministic exact-range scope record and
security challenge, and exact-head plus post-main CI, per
`FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## Evidence (`evidence/084-hub/`)

README, LIVE_TRUTH, QUALIFICATION, SECURITY, EXACT_RANGE_REVIEW,
EXACT_HEAD_QUALIFICATION, and after merge POST_MERGE_VERIFICATION and
CLOSURE.
