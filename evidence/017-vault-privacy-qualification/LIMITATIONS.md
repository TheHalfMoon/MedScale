# Spec 017 LIMITATIONS

- `PRIVATE_DATA_READY = FALSE`
- Open-work plaintext residual while unlocked is addressed by Spec **023** (SQLCipher page encryption); see `evidence/023-vault-open-metadata-privacy/`
- Work wipe is best-effort overwrite+unlink, not a measured secure OS deletion claim
- OS swap, hibernation, and volume snapshots are not qualified (still blocks PRIVATE_DATA_READY after Spec 023)
- SQLCipher enabled in Spec 023 via workspace-wide rusqlite sqlcipher features (ADR-023-001); ADR-017-001 historical deferral superseded for that path
- SyntheticVault remains plaintext by design (engineering path)
- OS keyring remains MemoryMock in CI (`KeyStoreAvailability::OsStoreDeferred`)
