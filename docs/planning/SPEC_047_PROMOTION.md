# Spec 047 promotion — release dry-run + cross-verifier (Q05 residual)

**Classification:** `EXISTING_Q05_RESIDUAL_ELIGIBLE_FOR_PROMOTION`  
**Not:** deferred advanced product (plugins/GraphRAG/replicas/etc.).  
**Not:** claiming `RELEASE_READY` or full release SBOM.

## Authority

| Source | Residual |
|---|---|
| Spec 039 FR-004 / `release-manifest.scaffold.json` | Digests left `null` with note: fill at dry-run time |
| `SIGNING_PROVENANCE_PREP.md` | Explicitly lists “release dry-run (not produced yet)” |
| Spec 046 / doctor missing class | Lock bound; still missing native/model assets + signing verification |
| `TRUSTED_V1_DELIVERY_PLAN` Q05 | Immutable source/tree/lock + evidence artifacts before RELEASE_READY |
| `RELEASE_READY_CHECKLIST` | Source/tree binding PARTIAL; checksums/provenance PARTIAL |

## Scope

- `scripts/release-dry-run.ps1`: bind live `source_sha` / `tree_sha` / `cargo_lock_sha256`, build-environment identity, admitted native-deps inventory honesty; optionally refresh SBOM + checksums; never set `release_ready=true`.
- `scripts/verify-release-manifest.ps1`: cross-check manifest ↔ live git/lock ↔ SBOM lock property ↔ checksum lock digest; fail closed on honesty violations.
- Doctor `release_dry_run_verifier_present=true`; keep `RELEASE_READY=false` and existing missing classes (including `release_sbom_native_model_assets`, signing).

## Non-claims

- No signed/notarized artifacts
- No reproducible binary install package
- No model/Pack weight SBOM completeness
- No branch-protection / license / WCAG closure

## Numbering

Spec **047** is this Q05 residual. Advanced deferred product renumbered **048+**.
