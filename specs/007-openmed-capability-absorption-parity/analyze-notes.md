# Analyze Notes: Spec 007 OpenMed Capability Absorption / Parity Research

**Date**: 2026-08-25  
**Command**: `/speckit.analyze` equivalent (planning package consistency)  
**Package**: `specs/007-openmed-capability-absorption-parity/`

## Artifacts reviewed

| Artifact | Present |
|---|---|
| spec.md | yes |
| clarifications.md | yes |
| research.md | yes |
| plan.md | yes |
| data-model.md | yes |
| contracts/openmed-baseline-pin.md | yes |
| contracts/parity-corpus.md | yes |
| contracts/donor-absorption.md | yes |
| contracts/terminology-licensing-track.md | yes |
| contracts/saudi-arabic-benchmark.md | yes |
| contracts/benchmark-manifest.md | yes |
| quickstart.md | yes |
| checklists/requirements.md | yes |
| tasks.md | yes |
| analyze-notes.md | yes (this file) |

## Alignment with roadmap / constitution

| Gate / invariant | Coverage | Status |
|---|---|---|
| Pin OpenMed v2.2.0 / `59d9cb0…` | US1, FR-001/002, D1, baseline contract | OK |
| Parity corpora / phase-scoped matrix | US2, FR-003/004, D3/D5, data-model | OK |
| Donor absorption dispositions | US3, FR-005, D4, donor-absorption | OK |
| Terminology/licensing track start | US4, FR-006, D6, terminology contract | OK |
| Saudi/Arabic benchmark program design | US5, FR-007, D7, saudi-arabic contract | OK |
| No runtime required by this spec | FR-008/009, D2, plan constraints | OK |
| No wholesale OpenMed copy / no Python trusted core | FR-008, D4, DO_NOT_COPY | OK |
| No REAL_PHI / MESC mutation / product network | FR-008, SC-007 | OK |
| Spec 008 consumes qualified 007 | Assumptions, tasks Phase 8, BUILD_QUEUE | OK |
| BenchmarkManifest for claims | FR-010, D9, benchmark-manifest | OK |

## Cross-document consistency checks

1. **SPECKIT_MASTER_ROADMAP 007 row**: pin baseline; corpora; absorption; terminology; Saudi/Arabic; no runtime — all present.
2. **MASTER_BUILD_PLAN §8**: comparator corpus, donor plan, licensing track, Arabic design, stage-promotion notes — covered (promotion pattern in D8/absorption).
3. **SOURCE_ACQUISITION**: exact commit/tree; operations vocabulary; forbidden interpretations — OK.
4. **OPENMED_PARITY_SURPASS_MATRIX_V2**: classifications + §3 manifest fields — OK.
5. **OSS_CODE_ABSORPTION_MATRIX_V2**: OpenMed + Presidio/medSpaCy/terminology dispositions — OK.
6. **Clarifications**: Zero NEEDS CLARIFICATION; autonomous defaults — OK.
7. **Tasks**: T001–T038 research-only; no Rust AI fabric; no OpenMed import — OK.
8. **BUILD_QUEUE**: RESEARCH_ELIGIBLE; 008 blocked by 007 — consistent with qualification target.

## Contradictions / gaps

| Item | Severity | Disposition |
|---|---|---|
| Full measured corpora vs design-only close | residual | SC allows design+registry; measurement in 008 |
| Counsel terminology licenses | external | EXTERNAL_GATES; non-blocking for research close |
| Local OpenMed clone may be unavailable | residual | `documented_pin_only` allowed |
| Tip vs pin confusion | mitigated | D1/C1 + contracts |
| Historical planning “PRODUCT_IMPLEMENTATION_NOT_STARTED” in matrices | informational | IMPLEMENTATION_AUTHORITY supersedes auth-only lines; architecture still binds |

**Unresolved design blockers inside Spec 007 package: NONE.**

**Runtime implementation blocker (intentional): Spec 007 must NOT implement product AI runtime.**

## Entry / exit readiness

```text
ENTRY: RESEARCH_ELIGIBLE (after Spec 004 path per roadmap; no models/PHI/runtime required)
PACKAGE_STATE: COMPLETE_SPEC_KIT_PACKAGE
ANALYZE_RESULT: PASS_NO_UNRESOLVED_DESIGN_BLOCKERS
ANALYZE_QUALIFICATION: QUALIFIED_FOR_RESEARCH_IMPLEMENTATION
PRODUCT_RUNTIME_QUALIFICATION: NOT_APPLICABLE / NOT_AUTHORIZED_IN_007
OPENMED_RUNTIME_IMPORT: FORBIDDEN
WHOLESALE_COPY: FORBIDDEN
READY_RESEARCH: YES
RUST_AI_FABRIC: DEFER_TO_008
```

## Recommendation

**QUALIFIED for research implementation** (docs/matrices/evidence/corpora design) on branch `spec/007-openmed-capability-absorption-parity` using `tasks.md`.

Do **not** implement Rust OpenMed/AI runtime in this unit. Do **not** treat missing counsel terminology licenses, missing OpenMed local clone, or unmeasured Saudi/Arabic scores as READY blockers for research close—record gates and continue. After research closeout/converge, Spec 008 may proceed on qualified 007 contracts.
