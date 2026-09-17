# Research OS Future Spec Template

Use this template only after canonical governance authorizes the next bounded dependency-ordered unit. Roadmap presence is not authorization.

A spec is not implementation-ready until every mandatory section below is concrete or explicitly `N/A` with justification.

```text
# Spec NNN — <name>

Status:
Promoted:
Base SHA:
Target branch:
Dependencies and exact predecessor closure evidence:
Supersedes:

## 1. Problem
What concrete user/system problem is still unsolved?

## 2. Goal
One bounded end state.

## 3. User stories / operational scenarios
Include normal, denied, offline, failure and recovery scenarios.

## 4. Required outcomes
Numbered, objectively testable outcomes.

## 5. Explicit non-goals
State adjacent roadmap features that this spec MUST NOT implement.

## 6. Existing-system compatibility
Exact existing types/routes/commands/storage semantics that must remain compatible.

## 7. Authority invariants
Who may create/change canonical state? Which outputs remain proposal/evidence/projection only?

## 8. Identity / authorization / capability rules
Actor types, project scope, capability names, approval requirements, revocation behavior.

## 9. Data classification / privacy rules
Accepted data classes, transforms, egress boundaries, retention/logging/cache rules.

## 10. Contracts and types
List exact new/changed Rust contract types, enum/state semantics and owning crate/module.

## 11. State machines
Every long-running or authority-sensitive lifecycle with all terminal/unknown/partial states.

## 12. Core commands / queries
Typed command/query names, preconditions, idempotency, revision behavior and result/error taxonomy.

## 13. Storage schema / indexes / transactions
Tables/columns/indexes, transaction boundaries, concurrency rules, deletion/tombstone behavior.

## 14. Migration and rollback
Pre-existing vault migration, forward compatibility, rollback/recovery checkpoint and test method.

## 15. Network / external effects
Destinations, broker route, credentials, timeouts, idempotency, confirmation and Unknown behavior.

## 16. Worker / sandbox behavior
If applicable: manifest, staged inputs, filesystem/network/resource limits, outputs and admission.

## 17. Source/dependency/donor decisions
For every new source: exact revision, files/component, permission/license/NOTICE, security review, reason to adopt, maintenance plan.

## 18. CLI behavior
Commands, JSON/human output, exit/error semantics and parity with Core.

## 19. Desktop behavior
Routes/panels/states, accessibility, visible trust/privacy/runtime state and no fake data.

## 20. Failure and recovery table
At minimum invalid/denied/conflict/unavailable/failed/cancelled/timed-out/unknown/partial/stale where applicable.

## 21. Security/threat delta
New attack surfaces, hostile inputs, abuse cases, denial tests.

## 22. Performance/resource envelope
Workload, supported hardware, startup/latency/memory/storage/network metrics and acceptance thresholds where claims are required.

## 23. Test plan
L0-L9 layers from RESEARCH_OS_VERIFICATION_MATRIX.md; exact focused/full/platform/external commands and fixtures.

## 24. Evidence plan
Exact evidence paths/artifacts and mapping from each acceptance criterion to proof.

## 25. Implementation slices
Dependency-ordered A/B/C/... slices. Each slice must be independently reviewable and state owned paths/types/tests.

## 26. Tasks
Every task names scope, dependencies, changed paths, acceptance test and evidence output.

## 27. Completion criteria
Exact conditions for CLOSED_CANONICAL / equivalent. Distinguish external blockers from repository-owned work.

## 28. Completion truth
What may be claimed after closure, and what remains explicitly deferred/unqualified.
```

## Required companion artifacts

A promoted spec should create only the companions it needs, but before implementation the information above must exist in inspectable repository artifacts. Typical companions:

```text
spec.md
plan.md
tasks.md
research.md
data-model.md
contracts.md
security.md
migration.md
evidence/README.md
```

Do not create empty paperwork for its own sake. Do not omit a required decision merely to keep the file count small.

## Promotion gate

Before setting a future spec to implementation-ready, verify:

- no unresolved architecture choice is being delegated to the implementer;
- evidence-selected engine/vendor choices have a qualification task and safe default;
- all predecessor contracts exist on canonical main;
- no duplicate authority/ID/provenance/audit model is introduced;
- local/offline behavior is defined;
- privacy/network/worker boundaries are explicit;
- migration/recovery is testable;
- CLI/Desktop both route through Core;
- acceptance criteria are executable or externally provable.

If any gate is missing, refine the spec instead of implementing.
