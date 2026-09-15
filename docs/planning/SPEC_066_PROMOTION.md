# Spec 066 Promotion — Desktop + CLI Hardening

**Current state:** `CLOSED_CANONICAL`
**Promotion:** `POST_SPEC_065_CANONICAL_SEQUENCE`

Spec 065 is `CLOSED_CANONICAL`: final exact head `06be6d0981af11316ddaab28de3f78c70c252be3` passed all six required jobs in run `34933002914`, PR #109 merged normally as `11a9652c6f92eb7e7ade4a5aef18ad9742b8a294`, and post-merge main run `34933737357` passed all six required jobs.

Spec 066 is therefore the next authorized Desktop+CLI product-launch unit. It hardens the native product surface without broadening authority: advertised routes must be real, demo copy must not imply clinical risk, minimum-window behavior is explicit, accessibility text tokens have an engineering contrast floor, and CI removes redundant spec-branch push runs while retaining pull-request and post-merge main qualification.

OpenCodeReview delegation is used as an additional review aid for supported diff types. It does not replace required tests, GitHub required checks, manual Slint/docs review, or canonical governance.

Real PHI, production credentials, MESC work, signing/notarization, qualified-hardware budget attainment, WCAG conformance, and release-readiness claims remain out of scope or evidence-gated.
