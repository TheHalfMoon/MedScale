# Clarification Closeout: Spec 002

**Date**: 2026-08-25  
**Command**: `/speckit.clarify` equivalent (autonomous defaults)

No founder questions. Ambiguities resolved via `docs/planning/IMPLEMENTATION_DECISION_DEFAULTS.md`, constitution, and Spec Kit roadmap V2. Details and alternatives live in `research.md`.

| ID | Ambiguity | Resolution |
|---|---|---|
| C1 | Crate split breadth for objects/topology | Keep types in `medscale-contracts` and semantics in `medscale-core` modules; defer `medscale-ffi` / `medscale-worker-*` crates until owning specs create real boundaries |
| C2 | Real OS IPC in Spec 002 | Contract + in-process host/lease simulator only; named pipe / UDS wiring owned by Spec 006 (surfaces) using these contracts |
| C3 | Serialization format | Versioned binary-friendly Rust types with `serde` JSON for tests/fixtures; schema version field on envelopes; no protobuf/gRPC product plane |
| C4 | Unicode library | Minimal native conversion tests first; admit ICU4X only if RawByte↔UnicodeScalar property suite needs it (narrow interface) |
| C5 | Effect state machine library | Hand-written explicit enum + transition table; defer Statig unless complexity justifies |
| C6 | ID generation | Opaque ULID/UUIDv7-style time-sortable IDs as strings in contracts; exact crate pin at implement time via dependency admission |
| C7 | Clock source | Injectable `Clock` trait in core for deterministic tests; wall clock adapter later; no network time |
| C8 | Worker supervision depth | Policy stubs + capability deny lists only; no OS sandbox APIs in 002 |
| C9 | Mobile Core Host | Record app-owned in-process Rust core pattern in contracts/docs; no mobile code in 002 |
| C10 | Content digest algorithm | SHA-256 as evidence digest default; never source identity |

**Outstanding NEEDS CLARIFICATION markers in spec.md**: none

**External gates**: none opened by Spec 002. REAL_PHI, MESC mutation, product network, models remain unauthorized.
