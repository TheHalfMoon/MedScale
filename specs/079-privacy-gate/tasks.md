# Tasks — Spec 079 Privacy Gate

A task is checked only with implementation + tests + evidence + the
qualifying CI run named.

## T079-00 — Live truth and baseline

- [x] Record base SHA, open PRs, CI state and schema version in
      `evidence/079-privacy-gate/LIVE_TRUTH.md`.

## T079-01 — Contracts freeze

- [x] Implement `privacy_gate.rs` contracts per `contracts.md`.
- [x] Unit tests: vocabularies round-trip; profile completeness; class
      restrictiveness order; receipt/classification basis invariants;
      limitation vocabulary has no absence claim; bounds.
- [x] Fill the freeze record in `contracts.md`.

## T079-02 — Storage v8

- [x] Tables per `migration.md`; v7 -> v8 migration; backup/restore rows;
      `verify_privacy_gate_consistency`.
- [x] Tests: populated v7 vault migrates and reopens; repeated open; crash
      mid-migration recovers from backup; uncommitted writes absent; stale
      revision refused; every invariant break detected; tampered backup
      refused; v7 backup restores with empty 079 tables.

## T079-03 — Recognizers and corpus

- [x] Deterministic, FHIR-aware and model recognizers with identity/version.
- [x] Synthetic corpus (EN/ES/FR/DE notes + FHIR Patient) with expected spans.
- [x] Class-wise count test (found/missed/extra per kind) recorded in
      `RECOGNIZER_BENCHMARK.md`.

## T079-04 — Classification + profiles

- [x] Core + envelope + CLI: classify, get classification, create/list/get/
      revoke profile.
- [x] Tests: default `LocalPhi`; survives reopen; incomplete profile refused
      before write; foreign Project refused; stale revision refused.

## T079-05 — Transform + receipt

- [x] Core transform writes a derived artifact, a receipt and a
      `DeidReceipt`-basis classification in one unit.
- [x] Tests: source unchanged; digests bound; residual re-scan; unavailable
      model recognizer -> `Unavailable`; revoked profile refused; bounds;
      invalid UTF-8 refused; no plaintext in persisted rows (T3).

## T079-06 — Pseudonym maps

- [x] Map create, keyed pseudonyms, sealed entries, re-identification with
      audit, revocation destroys key.
- [x] Tests: stability within map, difference across maps; capability
      required; audit before return; revoked/unknown/key-unavailable deny
      and audit; key bytes absent from DB and backups (T4).

## T079-07 — Egress decisions

- [x] `evaluate_egress` for every `EgressBoundary`; persisted decisions;
      revocation of receipts.
- [x] Tests: full boundary x class matrix; revoked receipt, digest mismatch,
      residual detected/unavailable, revoked profile all deny.

## T079-08 — Desktop + CLI parity

- [x] Privacy route (classification, profiles, transforms/receipts, egress
      check) backed by Core; CLI human + JSON output.
- [x] Tests: view-model functions over a real Core session; CLI across fresh
      sessions.

**Verified (T079-00..08):** code head `82d3154` passed exact-head run
`35802759307` (6/6: fmt, dependency direction, Clippy `-D warnings`, full
workspace tests on ubuntu/macOS/Windows, cargo-deny, supply chain, packaging).
Test-to-requirement mapping: `evidence/079-privacy-gate/` (contracts 13,
storage 12, Core 10, recognizers 12 incl. corpus benchmark, CLI 2, Desktop 2).
Honest residuals: no rendered Desktop screenshot; profiles, transforms and
re-identification are CLI-only; corpus is a development set, not a held-out
benchmark; non-English narrative fixtures not built (founder language
directive); OS keyring path not exercised in CI.

## T079-09 — Qualification and closure

- [x] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [x] Migration/recovery and security suites.
- [x] Rendered Desktop evidence or recorded residual.
- [x] Deterministic exact-range scope record (`EXACT_RANGE_REVIEW.md`).
- [ ] Exact-head required CI (6/6) on the final candidate head.
- [x] Evidence files complete.
- [ ] Merge on green exact head; post-main CI.
- [ ] Closure bookkeeping PR; `CLOSED_CANONICAL` only after post-main evidence.
- [ ] Recompute the next eligible unit.
