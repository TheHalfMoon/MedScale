# Research: Spec 019 Record Semantics (Q06)

## Problem

Trusted V1 review flagged partial dates/updates as insufficient: inventing Instant from year-only values creates false chronological precision; silent identity merge and overwrite-style corrections break provenance honesty.

## Evidence consulted

- Spec 002 MedicalTime / TimePrecision contracts
- Spec 004 presentation ordering rules (preserve precision)
- TRUSTED_V1_DELIVERY_PLAN Q06
- openEHR versioned composition pattern (reference only; do not replace MedScale object model)

## Decisions

1. **Precision bounds**: Year/Month/Day expose inclusive lexical ranges for ordering; timeline sort keys embed precision rank so Year is not Instant.
2. **Upgrade refuse**: `try_with_precision` errors when target rank is finer than current.
3. **Amendments**: Append-only AmendmentRecord; AmendAssertion creates superseding assertion + audit.
4. **Identity**: Candidate detection is advisory; only IdentityMergeDecision unifies subjects.
5. **Missingness**: Six-valued taxonomy; no collapse to null/boolean.

## Rejected

- Mutating ClinicalAssertion in place
- Auto-merge on shared MRN
- Claiming RELEASE_READY from this unit
