# Plan: Spec 028 OS Keyring Custody

1. Spec Kit package (specify/clarify/plan/research/ADR/tasks/checklist/analyze/converge).
2. Pin `keyring = 3.6.3` workspace dep with platform features; update admission `005-keyring.md`.
3. Implement `OsKeyStore`, `FakeOsKeyStore`, `select_keystore`, `KeyStoreDoctorPosture` in `medscale-keys`.
4. Extend `VaultPrivacyDoctorStatus` with `os_keyring_available` / `os_keyring_used`; `KeyStoreAvailability::OsStoreAvailable`.
5. Wire `medscale-core` doctor + CLI display; keep `private_data_ready=false`.
6. Tests: Memory + Fake always; OsKeyStore fail-soft; doctor honesty 028.
7. Evidence under `evidence/028-os-keyring-custody/`; EXTERNAL_GATES note residual swap/snapshot.
8. BUILD_QUEUE / roadmap / START_HERE → 028 CLOSED; deferred **029+**.
9. Gates: fmt, clippy `-D warnings`, `cargo test --workspace --locked` (`CARGO_TARGET_DIR=D:\medscale-target`).

## Architecture

```text
KeyProvider.store_wrapped_dek / unlock_keystore
        |
        v
   KeyStore trait
     /        \
MemoryKeyStore  OsKeyStore (keyring::Entry)
     |              |
  CI default     service=medscale
  force-env      account=medscale.vault.<id>
                     |
              probe -> doctor.os_keyring_*
                     |
              PRIVATE_DATA_READY = false
              (swap / hibernate / snapshot still open)
```
