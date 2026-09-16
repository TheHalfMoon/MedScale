# MESC Project Separation Decision

Date: 2026-09-16
Status: CANONICAL
Authority: Founder correction

## Decision

`TheHalfMoon/MESC` is a separate project/repository from `TheHalfMoon/MedScale`. MedScale must not treat MESC as a component, optional lane, external gate, release residual, completion axis, or repository-owned work item. Work on MESC belongs in the MESC repository under its own governance.

Historical MedScale Specs 012 and 036, their tests, evidence, and dormant fail-closed compatibility contracts are retained as interoperability/provenance history only. Their presence does not authorize current MESC integration work and does not affect MedScale implementation completion or release qualification. Current MedScale product surfaces must not present MESC as a MedScale integration.

Any future MedScale-to-MESC interoperability requires a new explicit founder decision and a separately promoted MedScale specification. That future work must define a stable cross-project contract and must not silently convert MESC repository state into MedScale authority.

## Immediate governance effect

- Exclude MESC from the current MedScale `BUILD_QUEUE` execution frontier.
- Exclude MESC from `EXTERNAL_GATES.md` and release-residual accounting.
- Exclude MESC-specific flags from `PROJECT_COMPLETION_STATUS.md`.
- Preserve historical Specs 012/036 and related evidence as historical provenance rather than current product scope.
- Evaluate Spec 072 and final MedScale repository closure without any MESC dependency, gate, or completion condition.
