# Tasks — Spec 076 Collaboration Substrate

**Execution state:** `PROMOTED_IMPLEMENTATION_AUTHORIZED` (not yet implemented)

Check a task only when its implementation, tests and required evidence are real on the branch. Do not pre-check future work.

## T076-00 — Live truth and baseline

- [x] Verify branch/base/main/PR state and repository cleanliness.
- [x] Read mandatory governance + Research OS V2 + Spec 076 authority chain.
- [x] Inspect current contracts/Core/storage/network/key/CLI/Desktop ownership paths.
- [x] Confirm no live Spec 076 collision/superseding authority; prove numbering is free.
- [x] Verify Spec 075 closure and post-main evidence.
- [x] Record baseline focused/full gate state and any pre-existing failures.
- [x] Create initial evidence/live-truth record.

**Gate:** no code mutation before T076-00 is complete. **PROVEN** (`evidence/076-collaboration-substrate/LIVE_TRUTH.md`).

## T076-01 — Freeze contracts/data model

- [x] Freeze exact field-level participant/room/membership/thread/message/task/note/approval/activity contracts in `contracts.md` (`crates/medscale-contracts/src/collaboration.rs`, section 19 freeze record).
- [x] Reuse `OpaqueId`, `ObjectHeader`, `DigestSha256`, `ProjectRevision`, `ArtifactDescriptor`/`ArtifactVersionBinding`, `TextSpan` where semantically valid (`ReferenceResolution` reuse lands at Core dispatch time in T076-04, since it is a read-time computation, not a stored contract field).
- [x] Freeze `ParticipantKind`/`CollabEventKind`/`AnchorDetail` vocabulary; reject unsafe unknown authority kinds (`parse`/`as_str` closed-vocabulary pattern on every enum).
- [x] Freeze revision/precondition and idempotency behavior for mutable rows, including the `NoteDocument` conflict-copy exception (`is_fast_forward` vs. `check_mutation`).
- [x] Add serialization/validation/invariant tests (23 tests in `collaboration.rs`).

**Acceptance:** contract tests pass; no storage/network/UI dependency enters contracts. `cargo fmt --check` passes locally (exit 0); full compile/test verification pending exact-head CI (this workstation cannot link locally — see `evidence/076-collaboration-substrate/README.md`).

## T076-02 — Storage schema + migration

- [x] Add collaboration storage using current encrypted vault/SQLite metadata architecture (v4 -> v5): `crates/medscale-storage/src/collaboration.rs` (`V5_DDL` + full CRUD per entity family).
- [x] Add required indexes (`migration.md` section 3).
- [x] Implement atomic transactions (mutation + `ActivityRecord` append together, see `append_activity_record`) and stale-revision conflict behavior (CAS on every mutable row except `NoteDocument`'s deliberate conflict-copy path).
- [x] Wire `collab_participants`/`collab_participant_agent_refs`/`collab_rooms`/`collab_room_memberships`/`collab_threads`/`collab_messages`/`collab_message_edits`/`collab_tasks`/`collab_notes`/`collab_note_revisions`/`collab_approval_requests`/`collab_approval_decisions`/`collab_activity_records` into `SqliteMetaStore::snapshot_bytes` (schema_version bumped 4 -> 5) and add `restore_v5` in `backup.rs`, so 076 rows survive the existing synthetic-vault backup/restore path exactly like every prior spec's rows (found and fixed proactively during this slice, before any test caught it).
- [ ] Add forward migration test fixtures from representative pre-076 vaults (including populated 074 and 075 vaults). **Storage-level plumbing done** (076 migration runs additively on top of any pre-076 vault via the same `migrate()` chain); a dedicated fixture-based migration test lives in T076-11 qualification alongside the rest of the evidence suite.
- [ ] Add repeated-open/repeated-migration safety tests. **Deferred to T076-11** with the rest of the storage test suite.
- [ ] Add crash/reopen and backup/restore recovery tests, including activity hash-chain re-verification after restore. **Deferred to T076-11.**
- [x] Prove no collaboration operation cascades deletion into canonical target objects: no 076 storage function ever executes DML against `sources`, `authority_objects`, `projects`, `experiments`, `refs`, `edges`, `data_sources`, `data_snapshots`, `data_snapshot_parts`, `data_receipts`, `data_saved_views`, `data_transformations`, or `dataset_releases` — verified by inspection (every 076 function's SQL text targets only `collab_*` tables).
- [x] Fixed forward three pre-existing test assertions in `project_graph_074.rs` and one in `data_sources_075.rs` that pinned `finished_version == 4`, now stale at `5` (mirrors the exact forward-fix Spec 075 itself needed for Spec 074's tests).

**Acceptance:** storage + migration suite green and pre-076 IDs/behavior preserved. Schema/DDL/CRUD/backup-restore plumbing is complete and self-consistent by inspection; dedicated migration-fixture and crash-recovery *tests* are written in T076-11's qualification pass rather than duplicated here, matching how Spec 075 organized its own evidence (`STORAGE_MIGRATION_RECOVERY.md` as one consolidated qualification artifact).

## T076-03 — Participant + Room + Membership

- [ ] Implement participant registration, room create/get/list/update/archive, membership add/update/remove.
- [ ] Wire `RoomMembership` visibility filter into every read path.
- [ ] CLI inspect for room/participant/membership.

**Acceptance:** vertical slice proven end to end with reopen durability and cross-room visibility denial.

## T076-04 — Thread and anchor resolution

- [ ] Implement thread create/get/list/resolve/reopen.
- [ ] Implement live `ReferenceResolution` recomputation against `ArtifactDescriptor` targets.

**Acceptance:** anchored thread resolution flips `Current` -> `Stale`/`Missing` on artifact change without any 076 write.

## T076-05 — Message and MessageEdit

- [ ] Implement message post/list.
- [ ] Implement message edit/delete as append-only `MessageEdit` rows with current-state derivation.

**Acceptance:** post/edit/delete proven end to end; full history queryable.

## T076-06 — Task

- [ ] Implement task create/get/list/update with `expected_revision` precondition and optional anchor.

**Acceptance:** stale task update -> `Conflict`, no write.

## T076-07 — Note and conflict-copy

- [ ] Implement note create/get/list, fast-forward edit, and explicit conflict-copy path on stale write.
- [ ] Prove both revisions readable after a conflict.

**Acceptance:** offline/stale concurrent note edit produces a conflict copy, never silent overwrite or auto-merge.

## T076-08 — Approval request and decision

- [ ] Implement approval request create/get/list/withdraw with `assignee_participant_ids` and `blind_until_closed`.
- [ ] Implement decision recording supporting multiple decisions per request.
- [ ] Implement `blind_until_closed` read-time filtering.
- [ ] Structurally prove decision recording has no call path into promotion/amendment/action-intent creation.

**Acceptance:** dual independent decisions recorded and correctly filtered under blind mode; decision recording has zero observable effect outside 076 tables + `ActivityRecord`.

## T076-09 — ActivityRecord and activity feed

- [ ] Implement `ActivityRecord` append inside every mutation's transaction with hash-chain `checkpoint_digest`.
- [ ] Implement chain verification and bounded/filterable activity read.

**Acceptance:** chain verifies from `seq = 1`; a tampered fixture row is detected.

## T076-10 — Native Desktop and CLI parity

- [ ] Add Collaboration navigation through current native Slint composition, backed exclusively by Core.
- [ ] Implement at minimum a comments/review panel and a task/decision list.
- [ ] Preserve design system, keyboard/focus/accessibility, light/dark parity.
- [ ] Complete CLI vertical slice with human + JSON output.

**Acceptance:** CLI/Desktop tests prove real Core-backed collaboration data; no direct storage/network access from CLI/Desktop.

## T076-11 — Qualification and closure

- [ ] Run format, dependency-direction, focused contract/storage/Core/CLI/Desktop tests.
- [ ] Run Clippy under current policy, workspace tests, cargo-deny/supply-chain gates.
- [ ] Run migration/reopen/recovery, malformed/corrupt, cross-scope-leakage suites.
- [ ] Run credential/log secret and content-leakage scans.
- [ ] Capture rendered Desktop evidence or record the honest residual if the toolchain/CI cannot produce it.
- [ ] Perform exact-range review of the full PR diff.
- [ ] Run exact-head required CI; merge only when green and governance permits.
- [ ] Verify post-merge main CI; update queue/status to `CLOSED_CANONICAL` only with real post-main evidence.
- [ ] Recompute the next eligible unit; do not implement 077+ without separate promotion.

**Acceptance:** all frozen acceptance requirements mapped to exact-head proof.
