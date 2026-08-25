# Tasks: Rust Repository + Spec Kit Bootstrap

**Input**: Design documents from `/specs/001-rust-repository-speckit-bootstrap/`

**Prerequisites**: plan.md, spec.md, research.md, contracts/, quickstart.md

## Phase 1: Setup (Shared Infrastructure)

- [x] T001 Confirm Spec Kit init (`specify-cli` v1.0.1) and constitution at `.specify/memory/constitution.md`
- [x] T002 [P] Add `.gitignore` for Rust/target, evidence secrets, and agent credential paths
- [x] T003 [P] Create `docs/engineering/` contribution and policy stubs

## Phase 2: Foundational

- [x] T004 Create workspace `Cargo.toml`, `rust-toolchain.toml`, and `Cargo.lock` strategy
- [x] T005 [P] Create `crates/medscale-contracts` with version identity
- [x] T006 [P] Create `crates/medscale-core` authority stub depending on contracts
- [x] T007 Create `crates/medscale-cli` with `--version` and `doctor`
- [x] T008 Add `deny.toml` and initialize `supply-chain/` cargo-vet policy
- [x] T009 [P] Add dependency-direction check script under `scripts/`
- [x] T010 [P] Add GitHub Actions CI workflow (fmt, clippy, test, deny)
- [x] T011 [P] Add CONTRIBUTING + dependency admission template
- [x] T012 Prove `cargo test --workspace` and archive evidence under `evidence/001-bootstrap/`

## Phase 3: User Story 1 — Spec Kit Governance (P1)

- [x] T013 [US1] Ensure Spec 001 package complete (spec/plan/research/tasks/checklists/contracts/quickstart/clarifications)
- [x] T014 [US1] Align README pointers to Spec Kit + engineering docs

## Phase 4: User Story 2 — Rust Workspace (P1)

- [x] T015 [US2] Unit tests for contracts version and core bootstrap report
- [x] T016 [US2] CLI integration smoke: `--version` and `doctor` output contracts

## Phase 5: User Story 3 — CI / Supply-chain (P1)

- [x] T017 [US3] Run cargo-deny and cargo-vet (or document install + policy scaffold evidence)
- [x] T018 [US3] Ensure CI YAML matches local gates

## Phase 6: User Story 4 — Contribution conventions (P2)

- [x] T019 [US4] Finalize `docs/engineering/CONTRIBUTING.md` and `DEPENDENCY_ADMISSION_TEMPLATE.md`
- [x] T020 [US4] Add PR checklist snippet in `.github/PULL_REQUEST_TEMPLATE.md`

## Phase 7: User Story 5 — Spec 002 package (P1)

- [x] T021 [US5] Create complete `specs/002-trusted-object-source-authority-foundation/` package
- [x] T022 [US5] Analyze Spec 002 for blockers; record analyze notes
- [x] T023 [US5] Update `BUILD_QUEUE.md` for post-merge 001 CLOSED / 002 READY (on closeout)

## Phase 8: Polish

- [x] T024 Validate quickstart.md commands
- [x] T025 Converge docs; open PR; merge after gates; verify main

## Notes

- Queue update included in this PR; merge completes canonical closeout.
