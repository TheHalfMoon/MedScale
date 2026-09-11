# SPEC_054_PROMOTION.md — Native / Full Release SBOM Qualification

- Promotion: EXISTING_Q05_RESIDUAL (`release_sbom_native_model_assets` scaffold gap).
- Generator: `medscale-core::release_sbom` (deterministic CycloneDX 1.5) +
  `scripts/generate-release-sbom.ps1` operator path (cargo metadata + live git
  SHAs + 047 native inventory + artifact hashes + offline Pack asset).
- Verifier: parses; requires kind/source/tree/lock/target/rustc/reproducible
  properties; all workspace packages represented; native deps classified;
  artifacts hashed; every component licensed honestly; fails closed on tamper,
  stale digests, missing packages, unclassified natives, missing properties.
- Evidence: `evidence/054-release-sbom/SBOM.cdx.json` (212 components) +
  `SUMMARY.md`; tests `release_sbom_054.rs` (11 tests incl. 7 negatives +
  committed-doc self-consistency + doctor honesty).
- Doctor `release_sbom_qualified=true`; `RELEASE_READY=false`;
  `reproducible_build=unproven`; missing class renamed to
  `release_sbom_signing_provenance` (signing identity external).
- Does NOT claim: RELEASE_READY, reproducible builds, SPDX license choice,
  signing/notarization, real installers, VEX analysis.
