# Tasks: FHIR / SMART / Network Broker

**Input**: Design documents from `/specs/013-fhir-smart-network-broker/`

**Prerequisites**: Spec 005 + Spec 006 `CLOSED_CANONICAL`; plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Required by roadmap exit — bypass, allowlist/receipts/capability, FHIR profile/integrity/conformance evidence, no uncontrolled provider client; synthetic/fixture-only.

**Note**: Planning package is QUALIFIED. Implement on `spec/013-fhir-smart-network-broker`. Do not authorize REAL_PHI. Do not require live partner endpoints. Do not mutate MESC. Do not implement Spec 014/015.

## Phase 1: Setup + Admissions

**Purpose**: Provenance before HTTP dependency lines; contract types

- [x] T001 Confirm Spec 005 + Spec 006 `CLOSED_CANONICAL` and workspace builds; note baseline commit/toolchain in `evidence/013-fhir-smart-network-broker/BASELINE.md`
- [x] T002 [P] Create admission `docs/engineering/admissions/013-network-broker-ureq.md` binding ureq **3.4.0** with `default-features = false`, features `rustls` + `json`; explicitly record reqwest/tokio/native-tls as NOT admitted in Spec 013
- [x] T003 [P] Extend `medscale-contracts` with NetworkBrokerRequest/Decision, EgressAllowlistEntry, purpose/data-class enums, doctor NetworkBroker section types (data-model.md)
- [x] T004 [P] Add contracts serde round-trips + reason_code exhaustiveness tests
- [x] T005 Confirm EXTERNAL_GATES: REAL_PHI NOT_AUTHORIZED; PRODUCTION_CREDENTIALS NOT_GRANTED; PARTNER_EHR_NPHIES_ENDPOINT NOT_GRANTED; MESC mutation NO
- [x] T006 [P] Scaffold evidence dir `evidence/013-fhir-smart-network-broker/` with limitations stub

**Checkpoint**: Admissions present; contracts compile; no ureq in workspace yet (or only after T002)

---

## Phase 2: Foundational Broker + Allowlist

**Purpose**: Fail-closed broker core before adapters

- [x] T007 Create `crates/medscale-network` workspace member; depend on ureq per admission; expose `BrokerTransport` trait
- [x] T008 Implement EgressAllowlist load/match (exact host + path_prefix rules); empty ⇒ all Deny
- [x] T009 Implement Network Broker invoke path in core facade (`Capability::NetworkBrokerInvoke`)
- [x] T010 [P] FixtureTransport (no real sockets) for unit tests
- [x] T011 [P] Tests: UnknownDestination / PurposeMismatch / DataClassRefused / Unauthorized → Deny + zero transport sends

**Checkpoint**: Broker Deny paths work offline

---

## Phase 3: User Stories 1–3 — Receipts + Bypass (P1) 🎯 MVP core

**Goal**: Durable receipts; sole egress; bypass suite

**Independent Test**: `cargo test -p medscale-network broker_receipts_and_bypass` (name illustrative)

- [x] T012 [US3] Persist Deny/Allow outcomes as ActionAuditRecord via DurableStore/EncryptedVault Core Host path
- [x] T013 [US3] Map logical NetworkBrokerReceipt query over audit ids; restart durability test
- [x] T014 [US1] Wire Allow + FixtureTransport success → ActionAuditRecord attempt/fixture_ok
- [x] T015 [US1] Architecture/dependency test: `medscale-cli` / `medscale-desktop` / workers do not depend on ureq
- [x] T016 [US2] Allowlist matrix evidence written to `ALLOWLIST_DENY_MATRIX.md`
- [x] T017 [US1] Document bypass proof in `BYPASS_TESTS.md`

**Checkpoint**: Receipts durable; bypass evidence archived

---

## Phase 4: User Story 4 — FHIR / SMART Stub Adapters (P1)

**Goal**: Adapter interfaces + fixture receipts; live mode refused

**Independent Test**: adapter fixture suite through broker

- [x] T018 [US4] Implement SmartAuthAdapter FixtureStub (discover + token_fixture) via broker purposes SmartDiscovery/SmartTokenFixture
- [x] T019 [US4] Implement FhirPartnerAdapter read/search fixture path via broker
- [x] T020 [US4] LivePartner / live_authorize paths return ExternalGateRequired; no socket
- [x] T021 [US4] Archive SMART_FIXTURE_RECEIPTS.json + FHIR fixture digests (redacted)
- [x] T022 [US4] Tests: adapters never call ureq directly (only broker)

**Checkpoint**: Stub adapters green offline

---

## Phase 5: User Story 5 — Profile / Integrity / Conformance Evidence (P1)

**Goal**: EvaluationRecord evidence_only attachments

**Independent Test**: integrity fail + profile fixture attach

- [x] T023 [US5] Integrity digest helper; mismatch fail-closed
- [x] T024 [US5] Attach profile/oracle fixture results as EvaluationRecord (`evidence_only=true`)
- [x] T025 [US5] Prove no ClinicalAssertion auto-create from brokered FHIR / evaluations
- [x] T026 [US5] Record Inferno v1.4.2 / SMART kit REFERENCE_ONLY notes + optional report digest in CONFORMANCE_NOTES.md
- [x] T027 [US5] Tests: tampered bytes denied; evidence_only invariant

**Checkpoint**: Conformance/integrity evidence exit artifacts

---

## Phase 6: Doctor + Polish + Closeout Prep

- [x] T028 Upgrade `medscale doctor` network_broker section from NotImplemented → Available/DefaultDenyExceptBroker (count-only allowlist)
- [x] T029 Secret-marker scan: doctor + receipt samples emit no tokens/client secrets
- [x] T030 Run `cargo test --workspace` + fmt/clippy on Windows+Linux as available
- [x] T031 Validate `quickstart.md` commands
- [x] T032 Ensure REAL_PHI gate unchanged; no MESC mutation; no reqwest/tokio admit; no Spec 014 state machine; no HF client
- [x] T033 Update `docs/planning/BUILD_QUEUE.md` on converge: Spec 013 `CLOSED_CANONICAL` (only at merge/converge—not during planning)

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 → Phase 2 → Phase 3 (receipts need broker)
- Phase 4 after Phase 3 (adapters need receipts + broker Allow path)
- Phase 5 parallelizable with Phase 4 after T012 (EvaluationRecord attach)
- Phase 6 last

### Parallel opportunities

- T002–T006 parallel in Phase 1
- T010–T011 after T008/T009 start
- T018–T022 after T014
- T023–T027 after T012

### MVP (Spec 013 exit minimum)

T001–T027 + T028–T032 (broker + receipts + bypass + stub adapters + evidence + doctor).

---

## Task summary counts

| Phase | Tasks |
|---|---|
| 1 Setup | T001–T006 |
| 2 Broker foundation | T007–T011 |
| 3 Receipts + bypass | T012–T017 |
| 4 FHIR/SMART adapters | T018–T022 |
| 5 Profile/integrity evidence | T023–T027 |
| 6 Doctor + polish | T028–T033 |
| **Total** | **T001–T033** |
