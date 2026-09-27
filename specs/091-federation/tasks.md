# Tasks — Spec 091 Federation

- [x] T091-00 Live truth and promotion (base = Spec 090 closure).
- [x] T091-01 Contracts: institution identity, peers, signed bundles, items, tombstones, receipts; unit tests.
- [x] T091-02 Storage v20: identity (secret never exported), peers, sequences, imported items, tombstones; atomic imports; backup/restore.
- [x] T091-03 Core authority (identity, trust, revoke, export, import, view), facade, CliSession; two-institution integration tests (policy, forgery, replay, revocation).
- [x] T091-04 CLI `medscale federation`.
- [x] T091-05 Exact-head CI; security challenge; evidence; merge; post-main; closure.

Reconciled 2026-09-27: final head `c9c523e` passed exact-head run `36331357728`
(6/6); PR #158 merged as `c4438ef`; post-main `36338491427`. See
`evidence/091-federation/CLOSURE.md`.
