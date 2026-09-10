# Clarifications — Spec 047

| ID | Question | Decision |
|---|---|---|
| Q1 | Overwrite Spec 039 scaffold in place? | Yes for digests + build_environment; keep schema_version 1; note Spec 047 dry-run. Also copy snapshot under evidence/047. |
| Q2 | Does native inventory clear `release_sbom_native_model_assets`? | No — model/Pack assets still missing; inventory is admissions honesty only. |
| Q3 | Can verifier claim RELEASE_READY when all digests match? | No — matching unsigned scaffolds ≠ release qualification. |
| Q4 | Refresh SBOM/checksums during dry-run? | Default yes (`-RefreshScaffolds`); verifier requires consistency. |
