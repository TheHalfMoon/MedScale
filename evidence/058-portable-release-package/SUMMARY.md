# Spec 058 — Portable Release Package Qualification

## Fresh audit basis

After Spec 057, the release doctor still listed `reproducible_release_package_contents` and `release_package_upgrade_rollback_proof`. Spec 049 explicitly documented that no real package artifacts existed, while the Trusted V1 delivery plan requires package installation/upgrade/rollback.

## Intended closure

Spec 058 provides real unsigned portable package artifacts, deterministic package-assembly proof, fail-closed verification, and install/upgrade/rollback proof on all three CI operating systems.

This does not close signing/provenance, qualified-hardware performance, macOS final-product, final-v0/WCAG, or material-findings gates.
