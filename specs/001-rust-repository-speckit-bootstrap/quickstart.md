# Quickstart: Spec 001 Bootstrap

## Prerequisites

- Rust toolchain via `rust-toolchain.toml` (rustup)
- Optional: `cargo install cargo-deny cargo-vet`

## Spec Kit

Pinned: `specify-cli` from `github/spec-kit@v1.0.1`.

```powershell
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git@v1.0.1
specify version
```

## Build & test

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p medscale-cli -- --version
```

## Supply chain (local)

```powershell
cargo deny check
cargo vet check
```

## Evidence

Archive command outputs under `evidence/001-bootstrap/` with commit SHA, `rustc -vV`, and `Cargo.lock` hash.
