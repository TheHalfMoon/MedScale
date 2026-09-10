# Spec 046 promotion — SBOM lockfile binding (Q05 residual)

**Classification:** `EXISTING_Q05_RESIDUAL_ELIGIBLE_FOR_PROMOTION`

## Authority
Spec 027 SBOM scaffold lacked Cargo.lock digest binding; release missing class includes lock/provenance honesty.

## Scope
- `generate-sbom-scaffold.ps1` embeds `cargo_lock_sha256` (+ optional source/tree).
- Doctor `sbom_lock_bound=true`.
- Still not full release SBOM (`release_sbom_native_model_assets` open); RELEASE_READY=false.

## Numbering
Spec **046** residual; advanced deferred **047+**.
