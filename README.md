# MedScale

MedScale is a Rust-owned, local-first, privacy-first medical intelligence platform.

This repository currently contains the **canonical planning system** only. Product implementation has not started.

## Start here

1. Read [`AGENTS.md`](AGENTS.md) for authority and execution rules.
2. Read [`docs/planning/START_HERE.md`](docs/planning/START_HERE.md) for the exact build order.
3. Read the canonical [`Master Build Plan V2`](docs/planning/MASTER_BUILD_PLAN_V2.md).
4. Follow the [`Spec Kit Master Roadmap V2`](docs/planning/SPECKIT_MASTER_ROADMAP_V2.md) in dependency order.

## Current state

```text
PLANNING = CANONICAL_V2
PRIMARY_LANGUAGE = RUST
TRUSTED_CORE_LANGUAGE = RUST
LOCAL_FIRST = CONSTITUTIONAL
PRIVACY_FIRST = CONSTITUTIONAL
NETWORK_EGRESS = DEFAULT_DENY
PRODUCT_IMPLEMENTATION = NOT_STARTED
REAL_PHI = NOT_AUTHORIZED
MESC_MUTATION_FROM_THIS_REPO = NOT_AUTHORIZED
```

## Canonical planning artifacts

- [Master Build Plan V2](docs/planning/MASTER_BUILD_PLAN_V2.md)
- [Spec Kit Master Roadmap V2](docs/planning/SPECKIT_MASTER_ROADMAP_V2.md)
- [Canonical Source Register V2](docs/planning/CANONICAL_SOURCE_REGISTER_V2.md)
- [OSS / Code Absorption Matrix V2](docs/planning/OSS_CODE_ABSORPTION_MATRIX_V2.md)
- [OpenMed Parity / Surpass Matrix V2](docs/planning/OPENMED_PARITY_SURPASS_MATRIX_V2.md)
- [GLM 5.3 Reconciliation V2](docs/planning/GLM53_RECONCILIATION_V2.md)

## Build principle

One MedScale platform, many surfaces. CLI, Desktop, iOS, Android, SDK bindings, and pack distribution must share the same Rust-owned medical semantics, source/provenance model, identity, authority, privacy rules, and pack contracts. No surface may create a second clinical-truth implementation.

OpenMed is a competitive capability floor and strategic donor. MESC is an independent scientific/model producer. Neither becomes MedScale's authority model or trusted product runtime.
