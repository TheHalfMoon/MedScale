# Plan — Spec 076 Collaboration Substrate

## Execution rule

Implement only the promoted Spec 076 scope. Follow `docs/planning/SPEC_076_PROMOTION.md`, the Research OS V2 master contracts/repository map/decision register/verification matrix, and this Spec 076 package. Live repository truth wins over stale assumptions. Note: `docs/planning/RESEARCH_OS_SPEC_IMPLEMENTATION_CONTRACTS.md` (non-V2) numbers Collaboration Substrate as candidate "075" under an earlier, superseded scheme — `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md` and `BUILD_QUEUE.md` are authoritative and agree on 076; do not follow the stale V1 numbering.

## Phase 0 — Reverify and freeze

Before material code changes:

1. Fetch/update local `main` and verify it contains merge `6021ff9aad397a8488087cae01e56528370e8211` or a proven later canonical descendant.
2. Confirm branch is `spec/076-collaboration-substrate` and no unexpected local changes exist.
3. Re-read `AGENTS.md`, `CURSOR.md`, `IMPLEMENTATION_AUTHORITY.md`, `SPEC_076_PROMOTION.md`, `START_HERE.md`, `BUILD_QUEUE.md`, the Research OS V2 execution roadmap/spec-implementation-contracts/repository-map addendum, and this Spec 076 package.
4. Verify no Spec 076 collision or newer founder authority supersedes this promotion.
5. Inventory exact live types/modules for `OpaqueId`, `ObjectHeader`, `DigestSha256`, `ProjectRevision`/`check_revision`/`initial_revision`, `ArtifactDescriptor`/`ArtifactVersionBinding`/`ReferenceResolution`, `TextSpan`, `AuthorityError`, `SessionRegistry`, storage migration (current version 4), backup/recovery, writer locking, CLI routing and Desktop composition.
6. Record baseline required CI/test state. Pre-existing failures are evidence, not permission to suppress tests.
7. Freeze the field-level contract/data model in `contracts.md` before 076-B.

## 076-A — Contracts and invariants (T076-01)

### Work

- Add the contract module(s) required by Spec 076 (`medscale-contracts/src/collaboration.rs` or the repository-conventional shape).
- Reuse `OpaqueId`, `ObjectHeader`, `DigestSha256`, `ProjectRevision`, `ArtifactDescriptor`/`ArtifactVersionBinding`/`ReferenceResolution`, `TextSpan`.
- Define `ParticipantIdentity`/`ParticipantKind`/`AgentParticipantIdentity`, `Room`/`RoomMembership`, `AnchorTarget`/`AnchorDetail`, `ThreadRef`, `Message`/`MessageEdit`, `Task`, `NoteDocument`/`NoteRevision`, `ApprovalRequest`/`ApprovalDecision`, `PresenceEvent`, `ActivityRecord`, `CollabEventKind`.
- Define strict validation and explicit revision/precondition behavior for mutable rows; define the `NoteDocument` conflict-copy result type distinctly from a plain `Conflict` error.
- Reuse `AuthorityError` as-is; add no new variant unless T076-01 proves a genuine gap, recorded in `contracts.md` section 16.

### Gate

Focused contract tests pass, dependency direction remains valid, no storage/UI/network dependency leaks into contracts, and the freeze record in `contracts.md` section 18 is filled in with exact Rust paths.

## 076-B — Storage and migration (T076-02)

### Work

- Extend the current encrypted metadata/vault migration mechanism (v4 -> v5); do not create a second DB.
- Add bounded tables/indexes for participants, rooms, memberships, threads, messages, message edits, tasks, notes, note revisions, approval requests, approval decisions, activity records (`migration.md` section 2).
- Implement transactional repository/storage functions required by Core; storage does not decide authority.
- Add migration from representative pre-076 vaults (including populated 074 and 075 vaults).
- Add reopen, crash-boundary and backup/restore recovery tests, including activity hash-chain re-verification after restore.
- Preserve all existing IDs and pre-076 behavior.

### Gate

Migration is repeat-safe under current framework, crash semantics are unambiguous (mutation and its `ActivityRecord` commit together or not at all), old workflows still pass, and no cascade can delete referenced canonical objects.

## 076-C — Participant + Room + Membership (T076-03)

### Work

- Implement participant registration (idempotent on `(room_id, holder_id)` or the repository-conventional equivalent), room create/get/list/update/archive, membership add/update/remove.
- Wire the `RoomMembership` visibility filter into every 076 read path from the start, so later slices inherit it rather than retrofitting it.
- CLI inspect for room/participant/membership.

### Gate

Room + participant + membership vertical slice proven end to end with reopen durability and cross-room visibility denial (`security.md` T3).

## 076-D — Thread and anchor resolution (T076-04)

### Work

- Implement thread create/get/list/resolve/reopen.
- Implement live `ReferenceResolution` recomputation against `ArtifactDescriptor` targets, reusing the Spec 074 resolution path; no cached resolution field is ever trusted without recompute.

### Gate

Thread anchored to an artifact reports `Current`; mutating/removing the artifact flips resolution to `Stale`/`Missing` on next read without any 076 write (`security.md` T7).

## 076-E — Message and MessageEdit (T076-05)

### Work

- Implement message post/list; message edit (body replace) and delete as append-only `MessageEdit` rows.
- Implement current-state derivation (latest edit/delete fold) without ever rewriting the original row.

### Gate

Post/edit/delete proven end to end; full history remains queryable; current-state view matches the latest edit.

## 076-F — Task (T076-06)

### Work

- Implement task create/get/list/update with `expected_revision` precondition and optional anchor.

### Gate

Stale task update -> `Conflict`, no write; anchored task's resolution behaves like a thread's (076-D).

## 076-G — Note and conflict-copy (T076-07)

### Work

- Implement note create/get/list, fast-forward edit, and the explicit conflict-copy path on stale `expected_revision` (`contracts.md` section 10).
- Prove both revisions are readable after a conflict, and that resolving a conflict is a separate, explicit later mutation.

### Gate

Offline/stale concurrent note edit produces a conflict copy, never silent overwrite and never silent auto-merge.

## 076-H — Approval request and decision (T076-08)

### Work

- Implement approval request create/get/list/withdraw, with `assignee_participant_ids` and `blind_until_closed`.
- Implement decision recording, allowing multiple decisions per request (dual/independent review).
- Implement `blind_until_closed` read-time filtering.
- Structurally prove `ApprovalDecision` recording has no call path into `authority::promote`, `authority::amend`, or `contracts::actions` (`security.md` T1).

### Gate

Dual independent decisions on one request are both recorded and correctly filtered under blind mode; decision recording has zero observable effect outside 076 tables + `ActivityRecord`.

## 076-I — ActivityRecord and activity feed (T076-09)

### Work

- Implement the `ActivityRecord` append (inside every mutation's transaction, per `migration.md` section 5) and the hash-chain `checkpoint_digest`.
- Implement chain verification and a bounded, filterable activity read (by room, actor, event kind, target).

### Gate

Chain verifies from `seq = 1`; a tampered fixture row is detected; the activity feed is the same data the audit trail is built from (no duplicate log).

## 076-J — Native Desktop and CLI parity (T076-10)

### Work

- Add Collaboration navigation through current native Slint composition, backed exclusively by Core: at minimum a comments/review panel (rooms/threads/messages/approvals) and a task/decision list.
- Preserve current design system, keyboard/focus/accessibility and light/dark parity conventions.
- Complete CLI vertical slice with human + JSON output matching Spec 074/075 conventions.

### Gate

CLI/Desktop tests prove real Core-backed collaboration data (no fake product data); no direct storage/network access from CLI/Desktop.

## 076-K — Qualification, review and closure (T076-11)

### Work

1. Run format, dependency-direction, focused contract/storage/Core/CLI/Desktop tests.
2. Run Clippy under current policy, full workspace tests, cargo-deny/supply-chain gates.
3. Run migration/reopen/recovery suite, including pre-076 fixtures and backup/restore, and activity hash-chain re-verification.
4. Run malformed/hostile-input, cross-scope-leakage, and structural no-effect (T1) suites.
5. Run credential/log secret and content-leakage scans (`security.md` T11).
6. Capture rendered Desktop evidence where the CI/toolchain allows it; if not, record the same honest residual pattern Spec 075 recorded rather than fabricating a render.
7. Perform exact-range review of the full PR diff.
8. Run exact-head required CI.
9. Create/update `evidence/076-collaboration-substrate/` with exact commands, platform, SHAs, fixtures and results.
10. Open/update the Spec 076 PR with real evidence.
11. Merge only after exact-head required gates pass and governance allows it.
12. Verify post-merge `main` CI.
13. Update canonical status/queue to `CLOSED_CANONICAL` only after the post-main evidence exists.
14. Recompute the next eligible unit; do not implement 077+ without separate promotion.

### Required evidence set under `evidence/076-collaboration-substrate/`

```text
README.md (index + per-file rules)
LIVE_TRUTH.md (T076-00 branch/base/main/PR/CI state)
CONTRACT_QUALIFICATION.md (frozen fields to Rust paths + tests)
STORAGE_MIGRATION_RECOVERY.md (fixtures, before/after, backup/migrate/reopen/restore)
CORE_AUTHORITY_QUALIFICATION.md (mutation/query matrix + denial/conflict/scope)
CLI_QUALIFICATION.md (human + JSON, exits, Core parity)
DESKTOP_QUALIFICATION.md (native state/adapter/a11y + renders or honest residual)
SECURITY_ADVERSARIAL.md (threat-gate results from security.md, including the T1 structural no-effect proof)
NO_NETWORK_LOCAL_PATH.md (egress-disabled proof)
ACTIVITY_CHAIN_VERIFICATION.md (hash-chain construction + tamper-detection proof)
EXACT_RANGE_REVIEW.md (authorized scope only, unexpected files, new dependencies)
EXACT_HEAD_QUALIFICATION.md (candidate head + live CI)
POST_MERGE_VERIFICATION.md (merge SHA + main checks)
CLOSURE.md (terminal truth block)
logs/ (CLI vertical-slice log, any UI transcript)
```

## Parallelism

Default is sequential T076-00 -> T076-11 slices because contracts/storage/Core are shared boundaries. 076-D through 076-I touch mostly disjoint entity families once 076-A/B/C land, so focused test/evidence writing for those slices may proceed in parallel once the shared contract/storage/membership-filter foundation (076-A/B/C) is merged into the working branch — but no slice may widen a frozen contract without amending `contracts.md` first.

## Stop conditions

Stop the current mutation, record evidence, and resolve before continuing if:

- live main contradicts the promoted base in a material way;
- another actor has already claimed Spec 076 with conflicting work;
- a required migration would rewrite existing canonical object IDs;
- an implementation requires a new authority/ID/provenance system;
- the proposed change requires scope from Spec 077+ (agent execution, model runtime, Hub sync, privacy-gate transforms);
- a test exposes an `ApprovalDecision` reaching a canonical/external effect, a cross-room/cross-project leak, an authority bypass, or secret/content leakage;
- required CI fails for the current change;
- real PHI or gated credentials/terms would be required.

Do not stop for ordinary implementation decisions already resolved by canonical planning; use the decision register, tests and safest minimal design.
