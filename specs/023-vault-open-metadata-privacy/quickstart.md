# Quickstart: Spec 023

## Verify (Windows)

```powershell
$env:CARGO_TARGET_DIR="D:\medscale-target"
cargo fmt
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test -p medscale-storage --locked vault_open_metadata_023
cargo test -p medscale-core --locked doctor_vault_privacy_023
```

## Expected honesty

- `sqlcipher_enabled=true`
- `open_work_page_encrypted=true`
- `private_data_ready=false`
- Evidence LIMITATIONS lists OS keyring / swap / snapshot gaps

## Notes

SyntheticVault remains engineering plaintext (unkeyed). Do not claim PRIVATE_DATA_READY.
Windows vendored OpenSSL needs Perl on PATH (e.g. Strawberry Perl); CI installs via Chocolatey.
