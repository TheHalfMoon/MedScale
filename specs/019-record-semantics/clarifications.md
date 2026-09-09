# Clarifications — Spec 019

| ID | Question | Decision |
|---|---|---|
| C01 | New ListIdentityCandidates op? | Prefer library helper `find_unresolved_identity_candidates`; no new facade op in READY_BASE. |
| C02 | Mutate prior assertion on amend? | Never. Append new assertion + AmendmentRecord + audit. |
| C03 | Timezone default? | Optional `timezone_offset_minutes`; absent means unspecified, not UTC invent. |
| C04 | Retraction payload? | New assertion with `retracted:` claim_kind prefix; lineage via AmendmentKind::Retraction. |
| C05 | Advanced deferred? | Plugins/GraphRAG/imaging/CUDA remain 020+ deferred tracks; Spec 020 is FHIR support matrix (Q08). |
