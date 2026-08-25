# Quickstart: Spec 013 FHIR / SMART / Network Broker

## Prerequisites

- Spec 005 + Spec 006 `CLOSED_CANONICAL`
- Rust toolchain from `rust-toolchain.toml`
- Read: [spec.md](./spec.md), [plan.md](./plan.md), [data-model.md](./data-model.md), [research.md](./research.md)
- Windows and/or Linux environment for CI claims

## Scope reminder

This unit is the **Network Broker + FHIR/SMART stub adapters** only:

- Synthetic/fixture clinical content — REAL_PHI remains EXTERNAL_GATES NOT_AUTHORIZED
- Product runtime DEFAULT_DENY **except** broker-mediated allowlisted egress
- No MESC mutation / OpenMed runtime authority
- No live partner EHR/SMART required (EXTERNAL_GATES)
- No Spec 014 NPHIES / full action state machine
- No Spec 015 HF/pack online client
- HTTP client: **ureq 3.4.0 + rustls** only, isolated in network crate

## After implement (expected commands)

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p medscale-network -p medscale-core -p medscale-contracts
cargo test --workspace
```

Illustrative smokes (names until implement):

```powershell
cargo test -p medscale-network -- allowlist_deny
cargo test -p medscale-network -- broker_receipts
cargo test -p medscale-network -- smart_fixture_adapter
cargo test -p medscale-network -- fhir_fixture_adapter
cargo run -p medscale-cli -- doctor --json
```

## Evidence

Archive under `evidence/013-fhir-smart-network-broker/`:

- `git rev-parse HEAD`
- `rustc -V` / toolchain file
- hash of `Cargo.lock`
- ureq admission digests
- bypass + allowlist deny matrix
- receipt samples (redacted)
- SMART/FHIR fixture receipt digests
- profile/integrity/conformance notes
- limitations: synthetic-only; REAL_PHI unauthorized; live partners gated; no MESC mutation; DEFAULT_DENY except broker

## Design docs map

| Need | Doc |
|---|---|
| Requirements | [spec.md](./spec.md) |
| Decisions | [research.md](./research.md) / [clarifications.md](./clarifications.md) |
| Types | [data-model.md](./data-model.md) |
| Broker API | [contracts/network-broker.md](./contracts/network-broker.md) |
| Adapters | [contracts/fhir-smart-adapters.md](./contracts/fhir-smart-adapters.md) |
| Receipts | [contracts/broker-receipts.md](./contracts/broker-receipts.md) |
| Tasks | [tasks.md](./tasks.md) |
| Analyze | [analyze-notes.md](./analyze-notes.md) |

## Implement gate

Package is **QUALIFIED**. Start `/speckit.implement` when BUILD_QUEUE shows Spec 005+006 `CLOSED_CANONICAL` and Spec 013 `READY`.
