# Research — Spec 061

## Existing authority surfaces
- `SubjectTimelineV1`: deterministic longitudinal events with assertion/evidence references and medical-time ordering.
- `SubjectBriefV1`: LLM-free identity/vitals/conditions summary plus coverage rollup.
- `SubjectCoverageV1`: closed concept slots with Present/Absent/Unknown/Conflict/IncomparableUnits/UnhealthyEvidence/UnsupportedResourceType states.
- `DrillDownResult`: source id, digest, media type, span/citation, excerpt, and coverage state for provenance inspection.
- `CliSession::{timeline,brief,coverage}` demonstrates the existing authority path through `CoreFacade`; Desktop must preserve the same semantics rather than creating a second authority path.

## Data honesty constraints
- Patient, Observation, and Condition have supported trusted extractors.
- MedicationRequest is not a supported trusted presentation extractor today; medication UI must be explicit about that limitation.
- Document intake/OCR/ASR contracts remain bounded and do not authorize pretending extracted document content is complete clinical truth.
- Care-plan generation is consequential and remains review-first; patient workspace may present state/entry points but not silently commit actions.

## UI direction
Follow `DESIGN.md`: dense clinical rows and sections over nested cards, visible evidence/provenance, quiet missing/conflict states, first-class keyboard navigation, and no chatbot-first shell.
