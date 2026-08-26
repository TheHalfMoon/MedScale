# Feature Specification: MESC Artifact Integration (research prep)

**Branch**: `chore/followon-gates-capability-prep` (prep only)  
**Status**: `BLOCKED_BY_RELEASED_MESC_ARTIFACT` — Spec Kit research package only; no ARTIFACT_IMPORT  
**Input**: Admit immutable MESC artifacts via Pack with exact hashes/rights/SBOM/evaluation. Never copy MESC Python/runtime. Never mutate TheHalfMoon/MESC.

## User Stories (when gate closes)

### US1 Artifact admit (P1)
Operator admits a released MESC artifact by content digest + rights + SBOM + evaluation evidence through Pack path.

### US2 Refuse Python import (P1)
Architecture tests refuse linking/importing MESC Python package, shared DB, or shared keys.

### US3 Exceptional sandboxed service (P2)
Only if owning evidence selects it; separately sandboxed; no ambient authority.

## Anti-scope (now)
Any pack admit; any MESC mutation; any synthetic “fake MESC” artifact presentation as PASS.
