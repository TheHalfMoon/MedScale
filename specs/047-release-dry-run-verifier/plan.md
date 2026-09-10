# Implementation Plan: Spec 047 Release Dry-Run + Cross-Verifier

## Approach

Extend Q05 release-prep scaffolding without inventing a release package pipeline.

1. Add `scripts/release-dry-run.ps1` writing:
   - updated `evidence/039-release-honesty-packets/release-manifest.scaffold.json` (or Spec 047 evidence copy)
   - `evidence/047-release-dry-run-verifier/native-deps-inventory.json`
   - optional refresh of SBOM + package checksums via existing scripts
2. Add `scripts/verify-release-manifest.ps1` for fail-closed cross-checks.
3. Extend `ReleaseQualificationDoctorStatus` with `release_dry_run_verifier_present`.
4. Wire CLI doctor output + honesty tests.
5. Record SUMMARY/LIMITATIONS evidence.

## Technical notes

- Prefer `git log -1 --format=%T` for tree SHA (PowerShell-safe; avoid `HEAD^{tree}` escaping).
- Never set `release_ready`, `signed`, or `notarized` to true.
- Native inventory is admissions-derived honesty, not a CycloneDX native binary enumeration of installers.

## Test plan

- Unit: doctor honesty for new flag.
- Integration: PowerShell dry-run → verify exit 0 on clean tree (local/CI where pwsh available).
- Existing release_qualification_022 / CLI doctor JSON field presence.
