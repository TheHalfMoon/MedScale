# Implementation Plan: Rust Repository + Spec Kit Bootstrap

**Branch**: `spec/001-rust-repository-speckit-bootstrap` | **Date**: 2026-08-25 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/001-rust-repository-speckit-bootstrap/spec.md`

## Summary

Bootstrap MedScale into a Spec Kit–governed Rust engineering repository: constitution, minimal crates (`contracts` → `core` → `cli`), CI/format/lint/test/supply-chain evidence, contribution conventions, dependency-direction checks, and a complete Spec 002 package. No medical functionality.

## Technical Context

**Language/Version**: Rust stable pinned via `rust-toolchain.toml` (qualify exact version in evidence)

**Primary Dependencies**: None beyond Rust std for bootstrap crates; CI/dev tools: rustfmt, clippy, cargo-deny, cargo-vet

**Storage**: N/A (deferred to Spec 003/005)

**Testing**: `cargo test --workspace`

**Target Platform**: Windows + Linux development/CI for this phase

**Project Type**: Rust workspace (library crates + CLI binary)

**Performance Goals**: N/A beyond instantaneous bootstrap CLI version output

**Constraints**: No product runtime network; no medical/model deps; DEFAULT_DENY egress preserved by absence

**Scale/Scope**: 3 crates + governance/CI/docs; Spec 002 package

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Rust-owned trusted core preserved (stub only; no UI/DB bypass)
- [x] Local-first / privacy-first (no telemetry, no runtime egress)
- [x] Source/authority discipline not violated (no medical objects yet)
- [x] Evidence-before-claims (evidence/ + CI)
- [x] Fail-closed supply-chain and lint gates
- [x] MESC/PHI/network anti-scope honored

## Project Structure

### Documentation (this feature)

```text
specs/001-rust-repository-speckit-bootstrap/
├── plan.md
├── research.md
├── quickstart.md
├── contracts/
├── checklists/
└── tasks.md
```

### Source Code (repository root)

```text
Cargo.toml                 # workspace
Cargo.lock
rust-toolchain.toml
deny.toml
supply-chain/              # cargo-vet
crates/
  medscale-contracts/
  medscale-core/
  medscale-cli/
docs/engineering/          # contribution, dependency direction, fuzz policy
.github/workflows/
evidence/001-bootstrap/
.specify/                  # Spec Kit (already initialized)
specs/002-*/               # next package created before closeout
```

## Implementation approach

1. Keep Spec Kit init artifacts; constitution already written.
2. Create workspace + three crates with version command and core stub API.
3. Add deny/vet/CI/docs/scripts.
4. Qualify locally; archive evidence.
5. Author Spec 002 package; analyze consistency.
6. Update BUILD_QUEUE on merge closeout.

## Complexity Tracking

No constitution violations requiring justification.
