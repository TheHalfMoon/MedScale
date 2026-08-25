# Implementation Plan: Trusted Object / Source / Authority + Process/Text Foundation

**Branch**: `spec/002-trusted-object-source-authority-foundation` | **Date**: 2026-08-25 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/002-trusted-object-source-authority-foundation/spec.md`

## Summary

Expand `medscale-contracts` and `medscale-core` with durable object classes, tagged text coordinates, realm/authority scope, proposal/promotion and effect/retry vocabulary, single-writer Core Host lease semantics (in-process), versioned authority-facade contracts, and native/FFI + worker supervision policy stubs. Prove invariants with property and serialization tests. No H0 ingest, presentation, encryption, models, network, or PHI.

## Technical Context

**Language/Version**: Rust stable per workspace `rust-toolchain.toml` (inherited from Spec 001)

**Primary Dependencies**: Workspace crates only; `serde` / `serde_json` if admitted for envelopes/fixtures; no SQLite, crypto, network, or model crates

**Storage**: None (in-memory objects and lease simulator only; durable store is Spec 003/005)

**Testing**: `cargo test` unit + property/serialization suites in contracts/core; no fuzz corpus required beyond property tests unless hostile parsers appear (they should not in 002)

**Target Platform**: Same as Spec 001 CI/dev matrix (Windows + Linux); topology contracts document mobile pattern without implementing it

**Project Type**: Rust workspace libraries (`medscale-contracts`, `medscale-core`); CLI may expose read-only introspect commands later but is not a product surface requirement for 002

**Performance Goals**: Deterministic in-memory operations; lease acquire/deny instantaneous for simulator scale (single vault, handful of clients)

**Constraints**: Synthetic-only; DEFAULT_DENY network by absence; no PHI; no panic-across-FFI patterns introduced; no canonical DB handles in any API

**Scale/Scope**: Foundation types + facade + policy stubs; future crate splits noted only

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Rust-owned trusted core: all authority semantics in Rust contracts/core
- [x] Local-first / privacy-first: no telemetry, no runtime egress, synthetic-only
- [x] Source/authority discipline: distinct classes; Proposal ≠ Assertion; source ≠ hash
- [x] Evidence-before-claims: tests + evidence archive path defined
- [x] Fail-closed: lease contests, illegal promotions, UNKNOWN retry, incomplete FFI records
- [x] MESC/PHI/network anti-scope honored
- [x] Minimal reversible architecture: modules in existing crates, not crate-per-noun

## Project Structure

### Documentation (this feature)

```text
specs/002-trusted-object-source-authority-foundation/
├── spec.md
├── clarifications.md
├── research.md
├── plan.md
├── data-model.md
├── quickstart.md
├── analyze-notes.md
├── tasks.md
├── contracts/
│   ├── authority-facade.md
│   └── object-classes.md
└── checklists/
    └── requirements.md
```

### Source Code (repository root — implement phase)

```text
crates/
  medscale-contracts/
    src/
      lib.rs
      objects/          # durable class types + IDs
      text/             # spans, coordinate systems
      envelopes/        # schema_versioned messages
      ffi_policy/       # FfiAdmissionRecord types
      worker_policy/    # capability deny lists (types)
  medscale-core/
    src/
      lib.rs
      authority/        # CoreFacade, promotion, capabilities
      process/          # CoreHost lease simulator
      effects/          # effect state transitions
      text/             # conversions RawByte ↔ UnicodeScalar
      validate/         # FFI/worker policy validators
  medscale-cli/         # optional: doctor/object-schema introspect only if needed
evidence/
  002-trusted-object-foundation/
```

**Structure Decision**: Expand Spec 001 crates via modules. Note future splits (`medscale-worker-protocol`, `medscale-ffi`) without creating empty crates in 002.

## Implementation approach

1. Land contract types and schema versions (objects, spans, realm/scope, effect enums).
2. Implement CoreFacade + in-process single-writer lease + promotion/deny paths.
3. Implement text conversion helpers + fail-closed offset checks.
4. Implement effect transition table + UNKNOWN retry denial.
5. Implement FFI admission + worker policy validators.
6. Property/serde tests; archive evidence; converge docs.

## Complexity Tracking

No constitution violations requiring justification.
