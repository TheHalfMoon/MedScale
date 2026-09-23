# Plan — Spec 079 Privacy Gate

## Placement

| Layer | Path |
|---|---|
| Contracts | `crates/medscale-contracts/src/privacy_gate.rs` |
| Storage | `crates/medscale-storage/src/privacy_gate.rs` (+ v8 registration in `sqlite_meta.rs`, rows in `backup.rs`) |
| Core recognizers | `crates/medscale-core/src/authority/privacy_recognizers.rs` (pure functions, no I/O except the model recognizer via `medscale-pack`) |
| Core authority | `crates/medscale-core/src/authority/privacy_gate.rs` (classification, profiles, transform, receipts, maps, egress) |
| Envelope | `crates/medscale-contracts/src/envelopes/mod.rs` (capabilities + request/response bodies) |
| CLI | `crates/medscale-cli/src/privacy_gate.rs` |
| Desktop | `crates/medscale-desktop/src/privacy_workspace.rs` + Privacy route in `ui/app.slint` |
| Tests | `crates/medscale-storage/tests/privacy_gate_079.rs`, `crates/medscale-core/tests/privacy_gate_079.rs` |
| Corpus | `crates/medscale-core/tests/fixtures/privacy_079/` (synthetic, English/Spanish/French/German + FHIR Patient JSON, each with an expected-span manifest) |

No new crate. No new third-party dependency. HMAC-SHA256 is implemented on
the workspace `sha2` and tested against RFC 4231 vectors. Sealing uses
`medscale_keys::aead_wrap::{seal, unseal}`. Keys use the `KeyStore` trait
(`MemoryKeyStore` in tests).

## Slices

- 079-A (T079-00/01) live truth, contracts, invariant unit tests.
- 079-B (T079-02) storage v8, migration, backup/restore, consistency,
  crash/reopen tests.
- 079-C (T079-03) recognizers + corpus + class-wise count test.
- 079-D (T079-04) classification + profiles through Core/envelope/CLI.
- 079-E (T079-05) transform + receipt + residual scan.
- 079-F (T079-06) pseudonym maps, key separation, re-identification,
  revocation.
- 079-G (T079-07) egress decisions for all boundaries.
- 079-H (T079-08) Desktop route + CLI parity.
- 079-I (T079-09) qualification and closure.

## Qualification (T079-09)

1. `cargo fmt --all -- --check`; dependency-direction script.
2. Clippy `-D warnings`; full workspace tests; cargo-deny/supply chain (CI).
3. Migration/reopen/recovery suite, including v7 fixtures and backup/restore.
4. Security suite T1-T14 (`security.md`).
5. Leakage scans (T3/T4) as automated tests.
6. Rendered Desktop evidence if CI allows; otherwise the honest residual
   recorded by Specs 075-078.
7. Deterministic exact-range scope record (`EXACT_RANGE_REVIEW.md`) per
   `FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`; no external reviewer.
8. Exact-head required CI (6/6); evidence under `evidence/079-privacy-gate/`.
9. Merge on green exact head; post-main CI; closure PR; recompute queue.

## Evidence set (`evidence/079-privacy-gate/`)

```text
README.md, LIVE_TRUTH.md, CONTRACT_QUALIFICATION.md,
STORAGE_MIGRATION_RECOVERY.md, RECOGNIZER_BENCHMARK.md,
CORE_AUTHORITY_QUALIFICATION.md, CLI_QUALIFICATION.md,
DESKTOP_QUALIFICATION.md, SECURITY_ADVERSARIAL.md, NO_NETWORK_LOCAL_PATH.md,
EXACT_RANGE_REVIEW.md, EXACT_HEAD_QUALIFICATION.md,
POST_MERGE_VERIFICATION.md, CLOSURE.md
```
