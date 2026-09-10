# Research — Spec 032

## Decision: probes report existence, never readiness

OS swap/hibernate/pagefile/snapshot qualification for `PRIVATE_DATA_READY` remains an EXTERNAL_GATES measurement problem. Spec 032 ships **detect/report** scaffolding so operators see residual classes honestly. Access-denied on Windows system files is treated as existence-inferred (`Detected`) when that is the platform's typical signal for protected pagefile/hiberfil; otherwise `NotReadable` / `NotApplicable`.

## Decision: reuse Spec 017 leftover/crash helpers

`EncryptedVault::leftover_work_present` and wipe sidecars already cover crash leftovers. Spec 032 exports them through a probe API surface and doctor flags (`vault_leftover_scan_available`, `crash_sidecar_detect_available`) without claiming secure OS deletion.

## Decision: NOTICE inventory from cargo metadata + deny allowlist

Mirror Spec 027 SBOM scaffold style. Inventory lists third-party crate license strings vs `deny.toml` allowlist. MedScale workspace crates remain unpublished / UNLICENSED privately; `rights_license_decision=false` until EXTERNAL_GATES `PUBLIC_SOURCE_LICENSE_CHOICE`.

## Decision: strengthen existing perf harness rather than new harness crate

Extend `perf_harness_027` JSON schema with binding fields (git, rustc, lock digest, fixture identity). Keep `budgets_claimed_met=false`.

## Alternatives rejected

| Alternative | Why rejected |
|---|---|
| Flip PRIVATE_DATA_READY | Gate still OPEN; no measured swap/snapshot proof |
| Choose MIT/Apache for MedScale | EXTERNAL_GATES license choice is founder/counsel |
| Claim budgets met from READY_BASE scale | Delivery-plan 10k/1MiB not exercised as acceptance |
