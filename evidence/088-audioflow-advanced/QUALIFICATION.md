# Qualification — Spec 088 AudioFlow Advanced (huddle foundation)

`PASS` is recorded only for rows that exact-head CI ran green on Linux,
Windows and macOS (see `EXACT_HEAD_QUALIFICATION.md`, written with the
closure); until then rows are `PENDING`.

| # | Requirement | Tests | Status |
|---|---|---|---|
| Q1 | Per-act consent of every human; join gates everything; withdrawal is immediate | core `every_act_needs_every_humans_consent_for_that_act`; contract `consent_needs_joining_first` | PENDING |
| Q2 | Agents only join, never consent for humans; agent audio labeled synthetic | core `agents_never_consent_for_humans_and_their_audio_is_labeled`; contract `synthetic_audio_is_always_labeled`; storage `unlabeled_synthetic_media_is_refused` | PENDING |
| Q3 | Proposals cite segments and stay proposals until human review | core `proposals_stay_proposals_until_a_human_reviews_them`; contract `proposals_cite_segments_and_reviews_name_reviewers` | PENDING |
| Q4 | Retention and deletion remove audio and transcripts with receipts | core `deletion_and_retention_remove_audio_and_transcripts_with_receipts`; storage `changes_are_compare_and_set_and_deletion_is_atomic` | PENDING |
| Q5 | Storage v17 migration, tamper refusal, backup/restore | storage `migration_to_v17_is_additive`, `tampered_huddle_backups_are_refused`, `backup_restore_round_trips_huddle_rows` | PENDING |
| Q6 | CLI through Core with a closed act vocabulary | CLI `acts_parse_strictly` | PENDING |
| Q7 | Exact-head and post-main CI | - | PENDING |

Not claimed: speech synthesis, voice cloning and duplex agent voice are NOT_ADMITTED; medical ASR quality is UNMEASURED.
