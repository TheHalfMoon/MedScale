# Plan: Spec 059 Final Release Closure Audit

## Architecture / evidence

- Add `material_findings_clearance` to release doctor honesty and remove only `unresolved_material_findings_clearance` after bounded audit evidence.
- Add a regression test that requires the remaining doctor release residual set to equal the five external classes exactly.
- Synchronize the living release checklist, signing packet, external gates, queue, roadmap, and entry-point status.
- Preserve historical evidence as historical rather than rewriting closure-time facts.
- Record Spec 058 merged/post-merge evidence and Spec 059 audit inputs.

## Qualification

Local fmt/clippy/workspace tests plus existing required GitHub checks. First exact-head CI qualifies the audit candidate. A separate closure metadata commit records the run and terminal implementation status, then must pass exact-head CI again before merge. Post-merge main verification is mandatory.
