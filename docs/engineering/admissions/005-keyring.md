# Dependency admission: keyring (Spec 005)

## keyring-core + platform stores (planning pins)

| Crate | Version | Role |
|---|---|---|
| keyring-core | 1.0.0 (candidate) | API layer |
| windows-native-keyring-store | 1.1.0 | Windows |
| linux-keyutils-keyring-store | 1.0.0 | Linux |
| apple-native-keyring-store | 1.0.2 | Optional macOS |

| Field | Value |
|---|---|
| Owning Spec | 005 / 009 |
| Purpose | Store **wrapped** VaultDEK / unlock token only; no ambient secret API |
| Namespace | `medscale.vault.<vault_id>` |
| Spec 005.1 disposition | **In-process `MemoryKeyStore`** is the admitted default for CI/unit proofs. OS keyring crates are wired behind `KeyStore` trait and may be enabled when platform packages resolve cleanly on CI; never required for encryption correctness. |
| Exit strategy | Swap KeyStore implementation without changing wrap formats |
| Tests | Mock store round-trip; wipe → unlock via passphrase/recovery |
