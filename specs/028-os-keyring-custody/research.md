# Research — Spec 028 OS Keyring Custody

## R1: keyring 3.6.3 vs 4.2.0

| Option | rust-version | API | Decision |
|---|---|---|---|
| keyring 3.6.3 | 1.75 | Entry + platform features | **Selected** — matches workspace 1.85 |
| keyring 4.2.0 | 1.88 | keyring-core + store crates | Deferred until rust-version bump |

Admission historically named keyring-core + platform stores (4.x shape). Spec 028 freezes **3.6.3** with equivalent platform coverage via features.

## R2: Linux backend

Prefer `linux-native` (keyutils) over D-Bus Secret Service to avoid requiring a session secret-service daemon in CI. If keyutils is unavailable, probe fails soft.

## R3: Doctor fields

Add explicit booleans rather than overloading only `KeyStoreAvailability`, so operators see both availability and selection. Keep `private_data_ready=false` because swap/hibernate/snapshots remain unqualified (EXTERNAL_GATES).

## R4: Force Memory

`MEDSCALE_FORCE_MEMORY_KEYSTORE=1` lets CI and deterministic tests avoid OS side effects while still compiling OsKeyStore.
