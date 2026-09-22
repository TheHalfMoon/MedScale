# Contracts — Spec 078 Model Fleet + Compare

**Status:** implementation contract (initial freeze at T078-00; field-level
freeze record to be completed at T078-01 against the real Rust source,
mirroring Spec 077's `contracts.md` section 8 discipline)

## 1. Reused primitives (no new foundation)

- `OpaqueId`, `ObjectHeader`, `DigestSha256`, `RealmId`, `AuthorityScopeId`
  (`medscale-contracts::objects`).
- `ProjectRevision`/`check_revision`/`initial_revision` for optimistic
  concurrency on every mutable 078 row.
- Spec 077's `AgentIdentity`, `AgentIdentityStatus`,
  `AgentCapabilityManifest`, `ContextManifest`, `AgentRun`,
  `AgentRunState`, `AgentTurn`, `AgentTurnKind`, `ToolKind`,
  `ToolInvocation`, `ToolInvocationStatus`, `ToolReceipt`, `RunReceipt`,
  `AgentProposal` (`medscale-contracts::medagent`) — reused exactly as
  frozen, zero modification.
- `Proposal`/`ProducerKind` (`medscale-contracts::objects::authority_classes`)
  is reached only indirectly, through Spec 077's existing
  `AgentProposal`/`CreateProposal` path inside each lane's own `AgentRun`;
  078 adds no new `ProducerKind` variant and no direct `CreateProposal`
  call of its own.
- `PackManifestV0`/`PackStore` (`medscale-contracts::packs`,
  `medscale-pack::store`) as the admitted-model-Pack source of truth,
  reached only through Spec 077's existing `AgentIdentity` registration
  path (078 never calls `PackStore` directly).
- Spec 077's Core module (`medscale-core::authority::medagent::MedAgent`)
  exposes the run-execution surface 078 drives; in particular
  `execute_agent_run`, `create_agent_run`, `start_agent_run`,
  `cancel_agent_run` are called by 078's Core module exactly as any other
  caller would call them, never re-implemented.

## 2. Lane vocabulary

### `AgentLane`

```text
AgentLane {
  header: ObjectHeader,
  revision: ProjectRevision,
  project_id: OpaqueId,           // scoping Project (074)
  agent_identity_id: OpaqueId,    // Spec 077 AgentIdentity this lane wraps
  context_manifest_id: OpaqueId,  // Spec 077 ContextManifest this lane's runs bind
  role_label: String,             // bounded UTF-8, e.g. "baseline", "strict-evidence"
  policy: LanePolicy,
  status: AgentLaneStatus,        // Active | Retired
}
```

An `AgentLane` never widens what its bound `AgentIdentity`/
`ContextManifest` already allow; `policy` (below) may only *narrow* the
effective grant a lane's runs use, never grant anything the underlying
identity's `AgentCapabilityManifest` or the bound `ContextManifest` does
not already permit. This is enforced at lane-creation time (a policy
naming a tool kind the identity's capability manifest does not grant, or an
artifact the context manifest does not name, is refused, `InvalidArgument`)
and re-checked at every lane-run dispatch (never cached).

### `LanePolicy`

```text
LanePolicy {
  granted_tool_kinds: Option<Vec<ToolKind>>,   // None = inherit identity's
                                                // full capability manifest;
                                                // Some(set) = narrow subset,
                                                // must be a subset of the
                                                // identity's granted set
  context_artifact_ids: Option<Vec<OpaqueId>>, // None = inherit the full
                                                // bound ContextManifest;
                                                // Some(set) = narrow subset,
                                                // must be a subset of the
                                                // manifest's own selected
                                                // artifacts
}
```

`LanePolicy` is immutable once set (created together with `AgentLane` in
one transaction, matching Spec 077's `AgentCapabilityManifest` precedent;
widening a lane's policy requires retiring and re-creating the lane, an
explicit, auditable action).

### `LaneTransform`

```text
LaneTransform {
  // Reserved, closed-empty vocabulary at 078: no transform kind is defined
  // yet. A LaneTransform is named in the roadmap's contract list but this
  // spec's authorized scope has no concrete transform to apply (078 does
  // not admit new Packs, does not build a privacy transform pipeline --
  // that is Spec 079's authority). Declared here as a typed, closed-empty
  // enum (not a Value passthrough) so that a later spec extends it
  // additively rather than this spec inventing an untyped placeholder.
}
```

`LaneTransform` is frozen as a closed enum with **zero variants** at T078-01
unless T078-01 discovers a genuine, minimal, in-scope transform need (e.g.
a bounded prompt-prefix per role label) while implementing `LanePolicy`
above — if so, that variant is added and recorded here with rationale,
never spun up as unscoped general infrastructure.

### `AgentLaneStatus`

```text
Active | Retired
```

Mirrors `AgentIdentityStatus`'s `Active | Revoked` shape and semantics
(closed vocabulary, `as_str()`/`parse()` round trip); `Retired` is the lane
equivalent of "revoked" — a retired lane may not be bound to any new
`FleetRun`, but its past `LaneRunRef`s and any `ComparisonReport`s that
already reference it remain fully inspectable.

## 3. Fleet vocabulary

### `FleetRunState`

```text
Pending -> Running -> Completed
                    -> PartiallyFailed
                    -> Failed
                    -> Cancelled
```

`Pending`: created, no lane run dispatched yet. `Running`: at least one
lane run is `Pending`/`Running` in Spec 077 terms. `Completed`: every lane
reached `AgentRunState::Completed`. `PartiallyFailed`: at least one lane
reached `Completed` and at least one other lane reached
`Cancelled`/`Failed` — the fleet has *some* usable comparison material but
not from every lane. `Failed`: every lane reached `Cancelled`/`Failed`
(zero usable lane output). `Cancelled`: the fleet itself was cancelled
before any lane reached a terminal state (this cancels every still-running
lane through Spec 077's own `cancel_agent_run`, it does not invent a
second cancellation mechanism).

No other edge exists. `Pending -> Cancelled` directly (cancel before any
lane starts) is permitted, mirroring Spec 077's own early-cancel edge.
Every other transition is `Conflict`/`InvalidArgument`.

### `FleetRun`

```text
FleetRun {
  header: ObjectHeader,
  revision: ProjectRevision,
  project_id: OpaqueId,
  task_prompt: String,            // bounded UTF-8, dispatched identically
                                   // to every lane's own AgentRun.prompt
  status: FleetRunState,
}
```

### `LaneRunRef`

```text
LaneRunRef {
  header: ObjectHeader,
  fleet_run_id: OpaqueId,
  agent_lane_id: OpaqueId,
  agent_run_id: OpaqueId,         // the real Spec 077 AgentRun this lane's
                                   // task dispatch created; 1:1
}
```

Created atomically with each lane's own `AgentRun` creation
(`migration.md` section 5); a `FleetRun` with N bound lanes always has
exactly N `LaneRunRef` rows once dispatch completes, never a partial subset
silently missing a lane.

## 4. Comparison vocabulary

### `ComparisonRequest`

```text
ComparisonRequest {
  fleet_run_id: OpaqueId,         // must be in a terminal-or-partial state:
                                   // Completed | PartiallyFailed (a
                                   // Failed/Cancelled fleet has no usable
                                   // output to compare; requested anyway,
                                   // it is refused, InvalidArgument)
}
```

Not a durable row — a `ComparisonRequest` is the typed input to the
comparison computation; only its output (`ComparisonReport`) is persisted.

### `ComparisonObservation`

```text
ComparisonObservation {
  kind: ComparisonObservationKind, // Agreement | Disagreement |
                                    // ContradictionCandidate |
                                    // EvidenceOverlap | UnsupportedClaim |
                                    // Abstention | SchemaValidity |
                                    // ResourceRuntimeFact
  participating_lane_ids: Vec<OpaqueId>, // which lanes this observation
                                          // concerns (>=1; >=2 for
                                          // Agreement/Disagreement/
                                          // ContradictionCandidate/
                                          // EvidenceOverlap)
  detail: String,                 // bounded UTF-8, human-readable factual
                                   // description; never a score, never a
                                   // ranking phrase
  evidence_refs: Vec<OpaqueId>,   // AgentProposal/RunReceipt/tool-receipt
                                   // ids this observation is grounded in
}
```

**Structural constraint (frozen, security-relevant):** `ComparisonObservation`
has no numeric quality/confidence/rank field anywhere in its shape. Every
kind is a factual, evidence-grounded observation, never a verdict. This is
proven by contract shape (the struct has no such field, not merely "the
field happens to be unused") and re-verified by review.

### `ComparisonReport`

```text
ComparisonReport {
  header: ObjectHeader,
  fleet_run_id: OpaqueId,         // 1:1 with the FleetRun it was computed over
  observations: Vec<ComparisonObservation>,
  participating_lane_ids: Vec<OpaqueId>,   // every lane actually included
  excluded_lane_ids: Vec<OpaqueId>,        // lanes bound to the FleetRun
                                             // but excluded from this report
                                             // (failed/cancelled lanes;
                                             // named explicitly, never
                                             // silently dropped)
  classification: DataClassification,      // most-restrictive-inherits;
                                             // reuses whatever
                                             // classification primitive
                                             // Spec 074/077 context
                                             // artifacts already carry --
                                             // exact type named at T078-01
                                             // against real source, not a
                                             // new type invented here
}
```

A `ComparisonReport` is computed once per completed comparison request and
persisted immutably; re-running a comparison over the same `FleetRun`
(e.g. after a later spec adds a new observation kind) creates a new
`ComparisonReport`, never an in-place mutation of a prior report (matching
the append-only, audit-preserving discipline Spec 077 established for
`RunReceipt`).

## 5. Lane-plurality design decision (frozen at promotion, see
## `docs/planning/SPEC_078_PROMOTION.md`)

`AgentLane.agent_identity_id` may point at an `AgentIdentity` that another
`AgentLane` in the same or a different `FleetRun` also points at. Two lanes
sharing one `AgentIdentity` (and therefore one admitted Pack) are still two
distinct `AgentLane` objects with independently-chosen `context_manifest_id`
and `LanePolicy`, and therefore produce two independently-executed,
independently-receipted `AgentRun`s. Nothing in this spec's contracts
requires `AgentLane.agent_identity_id` to be unique within a `FleetRun`;
requiring that would silently assume a repository model inventory this
spec cannot guarantee.

## 6. Freeze record (T078-01, to be filled in against real Rust source)

```text
CONTRACTS_MODULE = <exact path, expected crates/medscale-contracts/src/model_fleet.rs>
SCHEMA_VERSION_CONST = <exact const, expected MODEL_FLEET_SCHEMA_VERSION: u32 = 1>
BOUND_CONSTANTS = <exact names/values for role_label, task_prompt,
  observation detail, and any other bounded free-text field>
ENUM_VOCABULARIES = <exact Rust enum definitions once frozen>
VALIDATION_HELPERS = <reused-vs-reimplemented decision, mirroring 077's
  T077-01 "deliberately re-implemented, not imported" convention if this
  module should stay independent of medagent.rs's own private helpers>
LANETRANSFORM_VARIANT_COUNT = <0 unless T078-01 finds a genuine minimal
  need, per section 2 above>
CLASSIFICATION_TYPE_REUSED = <exact type name/path>
TEST_COUNT = <exact count and names once written>
IMPORT_PURITY = <exact use-block audit once written; expected: only serde/
  serde_json + crate::objects + crate::project_graph + crate::medagent
  (Spec 077's own frozen contracts module) -- no storage/network/UI/
  model-runtime dependency>
```
