# Clarification Closeout: Spec 013

**Date**: 2026-08-25  
**Command**: `/speckit.clarify` equivalent (autonomous defaults)

No founder questions. Ambiguities resolved via `IMPLEMENTATION_DECISION_DEFAULTS.md`, constitution, MASTER_BUILD_PLAN_V2 §12, SPECKIT_MASTER_ROADMAP_V2 Spec 013 row, SOURCE_ACQUISITION FHIR/SMART/Inferno rows, Spec 002 EvaluationRecord/ActionAuditRecord patterns, Spec 005+006 CLOSED foundations, and EXTERNAL_GATES. Details live in `research.md`.

| ID | Ambiguity | Resolution |
|---|---|---|
| C1 | HTTP client crate family | **ureq 3.4.0** only product HTTP client, `default-features = false`, features `rustls` + `json` (rustls TLS; no native-tls; no reqwest/tokio in 013) |
| C2 | Allowlist posture | **Fail-closed**. Unknown destinations denied; no transport send on Deny |
| C3 | SMART live vs stub | **Stub/adapter interface + fixture receipts** in Spec 013. Live partner SMART/EHR endpoints stay EXTERNAL_GATES (`PARTNER_EHR_NPHIES_ENDPOINT`, `PRODUCTION_CREDENTIALS`) |
| C4 | Receipt durability | Reuse Spec 002 **ActionAuditRecord** (egress audit/attempt) + **EvaluationRecord** (profile/integrity/conformance, `evidence_only = true`). No new clinical object class for “network truth” |
| C5 | DEFAULT_DENY vs broker | Product posture remains DEFAULT_DENY **except** broker-mediated allowlisted authorized egress |
| C6 | Full action state machine | **Out of scope** for 013 (Spec 014). 013 records attempt/deny outcomes compatible with later binding; no blind retry loops |
| C7 | Uncontrolled provider SDKs | **Forbidden**. No cloud EHR SDK, no HF client, no direct reqwest in CLI/workers |
| C8 | FHIR role | Interchange R4 4.0.1 only; profile/conformance evidence via EvaluationRecord; never canonical DB / ClinicalAssertion auto-create |
| C9 | Inferno / SMART test kit | EXTERNAL_CONFORMANCE / REFERENCE_ONLY evidence; not CI hard-required live run if fixture digests cover shape |
| C10 | REAL_PHI / MESC | Unchanged: NOT_AUTHORIZED / no MESC mutation |
| C11 | Doctor network_broker axis | Upgrade Spec 006 placeholder → implemented status reflecting broker + allowlist posture |
| C12 | Dependency gate | Implement gated on Spec 005 + Spec 006 `CLOSED_CANONICAL` |

**Outstanding NEEDS CLARIFICATION markers in spec.md**: none

**External gates**: REAL_PHI NOT_AUTHORIZED; PRODUCTION_CREDENTIALS NOT_GRANTED; PARTNER_EHR_NPHIES_ENDPOINT NOT_GRANTED (covers live SMART/partner FHIR); MESC mutation NO. Product runtime network DEFAULT_DENY except Network Broker.
