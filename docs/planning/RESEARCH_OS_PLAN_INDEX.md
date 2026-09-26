# MedScale Research OS Planning Index V2

**Status:** Planning candidate — not implementation authority

This index is the entry point for the Research OS expansion proposal after Program Amendment 001 added first-class Data Sources, R Workspace, and Community Extensions.

## Read in this order

### A. Program authority and V2 amendment

1. [`RESEARCH_OS_PROGRAM_AMENDMENT_001_DATA_EXTENSIONS.md`](./RESEARCH_OS_PROGRAM_AMENDMENT_001_DATA_EXTENSIONS.md) — founder-directed V2 amendment; supersedes old candidate numbering at 075+ for future planning only.
2. [`RESEARCH_OS_EXECUTION_ROADMAP.md`](./RESEARCH_OS_EXECUTION_ROADMAP.md) — candidate dependency graph 074-092.
3. [`RESEARCH_OS_V2_DECISION_REGISTER.md`](./RESEARCH_OS_V2_DECISION_REGISTER.md) — mandatory V2 defaults/evidence-selected decisions for Data Sources, R and Extensions.
4. [`RESEARCH_OS_PLAN_STATUS.md`](./RESEARCH_OS_PLAN_STATUS.md) — current packet state and governance boundary.
5. [`RESEARCH_OS_PACKET_VERSION.md`](./RESEARCH_OS_PACKET_VERSION.md) — packet snapshot/version scope.

### B. Product and architecture

6. [`RESEARCH_OS_VISION.md`](./RESEARCH_OS_VISION.md) — product thesis, target users, deployment ladder, Project Graph and Research Packs.
7. [`RESEARCH_OS_PRINCIPLES.md`](./RESEARCH_OS_PRINCIPLES.md) — non-negotiable product/engineering principles.
8. [`RESEARCH_OS_DECISIONS.md`](./RESEARCH_OS_DECISIONS.md) — original architectural decisions and boundaries; V2 Decision Register supersedes conflicting 075+ choices.
9. [`RESEARCH_OS_ARCHITECTURE.md`](./RESEARCH_OS_ARCHITECTURE.md) — original authority/intelligence/browse/analytics/collaboration/audio/compute/sync planes; read together with V2 addenda.
10. [`RESEARCH_OS_PRODUCT_MAP.md`](./RESEARCH_OS_PRODUCT_MAP.md) — original product surfaces and Project workspace composition.
11. [`DATA_SOURCE_FABRIC_PLAN.md`](./DATA_SOURCE_FABRIC_PLAN.md) — Data Sources workspace, connectors, imports, snapshots, database/Kaggle/Hugging Face semantics.
12. [`R_WORKSPACE_PRODUCT_PLAN.md`](./R_WORKSPACE_PRODUCT_PLAN.md) — R/RStudio/Posit integration and reproducibility boundary.
13. [`COMMUNITY_EXTENSIONS_PRODUCT_PLAN.md`](./COMMUNITY_EXTENSIONS_PRODUCT_PLAN.md) — Extension SDK, Community Registry, sandbox/capability model.
14. [`AUDIOFLOW_PRODUCT_PLAN.md`](./AUDIOFLOW_PRODUCT_PLAN.md) — audio/voice subsystem.
15. [`RESEARCH_OS_THREAT_AND_SCALE_MODEL.md`](./RESEARCH_OS_THREAT_AND_SCALE_MODEL.md) — original trust zones, threats, deployment and failure scenarios; V2 verification/addenda extend it.

### C. Sources and donor governance

16. [`RESEARCH_OS_SOURCE_LEDGER.md`](./RESEARCH_OS_SOURCE_LEDGER.md) — source universe and source-governance context.
17. [`SOURCE_ADOPTION_MATRIX.md`](./SOURCE_ADOPTION_MATRIX.md) — V2 donor/source roles including Kaggle, Hugging Face datasets, Arrow/R, Posit/renv, Obsidian, Wasmtime/Extism, Himsat and MESC patterns.
18. [`RESEARCH_OS_DONOR_RULE.md`](./RESEARCH_OS_DONOR_RULE.md) — provenance/adoption minimums.
19. [`RESEARCH_OS_V2_ORCA_DONOR_ADDENDUM.md`](./RESEARCH_OS_V2_ORCA_DONOR_ADDENDUM.md) — qualified planning posture for `stablyai/orca`; maps reusable agent-workbench, parallel-lane, terminal/review and browser-surface components into candidate Specs 077/078/080 without changing Spec 074 or transferring authority.
20. [`RESEARCH_OS_DECISION_RESOLUTION_REGISTER.md`](./RESEARCH_OS_DECISION_RESOLUTION_REGISTER.md) — original V1 safe defaults; use the V2 Decision Register for V2 additions and renumbered ownership.

### D. Implementation contract

21. [`RESEARCH_OS_MASTER_IMPLEMENTATION_CONTRACT.md`](./RESEARCH_OS_MASTER_IMPLEMENTATION_CONTRACT.md) — original program-wide authority rules that remain inherited unless V2 explicitly refines them.
22. [`RESEARCH_OS_V2_IMPLEMENTATION_CONTRACT_ADDENDUM.md`](./RESEARCH_OS_V2_IMPLEMENTATION_CONTRACT_ADDENDUM.md) — V2 shared contracts for Data Source Fabric, R Workspace, Community Extensions and cross-plane behavior.
23. [`RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md`](./RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md) — per-candidate implementation contracts for 075-092. This is the current per-spec planning truth for 075+.
24. [`RESEARCH_OS_REPOSITORY_IMPLEMENTATION_MAP.md`](./RESEARCH_OS_REPOSITORY_IMPLEMENTATION_MAP.md) — original repository/crate map.
25. [`RESEARCH_OS_V2_REPOSITORY_MAP_ADDENDUM.md`](./RESEARCH_OS_V2_REPOSITORY_MAP_ADDENDUM.md) — V2 ownership direction for source adapters, R staging/Compute, extension host/runtime/registry and renumbered planes.
26. [`RESEARCH_OS_SPEC_IMPLEMENTATION_CONTRACTS.md`](./RESEARCH_OS_SPEC_IMPLEMENTATION_CONTRACTS.md) — original V1 074-089 implementation shapes; historical for candidate 075+ when conflicting with V2.
27. [`RESEARCH_OS_IMPLEMENTER_INSTRUCTIONS.md`](./RESEARCH_OS_IMPLEMENTER_INSTRUCTIONS.md) — mandatory implementer discipline after promotion.
28. [`RESEARCH_OS_FUTURE_SPEC_TEMPLATE.md`](./RESEARCH_OS_FUTURE_SPEC_TEMPLATE.md) — required content for any future promoted spec.
29. [`RESEARCH_OS_MIGRATION_STRATEGY.md`](./RESEARCH_OS_MIGRATION_STRATEGY.md) — compatibility/migration/recovery posture inherited by V2.
30. [`RESEARCH_OS_EVIDENCE_RECEIPTS.md`](./RESEARCH_OS_EVIDENCE_RECEIPTS.md) — original typed receipt families; V2 contracts add source/R/extension receipt families.

### E. Verification and execution

31. [`RESEARCH_OS_ACCEPTANCE_FRAMEWORK.md`](./RESEARCH_OS_ACCEPTANCE_FRAMEWORK.md) — universal/subsystem gates inherited by V2.
32. [`RESEARCH_OS_VERIFICATION_MATRIX.md`](./RESEARCH_OS_VERIFICATION_MATRIX.md) — original verification layers.
33. [`RESEARCH_OS_V2_VERIFICATION_ADDENDUM.md`](./RESEARCH_OS_V2_VERIFICATION_ADDENDUM.md) — Data Source, R, Extensions and cross-plane V2 qualification campaigns.
34. [`RESEARCH_OS_DEFINITION_OF_READY.md`](./RESEARCH_OS_DEFINITION_OF_READY.md) — when a future unit may be implemented.
35. [`RESEARCH_OS_BUILD_RULES.md`](./RESEARCH_OS_BUILD_RULES.md) — build/execution discipline.
36. [`RESEARCH_OS_COMPLETION_CRITERIA.md`](./RESEARCH_OS_COMPLETION_CRITERIA.md) — original program-level planning completion semantics.
37. [`RESEARCH_OS_GAP_CLOSURE_REVIEW.md`](./RESEARCH_OS_GAP_CLOSURE_REVIEW.md) — V1 gap review.
38. [`RESEARCH_OS_V2_GAP_CLOSURE_REVIEW.md`](./RESEARCH_OS_V2_GAP_CLOSURE_REVIEW.md) — V2 cross-plane/stale-numbering/data/R/extensions gap audit.
39. [`RESEARCH_OS_REVIEW_CHECKLIST.md`](./RESEARCH_OS_REVIEW_CHECKLIST.md) — review checklist.
40. [`RESEARCH_OS_FINAL_PLANNING_ASSERTIONS.md`](./RESEARCH_OS_FINAL_PLANNING_ASSERTIONS.md) — current summary assertions after Amendment 001.

### F. Supporting context

- [`RESEARCH_OS_OPEN_QUESTIONS.md`](./RESEARCH_OS_OPEN_QUESTIONS.md) is research inventory only; do not choose architecture from it directly.
- [`RESEARCH_OS_LAB_ADOPTION_JOURNEYS.md`](./RESEARCH_OS_LAB_ADOPTION_JOURNEYS.md) captures adoption scenarios.
- [`RESEARCH_OS_METRICS.md`](./RESEARCH_OS_METRICS.md) captures product/technical measures.
- [`RESEARCH_OS_NON_GOALS.md`](./RESEARCH_OS_NON_GOALS.md) and [`RESEARCH_OS_SCOPE_BOUNDARY.md`](./RESEARCH_OS_SCOPE_BOUNDARY.md) prevent expansion by implication; V2 amendment explicitly promotes Data Sources/R/Extensions into planning despite older deferrals.
- [`RESEARCH_OS_PLAN_MANIFEST.md`](./RESEARCH_OS_PLAN_MANIFEST.md) describes the original packet and remains historical unless separately reconciled.

## Governance boundary

The V2 amendment is planning-only. It does not alter already-promoted Spec 074 scope and does not authorize candidate Spec 075 or later.

Before any candidate 075+ promotion:

1. reverify live repository frontier and unused numbering;
2. read Program Amendment 001 + V2 Roadmap + V2 Decision Register + V2 implementation/spec/repository/verification addenda + V2 Gap Review;
3. treat conflicting V1 candidate-number references as historical planning context;
4. bind exact current repository paths/types/tests;
5. resolve technology choices through the V2 Decision Register and evidence, not implementer preference;
6. define migration/recovery/rollback/security/evidence;
7. authorize only one bounded dependency-ready unit.

For candidate Specs 077, 078 or 080, the Orca donor addendum is an additional required planning input whenever Orca-derived code or patterns are considered. It does not itself authorize code transfer; the owning promoted spec must pin exact upstream paths/revision and satisfy normal donor qualification.

## Current program numbering

```text
074 Project + Artifact Graph Foundation       [promoted separately]
075 Data Source Fabric                        [candidate only]
076 Collaboration Substrate                   [candidate only]
077 MedAgent Workbench                        [candidate only]
078 Model Fleet + Compare                     [candidate only]
079 Privacy Gate                              [candidate only]
080 Governed Browse                           [candidate only]
081 AudioFlow Foundation                      [candidate only]
082 Analytics Gate                            [candidate only]
083 Knowledge + Research Canvas               [candidate only]
084 MedScale Hub                              [candidate only]
085 MedScale Compute                          [candidate only]
086 R Workspace                               [candidate only]
087 Community Extensions                      [candidate only]
088 AudioFlow Advanced                        [candidate only]
089 Research Packs                            [candidate only]
090 Institutional Adapters                    [candidate only]
091 Federation                                [candidate only]
092 Whole-Platform Qualification              [candidate only]
```

## North-star statement

> **One workspace. Many engines. One authority. User-owned data. Local by default. Evidence everywhere. Extensible by capability, never by ambient trust.**