# Qualification — Spec 092 Whole-Platform Qualification

`PASS` is recorded only for rows that exact-head CI ran green on Linux,
Windows and macOS (see `EXACT_HEAD_QUALIFICATION.md`, written with the
closure); until then rows are `PENDING`.

| # | Requirement | Tests | Status |
|---|---|---|---|
| Q1 | One synthetic vault crosses every Research OS plane through Core, then backup, restore, every storage consistency verifier and restart; honest defaults hold afterwards | core `every_plane_survives_backup_restore_and_restart_through_core` | PENDING |
| Q2 | The campaign's defect (id sequences lost on restore) is fixed and regression-tested | storage `restored_vaults_continue_every_id_sequence`, `malformed_sequences_are_refused_and_old_backups_still_restore` (PR #160) | PENDING |
| Q3 | 35-area matrix with PROVEN / IMPLEMENTED / PARTIAL / MISSING / DEFERRED / EXTERNAL / N/A, bound to `EXTERNAL_GATES.md` | `MATRIX.md` (document; not a test) | RECORDED |
| Q4 | Exact-head and post-main CI | - | PENDING |

Not claimed: release readiness, private-data readiness, platform qualification, clinical validation, regulatory status.
