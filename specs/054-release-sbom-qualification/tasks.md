# Tasks: Spec 054 Release SBOM Qualification

- [ ] T1: Add `medscale-core::release_sbom` module (types + deterministic
  generator + verifier + failure codes).
- [ ] T2: Add `release_sbom_qualified` to contracts doctor status; update
  `prep_ready_base()` missing list.
- [ ] T3: Wire core doctor computation + report text.
- [ ] T4: Add `release_sbom_054.rs` integration tests (positive + 7 negatives +
  determinism + doctor honesty).
- [ ] T5: Add `scripts/generate-release-sbom.ps1` operator generator.
- [ ] T6: Generate + commit `evidence/054-release-sbom/SBOM.cdx.json` bound to
  live HEAD/tree/lock; write SUMMARY.md.
- [ ] T7: Docs — `SPEC_054_PROMOTION.md`, BUILD_QUEUE row, START_HERE sync if
  needed.
- [ ] T8: Gates — fmt, clippy, workspace tests, deny; exact-head CI green.
