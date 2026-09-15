# Spec 061 Promotion — Patient Workspace + Longitudinal UX

**Current state:** `IN_REVIEW`
**Promotion:** `POST_SPEC_060_CANONICAL_SEQUENCE`

Spec 060 is `CLOSED_CANONICAL` on main after its implementation exact-head CI, merge, successful post-merge verification, and canonical metadata closure. Spec 061 is therefore the next authorized Desktop+CLI product-launch unit.

This unit builds a native patient workspace over existing trusted presentation semantics (`SubjectTimelineV1`, `SubjectBriefV1`, `SubjectCoverageV1`, and provenance drill-down). Unsupported medication/document/care-plan capabilities must remain explicit and non-authoritative. Synthetic/permitted fixtures only; no real PHI, MESC work, signing/notarization, WCAG claim, or release-readiness claim.

Local implementation qualification is complete: fmt, workspace clippy with `-D warnings`, targeted Spec 061 tests, Desktop tests, full workspace tests, Rust 1.88 MSRV check, headless smoke/perf probes, and diff check all passed. Exact-head required CI, merge, post-merge verification, and canonical closure remain pending.
