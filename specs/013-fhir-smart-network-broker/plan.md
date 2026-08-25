# Implementation Plan: FHIR / SMART / Network Broker

**Branch**: `spec/013-fhir-smart-network-broker` | **Date**: 2026-08-25 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/013-fhir-smart-network-broker/spec.md`

## Summary

Introduce the **Network Broker** as MedScale’s sole product online egress abstraction: every call declares destination / purpose / data-class / authorization and yields a durable receipt. Enforce a **fail-closed allowlist**. Pin a minimal **ureq 3.4.0 + rustls** transport behind the broker only. Ship partner **FHIR** and **SMART** adapter interfaces with **stub + fixture receipts**. Attach FHIR profile/integrity/conformance as EvaluationRecord evidence only. Prove bypass impossibility. Synthetic/fixture-only; REAL_PHI unauthorized; live partners EXTERNAL_GATES; no MESC mutation; no Spec 014/015 scope creep.

## Technical Context

**Language/Version**: Rust stable per workspace `rust-toolchain.toml`

**Primary Dependencies** (pins in research.md):

- ureq **3.4.0** (`default-features = false`, features `rustls`, `json`)
- Existing workspace: medscale-contracts, medscale-core, medscale-storage, medscale-fhir, serde/serde_json
- **Not admitted**: reqwest, tokio (013), native-tls product path, cloud EHR SDKs, HF clients

**Storage**: Receipts durable via Spec 002 ActionAuditRecord / EvaluationRecord through Core Host → EncryptedVault / DurableStore path from Spec 005

**Testing**: Bypass/architecture, allowlist deny matrix, receipt durability, SMART/FHIR fixture adapters, profile/integrity evidence, doctor network_broker axis; Windows+Linux as available

**Target Platform**: Windows + Linux CI; no live partner network required

**Project Type**: Rust workspace — add `medscale-network` (or equivalent), extend contracts/core/cli doctor

**Performance Goals**: Correctness, fail-closed deny, deterministic fixture receipts; bounded synthetic fixtures

**Constraints**: Synthetic-only clinical content; DEFAULT_DENY except broker; no MESC mutation; no uncontrolled HTTP client; REAL_PHI unauthorized; live SMART/EHR gated

**Scale/Scope**: Network Broker + FHIR/SMART stub adapters + evidence — not NPHIES actions (014), not HF packs (015), not REAL_PHI

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Rust-owned trusted core: broker owned by Core Host / network crate; clients call facade only
- [x] Local-first / privacy-first: DEFAULT_DENY except allowlisted broker egress; minimum disclosure data-class
- [x] Source/authority discipline: FHIR interchange ≠ canonical DB; EvaluationRecord evidence_only
- [x] Evidence-before-claims: receipts + conformance notes with limitations; no live-partner PASS required
- [x] Fail-closed: unknown destination / purpose / data-class / unauthorized → Deny
- [x] MESC/PHI/network anti-scope honored (PHI gate; MESC no mutation; network only via broker)
- [x] Minimal reversible architecture: BrokerTransport trait; ureq swappable; SMART stub replaceable

## Project Structure

### Documentation (this feature)

```text
specs/013-fhir-smart-network-broker/
├── spec.md
├── clarifications.md
├── research.md
├── plan.md
├── data-model.md
├── quickstart.md
├── analyze-notes.md
├── tasks.md
├── contracts/
│   ├── network-broker.md
│   ├── fhir-smart-adapters.md
│   └── broker-receipts.md
└── checklists/
    └── requirements.md
```

### Source Code (repository root — implement phase only; not this planning package)

```text
crates/
  medscale-network/       # NEW: broker, allowlist, ureq transport, adapters
  medscale-contracts/     # broker request/decision types; allowlist enums
  medscale-core/          # facade capabilities; doctor network_broker axis
  medscale-cli/           # optional doctor surface only (no direct ureq)
  medscale-fhir/          # profile/integrity helpers if needed (no sockets)
docs/engineering/admissions/
  013-network-broker-ureq.md
evidence/
  013-fhir-smart-network-broker/
```

**Structure Decision**: Isolate `ureq` inside `medscale-network`. All other product crates depend on broker APIs only.

## Implementation approach

1. Write ureq/rustls admission **before** dependency lines.
2. Add contracts: NetworkBrokerRequest/Decision, allowlist, adapter traits sketches.
3. Implement broker + fail-closed allowlist + fixture/loopback transport.
4. Persist receipts as ActionAuditRecord (+ EvaluationRecord for profile/conformance).
5. Implement SMART + FHIR stub adapters through broker.
6. Bypass + deny-matrix + receipt + adapter tests.
7. Update doctor network_broker axis; archive evidence.
8. Confirm EXTERNAL_GATES unchanged for live partners / REAL_PHI / MESC.

## Complexity Tracking

No constitution violations. Stub SMART/live-gate split is intentional. Full external-action machine deferred to Spec 014 by design.
