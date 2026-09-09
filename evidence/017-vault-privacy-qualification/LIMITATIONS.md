# Spec 017 LIMITATIONS

- `PRIVATE_DATA_READY = FALSE`
- EncryptedVault still materializes plaintext `meta.work.sqlite3` while unlocked
- Work wipe is best-effort overwrite+unlink, not a measured secure OS deletion claim
- OS swap, hibernation, and volume snapshots are not qualified
- SQLCipher not enabled (ADR-017-001); crate isolation still required
- SyntheticVault remains plaintext by design (engineering path)
- OS keyring remains MemoryMock in CI (`KeyStoreAvailability::OsStoreDeferred`)
