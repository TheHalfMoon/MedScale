# Quickstart: Spec 003 H0-A Trusted Ingest + Durability

## Prerequisites

- Spec 002 `CLOSED_CANONICAL` (object classes, Core Host lease, authority facade)
- Spec 001 workspace builds (`medscale-contracts`, `medscale-core`, …)
- Rust toolchain from `rust-toolchain.toml`
- Read: [spec.md](./spec.md), [plan.md](./plan.md), [data-model.md](./data-model.md), [research.md](./research.md)

## Scope reminder

This unit is **synthetic FHIR R4 trusted custody + durability** only:

- No real PHI
- No H0-B timeline/Brief UI (004)
- No SQLCipher / KeyProvider / production recovery UX (005)
- No models, product network, OpenMed/MESC runtime, OCR/ASR

## After implement (expected commands)

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p medscale-storage -p medscale-fhir -p medscale-core -p medscale-contracts
cargo test --workspace
```

Focus suites (names illustrative until implement):

```powershell
cargo test -p medscale-fhir lexical_fhir_ingest
cargo test -p medscale-fhir duplicate_key_reject
cargo test -p medscale-core ingest_identity_no_silent_merge
cargo test -p medscale-storage blob_digest_verify
cargo test -p medscale-storage gc_promotion_race
cargo test -p medscale-storage crash_fault_ingest
cargo test -p medscale-storage backup_restore_closure
cargo test -p medscale-storage migration_interrupt
```

## Evidence

Archive under `evidence/003-h0a-trusted-ingest-durability/`:

- `git rev-parse HEAD`
- `rustc -V` / toolchain file
- hash of `Cargo.lock`
- fixture corpus hashes (synthetic FHIR + adversarial)
- test command transcripts
- **FilesystemClaimScope** (OS + filesystem class + vault root policy)
- explicit limitations (synthetic-only; unencrypted store; validator may be fixture oracle; SQLCipher not used)

## Design docs map

| Need | Doc |
|---|---|
| Requirements | [spec.md](./spec.md) |
| Decisions | [research.md](./research.md) / [clarifications.md](./clarifications.md) |
| Types | [data-model.md](./data-model.md) |
| Ingest API | [contracts/ingest-api.md](./contracts/ingest-api.md) |
| Durability / backup | [contracts/durability-backup.md](./contracts/durability-backup.md) |
| Tasks | [tasks.md](./tasks.md) |
| Analyze | [analyze-notes.md](./analyze-notes.md) |

## Implement gate

Do **not** start `/speckit.implement` until BUILD_QUEUE shows Spec 002 `CLOSED_CANONICAL` and Spec 003 `READY`.
