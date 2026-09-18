# Spec 074 Promotion — Project + Artifact Graph Foundation

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`  
**Founder promotion date:** 2026-09-17  
**Canonical base:** `a80c33307afc4577790282652e5b20911beb4bbe`  
**Target branch:** `spec/074-project-artifact-graph-foundation`

## Authority

The founder explicitly accepted the MedScale Research OS planning direction and instructed the project to proceed and prepare Muse to start building. This document promotes **only Spec 074 — Project + Artifact Graph Foundation** for implementation.

Research OS planning PR #121 was merged normally into `main` as `a80c33307afc4577790282652e5b20911beb4bbe` after exact-head CI run `35203360771` completed successfully across all six jobs.

Standing implementation authority in `IMPLEMENTATION_AUTHORITY.md` remains active. Normal branch/commit/push/PR/merge authority applies only within the promoted scope and only after real required gates pass.

## Authorized scope

Spec 074 may implement only the minimum durable substrate required to organize existing MedScale work into Projects and Experiments without creating a second authority system:

- Project lifecycle;
- Experiment lifecycle;
- generic artifact descriptors that reference existing canonical objects rather than copying them;
- Project-to-artifact attachment references;
- typed Project Graph edges;
- Project context resolution;
- encrypted local persistence and migration;
- Core authority paths;
- CLI project/artifact inspection and mutation paths;
- native Desktop Projects workspace;
- migration, recovery, compatibility, scale, security, accessibility and exact-head evidence required for closure.

## Explicitly not authorized

This promotion does **not** authorize implementation of:

- Spec 075 Collaboration Substrate;
- Spec 076 MedAgent;
- Spec 077 Fleet/Compare;
- Spec 078 Privacy Gate expansion;
- Spec 079 Governed Browse;
- Spec 080 AudioFlow;
- Spec 081 Analytics Gate;
- Spec 082 Knowledge/RAG/Research Canvas;
- Spec 083 MedScale Hub;
- Specs 084-089;
- real PHI;
- MESC work;
- new cloud/runtime network authority;
- donor wholesale copying;
- a new ID, provenance, audit or authority foundation when current MedScale primitives are sufficient.

## Mandatory architecture constraints

1. Reuse current MedScale `OpaqueId`, `ObjectHeader`, digest/provenance, audit, vault, migration and Core patterns where applicable.
2. Existing patient/FHIR/source/document/evidence/model/Pack objects remain canonical in their current owning systems. Project membership references them by stable identity; it does not duplicate or flatten them.
3. `ProjectGraphEdge` authority is limited to explicit typed organizational relationships created through Core. Search/vector/inferred relationships remain projections/evidence and are not part of Spec 074.
4. CLI and Desktop must use the same Core command/query semantics.
5. Local/offline operation is mandatory. Spec 074 adds no server requirement.
6. Archive/tombstone is the initial deletion posture. Destructive deletion is not authorized by this spec.
7. Migration must preserve all pre-074 workflows and object identities.
8. No later Research OS unit may be started merely because its planning document exists.

## Implementation order

```text
074-A Contracts and invariant tests
074-B Storage schema, migration, crash/reopen/recovery tests
074-C Core project lifecycle + Project Graph authority
074-D CLI vertical slice
074-E Native Desktop Projects workspace
074-F Compatibility + scale + failure/security + exact-head closure evidence
```

A later slice may not paper over a failed earlier invariant.

## Completion rule

Spec 074 becomes `CLOSED_CANONICAL` only when all acceptance criteria in the owning Spec 074 package are proven on the exact reviewed head, required CI is green, the PR is merged normally, and post-merge main verification is recorded.

Closure of 074 does not itself authorize 075. After closure, live governance must recompute the next eligible unit and explicitly promote it.