# Clarifications — Spec 061

- “Patient Workspace” is a trusted presentation surface, not a new clinical authority layer.
- Existing `SubjectTimelineV1`, `SubjectBriefV1`, and `SubjectCoverageV1` remain the semantic source for longitudinal presentation.
- The current trusted FHIR extraction surface supports Patient, Observation, and Condition concepts; unsupported MedicationRequest content must not be silently promoted into patient truth.
- Documents and care-plan areas are implemented as honest product states and safe navigation/review entry points where authoritative product capability is not yet available.
- Runtime fixture content remains deterministic synthetic demo data until a later authorized host/vault binding supplies trusted projections.
- No real PHI, MESC, mobile, signing, notarization, or WCAG/release claim is authorized by this spec.
