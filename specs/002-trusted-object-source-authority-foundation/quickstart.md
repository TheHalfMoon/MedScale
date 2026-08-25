# Quickstart: Spec 002 Trusted Object / Source / Authority Foundation

## Prerequisites

- Spec 001 workspace builds (`medscale-contracts`, `medscale-core`, `medscale-cli`)
- Rust toolchain from `rust-toolchain.toml`
- Read: [spec.md](./spec.md), [plan.md](./plan.md), [data-model.md](./data-model.md)

## Scope reminder

This unit is **types + authority/process/text foundation** only:

- No FHIR ingest (003)
- No timeline/Brief UI (004)
- No vault encryption (005)
- No models, product network, or real PHI

## After implement (expected commands)

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p medscale-contracts -p medscale-core
cargo test --workspace
```

Focus suites (names illustrative until implement):

```powershell
cargo test -p medscale-contracts object_serde
cargo test -p medscale-core lease_single_writer
cargo test -p medscale-core promote_proposal
cargo test -p medscale-core effect_unknown_no_retry
cargo test -p medscale-core text_span_coords
cargo test -p medscale-core ffi_admission_validate
```

## Evidence

Archive under `evidence/002-trusted-object-foundation/`:

- `git rev-parse HEAD`
- `rustc -vV`
- hash of `Cargo.lock`
- test command transcripts
- explicit limitations (in-process lease only; no OS IPC yet)

## Design docs map

| Need | Doc |
|---|---|
| Requirements | [spec.md](./spec.md) |
| Decisions | [research.md](./research.md) / [clarifications.md](./clarifications.md) |
| Types | [data-model.md](./data-model.md), [contracts/object-classes.md](./contracts/object-classes.md) |
| Facade / lease | [contracts/authority-facade.md](./contracts/authority-facade.md) |
| Tasks | [tasks.md](./tasks.md) |
| Analyze | [analyze-notes.md](./analyze-notes.md) |
