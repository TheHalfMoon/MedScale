# Spec 077 Promotion — MedAgent Workbench

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`
**Founder promotion date:** 2026-09-21
**Canonical base:** `80aefaecc513b504b89a8a6cf06b2a0bbd5e49ea`
**Target branch:** `spec/077-medagent-workbench`

## Authority

The founder's primary directive (Spec 076 continuation instruction) requires
recomputing the dependency graph and, if governance confirms the next
dependency-ready Research OS candidate, canonical promotion followed by
complete implementation through closure and continuation to the next
eligible unit, without routine per-spec approval.

Live verification performed at promotion time:

- PR #131 (Spec 076 implementation) exact-head CI run `35637687997` on head
  `10c99f40527ade420fdb99f59dfb207e15ead0e8`: 6/6 required jobs green;
  merged as `594c309a034bac1f2ba24b4e9d830dd6fbdbd857`; post-merge main CI
  run `35640113781`: 6/6 green.
- PR #132 (Spec 076 closure bookkeeping) merged as
  `80aefaecc513b504b89a8a6cf06b2a0bbd5e49ea`; its own exact-head CI run
  `35642851086` (head `1dac0600be35b2f4059953fc762f43f65cfe7b5c`) was 6/6
  green before merge.
- All run IDs, head SHAs, and merge SHAs verified live via `gh pr view`/
  `gh run view` against `TheHalfMoon/MedScale`, not assumed from prior
  conversation context.
- `docs/planning/BUILD_QUEUE.md` row 076 confirms `CLOSED_CANONICAL` with
  the same evidence and explicitly records the honest residual gaps from
  Spec 076 (no rendered Desktop evidence; Notes/Approvals have no Desktop
  UI surface; T11 content-leakage proof is by inspection, not an automated
  test). Those residuals are carried forward as open, non-blocking
  observations; they do not gate Spec 077 because Spec 077 does not depend
  on Desktop rendering evidence or on the specific 076 UI gaps.

This document promotes **only Spec 077 — MedAgent Workbench** for
implementation.

Predecessor closure proof: Spec 076 is `CLOSED_CANONICAL` (exact-head
`10c99f4…`, run `35637687997`, PR #131 merged as `594c309…`, post-merge
main run `35640113781`; closure bookkeeping PR #132 merged as `80aefae…`;
see `evidence/076-collaboration-substrate/CLOSURE.md`).

Numbering proof: no `specs/077-*` package, no `SPEC_077_PROMOTION.md`, and
no Spec 077 implementation code exists on the canonical base (verified via
`git ls-tree -r --name-only origin/main | grep -i 077`, empty).
`RESEARCH_OS_EXECUTION_ROADMAP.md`'s program dependency graph
(`074 --> 077 MedAgent Workbench --> 078 Model Fleet + Compare`) and
`RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md` both independently number
MedAgent Workbench as **077** with a hard dependency on **074 only**
(already `CLOSED_CANONICAL`); both documents additionally note that
"integration with 076 participant/activity semantics should be bound before
final closure if live contracts require it" — 076 is now also
`CLOSED_CANONICAL`, so this integration is available, not merely planned.
No unclosed hard prerequisite blocks this promotion.

Standing implementation authority in `IMPLEMENTATION_AUTHORITY.md` remains
active. Normal branch/commit/push/PR/merge authority applies only within
the promoted scope and only after real required gates pass.

## Authorized scope

Spec 077 may implement only the minimum governed, local-first agent
workbench described in `specs/077-medagent-workbench/spec.md`:

- `AgentIdentity`/`AgentProfile`/`AgentCapabilityManifest`: a registered,
  admitted local model Pack bound to an explicit, bounded capability set
  (no unbounded tool access, no ambient vault access);
- `ContextManifest`: an explicit, typed, revision-bound set of selected
  Project artifacts (074 `ArtifactDescriptor`/`ArtifactVersionBinding`)
  that forms the *only* data an agent run may read — never ambient vault
  access;
- `AgentRun`/`AgentRunState`/`AgentTurn`: a persisted, resumable run state
  machine (`Pending -> Running -> (Cancelled | Completed | Failed)`, with
  explicit interrupt/steer support within `Running`);
- `ToolManifest`/`ToolInvocation`/`ToolReceipt`: every tool call is typed,
  policy-checked, executed by Core (never by model-generated code), and
  receipted; model output never directly calls a Rust function;
- `RunReceipt`: durable, inspectable provenance for one run (exact model
  Pack identity/version, exact `ContextManifest` revision, every tool
  invocation and receipt, exact wall-clock/resource facts, final state);
- `AgentProposal`: an agent run's output is evidence-only, reusing the
  existing `Proposal`/`CreateProposal` authority path (Spec 002's
  `objects::authority_classes::Proposal`/`ProducerKind`), never a new
  authority plane and never directly promotable to `ClinicalAssertion` by
  anything in this spec's own code paths;
- one admitted local model Pack lane, reusing the existing
  `medscale-pack` runtime (`PackRuntimeAdapter`, `FixtureRuntime`, and/or
  the qualified `OnnxTokenClassifierRuntime` from Spec 069) — **no new
  model inference engine is built by this spec**; if the existing runtimes
  cannot support a genuine text-prompt-in/proposal-out vertical slice, that
  is a blocking finding to resolve during `T077-00`/`T077-01`, not a reason
  to silently build a second inference engine;
- integration with Spec 076's `ParticipantKind::Agent`/
  `AgentParticipantIdentity.agent_profile_ref`: an `AgentIdentity` may
  resolve the previously-unresolved forward reference, so an agent
  participant in a collaboration Room can be backed by a real `AgentRun`
  producer — this is the "integration with 076" the roadmap flags, now
  unblocked;
- cancel/interrupt of a running `AgentRun`;
- run history persisted as inspectable Project-scoped artifacts;
- CLI vertical slice and a native Slint Desktop split-pane workbench (run
  list/detail, context selection, cancel/interrupt controls), both backed
  exclusively by Core;
- encrypted local persistence and migration (storage schema v5 -> v6,
  additive only);
- migration, recovery, compatibility, security, and exact-head evidence
  required for closure.

Staging rule: the first implementation lands contracts and storage/Core
foundation (`AgentIdentity`/`ContextManifest`/`AgentRun` skeleton with no
real tool execution yet) before tool invocation, before real model-backed
proposal generation, before CLI, before Desktop — mirroring the 075/076
staging discipline. The spec cannot close without the declared foundation
set or an explicit canonical scope amendment recorded in the owning
package.

## Explicitly not authorized

This promotion does **not** authorize implementation of:

- Spec 078 Model Fleet + Compare (multiple simultaneous lanes, comparison
  reports) — 077 proves exactly one admitted local model lane;
- Spec 079 Privacy Gate (beyond reusing current data classification as it
  exists today; no new de-identification/redaction pipeline);
- Spec 080 Governed Browse + Medical Literature Acquisition — browser/web
  providers remain explicitly denied to every agent run in this spec;
- Spec 081 AudioFlow Foundation + Local Scribe MVP;
- Spec 082 Analytics Gate + Cohort Builder;
- Spec 083 Clinical Graph + Knowledge/Research Canvas;
- Spec 084 MedScale Hub, or any network sync execution;
- Spec 085 MedScale Compute (remote/bounded-worker execution of agent
  runs) — this spec's runs are local-process only;
- Specs 086-092;
- any mechanism by which an `AgentProposal`/tool invocation automatically
  performs a clinical, research-authority, or external effect — that
  remains owned by the existing, separately gated
  `PromoteProposal`/`TransitionEffect`/controlled-actions capabilities,
  invoked only by an explicit, separately authorized human caller action,
  never by agent output itself;
- a new general-purpose local-model inference engine (GGUF/llama.cpp-class
  runtime, remote-model client, or similar) — this spec proves the agent
  *workbench* (identity, context, run lifecycle, tool receipts, proposal
  output) against the model execution surface `medscale-pack` already
  qualifies, not a new inference stack;
- multi-agent orchestration, agent-to-agent delegation, or any agent
  self-registration/self-escalation path (an `AgentIdentity` is created
  only by an explicit human-initiated Core command, never by another
  agent's output);
- real PHI;
- MESC work;
- wholesale adoption of Buzz's agent/workflow UI, relay protocol, or
  execution model; Buzz patterns may be selectively studied for shape only
  per `RESEARCH_OS_DECISIONS.md` D4/D10 and `RESEARCH_OS_DONOR_RULE.md`,
  never imported as running code;
- a new ID, provenance, audit, or authority foundation when current
  MedScale primitives (`OpaqueId`, `ObjectHeader`, `ProjectRevision`,
  `Proposal`/`ProducerKind`, `ArtifactDescriptor`, the existing
  `medscale-pack` runtime, Spec 076's `ParticipantIdentity`/
  `ActivityRecord`) are sufficient.

## Mandatory architecture constraints

1. Reuse current MedScale `OpaqueId`, `ObjectHeader`, `DigestSha256`,
   realm/scope, `ProjectRevision`/`check_revision`/`initial_revision`,
   `ArtifactDescriptor`/`ArtifactVersionBinding`/`ReferenceResolution`,
   `Proposal`/`ProducerKind`, digest/provenance, audit, vault, migration,
   and Core patterns where applicable.
2. Existing patient/FHIR/source/document/evidence/model/Pack/Project/
   data-source/collaboration objects remain canonical in their owning
   systems. An agent run references them by stable identity/version
   binding; it never copies or re-derives their content outside its own
   receipted run record.
3. An `AgentRun` reads *only* what its `ContextManifest` explicitly names.
   There is no code path from agent execution to unscoped vault/storage
   access. This is proven by review and by a dedicated test, not asserted
   by comment (mirrors 076's T1 discipline for `ApprovalDecision`).
4. Model output is data, never code: no agent run path evaluates,
   interprets, or executes model-generated text as a Rust function call,
   shell command, or SQL fragment. Every tool invocation is a Core-typed,
   policy-checked operation the model can only *request* by name/typed
   arguments, never author directly.
5. `AgentProposal` output is `Proposal`-class only. No code path in this
   spec creates a `ClinicalAssertion` or calls `PromoteProposal`/
   `TransitionEffect`/any controlled-action capability.
6. Cancel/interrupt must be observable and effective: a cancelled run
   transitions to `Cancelled` and stops issuing further tool invocations;
   it never silently continues in the background after the caller is told
   it stopped.
7. CLI and Desktop must use the same Core command/query semantics. No
   direct storage/model-runtime access from UI.
8. Local/offline operation is mandatory: with network egress disabled,
   every 077 workflow remains fully useful (the one admitted model Pack
   lane runs fully offline). Spec 077 opens no network connection to any
   external model/browser provider and requires none.
9. Migration must preserve all pre-077 workflows and object identities (no
   existing object ID rewrite).
10. Unknown/unavailable/stale/partial/cancelled/failed states stay
    distinct; missing input is not zero; partial run output is never
    silently presented as complete.
11. No later Research OS unit may be started merely because its planning
    document exists.

## Implementation order

```text
T077-00 Live truth and baseline (no material product mutation before it closes)
T077-01 Contracts freeze + invariant tests (AgentIdentity, AgentProfile,
        AgentCapabilityManifest, ContextManifest, AgentRun/AgentRunState/
        AgentTurn, ToolManifest/ToolInvocation/ToolReceipt, RunReceipt,
        AgentProposal)
T077-02 Storage schema v6, migration, crash/reopen/recovery tests
T077-03 AgentIdentity + AgentProfile + capability manifest vertical slice
T077-04 ContextManifest (explicit artifact selection, revision-bound)
T077-05 AgentRun lifecycle (Pending/Running/Cancelled/Completed/Failed,
        interrupt/steer)
T077-06 ToolManifest/ToolInvocation/ToolReceipt (typed, policy-checked,
        Core-executed only)
T077-07 Local model Pack lane integration (medscale-pack runtime) +
        AgentProposal output (evidence-only, Proposal-class)
T077-08 RunReceipt + run history as Project artifacts
T077-09 Native Desktop split-pane workbench + CLI parity
T077-10 Exact-head qualification, review, and closure
```

A later slice may not paper over a failed earlier invariant.

## Frozen acceptance requirements

Spec 077 can close only when all of the following are proven on the exact
reviewed head:

1. An `AgentIdentity` can be registered against one admitted local model
   Pack, with an explicit `AgentCapabilityManifest`, and this survives
   close/reopen.
2. A `ContextManifest` binds exact artifact revisions; an `AgentRun`
   reading outside its named context is structurally impossible, not
   merely untested.
3. A local Project-grounded run (text prompt, explicit selected artifacts,
   zero network) completes and produces a persisted `AgentProposal` plus a
   `RunReceipt` with exact model/context/tool provenance.
4. A running `AgentRun` can be cancelled/interrupted; it stops issuing
   further tool invocations and transitions to `Cancelled`, never silently
   continuing.
5. Every `ToolInvocation` is typed, policy-checked, executed by Core (not
   model-authored code), and produces a `ToolReceipt`.
6. No code path exists from `AgentRun`/`AgentProposal` output into
   `ClinicalAssertion` creation, `PromoteProposal`, `TransitionEffect`, or
   any controlled-action capability — proven structurally, not merely by
   absence of a test failure.
7. Partial/failed runs are explicit and distinguishable from completed
   runs; no fake completion.
8. Pre-077 vaults (including populated 074/075/076 vaults) migrate
   (v5 -> v6), reopen, and recover from backup.
9. No alternate authority/storage/network path exists (dependency-direction
   gate holds).
10. Every 077 workflow remains usable with network disabled.
11. No real-PHI authorization is implied; fixtures are synthetic/permitted
    non-PHI.
12. Exact-head required CI and post-main verification pass.

## Evidence paths

All qualification evidence lives under `evidence/077-medagent-workbench/`
(see `specs/077-medagent-workbench/plan.md` for the required evidence set,
mirroring the Spec 074/075/076 evidence structure).

## Completion rule

Spec 077 becomes `CLOSED_CANONICAL` only when all acceptance criteria in
the owning Spec 077 package are proven on the exact reviewed head, required
CI is green, the PR is merged normally, and post-merge main verification is
recorded.

Closure of 077 does not itself authorize 078. After closure, live
governance must recompute the next eligible unit and explicitly promote it.
