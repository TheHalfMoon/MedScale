# Plan: Spec 022 Release Qualification Prep

1. Spec Kit package (specify/clarify/plan/research/ADR/checklist/tasks/analyze).
2. CI: add `--locked` to clippy and test; keep cargo-deny / supply-chain jobs.
3. Docs: README + CONTRIBUTING verify paths use `--locked`.
4. Contracts: `ReleaseQualificationDoctorStatus` on `DoctorReport`.
5. Core doctor + CLI display + unit/integration honesty tests.
6. Evidence: Q05 expansion + `evidence/022-release-qualification-prep/` SUMMARY/LIMITATIONS/CHECKLIST.
7. EXTERNAL_GATES: `REPO_BRANCH_PROTECTION_REQUIRED_CHECKS` (owner settings).
8. BUILD_QUEUE / SPECKIT_MASTER_ROADMAP_V2 / START_HERE: 022 CLOSED_CANONICAL READY_BASE (prep); deferred advanced **023+**.
9. Gates: `cargo fmt`; clippy `-D warnings`; `cargo test --workspace --locked` with `CARGO_TARGET_DIR=D:\medscale-target`.

## Architecture

- Reuse existing CI and cargo-deny; do not invent a release pipeline.
- Doctor reports prep posture and missing evidence classes; never sets `release_ready=true`.
- Repository settings remain external; Cursor does not call GitHub settings APIs.
