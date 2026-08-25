# MedScale Autonomous Build Queue

**Queue owner:** repository canonical plan  
**Execution agent:** Cursor  
**Rule:** update this file whenever a unit enters or leaves a canonical state.

## Current queue

| Order | Spec | State | Next action |
|---:|---|---|---|
| 000 | Constitution + Source Authority | `CLOSED_CANONICAL` | Planning V2/source authority already canonicalized; Spec 001 materializes it into the Spec Kit repository structure without reopening founder decisions. |
| 001 | Rust Repository + Spec Kit Bootstrap | `CLOSED_CANONICAL` | Spec Kit + Rust workspace + CI/supply-chain bootstrap merged; continue Spec 002. |
| 002 | Trusted Object / Source / Authority + Process/Text Foundation | `CLOSED_CANONICAL` | Object/authority foundation merged; continue Spec 003. |
| 003 | H0-A Trusted Ingest + Durability | `CLOSED_CANONICAL` | Synthetic FHIR R4 ingest + durability merged; continue Spec 004. |
| 004 | H0-B Trusted Presentation + Coverage | `CLOSED_CANONICAL` | Deterministic timeline/Brief/coverage merged; continue Spec 005. |
| 005 | Local Private Vault + Encryption + Recovery | `READY` | Pre-PHI technical qualification. |
| 006 | CLI + Desktop Foundation | `BLOCKED_BY_005` | Trusted Local Longitudinal Record; integrate v0 UI when available. |
| 007 | OpenMed Absorption / Parity Research | `RESEARCH_ELIGIBLE` | May run research in parallel with 005; use exact source acquisition plan. |
| 008 | Local AI Capability Fabric | `BLOCKED_BY_005_006_007` | Offline pack admission + worker confinement. |
| 009 | Mobile iOS + Android | `BLOCKED_BY_005_006` | AI pack features also depend on 008. |
| 010 | Documents + OCR + Voice | `BLOCKED_BY_005_008` | P1 hostile-input workers. |
| 011 | Evidence / Retrieval / Medical Intelligence | `BLOCKED_BY_004_008` | Relevance never authority. |
| 012 | MESC Artifact Integration | `BLOCKED_BY_008_AND_RELEASED_MESC_ARTIFACT` | Artifact import only; never mutate MESC. |
| 013 | FHIR / SMART / Network Broker | `BLOCKED_BY_005_006` | Sole online product egress path. |
| 014 | Controlled Actions / NPHIES | `BLOCKED_BY_013_AND_WORKFLOW_EVIDENCE` | No blind retry. |
| 015 | HF + Online Pack Ecosystem | `BLOCKED_BY_008_013` | HF is distribution, not runtime requirement. |
| 016+ | Advanced work | `DEFERRED_BY_CANONICAL_DESIGN` | Promote only with new evidence/spec authority. |

## Automatic progression

For the first `READY` unit: create/complete its Spec Kit package, analyze it, implement tasks in dependency order, qualify exact head, converge, merge if all required gates pass, mark `CLOSED_CANONICAL`, recompute this queue, and immediately start the next eligible unit.

Do not stop merely because a PR merged, one milestone passed, or an external optional gate exists.