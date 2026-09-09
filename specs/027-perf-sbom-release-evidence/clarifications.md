# Clarifications — Spec 027

| ID | Question | Resolution |
|---|---|---|
| C1 | Use Criterion? | Prefer deterministic harness test; Criterion only if deny-compatible and needed — not required for READY_BASE. |
| C2 | Assert budget pass in CI? | **No.** Assert harness runs and emits numbers only. |
| C3 | Full 10k event/record scale in CI? | READY_BASE may use smaller synthetic scale; document scale vs delivery-plan target; never claim budgets met. |
| C4 | Full CycloneDX tool? | Minimal cargo-metadata scaffold acceptable; document vs release SBOM. |
| C5 | Signed packages? | Out of scope; checksums only. |
| C6 | RELEASE_READY? | Remains FALSE. |
