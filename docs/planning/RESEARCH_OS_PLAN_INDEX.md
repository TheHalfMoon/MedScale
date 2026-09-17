# MedScale Research OS Planning Index

**Status:** Planning candidate — not implementation authority

This index is the entry point for the Research OS expansion proposal.

## Read in this order

### A. Product and architecture

1. [`RESEARCH_OS_VISION.md`](./RESEARCH_OS_VISION.md) — product thesis, target users, deployment ladder, Project Graph and Research Packs.
2. [`RESEARCH_OS_PRINCIPLES.md`](./RESEARCH_OS_PRINCIPLES.md) — compact non-negotiable product/engineering principles.
3. [`RESEARCH_OS_DECISIONS.md`](./RESEARCH_OS_DECISIONS.md) — architectural decisions and boundaries.
4. [`RESEARCH_OS_ARCHITECTURE.md`](./RESEARCH_OS_ARCHITECTURE.md) — authority/intelligence/browse/analytics/collaboration/audio/compute/sync planes and boundaries.
5. [`RESEARCH_OS_PRODUCT_MAP.md`](./RESEARCH_OS_PRODUCT_MAP.md) — product surfaces and Project workspace composition.
6. [`AUDIOFLOW_PRODUCT_PLAN.md`](./AUDIOFLOW_PRODUCT_PLAN.md) — audio/voice subsystem, speech routing, meetings/huddles and evidence model.
7. [`RESEARCH_OS_THREAT_AND_SCALE_MODEL.md`](./RESEARCH_OS_THREAT_AND_SCALE_MODEL.md) — trust zones, threats, deployment/scale tiers and failure scenarios.

### B. Sources and decisions

8. [`RESEARCH_OS_SOURCE_LEDGER.md`](./RESEARCH_OS_SOURCE_LEDGER.md) — source universe and source-governance context.
9. [`SOURCE_ADOPTION_MATRIX.md`](./SOURCE_ADOPTION_MATRIX.md) — donor/source roles and qualification posture.
10. [`RESEARCH_OS_DONOR_RULE.md`](./RESEARCH_OS_DONOR_RULE.md) — minimum provenance/adoption rule.
11. [`RESEARCH_OS_DECISION_RESOLUTION_REGISTER.md`](./RESEARCH_OS_DECISION_RESOLUTION_REGISTER.md) — safe default resolution for architecture questions; evidence-selected choices are explicitly identified.

### C. Implementation contract

12. [`RESEARCH_OS_MASTER_IMPLEMENTATION_CONTRACT.md`](./RESEARCH_OS_MASTER_IMPLEMENTATION_CONTRACT.md) — program-wide authority, identity, revision, state-machine, privacy, network, agent, Browse, audio, analytics, Hub, compute and schema rules.
13. [`RESEARCH_OS_REPOSITORY_IMPLEMENTATION_MAP.md`](./RESEARCH_OS_REPOSITORY_IMPLEMENTATION_MAP.md) — mapping of the plan onto current MedScale crates/modules/storage/network/Pack/CLI/Desktop boundaries.
14. [`RESEARCH_OS_SPEC_IMPLEMENTATION_CONTRACTS.md`](./RESEARCH_OS_SPEC_IMPLEMENTATION_CONTRACTS.md) — minimum implementation shape for candidate Specs 074-089.
15. [`RESEARCH_OS_IMPLEMENTER_INSTRUCTIONS.md`](./RESEARCH_OS_IMPLEMENTER_INSTRUCTIONS.md) — mandatory instructions for Muse/Codex/Claude/Cursor/human implementers after canonical promotion.
16. [`RESEARCH_OS_FUTURE_SPEC_TEMPLATE.md`](./RESEARCH_OS_FUTURE_SPEC_TEMPLATE.md) — mandatory content for any future promoted Research OS spec.
17. [`RESEARCH_OS_MIGRATION_STRATEGY.md`](./RESEARCH_OS_MIGRATION_STRATEGY.md) — compatibility, migration and recovery posture.
18. [`RESEARCH_OS_EVIDENCE_RECEIPTS.md`](./RESEARCH_OS_EVIDENCE_RECEIPTS.md) — typed receipt families and evidence linkage.

### D. Verification and execution

19. [`RESEARCH_OS_ACCEPTANCE_FRAMEWORK.md`](./RESEARCH_OS_ACCEPTANCE_FRAMEWORK.md) — universal and subsystem-specific gates.
20. [`RESEARCH_OS_VERIFICATION_MATRIX.md`](./RESEARCH_OS_VERIFICATION_MATRIX.md) — required verification layers and evidence expectations.
21. [`RESEARCH_OS_DEFINITION_OF_READY.md`](./RESEARCH_OS_DEFINITION_OF_READY.md) — when a future unit may be implemented.
22. [`RESEARCH_OS_BUILD_RULES.md`](./RESEARCH_OS_BUILD_RULES.md) — build/execution discipline.
23. [`RESEARCH_OS_EXECUTION_ROADMAP.md`](./RESEARCH_OS_EXECUTION_ROADMAP.md) — dependency-ordered candidate Specs 074-089, subject to live frontier reconciliation.
24. [`RESEARCH_OS_COMPLETION_CRITERIA.md`](./RESEARCH_OS_COMPLETION_CRITERIA.md) — program-level planning completion semantics.
25. [`RESEARCH_OS_GAP_CLOSURE_REVIEW.md`](./RESEARCH_OS_GAP_CLOSURE_REVIEW.md) — recorded cross-plane/stale-reference gap audit; closes the originally under-specified Governed Browse lane.
26. [`RESEARCH_OS_REVIEW_CHECKLIST.md`](./RESEARCH_OS_REVIEW_CHECKLIST.md) — review checklist for this packet and later promoted work.

### E. Supporting planning context

- [`RESEARCH_OS_OPEN_QUESTIONS.md`](./RESEARCH_OS_OPEN_QUESTIONS.md) is research inventory. **Implementers MUST NOT choose from it directly**; use the Decision Resolution Register.
- [`RESEARCH_OS_LAB_ADOPTION_JOURNEYS.md`](./RESEARCH_OS_LAB_ADOPTION_JOURNEYS.md) captures adoption scenarios.
- [`RESEARCH_OS_METRICS.md`](./RESEARCH_OS_METRICS.md) captures candidate product/technical measures.
- [`RESEARCH_OS_NON_GOALS.md`](./RESEARCH_OS_NON_GOALS.md) and [`RESEARCH_OS_SCOPE_BOUNDARY.md`](./RESEARCH_OS_SCOPE_BOUNDARY.md) prevent expansion by implication.
- [`RESEARCH_OS_PLAN_STATUS.md`](./RESEARCH_OS_PLAN_STATUS.md), [`RESEARCH_OS_PLAN_MANIFEST.md`](./RESEARCH_OS_PLAN_MANIFEST.md), and [`RESEARCH_OS_PACKET_VERSION.md`](./RESEARCH_OS_PACKET_VERSION.md) describe planning-packet state.

## Governance boundary

These documents intentionally live under `docs/planning/`. They do not alter `specs/CURRENT.md`, do not declare any candidate spec executable, and do not supersede existing MedScale Product/Design authority.

Before promotion, canonical governance must:

1. reverify the live repository frontier;
2. reconcile numbering/dependencies with any specs promoted after this planning branch was created;
3. challenge architecture against current implementation and external evidence;
4. use the Decision Resolution Register rather than delegating unresolved architecture to the implementer;
5. refine the next candidate unit using the Future Spec Template;
6. bind exact repository paths/types after live inspection;
7. establish migration/recovery/test/evidence commands;
8. authorize only the minimum dependency-ordered unit required.

## Implementer entry rule

A future implementation agent should not read only the roadmap. Minimum reading order after a Research OS unit is canonically promoted:

```text
AGENTS.md
live specs/CURRENT.md and active spec authority chain
RESEARCH_OS_MASTER_IMPLEMENTATION_CONTRACT.md
RESEARCH_OS_REPOSITORY_IMPLEMENTATION_MAP.md
RESEARCH_OS_SPEC_IMPLEMENTATION_CONTRACTS.md
RESEARCH_OS_DECISION_RESOLUTION_REGISTER.md
RESEARCH_OS_VERIFICATION_MATRIX.md
RESEARCH_OS_IMPLEMENTER_INSTRUCTIONS.md
relevant source/donor records
```

If live repository truth conflicts with this packet, live repository truth wins and the planning assumption must be reconciled explicitly before implementation continues.

## North-star statement

> **One workspace. Many engines. One authority. User-owned data. Local by default. Evidence everywhere.**

The expansion succeeds only if MedScale becomes more capable without weakening explicit authority, provenance, privacy boundaries, reproducibility, user-controlled infrastructure and honest evidence.
