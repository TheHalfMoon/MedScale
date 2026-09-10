# Spec 043 promotion — swap / snapshot / core-dump honesty (Q03 residual)

**Classification:** `EXISTING_Q03_RESIDUAL_ELIGIBLE_FOR_PROMOTION`  
**Not:** deferred advanced product (`043+` plugins/GraphRAG/CUDA/etc.).  
**Not:** authority conflict.

## Authority

| Source | Binding |
|---|---|
| `EXTERNAL_GATES.md` `OS_KEYRING_SWAP_SNAPSHOT_PRIVATE_DATA` | Explicit residual: Spec 032 probes_present but swap/hibernate/snapshot still open |
| Spec 032 LIMITATIONS | Existence-only; no qualification of swap/hibernate/snapshots |
| `BUILD_QUEUE` next-eligible | Lists swap/snapshot PRIVATE_DATA_READY honesty as Trusted V1 residual |
| Founder continuous loop | Snapshot-existence/protection honesty + Q03 privacy qualification as candidate residual |

## Scope (bounded)

- Classify each residual surface with honest classes:
  - `CONFIGURATION_DETECTED`
  - `PROTECTION_MEASURED` (must remain unused / false for these surfaces in this unit)
  - `NOT_MEASURABLE_ON_HOST`
  - `OWNER_OR_OS_POLICY_REQUIRED`
- Extend probes: distinct swap vs pagefile where OS differs; snapshot existence best-effort; core-dump/crash-dump config detection.
- Doctor fields + evidence; keep `PRIVATE_DATA_READY=false`.
- Do **not** promise universal secure deletion or clear `OS_KEYRING_SWAP_SNAPSHOT_PRIVATE_DATA`.

## Out of scope

- PRIVATE_DATA_READY = true
- Universal secure wipe of pagefile/swap
- Owner OS policy changes
- Advanced product 044+ (plugins/GraphRAG/replicas/imaging/CUDA/…)

## Numbering

Spec **043** is this Q03 residual. Advanced deferred product renumbered **044+**.
