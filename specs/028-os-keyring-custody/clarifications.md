# Clarifications — Spec 028

| ID | Question | Resolution |
|---|---|---|
| C01 | keyring 3 vs 4? | 3.6.3 (rustc 1.85) |
| C02 | Clear PRIVATE_DATA_READY? | No — swap/snapshot residual |
| C03 | Secret Service required on Linux CI? | No — linux-native keyutils; fail-soft |
| C04 | Force Memory? | `MEDSCALE_FORCE_MEMORY_KEYSTORE=1` |
| C05 | Namespace? | service `medscale`, account `medscale.vault.<vault_id>` |
