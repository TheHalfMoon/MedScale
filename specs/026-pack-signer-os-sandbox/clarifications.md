# Clarifications — Spec 026

| Question | Resolution |
|---|---|
| Clear WORKER_OS_SANDBOX_PLATFORM_QUALIFIED? | No — remains OPEN until multi-OS measured evidence. |
| Set PlatformQualified on Linux? | No — use ReadyBaseMeasured only. |
| Real model Packs? | No — synthetic/fixture Packs only. |
| Where does ed25519 live? | `medscale-keys` helpers; pack crate verifies. |
| Landlock tests on Windows? | `#[cfg(target_os = "linux")]`; stubs must compile on Windows. |
| Deferred advanced work ID? | **027+** (was 026+). |
