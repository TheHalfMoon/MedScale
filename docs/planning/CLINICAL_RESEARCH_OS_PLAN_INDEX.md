# MedScale Clinical + Research Intelligence OS — Plan Index

**Date:** 2026-09-19  
**Status:** `PLANNING_CANDIDATE_ONLY`  
**Entry point:** use this file to review the implementation-ready Clinical + Research Intelligence expansion.

## Read order

### 1. Product direction

1. `RESEARCH_OS_V2_CLINICAL_INTELLIGENCE_EXPANSION.md`
   - product thesis;
   - Abridge/OpenEvidence/OpenMed capability floor;
   - Linked Evidence Everywhere;
   - Clinical Graph;
   - Data Workbench;
   - Research OS 075–092 mapping.

2. `CLINICAL_COMPETITOR_CAPABILITY_MATRIX_2026-09-19.md`
   - dated competitor/category capability matrix;
   - measurable future parity/surpass criteria.

### 2. Architecture and implementation

3. `RESEARCH_OS_V2_IMPLEMENTATION_READY_MASTER_PLAN.md`
   - system invariants;
   - architecture planes;
   - cross-program contracts;
   - architecture decisions;
   - complete 075–092 implementation decomposition;
   - vertical milestones;
   - promotion protocol.

4. `RESEARCH_OS_V2_EXACT_REPOSITORY_IMPLEMENTATION_MAP_2026-09-19.md`
   - maps every candidate spec to the current nine-crate Rust workspace;
   - binds existing Core/storage/network/Pack/Desktop anchors;
   - defines default module/file/test ownership;
   - constrains new-crate creation.

5. `LOCAL_MEDICAL_SCRIBE_PLAN.md`
   - offline capture;
   - two-pass ASR;
   - diarization;
   - clinical extraction;
   - context assembly;
   - note drafting;
   - Linked Evidence;
   - clinician review;
   - coding/order/nursing/patient-instruction boundaries.

6. `CLINICAL_GRAPH_DATA_WORKBENCH_PLAN.md`
   - Graphify-inspired clinical graph;
   - AFFiNE-inspired workspace views;
   - Airtable-class MedScale Data Workbench;
   - cohort/data/analytics lineage;
   - donor-specific licensing boundaries.

### 3. Donors and sources

7. `FOUNDER_GITHUB_DONOR_SYNTHESIS_2026-09-19.md`
   - useful public sibling-project patterns;
   - private-repository disclosure boundary;
   - anti-monolith rule.

8. `RESEARCH_OS_V2_SOURCE_QUALIFICATION_LEDGER_2026-09-19.md`
   - exact research pins;
   - license posture;
   - candidate adoption mode;
   - transfer gates.

9. `MEDICAL_EVIDENCE_SOURCE_STRATEGY_2026-09-19.md`
   - open/licensed/institution/user evidence classes;
   - PubMed/PMC/Crossref/OpenAlex/ClinicalTrials.gov strategy;
   - guideline, retraction and evidence-Pack rights model.

### 4. Evaluation protocols

10. `LOCAL_MEDICAL_SCRIBE_EVALUATION_PROTOCOL_2026-09-19.md`
   - medical-critical ASR metrics;
   - diarization;
   - note factuality/omission taxonomy;
   - Linked Evidence;
   - privacy/offline/resource/human review campaigns.

11. `EVIDENCE_ENGINE_EVALUATION_PROTOCOL_2026-09-19.md`
   - retrieval;
   - citation identity;
   - claim support;
   - evidence appraisal;
   - applicability/contradiction/retraction;
   - deep synthesis and trial-matching evaluation.

### 5. Traceability and gap challenge

12. `RESEARCH_OS_V2_REQUIREMENTS_TRACEABILITY_2026-09-19.md`
   - 115 product/platform requirements;
   - competitor/founder rationale;
   - owning specs;
   - required proof;
   - zero unassigned requirements at planning time.

13. `RESEARCH_OS_V2_CLINICAL_GAP_CLOSURE_2026-09-19.md`
   - 115 challenged gaps/failure risks;
   - owning spec;
   - closure evidence;
   - external/non-software gates;
   - no-gap promotion checklist.

### 6. Implementation handoff

14. `MUSE_CLINICAL_RESEARCH_OS_IMPLEMENTATION_HANDOFF.md`
   - agent execution instructions after canonical acceptance/promotion;
   - live-truth rebuild;
   - dependency order;
   - evidence requirements;
   - stop conditions.

---

# Existing Research OS authority to read with this expansion

This packet does not replace existing canonical Research OS V2 planning. It refines and deepens it.

Read together with:

- `RESEARCH_OS_PROGRAM_AMENDMENT_001_DATA_EXTENSIONS.md`;
- `RESEARCH_OS_EXECUTION_ROADMAP.md`;
- `RESEARCH_OS_V2_DECISION_REGISTER.md`;
- `RESEARCH_OS_MASTER_IMPLEMENTATION_CONTRACT.md`;
- `RESEARCH_OS_V2_IMPLEMENTATION_CONTRACT_ADDENDUM.md`;
- `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md`;
- `RESEARCH_OS_ACCEPTANCE_FRAMEWORK.md`;
- `RESEARCH_OS_DEFINITION_OF_READY.md`;
- `RESEARCH_OS_V2_VERIFICATION_ADDENDUM.md`;
- `SOURCE_ADOPTION_MATRIX.md`;
- `OSS_CODE_ABSORPTION_MATRIX_V2.md`;
- `V0_UI_INTEGRATION_CONTRACT.md`.

If this packet conflicts with a currently promoted spec or live canonical repository authority, the live promoted authority wins until a normal reconciliation/promotion changes it.

---

# Relationship to parallel Draft PRs

At preparation time:

- the Orca donor plan is maintained in a separate Draft PR;
- the Identity V2 / V0 UI handoff is maintained in a separate Draft PR;
- this packet does not modify their branches or implementation authority.

Shared Research OS index/source-matrix reconciliation should happen in normal merge order after the planning branches are accepted rather than by copying stale versions across branches.

---

# Planning completion test

This packet may be considered ready for founder/canonical planning acceptance when reviewers can trace:

```text
Product capability
 -> owning Research OS spec
 -> implementation slice
 -> authority/privacy boundary
 -> donor/source decision
 -> failure/recovery behavior
 -> qualification evidence
 -> release claim boundary
```

for every major capability in scope.

Implementation still requires normal per-spec promotion.
