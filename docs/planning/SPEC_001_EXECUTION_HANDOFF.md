# Spec 001 — Cursor Execution Handoff

**Spec:** `001-rust-repository-speckit-bootstrap`  
**State:** `READY`  
**Authority:** standing founder authorization in `IMPLEMENTATION_AUTHORITY.md`  
**Medical functionality:** forbidden in this unit

## Objective

Turn the canonical planning repository into a reproducible Rust/Spec Kit engineering repository without changing frozen medical architecture. Materialize the already-closed Spec 000 constitution/source authority into repository governance, establish the minimal workspace and CI/security/evidence skeleton, then prepare Spec 002.

## Required preflight

Before mutation, verify live `main` head/tree, branches, open PRs, repository files, current Cursor rules, and `BUILD_QUEUE.md`. If repository truth moved, reconcile against current canonical docs rather than stale SHAs.

## Required outputs

1. Bootstrap/install GitHub Spec Kit according to current official upstream instructions; record exact upstream revision/tool version and commands.
2. Materialize `.specify/memory/constitution.md` from current canonical founder/frozen rules. Do not reopen or weaken them.
3. Create the complete Spec 001 package under the repository's chosen Spec Kit convention: `spec.md`, clarification closeout, `research.md`, `plan.md`, contracts where needed, `quickstart.md`, checklists, `tasks.md`, implementation handoff.
4. Create a minimal Rust workspace. Start dependency/unsafe-boundary driven; do not create a crate for every noun.
5. Pin Rust toolchain and reproducible dependency lock strategy.
6. Establish formatting/lint/test baseline: rustfmt, clippy with repository-selected deny policy, unit/doc tests as applicable.
7. Establish supply-chain/evidence baseline: cargo-deny, RustSec/OSV policy, cargo-vet policy, cargo-auditable, CycloneDX/SBOM path, secret scanning as appropriate, provenance/evidence directory conventions.
8. Establish CI for supported development platforms needed by the current phase. No hidden network/telemetry product behavior is introduced by CI tooling.
9. Add contribution/commit/PR/evidence conventions and dependency admission template.
10. Add architecture/dependency-direction checks sufficient to keep app/UI/worker code from owning or bypassing trusted contracts as the workspace grows.
11. Create/qualify the complete Spec 002 planning package before closing 001, so execution can continue immediately after merge.
12. Update `BUILD_QUEUE.md` and repository status after canonical merge.

## Candidate initial workspace

Treat this as a minimal starting direction, not permission for crate explosion:

```text
crates/
  medscale-contracts/
  medscale-core/
  medscale-storage/
  medscale-keys/
  medscale-fhir/
  medscale-projections/
  medscale-worker-protocol/
  medscale-worker-host/
  medscale-cli/

apps/
  desktop/
  ios/
  android/

workers/
  models/
  documents/
  voice/
```

Spec 001 may create only the crates/modules actually needed to establish the dependency skeleton; empty speculative crates should be avoided. Spec 002 owns real trusted object/authority semantics.

## Dependency rule

Spec 001 may install/pin development/build dependencies required for bootstrap, tests, CI and supply-chain evidence. Do not preinstall model, OCR, ASR, vector, cloud, FHIR-server, database-server or agent frameworks. Runtime/domain dependencies belong to their owning specs.

## Tests/evidence

At minimum prove:

- workspace builds/tests on the declared development platforms;
- format/lint gates are deterministic;
- lock/toolchain identity is captured;
- dependency/security policy runs and produces archived evidence;
- no product medical/model/network behavior exists yet;
- repository instructions (`AGENTS.md`, Cursor rules, Spec Kit constitution) agree on authority and sequencing;
- the Spec 002 package passes `/speckit.analyze` with no unresolved blocker before 001 closeout.

## Git lifecycle

Use branch `spec/001-rust-repository-speckit-bootstrap` unless live truth already has a canonical eligible branch. Use focused English commits. Open a PR, qualify exact head, repair findings, merge only after required gates pass, verify post-merge `main`, mark 001 `CLOSED_CANONICAL`, mark 002 `READY`, and immediately begin 002.

Do not ask the founder for routine tooling/version/layout choices; apply `IMPLEMENTATION_DECISION_DEFAULTS.md` and record the result in Spec 001 research/plan.