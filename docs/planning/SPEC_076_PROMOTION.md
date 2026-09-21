# Spec 076 Promotion — Collaboration Substrate

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`
**Founder promotion date:** 2026-09-21
**Canonical base:** `6021ff9aad397a8488087cae01e56528370e8211`
**Target branch:** `spec/076-collaboration-substrate`

## Authority

The founder directed live re-verification of Spec 075 closure and, if governance confirmed Spec 076 — Collaboration Substrate — remains the next dependency-ready Research OS candidate, canonical promotion followed by complete implementation through closure and continuation to the next eligible unit.

Live verification performed at promotion time:

- PR #129 (Spec 075 implementation) exact-head CI run `35580861670` on head `9f84a6e74805d2c326cec40ab4703cc3fff0e069`: 6/6 required jobs green; merged as `89a88cfbbe7b67582886fb07fb6de4fd9ca28dff`; post-merge main CI run `35582548200`: 6/6 green.
- PR #130 (Spec 075 closure bookkeeping) exact-head CI run `35585345326` on head `f0e3cd1f7b7dc9aaec0c6046a97e9517644db67b`: 6/6 green; merged as `6021ff9aad397a8488087cae01e56528370e8211`; post-merge main CI run `35587568653`: 6/6 green.
- All four run IDs, head SHAs, and merge SHAs verified live via `gh pr view`/`gh run view` against `TheHalfMoon/MedScale`, not assumed from prior conversation context.
- `docs/planning/BUILD_QUEUE.md` row 075 confirms `CLOSED_CANONICAL` with the same evidence and explicitly records the two honest residual gaps from Spec 075 (remote dataset live-fetch not exercised beyond fail-closed deny-before-socket; no rendered Desktop evidence exists because there is no CI rendering step / headless UI harness). Those residuals are carried forward as open, non-blocking observations; they do not gate Spec 076 because Spec 076 does not depend on remote dataset fetch or on the specific Desktop rendering gap, and they are not silently dropped from the record.

This document promotes **only Spec 076 — Collaboration Substrate** for implementation.

Predecessor closure proof: Spec 075 is `CLOSED_CANONICAL` (exact-head `9f84a6e…`, run `35580861670`, PR #129 merged as `89a88cf…`, post-merge main run `35582548200`; closure bookkeeping PR #130 merged as `6021ff9…`, post-merge main run `35587568653`; see `evidence/075-data-source-fabric/CLOSURE.md`).

Numbering proof: no `specs/076-*` package, no `SPEC_076_PROMOTION.md`, and no Spec 076 implementation code exists on the canonical base (verified via `git ls-tree -r --name-only origin/main | grep -i 076`, empty). `BUILD_QUEUE.md`'s `076+` row lists Collaboration Substrate among candidates requiring fresh promotion; `RESEARCH_OS_EXECUTION_ROADMAP.md`'s program dependency graph and `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md` both independently number Collaboration Substrate as **076** with a hard dependency on **074 only** (already `CLOSED_CANONICAL`), so no unclosed hard prerequisite blocks this promotion. `RESEARCH_OS_EXECUTION_ROADMAP.md`'s stated default ("prefer sequential promotion through 079") further supports promoting 076 immediately after 075 rather than skipping ahead.

Housekeeping note (non-blocking): `docs/planning/RESEARCH_OS_SPEC_IMPLEMENTATION_CONTRACTS.md` (the pre-Amendment-001 V1 document) numbers "Collaboration Substrate" as candidate "075" and "MedAgent Workbench" as "076" under an older scheme. This is superseded by Amendment 001 and by the V2 documents (`RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md`, `RESEARCH_OS_EXECUTION_ROADMAP.md`), which agree with the live `BUILD_QUEUE.md` numbering used for every real promotion since Spec 075. This promotion follows the V2/live numbering. The stale V1 document is not corrected by this promotion; it is flagged here so no future reader is misled by it.

Standing implementation authority in `IMPLEMENTATION_AUTHORITY.md` remains active. Normal branch/commit/push/PR/merge authority applies only within the promoted scope and only after real required gates pass.

## Authorized scope

Spec 076 may implement only the minimum governed local-first collaboration foundation described in `specs/076-collaboration-substrate/spec.md`:

- `ParticipantIdentity`/`ParticipantKind` (human/service/agent) and `AgentParticipantIdentity` as an opaque, unresolved forward reference toward the future Spec 077 agent identity;
- `Room`/`RoomMembership` scoped to an existing Spec 074 Project, with a collaboration-visibility filter that never substitutes for Core `Capability` authority;
- `AnchorTarget`/`AnchorDetail` reusing Spec 074's `ArtifactDescriptor`/`ArtifactVersionBinding`/`ReferenceResolution` for exact artifact-revision anchoring, resolved live at read time;
- `ThreadRef` with resolve/reopen status transitions;
- append-only `Message`/`MessageEdit`;
- revision-guarded `Task`;
- `NoteDocument`/`NoteRevision` with explicit conflict-copy semantics on stale concurrent edits (no CRDT, no silent merge, no silent overwrite);
- `ApprovalRequest`/`ApprovalDecision` supporting multiple independent decisions per request and an optional `blind_until_closed` read-time filter, with a structurally enforced guarantee that recording a decision never triggers a canonical/external effect;
- ephemeral, non-persisted `PresenceEvent`;
- tamper-evident, hash-chained `ActivityRecord` serving as both audit trail and local activity feed;
- an inert, storage-free `SyncCursor` concept (computed from `ActivityRecord.seq`) that reserves the naming/seq convention a future Hub sync will need, without implementing any sync execution or network path;
- CLI vertical slice and a native Slint Desktop comments/review panel and task/decision list, both backed exclusively by Core;
- encrypted local persistence and migration (storage schema v4 → v5, additive only);
- migration, recovery, compatibility, security, and exact-head evidence required for closure.

Staging rule: the first implementation lands contracts and storage/Core foundation (participant/room/membership, then thread/anchor resolution) before message/task/note/approval slices, and lands CLI before Desktop, mirroring the 075 staging discipline. The spec cannot close without the declared foundation set or an explicit canonical scope amendment recorded in the owning package.

## Explicitly not authorized

This promotion does **not** authorize implementation of:

- Spec 077 MedAgent Workbench (agent run execution, tool invocation, model Pack integration) — `AgentParticipantIdentity.agent_profile_ref` remains unresolved in 076;
- Spec 078 Model Fleet + Compare;
- Spec 079 Privacy Gate (beyond reusing current data classification);
- Spec 080 Governed Browse + Medical Literature Acquisition;
- Spec 081 AudioFlow Foundation + Local Scribe MVP;
- Spec 082 Analytics Gate + Cohort Builder;
- Spec 083 Clinical Graph + Knowledge/Research Canvas;
- Spec 084 MedScale Hub, or any actual network sync execution behind `SyncCursor`;
- Spec 085 MedScale Compute;
- Spec 086 R Workspace;
- Spec 087 Community Extensions;
- Specs 088–092 (AudioFlow Advanced, Research/Evidence Packs, Institutional Adapters, Federation, Whole-Platform Qualification);
- any mechanism by which an `ApprovalDecision` automatically performs a clinical, research-authority, or external effect (that remains owned by the existing, separately gated `PromoteProposal`/`TransitionEffect`/controlled-actions capabilities, invoked only by an explicit, separately authorized caller action);
- CRDT-based merge of canonical clinical/research facts;
- full systematic-review/dual-screening/adjudication/blinded-review *product* workflow — 076 proves the primitives (multiple decisions per request, `blind_until_closed`) support this without redesign; it does not build the product surface;
- wholesale adoption of Buzz's Tauri/React UI, relay protocol, or Nostr-as-canonical-storage; Buzz room/event/agent/workflow/media/audit *patterns* may be selectively studied for shape only, per `RESEARCH_OS_DECISIONS.md` D4 and `RESEARCH_OS_DONOR_RULE.md`, never imported as running code;
- real PHI;
- MESC work;
- new cloud/runtime network authority of any kind;
- a new ID, provenance, audit, or authority foundation when current MedScale primitives are sufficient.

## Mandatory architecture constraints

1. Reuse current MedScale `OpaqueId`, `ObjectHeader`, `DigestSha256`, realm/scope, `ProjectRevision`/`check_revision`/`initial_revision`, `ArtifactDescriptor`/`ArtifactVersionBinding`/`ReferenceResolution`, digest/provenance, audit, vault, migration, and Core patterns where applicable.
2. Existing patient/FHIR/source/document/evidence/model/Pack/Project/data-source/snapshot objects remain canonical in their owning systems. Collaboration references them by stable identity/version binding; it never copies or re-derives their content.
3. `RoomMembership` governs collaboration-scope visibility only. It never grants, widens, or substitutes for a Core `Capability` grant, and a Core `Capability` grant never substitutes for `RoomMembership`. Both checks are independently required.
4. `ApprovalDecision` recording is structurally isolated from every effect/action/proposal-promotion path. This is proven by review and by a dedicated test, not asserted by comment.
5. Anchor resolution (`ReferenceResolution`) is always recomputed live at read time from the current artifact state. It is never cached or trusted as a write-time fact.
6. `NoteDocument` is the one and only place Spec 076 departs from plain reject-on-stale-revision, and it does so via an explicit, typed conflict-copy result — never a CRDT merge, never a silent overwrite.
7. CLI and Desktop must use the same Core command/query semantics. No direct storage/data access from UI.
8. Local/offline operation is mandatory: with network egress disabled, every 076 workflow remains fully useful. Spec 076 opens no network connection and requires none.
9. Migration must preserve all pre-076 workflows and object identities (no existing object ID rewrite).
10. Unknown/unavailable/stale/partial/conflicting/denied/corrupt states stay distinct; missing input is not zero.
11. No later Research OS unit may be started merely because its planning document exists.

## Implementation order

```text
T076-00 Live truth and baseline (no material product mutation before it closes)
T076-01 Contracts freeze + invariant tests
T076-02 Storage schema v5, migration, crash/reopen/recovery tests
T076-03 Participant + Room + Membership vertical slice
T076-04 Thread + live anchor resolution
T076-05 Message + MessageEdit (append-only)
T076-06 Task (revision-guarded)
T076-07 Note + explicit conflict-copy
T076-08 Approval request + decision (dual/blind-capable, structurally no-effect)
T076-09 ActivityRecord hash chain + activity feed
T076-10 Native Desktop comments/review panel + task/decision list + CLI parity
T076-11 Exact-head qualification, review, and closure
```

A later slice may not paper over a failed earlier invariant.

## Frozen acceptance requirements

Spec 076 can close only when all of the following are proven on the exact reviewed head:

1. A Room can be created in a Project, participants (including one agent) registered, membership managed, and this survives close/reopen.
2. A thread anchored to an exact artifact revision reports `Current`, then `Stale`/`Missing` after the artifact changes, purely from a live read-time recompute.
3. Messages are append-only; edit and delete never rewrite the original row; full history remains queryable.
4. A stale task update returns `Conflict` and writes nothing.
5. A stale concurrent note edit produces an explicit conflict copy preserving both revisions; nothing is silently overwritten or auto-merged.
6. Two independent approval decisions can be recorded against one request; `blind_until_closed` correctly hides co-assignee decisions until the request closes.
7. Recording an `ApprovalDecision` has zero observable effect outside 076 tables and its `ActivityRecord` entry — proven structurally, not merely by absence of a test failure.
8. `RoomMembership` visibility is independently enforced from Core `Capability` authority in both directions.
9. The `ActivityRecord` hash chain verifies from the beginning and detects a tampered fixture row.
10. Pre-076 vaults (including populated 074 and 075 vaults) migrate (v4 → v5), reopen, and recover from backup, including hash-chain re-verification.
11. No alternate authority/storage/network path exists (dependency-direction gate holds).
12. Every 076 workflow remains usable with network disabled.
13. No real-PHI authorization is implied; fixtures are synthetic/permitted non-PHI.
14. Exact-head required CI and post-main verification pass.

## Evidence paths

All qualification evidence lives under `evidence/076-collaboration-substrate/` (see `specs/076-collaboration-substrate/plan.md` for the required evidence set, mirroring the Spec 074/075 evidence structure).

## Completion rule

Spec 076 becomes `CLOSED_CANONICAL` only when all acceptance criteria in the owning Spec 076 package are proven on the exact reviewed head, required CI is green, the PR is merged normally, and post-merge main verification is recorded.

Closure of 076 does not itself authorize 077. After closure, live governance must recompute the next eligible unit and explicitly promote it.
