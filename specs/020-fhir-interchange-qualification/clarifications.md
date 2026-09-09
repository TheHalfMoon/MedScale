# Clarifications — Spec 020

| ID | Question | Decision |
|---|---|---|
| C01 | CapabilityStatement resource? | No live server CapabilityStatement. Ship typed `FhirSupportMatrix` via doctor + facade read. |
| C02 | Which resources qualify structurally? | Only Patient, Observation, Condition — matching closed extractors. All others Unsupported for structural axis. |
| C03 | Profile / terminology? | Profile Unsupported. Terminology Partial only for Observation UCUM subset pin; otherwise Unsupported. |
| C04 | Validator authority? | External validator output is evidence only; matrix `validator_is_authority=false`. |
| C05 | Export behavior for unsupported fields? | Document each with `LossClass` (Lossy/Unknown); preserve path inventory; do not invent clinical fill. |
| C06 | Next after close? | Spec 021 minimum lovable workflow (Q07) READY; advanced deferred → 022+. |
| C07 | RELEASE_READY? | Always false for this unit. |
