# Plan: Spec 056 Fixture A11y Semantics

## Architecture
Contracts-only deepening plus report text; no trusted-core behavior change.

- `medscale-contracts::fixture_ui`: new `FixtureUiRole`, `FixtureViewState`
  (with `canonical_announcement()`), `FixtureA11ySemantics` (with
  `is_honest()`); new `semantics` field on `FixtureUiViewModel` with serde
  default for pre-056 documents; new `has_required_a11y_semantics()`;
  all five constructors assign role + focus order + keyboard path.
- `medscale-contracts::doctor`: new `fixture_state_semantics_checked`
  (serde default) on `AccessibilityDoctorStatus`; set in `ready_base()`;
  required by `is_honest_ready_base()`.
- `medscale-cli`: report line gains `fixture_semantics={}`; fixture test
  asserts semantics honesty.
- `medscale-core/tests/fixture_a11y_semantics_056.rs`: six honesty tests.

## Files
- `crates/medscale-contracts/src/fixture_ui/mod.rs`
- `crates/medscale-contracts/src/doctor/mod.rs`
- `crates/medscale-cli/src/main.rs`
- `crates/medscale-core/tests/fixture_a11y_semantics_056.rs`
- `specs/056-fixture-a11y-semantics/*`, `evidence/056-fixture-a11y-semantics/*`
- `docs/planning/SPEC_056_PROMOTION.md`, `BUILD_QUEUE.md` row

## Test strategy
New integration tests plus existing 029/CLI tests (extended). Full workspace
gates (fmt/clippy/test/deny) must pass; exact-head CI green.
