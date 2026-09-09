# Trusted-record follow-on: implementation preparation

Status: READY_FOR_SPECIFICATION_REVIEW; runtime implementation NOT_QUALIFIED.
This bounded planning package refines V2 and preserves previous scoped closures. It does
not promote unrelated advanced capabilities. Its first implementation unit is durable trusted
record state (Q02), with Q03/Q04 interfaces designed before storage choices are frozen.

## Specification

User story: a synthetic-record user imports a supported FHIR fixture, reviews sources and
derived assertions, closes the process and reopens the same vault with identical source bytes,
stable identifiers, provenance and deterministic presentation. Rejected writes must not leave
partly visible canonical state. The same domain facade serves CLI and later desktop clients.

In scope: durable source/object/assertion/evaluation/audit state; schema versions and migration;
transactional promotion; restart/recovery; stable errors and source fidelity. Persist action
state through the same transaction boundary if touched, but keep external effects disabled.
Out of scope: live PHI, model runtime, network adapters, mobile apps, clinical interpretation,
new final UI, sync, changing upstream MESC or enabling deferred integrations.

Invariants: immutable original bytes; content digest validated on read; unique stable IDs;
realm/scope partition enforced on every lookup; source/proposal/assertion/evaluation distinctions;
atomic canonical promotion with audit; failure cannot silently reset a vault. Schema/record
versions are separate from clinical effective time. A correction supersedes with lineage;
it never overwrites source history. Time precision and unknown values must survive persistence.

## Design and decisions to resolve in the owning numbered spec

1. Inventory every InMemoryAuthorityStore field and facade operation; map each to durable
   representation, uniqueness/foreign-key constraints, scoped indexes and transaction boundary.
2. Retain one SQLite dependency family. Qualify metadata encryption against Spec 005's
   sealed-at-close amendment before promising privacy. Compare supported encrypted backend
   and application-encrypted records for query, migration, recovery and exposed metadata.
   Record a concrete ADR and dependency/build evidence; neither option is preapproved here.
3. Specify blob/metadata commit protocol: stage and verify blob, commit metadata reference,
   finalize visibility; after interruption distinguish recoverable staging from missing committed
   content. Document fsync/rename assumptions per OS and garbage collection reachability.
4. Define store trait and error taxonomy without a parallel authority service. Configure explicit
   vault root. Minimum OS-exclusive cross-process writer ownership is included in Q02 storage
   qualification; Q04 adds authenticated client identity and scoped session capabilities before
   multi-client release. Do not substitute lock-file presence for exclusive ownership.
5. Version backup manifest and validate complete object/blob closure before restore; restore into
   a fresh destination, validate, then switch. Wrong key/corruption must preserve existing data.

## Ordered tasks and acceptance

| Task | Dependencies | Deliverable / acceptance evidence |
|---|---|---|
| T01 scope/clarify | none | Owning numbered follow-on spec, requirement IDs, assumptions and unresolved choices; no contradiction with constitution or 005 amendment. |
| T02 schema/ADR | T01 | Field-by-field mapping, transactions, ID strategy, migration and encryption decision; reviewer checks all facade state represented. |
| T03 contracts/fixtures | T02 | Synthetic fixture hashes; expected object/provenance snapshots; stable typed error cases; trace each requirement to a check. |
| T04 storage implementation | T03, analyze PASS | Durable store through existing facade; no API bypass, no silent data resets; existing tests retained. |
| T05 restart qualification | T04 | Two separate processes import/reopen; IDs/source bytes/audit/projections identical; empty-memory construction cannot satisfy check. |
| T06 failure qualification | T04 | Interrupted transactions/migration and incomplete blob closure yield documented recovery or explicit refusal; no partial canonical promotion. |
| T07 isolation/recovery | T05–T06 | Scope mismatch denied, independent writer contention resolved by qualified ownership, backup restore validates lineage and bytes; wrong-key restore preserves destination. |
| T08 converge | T05–T07 | Format, dependency direction, clippy, locked tests, platform CI; exact-head independent review; migration/privacy limitations documented; merge only required gates satisfied. |

Q03 must independently qualify open-vault files, temporary/WAL/journal files, crash artifacts,
backup and OS key custody before PRIVATE readiness. Q04 must qualify actual process identity,
lease revocation and writer ownership before untrusted clients. Q06 follows with precision-aware
time/amendment semantics. Q07 then integrates the first full workflow. These are explicit
dependencies, not implicit claims that T04 alone produces a releasable app.

## Checklist and analysis

- [x] Evidence-backed gap and bounded user outcome defined.
- [x] Scope, dependencies, invariants, targets and acceptance checks identified.
- [x] External gates kept separate from ordinary engineering work.
- [x] Owning numbered Spec Kit specify/clarify/plan/checklist/tasks artifacts materialized (`specs/016-durable-trusted-record/`).
- [x] Storage/privacy ADR and migration compatibility drafted (`adr-016-001-durable-sqlite.md`); repository review via Spec 016 PR.
- [x] Requirement-to-test analysis passes with no material ambiguity (`analyze-notes.md` PASS).
- [x] Runtime implementation, exact-head qualification and convergence complete.

Analysis result: Spec 016 Q02 closed on main `419a468`. Continue Spec 017 Q03 vault privacy.
Estimated Q02 cost realized as merged unit; Q03 next.
