# MedScale â€” Start Here

## 1. Current gate

```text
PLAN = CANONICAL_V2
REPOSITORY_PLANNING_FINALIZATION = COMPLETE
SPEC_000 = CLOSED_CANONICAL
SPEC_001 = CLOSED_CANONICAL (see BUILD_QUEUE.md for live states)
FOUNDER_STANDING_CURSOR_IMPLEMENTATION_AUTHORITY = ACTIVE
REAL_PHI = NOT_AUTHORIZED
MESC_MUTATION = NOT_AUTHORIZED
```

The earlier planning-only gate has been superseded by `IMPLEMENTATION_AUTHORITY.md`. Cursor must follow live BUILD_QUEUE.md; Specs 001–011 and 013–015 are CLOSED_CANONICAL; Spec 012 remains MESC-artifact gated. It must not ask the founder for ordinary engineering decisions already governed by the plan.

## 2. Mandatory read order

1. `/CURSOR.md`
2. `/AGENTS.md`
3. `IMPLEMENTATION_AUTHORITY.md`
4. `BUILD_QUEUE.md`
5. `MASTER_BUILD_PLAN_V2.md`
6. `SPECKIT_MASTER_ROADMAP_V2.md`
7. `IMPLEMENTATION_DECISION_DEFAULTS.md`
8. `SOURCE_ACQUISITION_AND_COPY_PLAN.md`
9. `V0_UI_INTEGRATION_CONTRACT.md`
10. relevant source/OSS/OpenMed matrices and current spec package

## 3. Build order

```text
000 CLOSED_CANONICAL
  -> 001 Rust Repository + Spec Kit Bootstrap
  -> 002 Trusted Object / Source / Authority + Process/Text Foundation
  -> 003 H0-A Trusted Ingest + Durability
  -> 004 H0-B Trusted Presentation + Coverage
  -> 005 Local Private Vault + Encryption + Recovery
  -> 006 CLI + Desktop Foundation
```

Then dependency-controlled tracks:

```text
004 -> 007 OpenMed parity/absorption research
005 + 006 + qualified 007 -> 008 Local AI Capability Fabric
005 + 006 -> 009 Mobile base; AI features also require 008
008 -> 010 Documents/OCR/Voice
004 + 008 -> 011 Evidence/Retrieval/Medical Intelligence
008 + released MESC artifact -> 012 MESC Artifact Integration
005 + 006 -> 013 FHIR/SMART/Network Broker -> 014 Controlled Actions/NPHIES
008 + 013 (+ mobile pack constraints) -> 015 Online Pack/HF Ecosystem
016+ remains deferred until canonically promoted
```

## 4. Current follow-on work

Read [whole-product review](WHOLE_PRODUCT_REVIEW_2026-09-09.md),
[delivery plan](TRUSTED_V1_DELIVERY_PLAN.md), and live [BUILD_QUEUE.md](BUILD_QUEUE.md).
Specs 016–023 are `CLOSED_CANONICAL` READY_BASE (022 = Q05 release-qualification **prep** only;
023 = Q03 open-metadata SQLCipher; `PRIVATE_DATA_READY` still FALSE).
Spec 012 remains MESC-blocked. Deferred advanced work is **024+**.
Do **not** claim `RELEASE_READY`, `PRIVATE_DATA_READY`, or `MULTI_CLIENT_RELEASE_READY`.
Q03 residual page encryption landed in Spec 023; OS keyring/swap/snapshot still block PRIVATE_DATA_READY.
Foundation and prep closure is not product or privacy release readiness.

### Historical bootstrap requirements (already closed)

Spec 001 must:

- establish the Rust workspace and minimal dependency-directed crate skeleton;
- install/bootstrap GitHub Spec Kit and materialize the already-canonical constitution/source authority without reopening founder decisions;
- create CI, formatting/lint/test/evidence skeleton and supply-chain gates;
- establish repository contribution/review/evidence conventions;
- create the next complete Spec 002 package;
- add no medical functionality beyond bootstrap contracts required by 001.

When 001 closes, Cursor immediately starts 002.

## 5. First product wedge

The first useful product outcome is the **Trusted Local Longitudinal Record**: exact source custody, explicit identity/time, deterministic timeline and narrow LLM-free Brief, coverage/conflict/unknown-vs-absence accounting, provenance drill-down, and one Rust authority path shared by CLI/Desktop. It remains valuable with no model installed.

## 6. UI

The founder uses **v0** for visual UI. Cursor continues backend/core/integration work without waiting for final visual polish. v0 output enters through `imports/v0/` and the `V0_UI_INTEGRATION_CONTRACT.md`; generated server/database/network shortcuts never become MedScale authority automatically.

## 7. Evidence and completion

No `PASS`, `PARITY`, `SURPASS`, `PRIVATE`, `OFFLINE`, `CONFORMANT`, or `CLOSED_CANONICAL` claim is valid without exact evidence. Follow `DEFINITION_OF_DONE.md`. An external gate blocks only its own path; record it in `EXTERNAL_GATES.md` and continue.