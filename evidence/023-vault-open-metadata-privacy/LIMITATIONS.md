# Evidence LIMITATIONS — Spec 023

- `PRIVATE_DATA_READY` remains **FALSE**: OS keyring is still MemoryMock; swap/hibernate/snapshots unqualified (`EXTERNAL_GATES` row `OS_KEYRING_SWAP_SNAPSHOT_PRIVATE_DATA`).
- `open_work_plaintext_risk=true` remains for residual OS surfaces even though open-work pages are encrypted (`open_work_page_encrypted=true`).
- SyntheticVault remains plaintext by design (engineering path; unkeyed SQLCipher plaintext-compatible mode).
- Embedded SQLCipher reports `4.6.1 community` (rusqlite bundle); not a claim that acquisition tag v4.17.0 was rebuilt independently.
- AES-GCM `meta.sealed` still used at rest; Spec 023 does not claim a full SQLCipher-only durable format migration beyond open-work pages.
- REAL_PHI unauthorized; no release or multi-client readiness claim.
- WORKER_OS_SANDBOX PLATFORM_QUALIFIED unchanged / still OPEN.
