# MedScale Research OS Planning Completion Criteria

**Status:** Planning packet completion contract — not implementation or release authority.

The Research OS planning packet is structurally review-ready only when all criteria below are satisfied on the same planning head.

## 1. Product completeness

- product vision and target users are explicit from personal researcher through lab/institution scale;
- Projects are the organizing primitive;
- MedAgent, Fleet/Compare, Privacy Gate, Governed Browse, AudioFlow, Analytics, Knowledge/Research Canvas, collaboration/Hub, Compute, Research Packs, institutional adapters and federation all have bounded roles;
- "everything in one place" is implemented conceptually as one UX/authority model, not one unsafe process;
- local/offline Personal operation remains a first-class deployment profile.

## 2. Architecture completeness

- one Core authority plane is preserved;
- current MedScale IDs/provenance/audit/effect primitives are reused rather than duplicated;
- every plane has a defined authority boundary and data movement path;
- network, browser, worker, Hub, model and audio runtimes cannot obtain ambient vault authority;
- canonical artifacts are separated from collaboration events, projections, indexes and receipts;
- external effect `Unknown` semantics remain explicit;
- schema/version/migration/rollback rules exist.

## 3. Implementation completeness of the plan

- repository/crate/module ownership is mapped to current MedScale structure;
- every candidate Spec 074-089 has predecessor/dependency rules;
- every candidate spec has minimum contracts/state/failure/closure shape;
- future promoted specs must use the hardened Future Spec Template;
- implementers have explicit instructions forbidding architecture-by-preference;
- no new crate/service/database/framework is implicitly required without qualification;
- donor reuse is selective and provenance/permission/security governed.

## 4. Security/privacy completeness

- data classes and classification propagation are defined;
- de-identification/pseudonymization boundaries are defined without claiming perfect anonymization;
- Governed Browse includes prompt-injection, egress, credential, redirect/SSRF and download quarantine rules;
- AudioFlow includes visible capture, source/transcript lineage and speaker-identity separation;
- Hub includes tenant/project scope, revocation, conflict/deletion and backup semantics;
- Compute includes least-privilege staged inputs and no ambient vault mount;
- Research Packs cannot bypass Core authority or inject arbitrary trusted-process code by default.

## 5. Verification completeness

- universal acceptance gates exist;
- subsystem-specific gates exist for every major Research OS plane;
- verification layers L0-L9 are defined;
- each candidate spec has a verification campaign or inherits an explicit campaign;
- baseline repository gates are preserved;
- evidence-selected engines/vendors have qualification requirements and safe fallback behavior;
- `SKIPPED`, `NOT_RUN`, `UNAVAILABLE`, `UNKNOWN` and `PARTIAL` cannot become PASS by prose.

## 6. Decision completeness

- architecture questions are either closed by a safe planning default or explicitly evidence-selected;
- each evidence-selected choice has an owner spec and evidence required to change/select it;
- the original Open Questions file is treated as research inventory, not implementer authority;
- no material architecture choice is left for Muse/Codex/Claude/Cursor to choose by taste.

## 7. Consistency/gap audit

Before declaring the planning packet review-ready:

1. compare the planning branch to live `main` and confirm no production/spec authority was modified;
2. verify candidate numbering is internally consistent and unused on live main at that time;
3. search the complete planning diff for stale spec-number references after any roadmap insertion/renumbering;
4. ensure every roadmap unit exists in the Spec Implementation Contracts and Verification Matrix;
5. ensure every cross-plane dependency is represented in roadmap/contracts/repository map;
6. ensure every external data path passes Privacy + capability/network/effect rules;
7. ensure every source/donor mentioned has an adoption/qualification posture;
8. ensure private connected-source information is not disclosed publicly;
9. record unresolved items only when they are genuinely evidence/external decisions with a fail-safe default;
10. update PR body/status to reflect exact planning truth.

## 8. Completion truth

When the criteria above are satisfied, the only permitted claim is:

```text
RESEARCH_OS_PLANNING_PACKET_REVIEW_READY = TRUE
RESEARCH_OS_IMPLEMENTATION_AUTHORIZED = FALSE unless canonical governance separately promotes a unit
RESEARCH_OS_IMPLEMENTED = FALSE
RESEARCH_OS_RELEASE_READY = FALSE
REAL_PHI_AUTHORIZED_BY_THIS_PACKET = FALSE
```

Structural/planning completeness does not prove technical engine choices, implementation, privacy qualification, clinical validity or release readiness.
