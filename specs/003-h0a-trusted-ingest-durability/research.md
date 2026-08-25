# Research: Spec 003 H0-A Trusted Ingest + Durability

**Date**: 2026-08-25  
**Spec**: `003-h0a-trusted-ingest-durability`

## Decisions

### D1 — Storage stack (smallest reversible; SQLCipher deferred)

- **Decision**: Spec 003 implements durability behind narrow traits:
  - `DurableStore` — metadata, leases coordination with Core Host, migration journal, object indexes
  - `BlobStore` — claim-scoped blob put/get/verify/tombstone/sweep
  - Default candidate backend: **one process-linked SQLite** for metadata + **filesystem blobs** under vault root
  - **SQLCipher is NOT admitted in Spec 003**. Per SOURCE_ACQUISITION_AND_COPY_PLAN, SQLCipher v4.17.0 is `DEPENDENCY`/`FFI` owned by **Spec 005**, candidate not constitutional; durability/key tests mandatory there.
- **Alternatives**: (a) SQLCipher now; (b) pure filesystem JSON metadata; (c) embedded other DB.
- **Rationale**: Decision defaults 7–9; GLM F-10 separates synthetic backup/crash proof (003) from production encryption/recovery (005); OSS matrix “one SQLite copy per process”; avoid encrypting before KeyProvider exists.

### D2 — FHIR interchange pin and authority

- **Decision**: Admit **FHIR R4 4.0.1** JSON resources/Bundles as interchange only. Canonical truth remains MedScale object classes (SourceRecord, ClinicalAssertion, …). No FHIR-as-DB.
- **Alternatives**: Multi-version FHIR matrix in H0-A; FHIR resource rows as primary store.
- **Rationale**: Constitution IV / master plan; roadmap Spec 003 row.

### D3 — Lexical safety policy

- **Decision**:
  - Duplicate keys: **fail closed** (reject/quarantine); no last-wins accept path.
  - Decimals: parse with exact decimal/lexical preservation; **forbid** silent `f64` clinical coercion for FHIR `decimal`.
  - Version: require R4 4.0.1 admission; other versions → typed reject.
  - Size/depth limits: configured maxima; exceed → reject.
- **Alternatives**: serde_json default last-wins; float parsing for convenience.
- **Rationale**: Roadmap exit tests; fail-closed constitution VI; P0 memory-safe parse (GLM F-01).

### D4 — Parser / crate placement

- **Decision**: Implement ingest orchestration in Core Host / `medscale-fhir` (module or crate when justified). Use memory-safe Rust JSON handling with explicit duplicate-key detection (custom parse or hardened decoder). Do **not** link hostile native FHIR validator engines into the Core Host process.
- **Alternatives**: Embed HL7 Java validator in-process; copy HAPI into Rust via FFI as P0.
- **Rationale**: Placement classes — bounded Rust JSON/FHIR structural parse may be P0; complex hostile parsers are P1 workers (later). Validator remains evidence oracle.

### D5 — Validator evidence

- **Decision**: `EvaluationRecord` payloads carry validator identity, profile/version, outcome codes, and raw report digest when available. Fixture oracles satisfy H0-A if live sidecar is unavailable offline. Validators never promote to ClinicalAssertion.
- **Alternatives**: Treat “validator passed” as clinical authority; skip evidence records.
- **Rationale**: Constitution IV — external validators = evidence only; SOURCE_ACQUISITION marks HL7/HAPI/Inferno as REFERENCE/oracle.

### D6 — Blob-first visibility

- **Decision**: Put blob (temp → fsync → atomic rename) before publishing metadata visibility. Reads always `verify(digest, size)`. Corruption → `Quarantined` + Evaluation/Audit evidence. SourceRecord id ≠ digest.
- **Alternatives**: Metadata-first; trust filesystem without digest check.
- **Rationale**: GLM durability bullets; constitution III.

### D7 — GC algorithm

- **Decision**: Crash-safe **mark / tombstone / sweep**. Concurrent promotion must pin or re-mark blobs. Race tests mandatory. No GC of blobs referenced by visible metadata or in-flight promotion.
- **Alternatives**: Refcount only; stop-the-world delete without tombstones.
- **Rationale**: GLM53_RECONCILIATION_V2 durability notes.

### D8 — Projection rebuild hooks

- **Decision**: Provide `RebuildProjection { kind, subject_or_scope, built_from }` producing non-authoritative Projection. Stub kind (e.g. `IngestIndexStub` or `SubjectCustodyStub`) is enough; H0-B extractors deferred to 004.
- **Alternatives**: Full timeline/Brief in 003; skip hooks until 004.
- **Rationale**: Master plan H0-A lists projection rebuild; 004 owns presentation.

### D9 — Backup / restore interface (synthetic scope)

- **Decision**: Versioned `BackupArtifact` = manifest (schema_version, vault_id, object digests, blob digests) + metadata snapshot + blob files. `RestoreVault` enforces restore closure and single-writer. No encryption wrapper, no key-loss UX, no cloud upload.
- **Alternatives**: Defer all backup to 005; ad-hoc folder copy without closure check.
- **Rationale**: Roadmap explicit exit; GLM F-10.

### D10 — Filesystem claim scope

- **Decision**: Test vaults under a local non-sync path. Detect/refuse known remote/sync roots for **claim** paths (warning/refuse). Archive evidence with OS + filesystem class. Unsupported FS → outside PASS claim.
- **Alternatives**: Claim universal durability; ignore sync roots.
- **Rationale**: GLM F-03; master plan doctor/vault policy foreshadowing without implementing Spec 005 vault product UX.

### D11 — Migration interruption

- **Decision**: Numbered migrations with `MigrationJournal` (started/finished). Kill mid-migration → on open, resume idempotent steps or remain on last finished version; never serve torn dual-schema as healthy.
- **Alternatives**: Auto-vacuum destructive upgrades; ignore interrupt testing.
- **Rationale**: Master plan H0-A durability bullet.

### D12 — Identity evidence from ingest

- **Decision**: Emit IdentityAssertion (and optional Proposal) from Patient identifiers; wire Spec 002 merge rules on durable store. No probabilistic auto-merge.
- **Alternatives**: Auto-merge same identifier; skip identity until 004.
- **Rationale**: Roadmap identity tests; constitution identity merge explicit.

### D13 — Dependencies (admission posture)

- **Decision**: Admit only what H0-A requires behind interfaces:
  - SQLite binding (exact crate pin at implement via dependency admission; one copy/process)
  - Existing `serde`/`serde_json` if present — **not** as the sole lexical authority if it cannot detect duplicate keys
  - SHA-256 from audited crypto crate already in workspace or std-compatible
  - **Do not admit** SQLCipher, networking, model runtimes, OpenMed, MESC Python, OCR/ASR, sqlite-vec
- **Rationale**: SOURCE_ACQUISITION + decision defaults; smallest reversible.

### D14 — Fault injection approach

- **Decision**: Test harness with injectable failure points (pre/post fsync, pre/post metadata commit, mid-tombstone, mid-migration) and process-kill scripts where platform allows; in-process fault hooks acceptable if they model the same state machine.
- **Alternatives**: Manual-only crash tests; chaos frameworks early.
- **Rationale**: Deterministic evidence over brand preference (default 10).

## Clarifications closed (no founder ask)

See `clarifications.md` C1–C14.

## Anti-scope confirmation

| Deferred to | Not in Spec 003 |
|---|---|
| 002 | Object/authority foundation (prerequisite; do not reimplement or mutate 002 package) |
| 004 | Timeline, Brief, coverage UI, UCUM product surface, general FHIRPath |
| 005 | SQLCipher/encryption, KeyProvider, key-loss/recovery UX, production backup productization |
| 006 | Desktop/CLI product shell, OS IPC production wiring beyond using 002 contracts |
| 007+ | OpenMed absorption runtime, MESC artifacts, models, OCR/ASR, network broker, NPHIES |
| — | Real PHI, product runtime egress, MESC mutation |

## Evidence expectations at implement time

- `cargo test` suites for ingest lexical, identity, blob integrity, GC race, crash/fault, backup/restore, migration interrupt
- Archive under `evidence/003-h0a-trusted-ingest-durability/` with commit, toolchain, lock digest, platform, **filesystem claim scope**, fixture hashes
- Explicit limitations: synthetic-only; unencrypted store; validator may be fixture oracle
