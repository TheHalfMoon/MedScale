# Spec 052 quickstart

On Linux:

```bash
cargo test -p medscale-contracts --test os_sandbox_052 --locked
cargo run -p medscale-contracts --bin medscale-os-sandbox-probe -- landlock-composition
```

Expect exit 0 when FS deny + TCP deny + rlimit measured.
