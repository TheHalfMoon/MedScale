# Spec 052 research

Spec 026 measured Landlock ABI V1 FS allowlist only.
Spec 044 composition inventory lists `linux_landlock` without net/rlimit.
landlock crate 0.4 exposes `AccessNet::{BindTcp,ConnectTcp}` from ABI V4+.
Ubuntu GitHub runners generally support Landlock net; apply fails closed if not.
`libc` already appears transitively in Cargo.lock; Spec 052 adds direct linux dep
for `setrlimit`/`getrlimit` (DEPENDENCY, no donor copy).
