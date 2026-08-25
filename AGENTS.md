# AGENTS.md — MedScale Repository Execution Rules

## Authority order

1. Explicit founder instruction and `docs/planning/IMPLEMENTATION_AUTHORITY.md`.
2. Live GitHub/repository truth for mutable state.
3. Canonical planning/spec/task artifacts in this repository.
4. Frozen MedScale architecture and contracts.
5. Primary standards/upstream evidence within their scoped subject.
6. Historical planning/review material.

Never use a stale handoff or remembered SHA over live repository truth.

## Active execution state

```text
PHASE = AUTONOMOUS_IMPLEMENTATION_AUTHORIZED
CURSOR_IMPLEMENTATION_AUTHORITY = YES_WITHIN_CANONICAL_V2_PLAN
ASK_FOUNDER_FOR_ORDINARY_IMPLEMENTATION_DECISIONS = NO
CONTINUE_TO_NEXT_ELIGIBLE_UNIT = YES
REAL_PHI_AUTHORITY = NO
MESC_MUTATION_AUTHORITY = NO
PRODUCT_RUNTIME_NETWORK_EGRESS = DEFAULT_DENY
```

Earlier planning files may contain historical metadata saying implementation was not authorized. `docs/planning/IMPLEMENTATION_AUTHORITY.md` explicitly supersedes those authorization-only lines. Their architecture and scope rules remain canonical.

## Mandatory execution entry point

Cursor must read `CURSOR.md`, `docs/planning/START_HERE.md`, `docs/planning/BUILD_QUEUE.md`, and the current owning Spec Kit package before material work.

MedScale uses GitHub Spec Kit and a Spec-of-Specs process. For each material executable spec:

```text
/speckit.specify
/speckit.clarify
/speckit.plan
/speckit.checklist
/speckit.tasks
/speckit.analyze
-> qualification
-> /speckit.implement
-> /speckit.converge
```

Use `/speckit.constitution` when repository constitution is created/changed. Spec 000 is already `CLOSED_CANONICAL` at planning/source-authority level; Spec 001 is the first executable unit and materializes the constitution into the bootstrapped Spec Kit structure without reopening founder decisions.

## Autonomous behavior

Do not ask the founder for routine implementation choices already covered by the plan. Resolve them using `IMPLEMENTATION_DECISION_DEFAULTS.md`, primary evidence, tests, and the safest minimal design. Record material decisions in the owning spec/research/ADR and continue.

When a unit becomes `CLOSED_CANONICAL`, immediately recompute `BUILD_QUEUE.md` and start the next eligible unit. Do not stop after one task, PR, milestone, phase, or passing demo.

If a real human/external gate is required, record it in `EXTERNAL_GATES.md`, continue all independent work, and do not reinterpret the gate as project-wide permission to stop.

## Non-negotiable architecture

- Rust is the primary and trusted-core language.
- Local-first and privacy-first are constitutional.
- Useful core operation requires neither cloud account nor remote model.
- No hidden telemetry/remote logging/crash upload is required.
- Source bytes != derived representation; source identity != content hash.
- Durable object classes remain distinct. `Proposal != ClinicalAssertion`.
- Identity merge is explicit; projections are rebuildable/non-authoritative.
- FHIR R4 4.0.1 is initial interchange, not canonical DB; validator output is evidence only.
- One Rust authority path serves all product surfaces.
- Workers receive no ambient canonical DB, master keys, unrestricted filesystem, network, secrets, or authority.
- MESC is independent and `ARTIFACT_FIRST + EXCEPTIONAL_SANDBOXED_SERVICE`.
- External-action `UNKNOWN` is never blindly retried.

## Donor and source rule

`docs/planning/SOURCE_ACQUISITION_AND_COPY_PLAN.md` is mandatory before donor use. Never wholesale-copy OpenMed or another repository. Every copied/ported/vendor component must bind exact upstream URL/revision/path, rights/license/NOTICE, content identity, local target, modifications, tests, security placement, maintenance/update/exit strategy, and SBOM/provenance.

Permission to copy code does not imply permission to redistribute model weights, datasets, licensed terminology, or other assets.

## H0 boundary

H0-A/H0-B are synthetic-only and LLM-free. No real PHI, models, OpenMed/MESC integration, OCR/ASR/vector/agents, live SMART/NPHIES, external actions, or product runtime network.

## Mutation discipline

Before each material mutation, re-read live branch/head, active PRs, changed files, CI/review state, current spec/tasks, and dependency gates. No force-push, destructive rebase/history rewrite, skipped tests, or unsupported PASS claims.