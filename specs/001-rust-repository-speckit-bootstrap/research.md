# Research: Spec 001 Rust Repository + Spec Kit Bootstrap

**Date**: 2026-08-25  
**Spec**: `001-rust-repository-speckit-bootstrap`

## Decisions

### D1 — Spec Kit pin
- **Decision**: Install `specify-cli` from `git+https://github.com/github/spec-kit.git@v1.0.1`.
- **Resolved commit**: `9118ed15a0ba65053469a94c560ea5d233f75884`.
- **Integration**: `cursor-agent` with PowerShell scripts (`--script ps`).
- **Alternatives**: PyPI floating latest; older 0.16.x tags.
- **Rationale**: Official recommended pin-to-release; latest stable as of 2026-08-25.

### D2 — Rust toolchain
- **Decision**: Pin via `rust-toolchain.toml` to the stable channel matching the development host qualification (1.97.1 recorded in evidence).
- **Alternatives**: nightly; unpinned stable.
- **Rationale**: Reproducible builds; avoid nightly for trusted-core bootstrap.

### D3 — Minimal crate graph
- **Decision**: Create only:
  - `crates/medscale-contracts` — shared bootstrap types/version constants
  - `crates/medscale-core` — authority-facade stub (no medical logic)
  - `crates/medscale-cli` — version/`doctor` skeleton command surface
- **Defer**: storage, keys, fhir, projections, workers, apps until owning specs.
- **Rationale**: Handoff forbids empty speculative crates; Spec 002 owns real object semantics.

### D4 — Supply-chain baseline
- **Decision**:
  - `cargo deny` with `deny.toml` (licenses, advisories, bans)
  - `cargo vet` policy scaffold (`supply-chain/`) initiated
  - CycloneDX SBOM generation path documented and scripted where tooling installs cleanly
  - Evidence directory `evidence/` with per-spec subdirs
  - GitHub Actions + `gitleaks` or equivalent secret scan when available; otherwise document as follow-up gate
- **Alternatives**: defer all supply-chain to later; only Dependabot.
- **Rationale**: Roadmap requires deny/RustSec/OSV/vet/auditable/SBOM path in 001 skeleton.

### D5 — Clippy policy
- **Decision**: `-D warnings` in CI; workspace `Cargo.toml` sets deny for selected pedantic groups gradually; start with warnings-as-errors in CI for bootstrap crates.
- **Rationale**: Fail closed without drowning bootstrap in style debt.

### D6 — Dependency-direction check
- **Decision**: Document allowed edges in `docs/engineering/DEPENDENCY_DIRECTION.md` and add a small `cargo metadata` script check that fails if forbidden reverse edges appear (e.g., contracts → cli).
- **Rationale**: Compile-time ownership pressure without inventing a custom lint framework yet.

### D7 — Edition / license headers
- **Decision**: Edition 2024 if supported by pinned toolchain; otherwise 2021. Workspace license `LicenseRef-MedScale-Proprietary-Pending` or MIT/Apache only if already declared — check README; if undeclared, use `proprietary` placeholder in Cargo and document in EXTERNAL_GATES if public OSS license choice is pending.
- **Resolution**: Use `edition = "2024"` when toolchain supports it; package license field `UNLICENSED` pending founder public license decision recorded as non-blocking for private development.

## Clarifications closed (no founder ask)

| Topic | Default applied |
|---|---|
| Crate names | `medscale-*` prefix per roadmap |
| CI OS | `windows-latest` + `ubuntu-latest` |
| Auditable binaries | Document + optional CI step; not blocking if tool install fails |
| Fuzz | Policy stub only in 001 (`docs/engineering/FUZZ_POLICY.md`) |

## Anti-scope confirmation

No FHIR parsing, storage engines, encryption, models, UI shells, network clients, OpenMed/MESC code, or real PHI fixtures in Spec 001.
