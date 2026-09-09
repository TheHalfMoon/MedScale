# Research: Spec 020 FHIR / Interchange Qualification (Q08)

## Problem

Trusted V1 review flagged INTEROPERABILITY_RISK: a narrow FHIR R4 typed-extraction subset can be mistaken for full validation or SMART conformance. Operators need an honest, axis-separated support matrix and loss-aware export so unsupported interchange never disappears silently.

## Evidence consulted

- TRUSTED_V1_DELIVERY_PLAN Q08
- WHOLE_PRODUCT_REVIEW FHIR and interoperability review (lexical/structural/profile/terminology/reference/provenance/clinical)
- Spec 003 lexical gate + Spec 004 Patient/Observation/Condition extractors
- Spec 013 AttachValidatorEvidence path (evidence, not authority)
- HL7 FHIR R4 security note: modeling ≠ authentication/authorization

## Decisions

1. **Seven axes**: lexical, structural, profile, terminology, references, provenance, clinical_interpretation — each with Qualified / Partial / Unsupported / EvidenceOnly.
2. **READY_BASE honesty**: Patient/Observation/Condition structural Partial; lexical Qualified for admitted R4 JSON gate; profile Unsupported; clinical_interpretation Unsupported; references Unsupported; provenance Partial via source citations; terminology Partial only for UCUM subset on Observation.
3. **Validator**: EvidenceOnly / never authority.
4. **Export**: reuse existing `LossClass`; sidecar field-loss list preserves unsupported paths.
5. **No CapabilityStatement server** and no full conformance claim.

## Rejected

- Advertising full FHIR R4 conformance from the closed extractor subset
- Treating validator fixture PASS as MedScale authority
- Inventing profile/terminology qualification without admitted assets
- Claiming RELEASE_READY
