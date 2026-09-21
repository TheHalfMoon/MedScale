# Plan — Spec 077 MedAgent Workbench

## Execution rule

Implement only the promoted Spec 077 scope. Follow
`docs/planning/SPEC_077_PROMOTION.md`, the Research OS V2 master contracts/
repository map/decision register/verification matrix, and this Spec 077
package. Live repository truth wins over stale assumptions.

## Phase 0 — Reverify and freeze

Before material code changes:

1. Fetch/update local `main` and verify it contains merge
   `80aefaecc513b504b89a8a6cf06b2a0bbd5e49ea` or a proven later canonical
   descendant.
2. Confirm branch is `spec/077-medagent-workbench` and no unexpected local
   changes exist.
3. Re-read `AGENTS.md`, `CURSOR.md`, `IMPLEMENTATION_AUTHORITY.md`,
   `SPEC_077_PROMOTION.md`, `START_HERE.md`, `BUILD_QUEUE.md`, the Research
   OS V2 execution roadmap/spec-implementation-contracts/decisions, and
   this Spec 077 package.
4. Verify no Spec 077 collision or newer founder authority supersedes this
   promotion.
5. Inventory exact live types/modules for `OpaqueId`, `ObjectHeader`,
   `DigestSha256`, `ProjectRevision`/`check_revision`/`initial_revision`,
   `ArtifactDescriptor`/`ArtifactVersionBinding`/`ReferenceResolution`,
   `Proposal`/`ProducerKind`/`ClinicalAssertion`, `AuthorityError`,
   `SessionRegistry`, the `medscale-pack` runtime (`PackRuntimeAdapter`,
   `FixtureRuntime`, `OnnxTokenClassifierRuntime`, `PackManifestV0`,
   `PackStore`), Spec 076's `ParticipantIdentity`/`ParticipantKind::Agent`/
   `AgentParticipantIdentity`, storage migration (current version 5),
   backup/recovery, writer locking, CLI routing, and Desktop composition.
6. Record baseline required CI/test state. Pre-existing failures are
   evidence, not permission to suppress tests.
7. Freeze the field-level contract/data model in `contracts.md` before
   077-B.

## 077-A — Contracts and invariants (T077-01)

### Work

- Add the contract module(s) required by Spec 077
  (`medscale-contracts/src/medagent.rs` or the repository-conventional
  shape).
- Reuse `OpaqueId`, `ObjectHeader`, `DigestSha256`, `ProjectRevision`,
  `ArtifactDescriptor`/`ArtifactVersionBinding`/`ReferenceResolution`,
  `Proposal`/`ProducerKind`.
- Define `AgentIdentity`/`AgentProfile`/`AgentCapabilityManifest`,
  `ContextManifest`, `AgentRun`/`AgentRunState`/`AgentTurn`,
  `ToolManifest`/`ToolInvocation`/`ToolReceipt`, `RunReceipt`,
  `AgentProposal`.
- Define strict validation and explicit revision/precondition behavior for
  mutable rows; define the `AgentRunState` transition table explicitly
  (`Pending -> Running -> {Cancelled, Completed, Failed}`, no other edge).
- Reuse `AuthorityError` as-is; add no new variant unless T077-01 proves a
  genuine gap, recorded in `contracts.md`.
- Decide and record (frozen at T077-01) exactly how `AgentProposal` relates
  to the existing `Proposal`/`ProducerKind`: either `AgentProposal` is a
  thin 077-owned record carrying run provenance that, on completion,
  submits a real `Proposal` via the existing `CreateProposal` capability
  (extending `ProducerKind` with an `Agent` variant if the closed Spec 002
  enum needs it, additively), or an equivalent design that avoids a second
  proposal authority. No 077 code path may promote a proposal or create a
  `ClinicalAssertion`.

### Gate

Focused contract tests pass, dependency direction remains valid, no
storage/UI/network/model-runtime dependency leaks into contracts, and the
freeze record in `contracts.md` is filled in with exact Rust paths.

## 077-B — Storage and migration (T077-02)

### Work

- Extend the current encrypted metadata/vault migration mechanism
  (v5 -> v6); do not create a second DB.
- Add bounded tables/indexes for agent identities, agent profiles/
  capability manifests, context manifests (+ their selected-artifact rows),
  agent runs, agent turns, tool invocations, tool receipts, run receipts,
  agent proposals (`migration.md` section 2).
- Implement transactional repository/storage functions required by Core;
  storage does not decide authority.
- Add migration from representative pre-077 vaults (including populated
  074/075/076 vaults).
- Add reopen, crash-boundary and backup/restore recovery tests.
- Preserve all existing IDs and pre-077 behavior.

### Gate

Migration is repeat-safe under current framework, crash semantics are
unambiguous (a run's terminal state and its `RunReceipt` commit together or
not at all), old workflows still pass, and no cascade can delete referenced
canonical objects.

## 077-C — AgentIdentity + capability manifest (T077-03)

### Work

- Implement agent identity registration (bound to one admitted local model
  Pack via `PackStore`/`PackManifestV0`), get/list, and an explicit,
  immutable-once-set `AgentCapabilityManifest` (which tool kinds this
  identity may ever request).
- CLI inspect for agent identity.

### Gate

Agent identity vertical slice proven end to end with reopen durability; an
identity bound to a non-admitted/unknown Pack is refused at registration.

## 077-D — ContextManifest (T077-04)

### Work

- Implement `ContextManifest` create/get from an explicit list of
  `ArtifactDescriptor`s (reusing Spec 074's binding/resolution machinery).
- Implement the read-boundary enforcement: any Core path that resolves
  "what can this run read" consults only the bound `ContextManifest`, never
  a broader vault query.

### Gate

An `AgentRun` bound to a `ContextManifest` naming artifacts A and B cannot
resolve artifact C through any exposed Core path (`security.md` T-equivalent
of 076's T3 cross-scope test, adapted to context boundaries).

## 077-E — AgentRun lifecycle (T077-05)

### Work

- Implement `AgentRun` create (`Pending`), transition to `Running`, and
  terminal transitions to `Cancelled`/`Completed`/`Failed`.
- Implement cancel/interrupt: a cancel request against a `Running` run
  stops further tool dispatch and transitions the run to `Cancelled`
  before any further turn executes.
- Implement `AgentTurn` as the append-only per-step record within a run
  (prompt/response/tool-call boundary).

### Gate

Full state machine proven end to end, including cancel mid-run; no
transition outside the frozen table is reachable; a cancelled run never
produces a later `Completed` state.

## 077-F — Tool invocation (T077-06)

### Work

- Implement `ToolManifest` (the fixed set of tool kinds this spec admits;
  minimum viable set only, e.g. "read context artifact" / "search context
  artifacts" — no browser/network tool), `ToolInvocation` (typed request +
  policy check against the run's `AgentCapabilityManifest` and
  `ContextManifest`), `ToolReceipt` (typed result, committed).
- Every tool invocation is dispatched and executed by Core, never by
  model-authored code; the model may only *request* a tool by name and
  typed arguments.

### Gate

A tool invocation outside the granted capability manifest is refused
before execution and recorded as a refusal, not silently dropped or
silently executed.

## 077-G — Model Pack lane + AgentProposal (T077-07)

### Work

- Wire a real text-prompt-in/proposal-out run against the existing
  `medscale-pack` runtime (whichever admitted `PackRuntimeAdapter`
  implementation this repository state genuinely qualifies for
  general-purpose or bounded-task text output — record exactly which one
  and why in `contracts.md`/`CLOSURE.md`, not merely assumed).
- Persist the run's output as an `AgentProposal`, submitted through the
  existing `Proposal`/`CreateProposal` authority path per the T077-01
  design decision.

### Gate

A real local Project-grounded run (text prompt + explicit context, zero
network) completes and produces a persisted, evidence-only `AgentProposal`;
structurally, no code path from this slice reaches `PromoteProposal`,
`TransitionEffect`, or `ClinicalAssertion` creation (`security.md` T1
equivalent).

## 077-H — RunReceipt + run history (T077-08)

### Work

- Implement `RunReceipt` (exact model Pack identity/version, exact
  `ContextManifest` revision, every `ToolInvocation`/`ToolReceipt`,
  resource/timing facts, terminal state), committed atomically with the
  run's terminal transition.
- Implement bounded, filterable run-history read (by Project, by agent
  identity, by state).

### Gate

Every run's `RunReceipt` is inspectable after the fact with exact
provenance; a `RunReceipt` never exists for a run that did not actually
reach a terminal state, and a terminal run never lacks its `RunReceipt`.

## 077-I — Native Desktop and CLI parity (T077-09)

### Work

- Add a MedAgent navigation route through current native Slint
  composition, backed exclusively by Core: at minimum a split-pane run
  list/detail view, context-artifact selection, prompt submission, and
  cancel/interrupt controls.
- Preserve current design system, keyboard/focus/accessibility and
  light/dark parity conventions.
- Complete CLI vertical slice with human + JSON output matching Spec
  074/075/076 conventions.

### Gate

CLI/Desktop tests prove real Core-backed agent-run data (no fake product
data); no direct storage/model-runtime access from CLI/Desktop.

## 077-J — Qualification, review and closure (T077-10)

### Work

1. Run format, dependency-direction, focused contract/storage/Core/CLI/
   Desktop tests.
2. Run Clippy under current policy, full workspace tests, cargo-deny/
   supply-chain gates.
3. Run migration/reopen/recovery suite, including pre-077 fixtures and
   backup/restore.
4. Run malformed/hostile-input, context-boundary-leakage, and structural
   no-effect (agent output -> clinical authority) suites.
5. Run credential/log secret and content-leakage scans.
6. Capture rendered Desktop evidence where the CI/toolchain allows it; if
   not, record the same honest residual pattern Spec 075/076 recorded
   rather than fabricating a render.
7. Perform exact-range review of the full PR diff using OpenCodeReview
   (delegation mode), matching the discipline established in Spec 076,
   unless the founder gives a different explicit instruction for this
   spec.
8. Run exact-head required CI.
9. Create/update `evidence/077-medagent-workbench/` with exact commands,
   platform, SHAs, fixtures and results.
10. Open/update the Spec 077 PR with real evidence.
11. Merge only after exact-head required gates pass and governance allows
    it.
12. Verify post-merge `main` CI.
13. Update canonical status/queue to `CLOSED_CANONICAL` only after the
    post-main evidence exists.
14. Recompute the next eligible unit; do not implement 078+ without
    separate promotion.

### Required evidence set under `evidence/077-medagent-workbench/`

```text
README.md (index + per-file rules)
LIVE_TRUTH.md (T077-00 branch/base/main/PR/CI state)
CONTRACT_QUALIFICATION.md (frozen fields to Rust paths + tests)
STORAGE_MIGRATION_RECOVERY.md (fixtures, before/after, backup/migrate/reopen/restore)
CORE_AUTHORITY_QUALIFICATION.md (mutation/query matrix + denial/conflict/scope)
CLI_QUALIFICATION.md (human + JSON, exits, Core parity)
DESKTOP_QUALIFICATION.md (native state/adapter/a11y + renders or honest residual)
SECURITY_ADVERSARIAL.md (threat-gate results from security.md, including the
  structural no-effect proof for agent output -> clinical authority)
NO_NETWORK_LOCAL_PATH.md (egress-disabled proof)
MODEL_PACK_LANE_QUALIFICATION.md (exactly which medscale-pack runtime this
  spec uses and why, with real run evidence)
EXACT_RANGE_REVIEW.md (authorized scope only, unexpected files, new dependencies)
EXACT_HEAD_QUALIFICATION.md (candidate head + live CI)
POST_MERGE_VERIFICATION.md (merge SHA + main checks)
CLOSURE.md (terminal truth block)
logs/ (CLI vertical-slice log, any UI transcript)
```

## Parallelism

Default is sequential T077-00 -> T077-10 slices because contracts/storage/
Core are shared boundaries. 077-F/G/H touch mostly disjoint concerns once
077-A/B/C/D/E land, so focused test/evidence writing may proceed in
parallel once the shared contract/storage/run-lifecycle foundation lands —
but no slice may widen a frozen contract without amending `contracts.md`
first.

## Stop conditions

Stop the current mutation, record evidence, and resolve before continuing
if:

- live main contradicts the promoted base in a material way;
- another actor has already claimed Spec 077 with conflicting work;
- a required migration would rewrite existing canonical object IDs;
- an implementation requires a new authority/ID/provenance/model-inference
  foundation;
- the proposed change requires scope from Spec 078+ (multi-lane fleet
  comparison, privacy-gate transforms, browser tools, Hub/Compute);
- a test exposes an agent-run path reaching `ClinicalAssertion`/
  `PromoteProposal`/`TransitionEffect`, a context-boundary leak beyond the
  bound `ContextManifest`, an authority bypass, or secret/content leakage;
- the existing `medscale-pack` runtime genuinely cannot support a real
  text-prompt-in/proposal-out vertical slice without a new inference
  engine — this is a blocking finding to resolve explicitly (scope
  amendment or founder decision), not a reason to silently build one;
- required CI fails for the current change;
- real PHI or gated credentials/terms would be required.

Do not stop for ordinary implementation decisions already resolved by
canonical planning; use the decision register, tests and safest minimal
design.
