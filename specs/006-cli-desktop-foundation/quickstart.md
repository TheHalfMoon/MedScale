# Quickstart: Spec 006 CLI + Desktop Foundation

## Prerequisites

- Spec 005 `CLOSED_CANONICAL` (EncryptedVault, KeyProvider, claim paths)
- Spec 004 presentation capabilities available via Core Host
- Rust toolchain from `rust-toolchain.toml`
- Read: [spec.md](./spec.md), [plan.md](./plan.md), [data-model.md](./data-model.md), [research.md](./research.md)
- Windows and/or Linux environment for CLI CI claims

## Scope reminder

This unit is the **first product wedge** (CLI + Desktop foundation) only:

- Synthetic data only — REAL_PHI remains EXTERNAL_GATES NOT_AUTHORIZED
- No product runtime network (DEFAULT_DENY)
- No MESC mutation / OpenMed runtime
- **No Tauri/WebView admission** (deferred; PRIVACY_PROOF limitations)
- No inventing final v0 visual design if `imports/v0/` absent
- No models/packs/OCR/ASR/mobile product shells

## After implement (expected commands)

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p medscale-cli -p medscale-core -p medscale-contracts -p medscale-desktop
cargo test --workspace
```

CLI smoke (names illustrative until implement):

```powershell
cargo run -p medscale-cli -- --version
cargo run -p medscale-cli -- doctor
cargo run -p medscale-cli -- doctor --json
# vault create/open + ingest + presentation — see tasks / evidence scripts
cargo run -p medscale-desktop -- --smoke
```

## Evidence

Archive under `evidence/006-cli-desktop-foundation/`:

- `git rev-parse HEAD`
- `rustc -V` / toolchain file
- hash of `Cargo.lock`
- clap/anyhow admission digests
- doctor sample output (redacted/synthetic)
- longitudinal wedge transcript
- `PRIVACY_PROOF` artifact
- explicit limitations: synthetic-only; REAL_PHI unauthorized; no product network; Tauri deferred; v0 deferred if absent

## Design docs map

| Need | Doc |
|---|---|
| Requirements | [spec.md](./spec.md) |
| Decisions | [research.md](./research.md) / [clarifications.md](./clarifications.md) |
| Types | [data-model.md](./data-model.md) |
| CLI authority | [contracts/cli-authority.md](./contracts/cli-authority.md) |
| Doctor | [contracts/doctor-report.md](./contracts/doctor-report.md) |
| Privacy proof | [contracts/privacy-proof.md](./contracts/privacy-proof.md) |
| Tasks | [tasks.md](./tasks.md) |
| Analyze | [analyze-notes.md](./analyze-notes.md) |

## Implement gate

Package is **QUALIFIED**. Start `/speckit.implement` when BUILD_QUEUE shows Spec 005 `CLOSED_CANONICAL` and Spec 006 `READY`.
