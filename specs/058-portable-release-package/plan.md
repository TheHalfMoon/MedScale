# Plan: Spec 058 Portable Release Package Qualification

## Architecture

- `scripts/build-portable-release-package.ps1`: build/copy real CLI + Desktop binaries, legal/SBOM artifacts and deterministic manifest; emit deterministic no-compression ZIP + SHA-256 sidecar.
- `scripts/verify-portable-release-package.ps1`: fail-closed payload/hash/mandatory-file verification and packaged binary smoke tests.
- `scripts/install-portable-release-package.ps1`: verified versioned-slot install/upgrade/rollback state machine under an explicit install root.
- `scripts/qualify-portable-release-package.ps1`: release build, duplicate deterministic package comparison, baseline/candidate lifecycle proof, machine-readable evidence.
- Existing required three-OS `rust (...)` CI jobs run qualification and upload package/evidence artifacts.
- Release doctor gains `portable_release_package_qualified` and `package_lifecycle_qualified`.

## Evidence semantics

The qualification evidence is source/tree/lock/platform bound and asserts package-assembly reproducibility only. It keeps `release_ready=false`, `signed=false`, `notarized=false`, and `reproducible_binary_build=false`.

## Test strategy

Rust integration tests validate doctor/missing-class honesty and script contracts. Required CI executes the PowerShell lifecycle on Windows/Linux/macOS, plus existing fmt/clippy/workspace/cargo-deny/supply-chain gates.
