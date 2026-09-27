# Tasks — Spec 090 Institutional Adapters

- [x] T090-00 Live truth and promotion (base = Spec 089 closure).
- [x] T090-01 Contracts: adapters, intents, effect states, receipts; unit tests.
- [x] T090-02 Transport interface: unavailable product transport; in-process store with fault injection.
- [x] T090-03 Storage v19: adapters, intents, receipts; CAS, consistency, backup/restore.
- [x] T090-04 Core authority (register, reconfigure, rollback, state, intend, send, reconcile, retry, recover, list, view), facade, CliSession; integration tests including crash recovery to `unknown`.
- [x] T090-05 CLI `medscale adapter act|show`.
- [x] T090-06 Exact-head CI; security challenge; evidence; merge; post-main; closure.

Reconciled 2026-09-27: final head `b1dc801` passed exact-head run `36310428876`
(6/6); PR #157 merged as `2cc0f35`; post-main `36318426673`. See
`evidence/090-institutional-adapters/CLOSURE.md`.
