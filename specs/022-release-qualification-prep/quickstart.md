# Quickstart: Spec 022 verify path

```powershell
$env:CARGO_TARGET_DIR = "D:\medscale-target"
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run -p medscale-cli --locked -- doctor
```

Bind exact head (example):

```powershell
git rev-parse HEAD
git rev-parse HEAD^{tree}
Get-FileHash Cargo.lock -Algorithm SHA256
```

Expect doctor `release_qualification.release_ready = false`.
