# Plan — Spec 061

1. Close Spec 060 canonically and promote 061 to `IN_PROGRESS`.
2. Add a bounded Desktop patient-workspace view-model that maps trusted presentation semantics to display-only native UI state.
3. Extend reusable Slint components only where behavior/semantics justify reuse.
4. Implement patient header + overview + workspace tab navigation.
5. Implement source-aware timeline and provenance drill-down affordance.
6. Implement labs from trusted Observation-style fixture state with honest unit/coverage semantics.
7. Implement medication, document, and care-plan surfaces with explicit supported/unsupported/empty/review-first states; do not invent authority.
8. Wire shell search/summarize navigation into the Patients route without committing consequential actions.
9. Add accessibility/honesty/regression tests and evidence.
10. Run local fmt/clippy/tests/MSRV/diff checks and native smoke probes.
11. Qualify exact-head required CI, including portable package qualification on Windows/macOS/Linux.
12. Merge without bypass, verify post-merge main, close canonically, and promote Spec 062 only after closure.
