# Clarifications: Spec 023

| # | Question | Resolution |
|---|---|---|
| C1 | Isolation crate vs workspace-wide SQLCipher? | Workspace-wide SQLCipher; SyntheticVault unkeyed plaintext-compatible. |
| C2 | Drop AES-GCM `meta.sealed`? | No. Keep seal+wipe lifecycle; SQLCipher covers open-work pages. |
| C3 | Set PRIVATE_DATA_READY? | No. MemoryMock keyring + swap/snapshot unqualified. |
| C4 | Second vault API? | No. Migrate EncryptedVault path only. |
| C5 | `open_work_plaintext_risk` after page encryption? | Keep `true` for OS residual; add `open_work_page_encrypted=true`. |
| C6 | Clear REAL_PHI / WORKER_OS_SANDBOX? | No. EXTERNAL_GATES unchanged for those; note OS key/snapshot for PRIVATE_DATA_READY. |
| C7 | Renumber deferred advanced? | Yes: previously 023+ → **024+**. |
