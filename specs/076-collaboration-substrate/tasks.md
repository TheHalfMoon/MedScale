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

- [x] Implement participant registration (idempotent on scope+holder_id, kind immutable), revoke, room create/get/list/rename/archive, membership add/list/remove.
- [x] Wire `RoomMembership` visibility filter into every read path (`require_membership` gates room get/rename/archive/membership add-list-remove; `list_rooms` intersects with caller's active memberships).
- [x] Discovered and closed a real atomicity gap during this slice: the primary row write and its `ActivityRecord` append were two separate transactions. Added `append_activity_in_tx` and rebuilt `insert_room_with_activity`/`archive_room_with_activity`/`insert_membership_with_activity`/`remove_membership_with_activity` so both commit together (`migration.md` section 5, `security.md` T12) -- found and fixed before any test caught it, same discipline as the T076-02 backup/restore gap.
- [x] Wired 9 new `Capability` variants, 11 new `RequestBody` variants, 5 new `ResponseBody` variants into `envelopes/mod.rs`, plus the `collab()` facade helper and dispatch arms in `facade.rs` (mirrors the `pg`/`ds` pattern exactly).
- [ ] CLI inspect for room/participant/membership. **Deferred to T076-10** (CLI/Desktop slice), matching the plan.md staging order (Core vertical slices land before the CLI/Desktop pass).

**Acceptance:** Core authority layer complete and wired for participant/room/membership. End-to-end reopen-durability and cross-room-visibility-denial *tests* land in T076-11 qualification, matching how the storage-layer plumbing/testing split was organized in T076-02.

## T076-04 — Thread and anchor resolution

- [x] Implement thread create/get/list/resolve/reopen (`open_thread`, `get_thread`, `list_threads`, `set_thread_status` -- allows `Open->Resolved`, `Resolved->Reopened`, `Reopened->Resolved`; any other transition is `Conflict`).
- [x] Implement live `ReferenceResolution` recomputation against `ArtifactDescriptor` targets, attached to every thread read (`resolve_anchor_artifact`, mirroring `project_graph::ProjectGraph::resolve_descriptor` field-for-field against the same `packs`/`store`-backed canonical owners; deliberately not shared code since that would touch Spec 074's closed file). Coverage matches `ArtifactKind`'s current frozen vocabulary exactly: `PackManifest` and the H0-era `StoredObject` kinds resolve properly; kinds `ArtifactKind` does not yet model (notably Spec 075 data-source/snapshot objects, which have no `ArtifactKind` variant at all today) resolve as `UnsupportedKind` -- an honest fail-closed answer, not a fabricated `Current`. Recorded as a residual in `CLOSURE.md` at T076-11, not hidden.

**Acceptance:** thread CRUD + status-transition + live resolution vertical slice complete and CI-green (`open_thread`/`get_thread`/`set_thread_status` return `(ThreadRef, ReferenceResolution)`; `list_threads` returns the same pairing per thread). Frozen acceptance requirement 2 (`SPEC_076_PROMOTION.md`) is met for the artifact kinds `ArtifactKind` currently models.

## T076-05 — Message and MessageEdit

- [x] Implement message post/list (`post_message`, `list_messages`; only the room's active members may post).
- [x] Implement message edit/delete as append-only `MessageEdit` rows with current-state derivation left to the reader (edit log is exposed via `list_message_edits`/`insert_message_edit_with_activity`; only the original author may edit or delete their own message).

**Acceptance:** post/edit/delete vertical slice complete and CI-green; full history queryable via the edit log. End-to-end proof tests land in T076-11.

## T076-06 — Task

- [x] Implement task create/get/list/update with `expected_revision` precondition and optional anchor.

**Acceptance:** stale task update -> `Conflict` (enforced by `update_task_with_activity`'s CAS); vertical slice complete and CI-green.

## T076-07 — Note and conflict-copy

- [x] Implement note create/get/list-revisions, fast-forward edit, and explicit conflict-copy path on stale write (`edit_note` resolves fast-forward vs. conflict-copy via `NoteDocument::is_fast_forward`; the "current" fast-forward-chain row is looked up unambiguously as the unique `conflict_of IS NULL` row at the current revision number, since conflict copies always carry `conflict_of = Some(..)` even when they reuse an old revision number).
- [x] Prove both revisions readable after a conflict: `edit_note` returns `(current, conflict_copy, true)` without touching `current`; storage never deletes or overwrites either row.

**Acceptance:** offline/stale concurrent note edit produces a conflict copy, never silent overwrite or auto-merge. Core vertical slice complete and CI-green; dedicated test fixtures land in T076-11.

## T076-08 — Approval request and decision

- [x] Implement approval request create/get/withdraw with `assignee_participant_ids` and `blind_until_closed` (only the original requester may withdraw; only `Open -> Withdrawn` is implemented -- `Closed` has no dedicated `CollabEventKind` in the frozen vocabulary and its transition trigger is a deferred product decision, not silently claimed done).
- [x] Implement decision recording supporting multiple decisions per request (`decide_approval_request`; caller must be an assignee and the request must be `Open`).
- [x] Implement `blind_until_closed` read-time filtering (`list_approval_decisions` filters via `ApprovalRequest::hides_decision_from` before returning any row).
- [x] Structurally prove decision recording has no call path into promotion/amendment/action-intent creation: `decide_approval_request`'s only writes are `insert_approval_decision_with_activity` (collab tables + `ActivityRecord`) -- no call into `authority::promote`, `authority::amend`, or `contracts::actions` exists anywhere in `collaboration.rs`, verified by inspection; a structural regression test lands in T076-11's `SECURITY_ADVERSARIAL.md` per `security.md` T1.

**Acceptance:** Core vertical slice complete and CI-green. Dual-independent-decision and blind-mode end-to-end tests land in T076-11.

## T076-09 — ActivityRecord and activity feed

- [x] Implement `ActivityRecord` append inside every mutation's transaction with hash-chain `checkpoint_digest` (`append_activity_in_tx`, used by every T076-03 through T076-08 mutation's combined `*_with_activity` storage function).
- [x] Implement chain verification (`verify_activity_chain`, already added in T076-02) and bounded/filterable activity read (`list_activity`, membership-gated).

**Acceptance:** chain construction/verification plumbing complete and CI-green (unit-tested in `collaboration.rs`'s contract-level `activity_chain_detects_tampered_row` test since T076-01). End-to-end tamper-detection-on-real-storage test lands in T076-11.

## T076-10 — Native Desktop and CLI parity

- [x] Add Collaboration navigation through current native Slint composition, backed exclusively by Core: new "Collaboration" `NavItem`/route in `app.slint`, reusing the existing `desktop-projects` `CliSession` (Rooms are Project-scoped, so no second vault/session).
- [x] Implement at minimum a comments/review panel (Rooms -> Threads with live resolution -> Messages) and a task list (create + mark-done).
- [x] Preserve design system, keyboard/focus/accessibility, light/dark parity: every new element reuses existing themed components (`HonestyPanel`, `AdaptiveLineEdit`, `ToolbarAction`, `QuickAction`, `StatusPill`, `Theme.*` tokens) and `accessible-role`/`accessible-label` conventions exactly as the Projects/Data pages already establish; no new component or color was invented.
- [x] Complete CLI vertical slice with human + JSON output: `crates/medscale-cli/src/collaboration.rs` (`medscale collab ...` for participant/room/membership, `medscale collab-work ...` for thread/message/task/note/approval/activity), backed by ~30 new `CliSession::collab_*` convenience methods in `crates/medscale-core/src/cli_session.rs`. Anchor input is deliberately simplified to `IdentityOnly` bindings for CLI ergonomics (a CLI scope simplification, not a Core limitation -- Core's full `ArtifactVersionBinding` precision is exercised by the Core-layer tests instead).

Desktop scope note: the panel covers Rooms/Threads/Messages/Tasks (the plan.md minimum). Notes and Approvals have no Desktop surface in this closure -- CLI-only for those two, same honest-gap discipline as Spec 075's saved-views closure (only the Grid view was built; other view kinds were recorded as a gap, not silently claimed).

**Acceptance:** CLI vertical slice complete and CI-green. Desktop panel implemented and genuinely Core-backed (`collaboration_workspace.rs` calls real `CliSession::collab_*` methods, no synthetic data); this workstation cannot link locally to produce a rendered screenshot, so rendered Desktop evidence is deferred to the same honest-residual treatment Spec 075 recorded (`evidence/.../DESKTOP_QUALIFICATION.md` at T076-11), not fabricated.

## T076-11 — Qualification and closure

- [x] Desktop panel exact-head CI proof: commit `ebec119` (T076-10 Desktop half)
      ran on CI run `35624477017` and finished green on all 6 required jobs,
      including `rust (windows-latest)` (the native Slint compile target this
      workstation cannot exercise locally) -- the highest-risk commit of the
      spec compiled cleanly with no follow-up fix needed.
- [x] Added `crates/medscale-core/tests/collaboration_076.rs`: functional
      lifecycle coverage for every entity family (participant, room,
      membership, thread + live resolution, message + author-only edit,
      task + stale-conflict, note fast-forward/conflict-copy, approval
      request + blind-mode decisions + requester-only withdraw, activity
      feed ordering) plus dedicated tests for `security.md` T1 (structural
      no-call-into-promotion/amend/actions check), T2 (participant kind
      immutability), T3 (cross-room/cross-project visibility denial), T4
      (removed membership fails closed on the very next request), and T9
      (bound enforcement + hostile/SQL-like/control-character text stored
      byte-exact through parameterized storage).
- [x] Added `crates/medscale-storage/tests/collaboration_076.rs`: v4->v5
      migration preserving a pre-076 (074-era) `Project` row alongside new
      076 rows with a safe repeat-open; backup/restore roundtrip of
      participant/room/membership/task rows with post-restore activity
      hash-chain re-verification (`migration.md` section 11); `security.md`
      T10 (direct-DB tamper on a stored `ActivityRecord` row fails
      `verify_activity_chain` closed); `security.md` T12 (forcing the
      activity-append half of `insert_room_with_activity` to fail proves the
      primary row never commits without it, via a pre-occupied `(room_id,
      seq)` unique-index collision).
- [x] Ran format, dependency-direction, full workspace tests, Clippy under
      current policy, cargo-deny/supply-chain gates: exact-head CI run
      `35630430836` (head `bf9907c`) green 6/6 on every required job.
- [x] Ran credential/log secret and content-leakage scan (T11): inspection
      confirms zero `log::`/`tracing::` calls anywhere in the 076 code paths
      (`authority/collaboration.rs`, `storage/src/collaboration.rs`,
      `cli/src/collaboration.rs`, `desktop/src/collaboration_workspace.rs`,
      `facade.rs`, `cli_session.rs`); the only body-printing calls are the
      CLI's own explicit human-output printer, the same established pattern
      `data_source.rs` already uses. Recorded in `SECURITY_ADVERSARIAL.md`.
- [x] Captured Desktop evidence honestly: no rendered PNG exists (no local
      or CI rendering path, same residual Spec 075 recorded), but this
      qualification's own new in-module test
      (`collab_workspace_flows_through_real_core_session`) **found and
      fixed a real functional bug** before merge -- `ensure_self_participant`
      registered the operator under the hardcoded string `"desktop-operator"`
      instead of the session's actual bound holder id, so every real
      "Create Room" click would have failed closed with `Unauthorized`.
      Fixed by adding `CliSession::holder_id()` and registering under it.
      See `DESKTOP_QUALIFICATION.md` for the full failure-then-fix record
      (not hidden or silently corrected).
- [ ] Perform exact-range review of the full PR diff using OpenCodeReview
      (https://github.com/alibaba/open-code-review) exclusively, per explicit
      instruction -- no other review tool/method.
- [ ] Run exact-head required CI on the final reviewed head; merge only when
      green and governance permits.
- [ ] Verify post-merge main CI; update queue/status to `CLOSED_CANONICAL`
      only with real post-main evidence.
- [ ] Recompute the next eligible unit; do not implement 077+ without
      separate promotion.

**Acceptance:** all frozen acceptance requirements mapped to exact-head proof.
