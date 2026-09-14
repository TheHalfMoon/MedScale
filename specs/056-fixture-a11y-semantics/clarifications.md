# Clarifications: Spec 056

- Semantics are data honesty for shells/operators, not a WCAG claim.
- Pre-056 JSON without `semantics` parses but is not honest (no silent pass).
- Canonical announcement text is exact; tampering fails `is_honest()`.
- MESC not involved.
