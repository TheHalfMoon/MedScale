# Tasks: Spec 016 Durable Trusted Record

## Phase A — Specification (T01–T03)

- [x] T01 Materialize numbered Spec Kit package (`spec.md`, clarifications, research, plan)
- [x] T02 Schema + ADR-016-001 + data-model + field-mapping
- [x] T03 Requirement-linked fixture/contract plan + checklist + analyze notes
- [x] T03b Land synthetic restart fixture vectors (expected digests/IDs) under tests `durable_restart_016`

## Phase B — Implementation (T04)

- [x] T04a Schema v2 migration + `authority_objects` / `store_state` APIs in `medscale-storage`
- [x] T04b Writer lock module (OS exclusive)
- [x] T04c Facade load-on-open + persist helpers for all mutating capabilities when vault open
- [x] T04d Extend backup/restore manifest v2
- [x] T04e Keep EncryptedVault seal path covering new tables (no PRIVATE claim)

## Phase C — Qualification (T05–T07)

- [x] T05 Two-process restart qualification test
- [x] T06 Failure qualification (corrupt digest refuse)
- [x] T07 Isolation writer contention (+ scope covered by existing promote tests)

## Phase D — Converge (T08)

- [x] T08a fmt/clippy/locked tests/CI green on exact head
- [x] T08b Evidence SUMMARY + limitations; update BUILD_QUEUE; mark review package checklist items
- [x] T08c Exact-head review + merge; post-merge verify; start Q03 spec prep

## Dependency order

T01 → T02 → T03 → T04* → (T05 ∥ T06) → T07 → T08

**Closeout**: Spec 016 `CLOSED_CANONICAL` on main `419a468` (PR #35).
