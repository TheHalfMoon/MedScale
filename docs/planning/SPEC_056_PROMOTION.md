# SPEC_056_PROMOTION.md - Fixture Accessibility Semantics Deepening

- Promotion: EXISTING_A11Y_RESIDUAL (Spec 029 label honesty deepened).
- Contracts: `FixtureUiRole` / `FixtureViewState` /
  `FixtureA11ySemantics::is_honest` + `semantics` on every view model;
  doctor `fixture_state_semantics_checked=true` required by honesty.
- CLI: report line gains `fixture_semantics={}`; fixture test extended.
- Tests: `fixture_a11y_semantics_056.rs` (6 tests: doctor honesty, six
  states, tamper rejection, constructor semantics, legacy JSON, file
  presence).
- Does NOT claim: WCAG conformance, assistive-technology qualification,
  final v0 UI, or release readiness.
