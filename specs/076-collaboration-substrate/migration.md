# Migration and Recovery — Spec 076

**Status:** implementation contract

Spec 076 extends the existing encrypted MedScale vault. It does not create a second database, replace current canonical object storage, or rewrite pre-076 object identities.

## 1. Default migration posture

The migration is additive:

```text
pre-076 encrypted vault (storage schema v4)
  + participant identity structures
  + room + room-membership structures
  + thread (anchor) structures
  + message + message-edit structures
  + task structures
  + note-document + note-revision structures
  + approval-request + approval-decision structures
  + activity-record (audit/activity) structures
```

Existing canonical objects (patients, FHIR, sources, documents, evidence, models, Packs, Projects, Experiments, graph edges, data sources, snapshots) remain exactly where their current owners store them. No existing object is automatically enrolled in a Room by migration. Old workflows remain valid; collaboration is opt-in organization layered on top.

`PresenceEvent` (section 13 of `contracts.md`) and `SyncCursor` (section 15) are **not** migrated or stored: presence is in-memory only and `SyncCursor` is a computed read over `ActivityRecord.seq`. Neither appears in the schema below.

## 2. Required schema families

```text
collab_participants
collab_rooms
collab_room_memberships
collab_threads
collab_messages
collab_message_edits
collab_tasks
collab_notes
collab_note_revisions
collab_approval_requests
collab_approval_decisions
collab_activity_records
```

Every durable mutable row needs:

- stable opaque identity (`OpaqueId`);
- realm/authority scope or an unambiguous existing path to it (via `room_id -> project_id -> realm/scope`, reusing Spec 074's Project scope rather than duplicating it on every row);
- schema version;
- monotonic mutation revision (`ProjectRevision`) for mutable rows, or `seq` for append-only rows;
- status/tombstone state where applicable;
- bounded metadata only — no bulk patient/document/model payload ever lives in a 076 table.

## 3. Required indexes

At minimum, indexes must efficiently support:

- room list by Project and active/archive state;
- membership list by room, and by participant (a participant's rooms);
- thread list by room, newest-first;
- message list by thread, in `seq` order; message-edit list by message, in `seq` order;
- task list by room and by assignee;
- note-revision list by note, in `revision` order, including conflict-copy siblings;
- approval-decision list by request, in `seq` order;
- activity-record list by room, in `seq` order (the audit/activity read path);
- exact participant lookup by `holder_id` for idempotent registration.

Do not add speculative indexes without a query/measurement justification.

## 4. Referential behavior

076 tables may use internal foreign keys between 076-owned rows when compatible with current storage design.

They must **not** use cascade behavior that can delete or mutate current canonical patient/FHIR/source/document/evidence/model/Pack/Project/data-source/snapshot data. `AnchorTarget.artifact` (an `ArtifactDescriptor`) is validated for shape at write time; its live resolution is always recomputed at read time through the existing Project-graph resolution path (`contracts.md` section 6), never cached as a foreign-key join that could go stale silently.

Room archive removes only the Room's own active-visibility flag; it never cascades into threads/messages/tasks/notes/approvals/activity, all of which remain inspectable.

## 5. Transaction boundaries

Required atomic units:

### Participant registration

One transaction: `collab_participants` row (+ `collab_activity_records` `ParticipantRegistered` append) commit together, or neither does.

### Room create / room membership add-remove

One transaction per operation contains the mutable row write, its revision bump, and the `ActivityRecord` append.

### Thread create / resolve / reopen

Anchor validation (shape only; live resolution is a read-time concern) + the `ThreadRef` write + `ActivityRecord` append commit atomically.

### Message post / edit / delete

`Message` insert (post) or `MessageEdit` insert (edit/delete) + `ActivityRecord` append commit atomically. The original `Message` row is never rewritten by an edit or delete.

### Task create / update

`Task` row + revision bump + `ActivityRecord` append commit atomically. Stale `expected_revision` writes nothing.

### Note create / edit / conflict-copy

`NoteRevision` insert + (fast-forward: `NoteDocument.revision` bump) or (conflict: no pointer bump) + `ActivityRecord` append commit atomically. A conflict copy is never partially visible: readers either see it as one of the note's revisions or not at all, never a state where the row exists but `conflict_of` is dangling.

### Approval request / decision / withdraw

`ApprovalRequest` write or `ApprovalDecision` insert + `ActivityRecord` append commit atomically. An `ApprovalDecision` never triggers a second transaction against any other subsystem's tables (section 2 of `contracts.md`); if it did, that would itself be the defect T1 in `security.md` is written to catch.

### Multi-operation UI workflow

UI convenience must not turn several logically independent commands into an undocumented giant transaction. If the product needs atomic multi-command behavior, Core must expose an explicit typed operation and tests.

## 6. Crash points

Tests must exercise or deterministically simulate at least:

1. before migration begins;
2. after migration metadata indicates start but before commit;
3. after migration commit;
4. before a participant/room/membership insert transaction commits;
5. before a message/message-edit insert commits;
6. before a task/note/approval mutation commits;
7. before an `ActivityRecord` append commits (proving a mutation without its audit row is never left visible — the two must be one transaction, not two);
8. after commit before client acknowledgement;
9. during backup/restore workflow used for rollback/recovery.

Required outcome: reopening the vault produces either the complete committed old state or the complete committed new state, never a mutation visible without its `ActivityRecord` entry, and never an `ActivityRecord` entry for a mutation that did not actually commit.

## 7. Pre-076 migration fixtures

Use synthetic/permitted fixtures representing at least:

- empty encrypted vault;
- populated vault with current patient/FHIR/source/evidence/document/model metadata;
- populated 074 vault with Projects/Experiments/refs/edges;
- populated 075 vault with data sources/snapshots/saved views;
- a fixture close enough to current production-shaped schema to catch index/table migration regressions;
- backup/restore fixture already supported by current repository.

Do not introduce real PHI fixtures.

## 8. Migration qualification sequence

For each representative fixture:

```text
1. open with pre-076-compatible base behavior
2. record stable object IDs/digests/authority facts required for comparison
3. create verified backup/recovery checkpoint
4. apply 076 migration once
5. inspect schema/version metadata (expect v5)
6. execute pre-076 regression operations (074 project ops, 075 data-source ops)
7. execute 076 room/thread/message/task/note/approval operations
8. close process cleanly
9. reopen
10. verify exact room/thread/message/task/note/approval/activity identities and
    seq/revision values, plus old-object identities/revisions/digests
11. attempt normal open/migration again; it must be safe according to migration framework
12. restore the pre-migration backup in a separate recovery path and prove old
    state remains usable and 076 tables are simply absent, not half-populated
```

## 9. Rollback rule

Default rollback is **restore from the verified pre-migration backup**. Do not implement a destructive SQL down-migration. If 076-created state cannot be represented by the old schema, restoring the pre-migration checkpoint discards post-checkpoint 076 changes; this must be stated honestly in recovery UX/evidence, exactly as 075 states it for its own migration.

## 10. Schema version ownership

T076-02 must extend the exact current storage schema/migration mechanism (live value: v4, established by Spec 075) with the smallest v4 -> v5 transition compatible with it, following the same `if journal.finished_version < 5 { begin_migration(5); execute_batch(crate::collaboration::V5_DDL); finish_migration(5); }` pattern already used for v3 -> v4 in `crates/medscale-storage/src/sqlite_meta.rs`.

Do not invent a parallel schema-version tracker. Unsupported future schema versions fail closed rather than being opened with partial interpretation.

## 11. Backup behavior

076-owned metadata is included in the existing vault backup/recovery mechanism. `ActivityRecord.checkpoint_digest` chains must still verify after a restore; a restore that produces a broken hash chain is a corruption signal, not a silently accepted state.

## 12. Archive and deletion

076 supports archive/tombstone/remove, not destructive erasure as a product feature.

- Room archive retains threads/messages/tasks/notes/approvals/activity for inspection/recovery.
- Membership removal is a status tombstone; past authorship is never reattributed or anonymized by removal.
- Message/note/task/approval mutation never edits an existing immutable append row; the append log is the audit trail.
- canonical target object deletion (Projects, artifacts, data sources, snapshots) remains owned entirely by its existing system; 076 never deletes or mutates a canonical object.

## 13. Concurrency

Preserve current writer-lock/transaction model. Spec 076 does not introduce multi-device concurrent writers or any network write path. Within the admitted local single-writer model, every mutation of existing mutable 076 state uses `expected_revision` precondition semantics — except `NoteDocument`, whose stale-write path produces a conflict copy rather than a bare rejection (`contracts.md` section 10). Stale request on every other mutable row -> `Conflict`, no last-write-wins.

## 14. Migration evidence

Closure evidence must record:

```text
base SHA
candidate SHA
storage schema version before/after (expect 4 -> 5)
fixture identity/digest
migration command/test
backup checkpoint evidence
pre-076 regression results (074 + 075 operations)
new 076 workflow results
close/reopen results
repeat-open/migration results
restore results
activity-record hash-chain verification results
known limitations
```

A migration test that only creates a fresh empty database is insufficient.

## 15. T076-02 freeze block

Before T076-02 is declared complete, update:

```text
MIGRATION_CONTRACT = FROZEN_FOR_076
CURRENT_STORAGE_VERSION = <live value, expect 4>
076_STORAGE_VERSION = <new value, expect 5>
MIGRATION_CODE_PATH = <exact path>
BACKUP_CODE_PATH = <exact path>
COLLAB_TABLES = <exact names>
INDEXES = <exact names/queries>
ROLLBACK_METHOD = <verified method>
```
