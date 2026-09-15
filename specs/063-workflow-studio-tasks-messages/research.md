# Research — Spec 063

## Existing authority
- `OutboxEntry` is the rebuildable durable projection for external-action intents.
- `EffectState` is closed: Pending, Sent, Confirmed, Failed, Unknown.
- `CreateExternalActionIntentRequest` requires exact `DigestSha256` payload identity.
- `ListOutbox` is read-only under strict client/session enforcement.
- Core effect transition logic refuses UNKNOWN→retry without reconciliation evidence.

## Consequence
Desktop can safely visualize and organize review work over these contracts, but it must not create a second authority store. Tasks are projections/review cues. Messages are local review/status previews unless a future transport contract is separately authorized.

## Workflow Studio consequence
The founder-approved workflow visual can represent the real trust sequence: evidence, explicit review, payload identity, durable outbox, reconciliation. It must not imply that arranging visual nodes itself authorizes or executes clinical/external work.
