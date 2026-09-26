# Tasks — Spec 087 Community Extensions

## T087-00 — Live truth and promotion
- [x] Record base SHA (Spec 086 closure main), open PRs, CI state, schema version and sandbox state (`evidence/087-community-extensions/LIVE_TRUTH.md`).

## T087-01 — Contracts
- [x] `extensions.rs` manifest, pack, vocabularies, receipts; unit tests.

## T087-02 — Storage v16
- [x] Tables, migration, compare-and-set installs, atomic revocation, consistency, backup/restore; earlier rewind tests updated.

## T087-03 — Core authority
- [x] Verification, lifecycle, grants, invocation; facade and `CliSession`; integration tests.

## T087-04 — CLI and SDK
- [x] `medscale extension keygen|pack|trust|install|set|grant|invoke|revoke|list`.

## T087-05 — Qualification and closure
- [x] Exact-head CI; security challenge; evidence; merge; post-main; closure PR.

Reconciled 2026-09-26: final head `4405062` passed exact-head run `36258146789`
(6/6); PR #153 merged as `2045998`; post-main `36263227436`. See
`evidence/087-community-extensions/CLOSURE.md`.
