# MedScale Research OS Planning Status

**Planning branch:** `plan/medscale-research-os`  
**Review PR:** `#121` (draft, planning-only)  
**Planning base verified:** `main` = `c795796132ce633b23b693d55bccc0521d0d26aa`  
**Base truth:** repository-owned implementation is closed canonically through Spec 073; the Research OS packet is a founder-directed future expansion proposal and does not silently reopen closed specs.

## Scope

This branch is documentation-only planning for the proposed Research OS expansion. It intentionally does not:

- modify production code;
- modify active/current Spec Kit authority;
- promote Spec 074 or any later candidate;
- change current Product/Design authority;
- claim implementation, qualification or release readiness;
- authorize real PHI;
- change the separate MESC boundary;
- merge itself.

## Planning packet layers

### Product / architecture

- `RESEARCH_OS_PLAN_INDEX.md`
- `RESEARCH_OS_VISION.md`
- `RESEARCH_OS_PRINCIPLES.md`
- `RESEARCH_OS_DECISIONS.md`
- `RESEARCH_OS_ARCHITECTURE.md`
- `RESEARCH_OS_PRODUCT_MAP.md`
- `AUDIOFLOW_PRODUCT_PLAN.md`
- `RESEARCH_OS_THREAT_AND_SCALE_MODEL.md`

### Sources / donor governance

- `RESEARCH_OS_SOURCE_LEDGER.md`
- `SOURCE_ADOPTION_MATRIX.md`
- `RESEARCH_OS_DONOR_RULE.md`

### Implementation hardening

- `RESEARCH_OS_MASTER_IMPLEMENTATION_CONTRACT.md`
- `RESEARCH_OS_REPOSITORY_IMPLEMENTATION_MAP.md`
- `RESEARCH_OS_SPEC_IMPLEMENTATION_CONTRACTS.md`
- `RESEARCH_OS_IMPLEMENTER_INSTRUCTIONS.md`
- `RESEARCH_OS_DECISION_RESOLUTION_REGISTER.md`
- `RESEARCH_OS_FUTURE_SPEC_TEMPLATE.md`
- `RESEARCH_OS_MIGRATION_STRATEGY.md`
- `RESEARCH_OS_EVIDENCE_RECEIPTS.md`

### Verification / execution governance

- `RESEARCH_OS_ACCEPTANCE_FRAMEWORK.md`
- `RESEARCH_OS_VERIFICATION_MATRIX.md`
- `RESEARCH_OS_DEFINITION_OF_READY.md`
- `RESEARCH_OS_BUILD_RULES.md`
- `RESEARCH_OS_EXECUTION_ROADMAP.md`
- `RESEARCH_OS_COMPLETION_CRITERIA.md`
- `RESEARCH_OS_REVIEW_CHECKLIST.md`

### Supporting product planning

- `RESEARCH_OS_LAB_ADOPTION_JOURNEYS.md`
- `RESEARCH_OS_METRICS.md`
- `RESEARCH_OS_OPEN_QUESTIONS.md` (research inventory only; implementation defaults live in Decision Resolution Register)
- `RESEARCH_OS_NON_GOALS.md`
- `RESEARCH_OS_SCOPE_BOUNDARY.md`
- `RESEARCH_OS_PLAN_MANIFEST.md`
- `RESEARCH_OS_PACKET_VERSION.md`

## Implementation-readiness posture

The packet is intentionally more specific than a normal roadmap. It now defines:

- authority and dependency direction;
- canonical object/revision/relationship rules;
- state and error taxonomies;
- data classes and privacy boundary behavior;
- MedAgent context/tool/fleet boundaries;
- Governed Browse routing, egress, credential, hostile-content, redirect/SSRF, quarantine and evidence rules;
- collaboration conflict defaults;
- Hub trust/sync defaults;
- worker/compute least-privilege rules;
- AudioFlow routing/transcript lineage;
- Analytics read-only/query provenance rules;
- retrieval/index staleness/permission rules;
- Research Pack extension boundaries;
- per-candidate-spec implementation shape through candidate Spec 089;
- repository/crate/module ownership defaults;
- mandatory test/evidence layers;
- implementer stop conditions;
- safe defaults for previously open architecture questions.

This still does **not** make any candidate spec executable. A promoted unit must bind these program contracts to exact live paths/types/tests and current main.

## Required reconciliation before merge

1. Reverify current `main`, `AGENTS.md`, `BUILD_QUEUE.md`, implementation authority, Product and Design authority.
2. Confirm Spec 074+ numbering is still unused at merge time; renumber candidates if necessary.
3. Verify the plan does not contradict work promoted after the planning base.
4. Review source/license/permission statements against exact revisions before any transfer; planning ledger entries are not adoption approval.
5. Ensure no private connected-source information is disclosed publicly.
6. Keep engine/vendor choices explicitly evidence-selected where the Decision Resolution Register says so.
7. Review implementation contracts for duplicate ID/provenance/audit/policy models against current code.
8. Run a full planning-diff consistency search for stale candidate spec numbering and cross-plane dependencies after every roadmap insertion/renumbering.
9. Keep PR #121 planning-only; do not merge from implementation automation.
10. After planning acceptance, promote only the first dependency-ordered bounded unit (074 or renumbered equivalent) through normal Spec Kit/canonical governance.

## Planning completion condition

The planning packet may be considered review-ready when:

- all material brainstorm capabilities map to a bounded architecture surface;
- no known architecture question is left to implementer preference;
- evidence-selected choices have an owner, benchmark/qualification requirement and fail-safe default;
- every candidate spec has dependency, ownership, contracts, failure semantics and closure gates;
- implementation is mapped onto the current repository structure;
- verification and migration/recovery requirements are explicit;
- a gap-closure review finds no material unresolved cross-plane dependency or stale candidate-number reference.

Review-ready planning is not implementation completion and not project release readiness.
