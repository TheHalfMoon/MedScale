# Clarifications — Spec 032

| ID | Question | Resolution |
|---|---|---|
| C01 | Clear PRIVATE_DATA_READY? | No — probes only; swap/hibernate/snapshot/pagefile remain open |
| C02 | Clear OS_KEYRING_SWAP_SNAPSHOT gate? | No — EXTERNAL_GATES stays OPEN |
| C03 | Choose public SPDX for MedScale? | No — `rights_license_decision=false`; PUBLIC_SOURCE_LICENSE_CHOICE pending |
| C04 | Claim perf budgets met? | No — `budgets_claimed_met=false` |
| C05 | Combine Q03 probes + Q05 NOTICE + perf binding? | Yes — one Spec Kit unit as READY_BASE residuals |
| C06 | Deferred next? | **033+** (advanced deferred work) |
