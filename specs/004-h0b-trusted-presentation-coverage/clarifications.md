# Clarification Closeout: Spec 004

**Date**: 2026-08-25  
**Command**: `/speckit.clarify` equivalent (autonomous defaults)

No founder questions. Ambiguities resolved via `docs/planning/IMPLEMENTATION_DECISION_DEFAULTS.md`, constitution, MASTER_BUILD_PLAN_V2 H0-B, SPECKIT_MASTER_ROADMAP_V2 Spec 004 row, GLM53 F-08/F-14, SOURCE_ACQUISITION_AND_COPY_PLAN, Spec 002 time/span rules, and Spec 003 anti-scope. Details live in `research.md`.

| ID | Ambiguity | Resolution |
|---|---|---|
| C1 | Include Proposals in default Brief/timeline? | **No.** Default clinical Timeline/Brief use ClinicalAssertions only. Proposals remain non-authoritative; optional candidates view deferred |
| C2 | General FHIRPath vs typed extractors | **Typed extractors only** in H0-B; no FHIRPath engine (GLM F-14 / master plan §16) |
| C3 | Which FHIR resources in H0-B? | **Patient, Observation, Condition** admitted; others → Unsupported/Unknown until amendment |
| C4 | Timeline sort key | Effective time if present, else recorded; assertion id tie-break; never invent Instant precision |
| C5 | Conflict resolution policy | **Surface Conflict**; do not auto-resolve or latest-wins in H0-B |
| C6 | Unknown vs Absent | Absent = admitted extractor looked and found nothing; Unknown = unsupported/not evaluated |
| C7 | UCUM scope | **Bounded admitted subset** behind trait; unrecognized units fail closed for comparison; no licensed terminology dump |
| C8 | Brief narrative text | **No** LLM or free-text generation; closed structured sections only |
| C9 | Drill-down coordinates | Prefer **raw-byte TextSpan**; whole-resource citation allowed for structural fields; path notes are not FHIRPath execution |
| C10 | Persist Projections vs ephemeral | Persist rebuildable Projection kinds via existing RebuildProjection; golden equality required |
| C11 | Crate split | Prefer `medscale-fhir` extractors + `medscale-core` facade; no speculative presentation crate |
| C12 | Spec 003 durability rework | **Out of scope**; consume vault/ingest/visibility/rebuild hooks only |

**Outstanding NEEDS CLARIFICATION markers in spec.md**: none

**External gates**: none opened by Spec 004 planning. REAL_PHI, MESC mutation, product network, models, OpenMed runtime remain unauthorized. Implementation gate: Spec 003 `CLOSED_CANONICAL` (satisfied per BUILD_QUEUE).
