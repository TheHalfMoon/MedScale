# Analyze Notes: Spec 013 FHIR / SMART / Network Broker

**Date**: 2026-08-25  
**Command**: `/speckit.analyze` equivalent (planning package consistency)  
**Package**: `specs/013-fhir-smart-network-broker/`

## Artifacts reviewed

| Artifact | Present |
|---|---|
| spec.md | yes |
| clarifications.md | yes |
| research.md | yes |
| plan.md | yes |
| data-model.md | yes |
| contracts/network-broker.md | yes |
| contracts/fhir-smart-adapters.md | yes |
| contracts/broker-receipts.md | yes |
| quickstart.md | yes |
| checklists/requirements.md | yes |
| tasks.md | yes |
| analyze-notes.md | yes (this file) |

## Alignment with roadmap / constitution

| Gate / invariant | Coverage | Status |
|---|---|---|
| Sole online product egress abstraction | US1, FR-001/013, D1 | OK |
| Explicit destination/purpose/data-class/authorization/receipt | US1–US3, FR-002, data-model | OK |
| Partner FHIR/SMART adapters | US4, FR-007, fhir-smart-adapters.md | OK |
| Bypass tests; no uncontrolled provider client | US1, FR-005, SC-001, T015–T017 | OK |
| FHIR profile/integrity/conformance evidence | US5, FR-009, T023–T027 | OK |
| Fail-closed allowlist | US2, FR-003/004, D2 | OK |
| DEFAULT_DENY except broker | FR-013, C5 | OK |
| Synthetic/fixture; REAL_PHI unauthorized | FR-010, SC-007 | OK |
| No MESC mutation | FR-011 | OK |
| Depends on Spec 005 + 006 CLOSED | Assumptions, T001 | OK |
| ureq/rustls pins | D3, FR-012, T002 | OK |
| Receipts via Spec 002 patterns | US3, FR-006, D5, broker-receipts.md | OK |
| Live partners EXTERNAL_GATES | US4, FR-008, C3 | OK |

## Cross-document consistency checks

1. **SPECKIT_MASTER_ROADMAP 013 row**: partner FHIR/SMART + sole controlled egress; receipts/allowlist/capability; bypass; profile/integrity/conformance; no uncontrolled provider client — all present.
2. **MASTER_BUILD_PLAN §12**: Network Broker destination/purpose/data-class/authorization/receipt; no worker/provider/HF bypass — covered (HF client deferred to 015).
3. **SOURCE_ACQUISITION**: FHIR/SMART REFERENCE_ONLY; Inferno v1.4.2 EXTERNAL_CONFORMANCE — OK.
4. **Clarifications**: Zero NEEDS CLARIFICATION; autonomous defaults (ureq, fail-closed, SMART stub, Spec 002 receipts) — OK.
5. **Tasks**: T001–T033 Spec 013 only; no REAL_PHI flip; no live partner requirement; no 014/015 implementation — OK.
6. **Spec 006 doctor placeholder**: upgraded in FR-015 / T028 — OK.

## Contradictions / gaps

| Item | Severity | Disposition |
|---|---|---|
| Roadmap “partner adapters” vs stub-only | resolved | Interfaces + fixture stubs in 013; live gated |
| Full ExternalActionIntent machine | expected | Spec 014 owns; 013 records attempt/deny only |
| PackAcquire purpose | expected | Reserved; Deny in 013; Spec 015 enables |
| Allowlist persistence format | residual | Non-blocking; simplest vault-scoped config at implement (research D residuals) |
| Async/reqwest later | residual | Re-admit only behind BrokerTransport if 015 proves need |

**Unresolved design blockers inside Spec 013 package: NONE.**

**Implementation readiness blocker: NONE if Spec 005 and Spec 006 remain `CLOSED_CANONICAL`.**

## Entry / exit readiness

```text
ENTRY: Spec 005 CLOSED_CANONICAL + Spec 006 CLOSED_CANONICAL
PACKAGE_STATE: COMPLETE_SPEC_KIT_PACKAGE
ANALYZE_RESULT: PASS_NO_UNRESOLVED_DESIGN_BLOCKERS
ANALYZE_QUALIFICATION: QUALIFIED
IMPLEMENT_BLOCKED_UNTIL: Spec 005+006 must remain CLOSED_CANONICAL
READY: YES
IMPLEMENTATION: AUTHORIZED_TO_START (no Rust in this planning turn)
LIVE_PARTNER_REQUIRED_FOR_CLOSE: NO
REAL_PHI: NOT_AUTHORIZED
MESC_MUTATION: NO
HTTP_CLIENT: ureq-3.4.0-rustls (reqwest/tokio NOT admitted)
```

## Recommendation

**QUALIFIED for implementation.** Proceed with `/speckit.implement` on branch `spec/013-fhir-smart-network-broker` using `tasks.md`. Prefer broker + allowlist + receipts + bypass evidence first, then SMART/FHIR fixture adapters and EvaluationRecord evidence. Do **not** treat missing live partner endpoints, REAL_PHI gate, MESC, or Spec 014/015 work as READY blockers for this unit.
