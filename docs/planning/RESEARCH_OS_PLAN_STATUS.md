# MedScale Research OS Planning Status

**Planning branch:** `plan/medscale-research-os`

## Scope

This branch is documentation-only planning for the proposed Research OS expansion. It intentionally does not:

- modify production code;
- modify `specs/CURRENT.md`;
- promote Spec 074 or any later candidate;
- change current Product/Design authority;
- claim implementation, qualification or release readiness;
- merge itself.

## Planning artifacts

- `RESEARCH_OS_PLAN_INDEX.md`
- `RESEARCH_OS_VISION.md`
- `RESEARCH_OS_DECISIONS.md`
- `RESEARCH_OS_ARCHITECTURE.md`
- `RESEARCH_OS_PRODUCT_MAP.md`
- `AUDIOFLOW_PRODUCT_PLAN.md`
- `SOURCE_ADOPTION_MATRIX.md`
- `RESEARCH_OS_SOURCE_LEDGER.md`
- `RESEARCH_OS_THREAT_AND_SCALE_MODEL.md`
- `RESEARCH_OS_ACCEPTANCE_FRAMEWORK.md`
- `RESEARCH_OS_MIGRATION_STRATEGY.md`
- `RESEARCH_OS_BUILD_RULES.md`
- `RESEARCH_OS_LAB_ADOPTION_JOURNEYS.md`
- `RESEARCH_OS_OPEN_QUESTIONS.md`
- `RESEARCH_OS_EXECUTION_ROADMAP.md`

## Required reconciliation before merge

1. Reverify current `main`, `specs/CURRENT.md`, `AGENTS.md`, Product and Design authority.
2. Confirm whether Spec 074+ numbering is still unused on current `main`.
3. Verify this plan does not contradict active or already-promoted work.
4. Review all source/license statements against exact live revisions.
5. Ensure private connected-source information is not disclosed.
6. Treat candidate technical choices such as DataFusion/OpenFGA/OpenSandbox as qualification targets, not preselected implementation truth.
7. Open a reviewable PR; do not merge automatically.
