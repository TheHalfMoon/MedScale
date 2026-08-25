# MedScale

MedScale is a Rust-owned, local-first, privacy-first medical intelligence platform.

## Cursor: start here

Open the repository in Cursor and tell it to read [`CURSOR.md`](CURSOR.md) and continue MedScale from live repository truth. The repository contains standing founder authorization and an autonomous build queue; Cursor should not need routine founder decisions.

Read order:

1. [`CURSOR.md`](CURSOR.md)
2. [`AGENTS.md`](AGENTS.md)
3. [`docs/planning/START_HERE.md`](docs/planning/START_HERE.md)
4. [`docs/planning/BUILD_QUEUE.md`](docs/planning/BUILD_QUEUE.md)
5. [`docs/planning/MASTER_BUILD_PLAN_V2.md`](docs/planning/MASTER_BUILD_PLAN_V2.md)

## Active state

```text
PLAN = CANONICAL_V2
SPEC_000 = CLOSED_CANONICAL
FIRST_EXECUTABLE_SPEC = 001
CURSOR_AUTONOMOUS_IMPLEMENTATION = AUTHORIZED_WITHIN_PLAN
REAL_PHI = NOT_AUTHORIZED
MESC_MUTATION = NOT_AUTHORIZED
PRODUCT_RUNTIME_EGRESS = DEFAULT_DENY
UI_VISUAL_SOURCE = v0
```

## Engineering bootstrap

- Spec Kit constitution: [`.specify/memory/constitution.md`](.specify/memory/constitution.md)
- Active Spec 001 package: [`specs/001-rust-repository-speckit-bootstrap/`](specs/001-rust-repository-speckit-bootstrap/)
- Contributing: [`docs/engineering/CONTRIBUTING.md`](docs/engineering/CONTRIBUTING.md)
- Dependency direction: [`docs/engineering/DEPENDENCY_DIRECTION.md`](docs/engineering/DEPENDENCY_DIRECTION.md)

```powershell
cargo test --workspace
cargo run -p medscale-cli -- --version
cargo run -p medscale-cli -- doctor
```

## Execution support

- [Cursor execution playbook](docs/planning/CURSOR_EXECUTION_PLAYBOOK.md)
- [Implementation authority](docs/planning/IMPLEMENTATION_AUTHORITY.md)
- [Decision defaults](docs/planning/IMPLEMENTATION_DECISION_DEFAULTS.md)
- [Definition of done](docs/planning/DEFINITION_OF_DONE.md)
- [External gates](docs/planning/EXTERNAL_GATES.md)
- [Source acquisition and copy plan](docs/planning/SOURCE_ACQUISITION_AND_COPY_PLAN.md)
- [v0 UI integration contract](docs/planning/V0_UI_INTEGRATION_CONTRACT.md)

## Canonical architecture/planning

- [Master Build Plan V2](docs/planning/MASTER_BUILD_PLAN_V2.md)
- [Spec Kit Master Roadmap V2](docs/planning/SPECKIT_MASTER_ROADMAP_V2.md)
- [Canonical Source Register V2](docs/planning/CANONICAL_SOURCE_REGISTER_V2.md)
- [OSS / Code Absorption Matrix V2](docs/planning/OSS_CODE_ABSORPTION_MATRIX_V2.md)
- [OpenMed Parity / Surpass Matrix V2](docs/planning/OPENMED_PARITY_SURPASS_MATRIX_V2.md)
- [GLM 5.3 Reconciliation V2](docs/planning/GLM53_RECONCILIATION_V2.md)

One platform, many surfaces: CLI, Desktop, iOS, Android and SDK bindings share the same Rust-owned source/provenance, identity, time, authority, privacy and Pack semantics. OpenMed is the capability floor/strategic donor. MESC is an independent scientific producer. Neither owns MedScale clinical truth.