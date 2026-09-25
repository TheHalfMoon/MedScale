# Spec 084 Promotion — MedScale Hub Foundation

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`
**Promotion date:** 2026-09-24
**Canonical base:** `426bb3448d4e6e4b9c641fa256d6d2af530c2820` (Spec 083 closure PR #148)
**Target branch:** `spec/084-hub`

## Authority

The founder's standing continuation directive requires promoting the next
dependency-ready Research OS unit after each closure without routine
approval. `IMPLEMENTATION_AUTHORITY.md` remains active
(`DEPENDENCY_INSTALL_AUTHORITY = YES_IF_OWNING_SPEC_ADMITS_AND_LOCKS_IT`;
this spec admits nothing).

Live verification at promotion time:

- Spec 083 is `CLOSED_CANONICAL`: final head `194840c` passed exact-head
  run `36004914365` (6/6); PR #145 merged as `2892860`; post-merge main run
  `36014350193` passed 6/6. Closure PR #148 (exact-head run `36022980908`,
  6/6 on `f32a55d`) merged as `426bb34`; its post-main run is `36033406508`,
  verified before this spec's implementation PR merges (recorded in
  `evidence/084-hub/POST_MERGE_VERIFICATION.md`).
- Specs 074, 076 and 079 are `CLOSED_CANONICAL` (see `BUILD_QUEUE.md`).

Dependency proof: `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md`,
`RESEARCH_OS_EXECUTION_ROADMAP.md` and
`RESEARCH_OS_V2_IMPLEMENTATION_READY_MASTER_PLAN.md` number MedScale Hub
**084**, hard dependency **074 + 076 + 079**, all closed. No other open unit
is dependency-ready ahead of it: 085 (Compute, 077 + 079) is next in numeric
order and does not depend on 084.

Decisions that apply (`RESEARCH_OS_DECISION_RESOLUTION_REGISTER.md`):

- Q04: authenticated actor identity plus tamper-evident audit checkpoints;
  per-event end-user signatures are not mandatory. This foundation signs
  each submission with the device key because it is cheap and already
  admitted, not because Q04 requires it.
- Q06: transport encryption and project authorization; no end-to-end
  encryption of collaboration bodies.
- Q07: messages append; tasks use optimistic concurrency; notes keep
  conflict copies and need user resolution; canonical metadata is never
  last-writer-wins; no CRDT.
- Q08: MedScale-owned relationship semantics; no external policy engine.
- V2-Q29: no mandatory MedScale cloud.

The roadmap requires "transport encryption outside loopback/dev". This
foundation has no transport outside the local machine, so no TLS stack is
needed or admitted.

Review policy: `FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## What "Hub" means in this foundation

A Hub is a MedScale vault whose Core accepts signed submissions from
enrolled devices of other vaults and applies them through its own Spec 076
collaboration authority. Core stays the only authority on both sides:

- the Hub never writes a collaboration row except through the existing
  076 authority path, running as the submitting device's participant, with
  the existing membership, scope and revision checks and the existing
  hash-chained activity trail;
- a client never writes Hub state; it keeps an outbox of its own pending
  submissions and a read-only mirror of events the Hub accepted;
- the mirror is a projection, not a second collaboration store.

Transport is a transport-neutral request/response contract. This spec
serves it in-process and over the existing Spec 024 local-socket IPC
(`interprocess`, already admitted). A network transport is a later slice
with its own dependency, TLS and threat admission.

## Authorized scope

- Contracts (`medscale-contracts/src/hub.rs`): `HubIdentity`,
  `DeviceIdentity`, `HubInvitation`, `HubHandshake` (versioned challenge
  and signed response), `TenantScope`, `ProjectMembership`, `SyncEnvelope`
  (one signed collaboration intent), `SyncCursor`, `SyncOutcome`,
  `SyncConflict`, `HubEvent`, `RevocationEvent`, `HubAuditCheckpoint`.
- Device identity: an ed25519 key pair per enrolled device (`medscale-keys`,
  `ed25519-dalek` already admitted), secret stored in the client vault's
  metadata store and never sent; the Hub stores only the public key.
- Enrollment by invitation: the Hub operator issues a one-time invitation
  bound to one Project and one participant display name; the Hub stores
  only its digest; redemption registers the device and its Hub participant;
  a second redemption fails and the operator can revoke an unredeemed
  invitation. Core keeps no wall clock, so invitations do not expire by
  time.
- Handshake: protocol version check, Hub-issued single-use nonce, device
  signature over `(hub id, device id, nonce, protocol version)`.
- Sync of Spec 076 collaboration intents for Rooms of the enrolled Project
  in which the device's participant is a member: post message, create task,
  update task (expected revision), create note, edit note (expected
  revision). Each envelope carries a
  device-scoped monotonic sequence and a signature over its canonical bytes.
- Idempotency and replay: `(device, sequence)` is unique; a resubmitted
  envelope with identical bytes returns the stored outcome; different bytes
  under a used sequence, a sequence gap, a bad signature or an unknown or
  revoked device are refused and recorded.
- Conflicts (Q07): messages append; a task update at a stale revision
  returns `SyncConflict` with the current revision and is not applied; a
  note edit at a stale revision is resolved by the existing 076 conflict
  copy path and reported.
- Ordered events: every accepted or refused submission becomes a
  `HubEvent` with a per-Project monotonic cursor and a hash-chained
  `HubAuditCheckpoint`; clients pull events after their cursor.
- Revocation: revoking a device revokes its Hub participant and closes its
  sessions; later handshakes and submissions are refused (a submission that
  races the revocation inside a live session is recorded as
  `device_revoked`); nothing a revoked device sends after revocation is
  applied; the revocation is an event every other device mirrors.
- Tenant and Project scope checked before any lookup: realm, authority
  scope and Project membership bind every Hub read and write.
- Storage schema v12 -> v13 (additive), backup/restore with fail-closed
  parsing, consistency checks, restart recovery.
- CLI `medscale hub ...` (human and JSON), including `serve` over the local
  IPC. No Desktop surface in this foundation.

## Explicitly not authorized

- Any network listener, TCP/TLS/HTTP/WebSocket server or client, and any
  new dependency.
- Cloud accounts, federation, multi-Hub topology, relay, discovery.
- End-to-end encryption of collaboration bodies (Q06).
- Object/blob transfer (roadmap slice 084-02) and self-host deployment
  packaging (084-05): later slices after a transport admission.
- Sync of Spec 075/077-083 objects; approvals, threads and room
  administration through the Hub (the Hub operator opens threads and
  manages memberships locally).
- Real PHI, production credentials, multi-tenant production claims.

## Frozen acceptance requirements

1. Two client vaults enroll on one Hub vault by invitation; a reused,
   revoked or tampered invitation is refused.
2. A handshake with a wrong version, a replayed or foreign nonce, or a bad
   signature is refused.
3. Client A and client B each queue edits while disconnected; after
   reconnect both sets are applied in Hub order and both clients' mirrors
   converge on the same event list.
4. A stale task update yields a `SyncConflict`; a stale note edit yields a
   conflict copy; messages never conflict.
5. Replaying an accepted envelope changes nothing and returns the same
   outcome; a forged, reordered or re-signed envelope is refused.
6. After revocation the device's handshakes and submissions are refused;
   nothing it sends is applied; other devices mirror the revocation event.
7. A device of Project P cannot read or write Project Q, and nothing crosses
   realm or authority scope.
8. The Hub event chain verifies from the first event; an edited, removed or
   reordered event is detected on read and on restore.
9. Hub and client state survive reopen and backup/restore; tampered rows
   are refused; v12 backups restore with empty Hub tables; device secrets
   are never in a backup (a restored client re-enrolls); invitations open
   at backup time restore revoked; a device's public key is bound into the
   chained enrollment event.
10. The CLI reaches the Hub only through Core; no network code.
11. Exact-head and post-main CI pass.

Recorded residuals:

- local-machine transport only; Hub-to-device authentication relies on the
  local socket's OS access control;
- device secrets are protected by the vault, not an OS key store;
- no object transfer and no Desktop surface;
- a crash between applying a submission and recording its event can apply
  that submission again on retry;
- operator trust: the Hub operator can read everything the Hub stores (no
  end-to-end encryption).

## Completion rule

`CLOSED_CANONICAL` only after merge on a green exact head and recorded
post-main verification. Closure of 084 does not authorize 085.
