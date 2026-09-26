# Tasks — Spec 087 Community Extensions

## T087-00 — Live truth and promotion
- [x] Record base SHA (Spec 086 closure main), open PRs, CI state, schema version and sandbox state (`evidence/087-community-extensions/LIVE_TRUTH.md`).

## T087-01 — Contracts
- [ ] `extensions.rs` manifest, pack, vocabularies, receipts; unit tests.

## T087-02 — Storage v16
- [ ] Tables, migration, compare-and-set installs, atomic revocation, consistency, backup/restore; earlier rewind tests updated.

## T087-03 — Core authority
- [ ] Verification, lifecycle, grants, invocation; facade and `CliSession`; integration tests.

## T087-04 — CLI and SDK
- [ ] `medscale extension keygen|pack|trust|install|set|grant|invoke|revoke|list`.

## T087-05 — Qualification and closure
- [ ] Exact-head CI; security challenge; evidence; merge; post-main; closure PR.

Items are drafted locally; they are complete only once CI compiles and
passes them.
