# Research: Spec 002 Trusted Object / Source / Authority + Process/Text Foundation

**Date**: 2026-08-25  
**Spec**: `002-trusted-object-source-authority-foundation`

## Decisions

### D1 — Crate placement (modules first)

- **Decision**: Implement object types, text spans, realm/scope IDs, and message envelopes in `medscale-contracts`. Implement authority facade, lease/single-writer simulator, promotion rules, effect transitions, FFI/worker policy validators in `medscale-core` modules (e.g., `objects`, `text`, `authority`, `process`, `ffi_policy`, `worker_policy`).
- **Alternatives**: Immediate crate-per-noun (`medscale-objects`, `medscale-authority`, …); premature `medscale-ffi` crate.
- **Rationale**: Constitution VII / roadmap §5 — avoid crate-per-noun fragmentation; promote crates only under compile-time ownership or unsafe-boundary pressure. Specs 005/008/009 create real FFI/worker/network boundaries later.

### D2 — Core Host topology (single writer)

- **Decision**: One MedScale Core Host per open vault owns the writable canonical metadata connection and (later) active vault keys. Clients are local IPC peers. Per-vault exclusive lease. CLI may spawn/own a transient host when none exists. No DB/key handle transfer across IPC. Spec 002 proves this with an in-process simulator + versioned message contracts.
- **Alternatives**: Multi-writer SQLite; each CLI opens DB directly; UI embeds writable store.
- **Rationale**: Roadmap V2 §4 / GLM F-05 — decided topology; fail closed against dual writers.

### D3 — IPC transport timing

- **Decision**: Spec 002 freezes message shapes, capabilities, deadlines, and peer-role enums. OS transports (Windows named pipe, Unix domain socket) are specified as intended but not required to be production-wired in 002.
- **Alternatives**: Full IPC + peer identity in 002; defer all IPC language to 006.
- **Rationale**: Smallest reversible path that still meets the exit gate (“IPC contracts”) without expanding into Desktop shell work.

### D4 — Serialization

- **Decision**: Rust types + `serde` with JSON fixtures for property/golden tests; every authority-bearing envelope carries `schema_version`. Reject unknown authority-bearing fields fail-closed. Optional later CBOR/bincode for IPC frames under same logical schema.
- **Alternatives**: Protobuf; FlatBuffers; hand-rolled binary only.
- **Rationale**: Decision defaults prefer mature narrow deps and deterministic tests; no REST/gRPC product plane (anti-scope).

### D5 — Source identity vs content hash

- **Decision**: `SourceId` is an opaque assigned identity (vault-scoped). `content_digest` (SHA-256) is mandatory evidence metadata, never equality for source identity. Derived artifacts reference `SourceId` + transform descriptor.
- **Alternatives**: Content-address-only sources; hash-as-primary-key.
- **Rationale**: Constitution III / master plan §5 — `SOURCE_IDENTITY != CONTENT_HASH`.

### D6 — Tagged text coordinates

- **Decision**: `CoordinateSystem` enum at minimum: `RawByte`, `UnicodeScalar`. Conversions to UTF-16 code units / JS string indices / Swift `String.Index` / Kotlin are explicit adapter functions with tests added when those surfaces exist (009/010); Spec 002 provides the tagging model + RawByte↔UnicodeScalar suite.
- **Alternatives**: Untagged integer offsets; grapheme-only coordinates.
- **Rationale**: GLM F-08 / master plan §5 — tagged spans; prefer raw bytes for source drill-down.

### D7 — Proposal / promotion

- **Decision**: Promotion is an explicit facade command requiring an `AuthorityCapability::PromoteProposal` (or equivalent) and producing `ClinicalAssertion` + `AuditRecord` linking `proposal_id`. Scores/confidence on Proposal are informational only.
- **Alternatives**: Auto-promote high-confidence AI output; dual-write Proposal and Assertion.
- **Rationale**: Constitution IV — `Proposal != ClinicalAssertion`; `AI_OUTPUT != AUTHORITY`.

### D8 — Effect / retry vocabulary

- **Decision**: Freeze states `PENDING | SENT | CONFIRMED | FAILED | UNKNOWN`. Transition table in core. `UNKNOWN` requires reconciliation evidence before any retry path. Full durable outbox implementation is Spec 014; 002 owns vocabulary + illegal-transition tests.
- **Alternatives**: Adopt Statig now; invent different state names; allow UNKNOWN retry with backoff.
- **Rationale**: Master plan §12 / constitution VI; Statig deferred per OSS matrix unless complexity demands it.

### D9 — Identity merge

- **Decision**: `IdentityAssertion` records claims; `IdentityMergeDecision` (or equivalent explicit record) is required to unify subject IDs. Matching heuristics may later emit Proposals only—never silent merges.
- **Alternatives**: Probabilistic auto-merge; OpenCR-style automatic linking in core.
- **Rationale**: Master plan §2; OpenMed matrix — MedScale supersedes with explicit merge.

### D10 — Native/FFI hardening contract

- **Decision**: Publish `FfiAdmissionRecord` checklist matching OSS matrix §3: source/revision, build flags, ABI, ownership/freeing, thread affinity, callback/unwind (no panic across `extern "C"`), typed errors, allocator assumptions, arches, placement class P0–P3, fuzz/sanitizer evidence, SBOM, update/exit strategy. Validator fails incomplete records.
- **Alternatives**: Ad-hoc README; treat FFI as sandbox.
- **Rationale**: GLM F-01 — FFI ≠ confinement; placement classes are constitutional for later engines.

### D11 — Worker supervision stubs

- **Decision**: `WorkerSupervisionPolicy` defaults deny: ambient canonical DB, master keys, unrestricted filesystem, network, secrets, authority. Capability grants are explicit, enumerated, and expiring. No OS sandbox calls in 002.
- **Alternatives**: Implement 008S confinement now; skip stubs until 008.
- **Rationale**: Roadmap exit gate requires policy in 002; wiring belongs to 008/010.

### D12 — Time model

- **Decision**: Structured `MedicalTime` (name TBD in types) with optional `effective`, `recorded`, `acquired` instants, each with `TimePrecision` (`Year`…`Instant` + `Unknown`) and optional approximation flag. No implicit ordering across different precision without explicit rules.
- **Alternatives**: Single `DateTime<Utc>` everywhere; stringly FHIR date only.
- **Rationale**: Parity matrix — MedScale supersedes with richer time semantics for 004 timeline honesty.

### D13 — Dependencies

- **Decision**: Prefer std + existing workspace deps. Admit `serde`/`serde_json` if not already present via dependency admission. Do not admit ICU4X, Statig, networking, SQLite, or crypto crates in 002 unless a failing text property suite forces ICU4X behind a narrow trait.
- **Rationale**: Decision defaults 7–9; keep reversible and offline.

## Clarifications closed (no founder ask)

See `clarifications.md` C1–C10.

## Anti-scope confirmation

| Deferred to | Not in Spec 002 |
|---|---|
| 003 | FHIR ingest, blob store durability, validator evidence wiring, backup/restore proofs |
| 004 | Timeline/Brief/coverage presentation, FHIRPath, UCUM product surface |
| 005 | Encryption, KeyProvider, recovery, vault paths |
| 006 | Desktop/CLI product UX, real IPC server, Tauri |
| 008+ | Model packs, real workers, sandbox OS APIs |
| — | Real PHI, product network egress, MESC mutation, OpenMed runtime |

## Evidence expectations at implement time

- `cargo test -p medscale-contracts -p medscale-core` with property/serde suites
- Archive under `evidence/002-trusted-object-foundation/` with commit, toolchain, lock digest, platform
