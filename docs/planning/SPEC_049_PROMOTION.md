# Spec 049 promotion — package upgrade/rollback dry-run scaffold (Q05 residual)

**Classification:** `EXISTING_Q05_RESIDUAL_ELIGIBLE_FOR_PROMOTION`  
**Not:** deferred advanced product.  
**Not:** claiming `RELEASE_READY` or real installer upgrades.

## Authority

| Source | Residual |
|---|---|
| Doctor missing class `release_package_upgrade_rollback_proof` | Explicit residual after Spec 048 vault-level migration/recovery READY_BASE |
| `RELEASE_READY_CHECKLIST` / Trusted V1 delivery plan | Package install/upgrade/rollback proof required for RELEASE_READY |
| Spec 047 dry-run | Digests bound; no upgrade/rollback simulation yet |

## Scope

- `scripts/verify-package-upgrade-rollback.ps1`: dry-run scaffold comparing baseline vs candidate checksum/manifest identity; fail-closed on honesty (`release_ready` must stay false); document that real installers are absent.
- Doctor `package_upgrade_rollback_scaffold_present=true`; keep missing class until real package artifacts exist OR rename to clearer residual.
- Evidence under `evidence/049-package-upgrade-rollback/`.

## Non-claims

No MSI/MSIX/DMG/deb installers, no signed upgrade channel, no RELEASE_READY.

## Numbering

Spec **049** residual; advanced deferred **050+**.
