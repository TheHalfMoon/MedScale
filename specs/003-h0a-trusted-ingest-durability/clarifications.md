# Clarification Closeout: Spec 003

**Date**: 2026-08-25  
**Command**: `/speckit.clarify` equivalent (autonomous defaults)

No founder questions. Ambiguities resolved via `docs/planning/IMPLEMENTATION_DECISION_DEFAULTS.md`, constitution, SOURCE_ACQUISITION_AND_COPY_PLAN, MASTER_BUILD_PLAN_V2 H0-A, GLM53_RECONCILIATION_V2, and Spec 002 anti-scope. Details live in `research.md`.

| ID | Ambiguity | Resolution |
|---|---|---|
| C1 | Encrypted metadata store in H0-A | **Defer SQLCipher** to Spec 005 (`DEPENDENCY`/`FFI` candidate, not constitutional). Spec 003 uses unencrypted DurableStore candidate (SQLite) behind an interface for synthetic vaults only |
| C2 | SQLite vs pure filesystem metadata | Prefer **SQLite metadata + filesystem blobs** behind `DurableStore`/`BlobStore` traits; smallest reversible path that supports migration journal, single-writer, and restore closure. Pure FS metadata allowed only as a throwaway test double, not the durability claim path |
| C3 | FHIR parser choice | Memory-safe Rust JSON/FHIR structural parse for H0-A (P0). Do not admit hostile native FHIR C parsers into Core Host. Full profile validation may be external oracle/fixture |
| C4 | Duplicate JSON keys | Fail closed: reject or quarantine; never “last key wins” for accepted custody |
| C5 | Decimal / number policy | Preserve exact lexical decimal (string or decimal type); reject unsafe float coercion for FHIR decimals; document exact rule in research at implement pin time |
| C6 | Validator in CI | EvaluationRecord shape + evidence-only invariant required always. Live HL7/HAPI/Inferno runs are optional if fixture oracles cover evidence attachment; never block on network to validator |
| C7 | Blob addressing | Content digest (SHA-256) + size for integrity; blob **locator** is claim-scoped path/key; SourceRecord id remains opaque identity ≠ digest |
| C8 | GC algorithm | Crash-safe **mark → tombstone → sweep**; race test vs promotion mandatory; no reference-counting-only without crash recovery story |
| C9 | Backup format | Versioned manifest + metadata snapshot + blob payload set; synthetic-scope only; no KeyProvider wrapping in 003 |
| C10 | Filesystem claim | Default vault path under local app-data style test root; detect/refuse known sync/remote roots for claim path; durability PASS scoped to tested OS+FS class |
| C11 | Crate split | Prefer `medscale-storage` + thin `medscale-fhir` ingest module/crate when ownership pressure justifies; otherwise modules under `medscale-core` until split is forced. No crate-per-noun explosion |
| C12 | Projection rebuild depth | Stub projection kind + rebuild hook API sufficient for 003; H0-B typed extractors / timeline owned by Spec 004 |
| C13 | Idempotent re-ingest | Same scope + same content digest → typed duplicate/idempotent receipt; do not mutate prior SourceRecord bytes |
| C14 | Migration tool | Explicit versioned schema migrations with journal; interrupt → resume or prior schema; no auto-destructive upgrade |

**Outstanding NEEDS CLARIFICATION markers in spec.md**: none

**External gates**: none opened by Spec 003 planning. REAL_PHI, MESC mutation, product network, models, OpenMed runtime remain unauthorized. Implementation remains blocked solely by Spec 002 closeout.
