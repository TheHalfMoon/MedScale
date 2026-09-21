# Tasks — Spec 076 Collaboration Substrate

**Execution state:** `PROMOTED_IMPLEMENTATION_AUTHORIZED` (not yet implemented)

Check a task only when its implementation, tests and required evidence are real on the branch. Do not pre-check future work.

## T076-00 — Live truth and baseline

- [ ] Verify branch/base/main/PR state and repository cleanliness.
- [ ] Read mandatory governance + Research OS V2 + Spec 076 authority chain.
- [ ] Inspect current contracts/Core/storage/network/key/CLI/Desktop ownership paths.
- [ ] Confirm no live Spec 076 collision/superseding authority; prove numbering is free.
- [ ] Verify Spec 075 closure and post-main evidence.
- [ ] Record baseline focused/full gate state and any pre-existing failures.
- [ ] Create initial evidence/live-truth record.

**Gate:** no code mutation before T076-00 is complete.

## T076-01 — Freeze contracts/data model

- [ ] Freeze exact field-level participant/room/membership/thread/message/task/note/approval/activity contracts in `contracts.md`.
- [ ] Reuse `OpaqueId`, `ObjectHeader`, `DigestSha256`, `ProjectRevision`, `ArtifactDescriptor`/`ArtifactVersionBinding`/`ReferenceResolution`, `TextSpan` where semantically valid.
- [ ] Freeze `ParticipantKind`/`CollabEventKind`/`AnchorDetail` vocabulary; reject unsafe unknown authority kinds.
- [ ] Freeze revision/precondition and idempotency behavior for mutable rows, including the `NoteDocument` conflict-copy exception.
- [ ] Add serialization/validation/invariant tests.

**Acceptance:** contract tests pass; no storage/network/UI dependency enters contracts.

## T076-02 — Storage schema + migration

- [ ] Add collaboration storage using current encrypted vault/SQLite metadata architecture (v4 -> v5).
- [ ] Add required indexes (`migration.md` section 3).
- [ ] Implement atomic transactions (mutation + `ActivityRecord` append together) and stale-revision conflict behavior.
- [ ] Add forward migration from representative pre-076 vaults (including populated 074 and 075 vaults).
- [ ] Add repeated-open/repeated-migration safety tests.
- [ ] Add crash/reopen and backup/restore recovery tests, including activity hash-chain re-verification after restore.
- [ ] Prove no collaboration operation cascades deletion into canonical target objects.

**Acceptance:** storage + migration suite green and pre-076 IDs/behavior preserved.

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
