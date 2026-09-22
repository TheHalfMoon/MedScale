# Tasks — Spec 079 Privacy Gate

A task is checked only with implementation + tests + evidence + the
qualifying CI run named.

## T079-00 — Live truth and baseline

- [ ] Record base SHA, open PRs, CI state and schema version in
      `evidence/079-privacy-gate/LIVE_TRUTH.md`.

## T079-01 — Contracts freeze

- [ ] Implement `privacy_gate.rs` contracts per `contracts.md`.
- [ ] Unit tests: vocabularies round-trip; profile completeness; class
      restrictiveness order; receipt/classification basis invariants;
      limitation vocabulary has no absence claim; bounds.
- [ ] Fill the freeze record in `contracts.md`.

## T079-02 — Storage v8

- [ ] Tables per `migration.md`; v7 -> v8 migration; backup/restore rows;
      `verify_privacy_gate_consistency`.
- [ ] Tests: populated v7 vault migrates and reopens; repeated open; crash
      mid-migration recovers from backup; uncommitted writes absent; stale
      revision refused; every invariant break detected; tampered backup
      refused; v7 backup restores with empty 079 tables.

## T079-03 — Recognizers and corpus

- [ ] Deterministic, FHIR-aware and model recognizers with identity/version.
- [ ] Synthetic corpus (EN/ES/FR/DE notes + FHIR Patient) with expected spans.
- [ ] Class-wise count test (found/missed/extra per kind) recorded in
      `RECOGNIZER_BENCHMARK.md`.

## T079-04 — Classification + profiles

- [ ] Core + envelope + CLI: classify, get classification, create/list/get/
      revoke profile.
- [ ] Tests: default `LocalPhi`; survives reopen; incomplete profile refused
      before write; foreign Project refused; stale revision refused.

## T079-05 — Transform + receipt

- [ ] Core transform writes a derived artifact, a receipt and a
      `DeidReceipt`-basis classification in one unit.
- [ ] Tests: source unchanged; digests bound; residual re-scan; unavailable
      model recognizer -> `Unavailable`; revoked profile refused; bounds;
      invalid UTF-8 refused; no plaintext in persisted rows (T3).

## T079-06 — Pseudonym maps

- [ ] Map create, keyed pseudonyms, sealed entries, re-identification with
      audit, revocation destroys key.
- [ ] Tests: stability within map, difference across maps; capability
      required; audit before return; revoked/unknown/key-unavailable deny
      and audit; key bytes absent from DB and backups (T4).

## T079-07 — Egress decisions

- [ ] `evaluate_egress` for every `EgressBoundary`; persisted decisions;
      revocation of receipts.
- [ ] Tests: full boundary x class matrix; revoked receipt, digest mismatch,
      residual detected/unavailable, revoked profile all deny.

## T079-08 — Desktop + CLI parity

- [ ] Privacy route (classification, profiles, transforms/receipts, egress
      check) backed by Core; CLI human + JSON output.
- [ ] Tests: view-model functions over a real Core session; CLI across fresh
      sessions.

## T079-09 — Qualification and closure

- [ ] fmt, dependency direction, Clippy, workspace tests, cargo-deny.
- [ ] Migration/recovery and security suites.
- [ ] Rendered Desktop evidence or recorded residual.
- [ ] Deterministic exact-range scope record (`EXACT_RANGE_REVIEW.md`).
- [ ] Exact-head required CI (6/6) on the final candidate head.
- [ ] Evidence files complete.
- [ ] Merge on green exact head; post-main CI.
- [ ] Closure bookkeeping PR; `CLOSED_CANONICAL` only after post-main evidence.
- [ ] Recompute the next eligible unit.
