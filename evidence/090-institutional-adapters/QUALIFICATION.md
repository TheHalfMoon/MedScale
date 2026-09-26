# Qualification — Spec 090 Institutional Adapters (object storage path)

`PASS` is recorded only for rows that exact-head CI ran green on Linux,
Windows and macOS (see `EXACT_HEAD_QUALIFICATION.md`, written with the
closure); until then rows are `PENDING`.

| # | Requirement | Tests | Status |
|---|---|---|---|
| Q1 | Config validation: `local_phi` never leaves; credential handles, not secrets; plain object keys; payload-bound idempotency keys | contract `local_phi_never_leaves`, `credentials_are_handles_not_secrets`, `object_keys_are_relative_plain_paths`, `idempotency_keys_bind_the_payload` | PENDING |
| Q2 | Intent, send, confirm with receipts; confirmed writes never resend | core `a_write_is_intended_sent_and_confirmed_with_receipts` | PENDING |
| Q3 | Policy: unclassified and over-ceiling data never leave | core `local_phi_and_over_ceiling_data_never_leave` | PENDING |
| Q4 | Outage -> failed; lost answer -> unknown; no blind retry; reconciliation confirms, re-arms or fails | core `uncertain_writes_become_unknown_and_reconcile_never_blindly_retries` | PENDING |
| Q5 | Suspend, terminal revoke, reconfigure and rollback | core `revocation_and_rollback_are_explicit_and_final` | PENDING |
| Q6 | The product transport sends nothing | core `the_product_transport_sends_nothing` | PENDING |
| Q7 | Crash recovery (`sent -> unknown`, no resend); storage v19 migration, CAS, tamper refusal, backup/restore | core `a_crash_after_sent_recovers_as_unknown_never_as_a_resend`; storage `migration_to_v19_is_additive`, `intents_are_bound_and_changes_are_compare_and_set`, `backup_restore_round_trips_adapter_rows`, `tampered_adapter_backups_are_refused` | PENDING |
| Q8 | CLI through Core with a closed act vocabulary | CLI `acts_parse_strictly` | PENDING |
| Q9 | Exact-head and post-main CI | - | PENDING |

Not claimed: no network transport or real institutional endpoint; identity and compute institutional paths are not in this slice.
