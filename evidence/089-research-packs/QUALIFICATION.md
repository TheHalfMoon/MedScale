# Qualification — Spec 089 Research Packs (Clinical Research first)

`PASS` is recorded only for rows that exact-head CI ran green on Linux,
Windows and macOS (see `EXACT_HEAD_QUALIFICATION.md`, written with the
closure); until then rows are `PENDING`.

| # | Requirement | Tests | Status |
|---|---|---|---|
| Q1 | Built-in Pack versions validate; migrations are non-destructive | contract `built_in_pack_versions_validate_and_migrate_forward_only` | PENDING |
| Q2 | Fields checked by kind; explicit `unknown`; workflows name known states | contract `fields_check_their_kind_and_allow_unknown`, `workflows_name_known_states` | PENDING |
| Q3 | Evidence axes stay explicit; missing or retracted citations never support | contract `evidence_axes_stay_explicit`; core workflow test (assessment) | PENDING |
| Q4 | Pack adds validated artifacts and workflows without new authority | core `a_pack_adds_validated_artifacts_and_workflows_without_new_authority` | PENDING |
| Q5 | Upgrade migrates without loss; uninstall keeps data read-only; reinstall re-enables | core `upgrades_migrate_without_loss_and_uninstall_keeps_data` | PENDING |
| Q6 | Storage v18: rows checked against shipped Packs; migration; tamper refusal | storage `migration_to_v18_is_additive`, `rows_are_checked_against_the_shipped_pack`, `tampered_pack_backups_are_refused`, `backup_restore_round_trips_pack_rows` | PENDING |
| Q7 | CLI through Core with a closed act vocabulary | CLI `acts_parse_strictly` | PENDING |
| Q8 | Exact-head and post-main CI | - | PENDING |

Not claimed: no clinical correctness of any assessment; third-party Packs, other domains and executable Pack features are not in this slice.
