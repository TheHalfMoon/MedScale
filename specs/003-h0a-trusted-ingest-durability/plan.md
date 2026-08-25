# Implementation Plan: H0-A Trusted Ingest + Durability

**Branch**: `spec/003-h0a-trusted-ingest-durability` | **Date**: 2026-08-25 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/003-h0a-trusted-ingest-durability/spec.md`

## Summary

Add durable synthetic FHIR R4 ingest through the Spec 002 authority facade: lexical-safe parse, exact SourceRecord bytes, validator/identity Evaluation/Identity evidence, blob-first canonical visibility, projection rebuild hooks, crash-safe blob GC, migration journal, claim-scoped filesystem layout, and synthetic-scope backup/restore interface proof. Storage backends sit behind narrow interfaces; default candidate is unencrypted SQLite metadata + FS blobs. No SQLCipher, real PHI, models, network product egress, OpenMed/MESC runtime, OCR/ASR, or H0-B presentation.

## Technical Context

**Language/Version**: Rust stable per workspace `rust-toolchain.toml` (from Spec 001+)

**Primary Dependencies**: Spec 002 contracts/core; SQLite candidate behind `DurableStore` (pin at implement via admission); SHA-256; hardened JSON/FHIR structural parse. No SQLCipher, no network, no model crates.

**Storage**: DurableStore (metadata) + BlobStore (claim-scoped files); synthetic vaults only; unencrypted

**Testing**: Unit/integration + fault-injection + race tests; synthetic FHIR fixtures; evidence archive with FS claim scope

**Target Platform**: Same as Spec 001/002 CI/dev matrix (Windows + Linux as available); durability PASS scoped per OS/FS class tested

**Project Type**: Rust workspace libraries — prefer `medscale-storage` / `medscale-fhir` as modules or crates when ownership requires; wire through `medscale-core` facade

**Performance Goals**: Correctness and crash safety over throughput; bounded parse limits; no unbounded ingest buffers

**Constraints**: Synthetic-only; DEFAULT_DENY product network; single-writer Core Host; one SQLite per process; validators = evidence only; SQLCipher deferred to 005

**Scale/Scope**: H0-A custody + durability proofs; not product UI; not production vault crypto

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Rust-owned trusted core: ingest/durability via Core Host / facade
- [x] Local-first / privacy-first: no telemetry, no runtime egress, synthetic-only
- [x] Source/authority discipline: exact bytes; FHIR interchange ≠ canonical DB; Proposal/validator ≠ Assertion
- [x] Evidence-before-claims: FS claim scope + exact-head evidence path defined
- [x] Fail-closed: lexical traps, corrupt blobs, tampered backup, dual writers
- [x] MESC/PHI/network/OpenMed anti-scope honored
- [x] Minimal reversible architecture: traits + SQLite candidate; SQLCipher not constitutional

## Project Structure

### Documentation (this feature)

```text
specs/003-h0a-trusted-ingest-durability/
├── spec.md
├── clarifications.md
├── research.md
├── plan.md
├── data-model.md
├── quickstart.md
├── analyze-notes.md
├── tasks.md
├── contracts/
│   ├── ingest-api.md
│   └── durability-backup.md
└── checklists/
    └── requirements.md
```

### Source Code (repository root — implement phase only; not this planning PR)

```text
crates/
  medscale-contracts/     # ingest/backup envelopes, blob refs, receipts (extend 002)
  medscale-core/          # facade methods: IngestFhir, RebuildProjection, Backup/Restore
  medscale-storage/       # DurableStore + BlobStore + SQLite/FS backends + GC + migration
  medscale-fhir/          # lexical parse, version gate, identity evidence extract (thin)
evidence/
  003-h0a-trusted-ingest-durability/
fixtures/
  synthetic/fhir/r4/      # synthetic-only FHIR fixtures + adversarial lexical set
```

**Structure Decision**: Introduce `medscale-storage` (and thin `medscale-fhir`) when Spec 002 modules would otherwise own durable I/O or hostile-parse boundary. Do not create empty speculative crates beyond justified ownership. Do not modify Spec 001/002 planning packages during 003 planning.

## Implementation approach

1. Define storage traits + claim-scoped FS layout + migration journal.
2. Implement blob put/verify/quarantine + mark/tombstone/sweep GC with race tests.
3. Wire FHIR lexical ingest → SourceRecord + BlobRef + IngestReceipt via facade.
4. Attach validator EvaluationRecord (fixture oracle) + identity evidence.
5. Add projection rebuild hook stub.
6. Backup/restore interface + restore closure + tamper fail-closed.
7. Crash/fault + migration interrupt suites; archive evidence with FS claim scope.

## Complexity Tracking

No constitution violations requiring justification. SQLCipher deferral is intentional and documented (D1).
