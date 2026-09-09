# Clarifications: Spec 022

| # | Question | Resolution |
|---|---|---|
| C1 | Claim RELEASE_READY after locked CI? | No. Prep only; RELEASE_READY remains FALSE. |
| C2 | Configure branch protection via API? | No. EXTERNAL_GATES owner settings only. |
| C3 | Renumber deferred advanced specs? | Yes: previously 022+ → **023+**. |
| C4 | New release pipeline / signing? | No. Reuse CI + cargo-deny. |
| C5 | macOS CI job? | No. Document unqualified; do not fake qualification. |
