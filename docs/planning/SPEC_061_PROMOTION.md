# Spec 061 Promotion — Patient Workspace + Longitudinal UX

**Current state:** `CLOSED_CANONICAL`
**Promotion:** `POST_SPEC_060_CANONICAL_SEQUENCE`

Spec 060 is `CLOSED_CANONICAL` on main after its implementation exact-head CI, merge, successful post-merge verification, and canonical metadata closure. Spec 061 is therefore the next authorized Desktop+CLI product-launch unit.

This unit builds a native patient workspace over existing trusted presentation semantics (`SubjectTimelineV1`, `SubjectBriefV1`, `SubjectCoverageV1`, and provenance drill-down). Unsupported medication/document/care-plan capabilities must remain explicit and non-authoritative. Synthetic/permitted fixtures only; no real PHI, MESC work, signing/notarization, WCAG claim, or release-readiness claim.

Local implementation qualification completed: fmt, workspace clippy with `-D warnings`, targeted Spec 061 tests, Desktop tests, full workspace tests, Rust 1.88 MSRV check, headless smoke/perf probes, and diff check passed. Final exact-head `ee9e076fd39d4832c2037b077af18979e428b420` passed all six required jobs in pull-request run `34916956439` (the duplicate push run `34916952740` also passed all six). PR #105 merged without bypass as `1438460e1b965518a9cd09d4289e842c016d83df`; post-merge main run `34917999834` then passed all six required jobs and uploaded portable/runtime evidence for Windows, macOS, and Linux. Spec 061 is therefore `CLOSED_CANONICAL` and Spec 062 is authorized.
