<!-- Checkbox sync: tasks completed at CLOSED_CANONICAL closeout; markers aligned 2026-08-26. -->

# Tasks: CLI + Desktop Foundation

**Input**: Design documents from `/specs/006-cli-desktop-foundation/`

**Prerequisites**: Spec 005 `CLOSED_CANONICAL`; plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Required by roadmap exit — CLI authority non-bypass, doctor axes, longitudinal wedge, PRIVACY_PROOF, Desktop scaffold without Tauri; synthetic-only.

**Note**: Planning package is QUALIFIED. Implement on `spec/006-cli-desktop-foundation`. Do not authorize REAL_PHI. Do not admit Tauri/WebView. Do not invent final v0 UI if artifact absent.

## Phase 1: Setup + Admissions

**Purpose**: Provenance before CLI dependency lines; contract types

- [x] T001 Confirm Spec 005 `CLOSED_CANONICAL` and workspace builds; note baseline commit/toolchain in `evidence/006-cli-desktop-foundation/BASELINE.md`
- [x] T002 [P] Create admission `docs/engineering/admissions/006-cli-clap-anyhow.md` binding clap **4.6.6** + anyhow **1.0.104**; explicitly record Tauri **2.11.5** as NOT admitted
- [x] T003 [P] Extend `medscale-contracts` with `DoctorReport`, `PrivacyProof`, related enums (data-model.md)
- [x] T004 [P] Add contracts tests / serde round-trips for DoctorReport + PrivacyProof
- [x] T005 Confirm EXTERNAL_GATES: REAL_PHI NOT_AUTHORIZED; FINAL_V0_UI_ARTIFACT deferred-ok; `TAURI_WEBVIEW_PRIVACY_QUALIFICATION=DEFERRED`
- [x] T006 [P] Scaffold evidence dir `evidence/006-cli-desktop-foundation/` with limitations stub

**Checkpoint**: Admissions present; contracts compile; Tauri absent from workspace

---

## Phase 2: Foundational Doctor Aggregation + CLI Session

**Purpose**: Core doctor helpers + in-process session before full commands

- [x] T007 Implement doctor aggregation in `medscale-core` over vault location policy, sync-risk, key-store availability, claim status, privacy freshness hooks
- [x] T008 [P] Implement `CliSession` / in-process TransientHostOwner wiring in core or cli (facade-only)
- [x] T009 [P] Add workspace deps: clap 4.6.6, anyhow 1.0.104 to `medscale-cli` per admission
- [x] T010 Replace Spec 001 manual argv with clap command tree stubs (doctor, vault, ingest, timeline, brief, coverage, help, version)
- [x] T011 [P] Tests: doctor JSON contains required axes; secret-marker scan on doctor output

**Checkpoint**: `medscale doctor` product report works offline

---

## Phase 3: User Stories 1–3 — CLI Authority + Longitudinal Wedge (P1) 🎯 MVP

**Goal**: Vault create/open, synthetic ingest, timeline/brief/coverage via facade only

**Independent Test**: `cargo test -p medscale-cli cli_longitudinal_wedge` (name illustrative)

- [x] T012 [US1] Wire `vault create` / `vault open` through Spec 005 EncryptedVault capabilities (passphrase env/policy)
- [x] T013 [US1] Architecture/negative test: `medscale-cli` does not open rusqlite/EncryptedVault directly
- [x] T014 [US3] Wire `ingest` to Spec 003 synthetic FHIR ingest capability
- [x] T015 [US3] Wire `timeline` / `brief` / `coverage` to Spec 004 Get* capabilities (`--json` optional)
- [x] T016 [US1/US3] E2E wedge: create → ingest fixture → presentation digests; assert offline / synthetic_only
- [x] T017 [US2] Ensure doctor reports configured/open vault axes after wedge (no secrets)

**Checkpoint**: Trusted Local Longitudinal Record via CLI

---

## Phase 4: User Story 4 — PRIVACY_PROOF (P1)

**Goal**: Typed evidence artifact with mandatory limitations

**Independent Test**: evidence file validates schema + limitations

- [x] T018 [US4] Implement PrivacyProof builder/serializer in contracts/core
- [x] T019 [US4] Run log/crash marker scans for CLI/Core Host; record results
- [x] T020 [US4] Write `evidence/006-cli-desktop-foundation/PRIVACY_PROOF.json` with webview `NotApplicable`, Tauri deferred, synthetic-only
- [x] T021 [US4] Doctor privacy freshness references artifact when present; Missing when absent
- [x] T022 [US4] Tests: limitations MUST include Tauri deferred + no system-wide zero-packet claim

**Checkpoint**: PRIVACY_PROOF exit evidence

---

## Phase 5: User Story 5 — Desktop Scaffold + v0 Gate (P1)

**Goal**: Thin non-WebView Desktop; no Tauri; v0 non-blocking

**Independent Test**: `cargo run -p medscale-desktop -- --smoke`

- [x] T023 [US5] Scaffold `crates/medscale-desktop` (workspace member) calling CoreFacade smoke / doctor subset
- [x] T024 [US5] Assert Cargo.toml / lockfile contain **no** `tauri` package
- [x] T025 [US5] If `imports/v0/` absent: confirm EXTERNAL_GATES / evidence note for deferred visual UI; do not invent final UX
- [x] T026 [US5] If `imports/v0/` present: inventory provenance only as allowed by V0_UI_INTEGRATION_CONTRACT (optional path; do not block MVP)
- [x] T027 [US5] Document safer-shell policy in evidence limitations (change shell later rather than weaken privacy)

**Checkpoint**: Desktop foundation without WebView claim

---

## Phase 6: Polish + Closeout Prep

- [x] T028 Run `cargo test --workspace` + fmt/clippy on Windows+Linux as available
- [x] T029 Validate `quickstart.md` commands
- [x] T030 Archive wedge transcripts + doctor samples under evidence/
- [x] T031 Ensure REAL_PHI gate unchanged; no MESC mutation; no product network clients; no Tauri admit
- [x] T032 Update `docs/planning/BUILD_QUEUE.md` on converge: Spec 006 `CLOSED_CANONICAL` (only at merge/converge—not during planning)

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 → Phase 2 → Phase 3 (wedge needs doctor session + clap)
- Phase 4 after Phase 3 (proof from scans + wedge)
- Phase 5 parallelizable with Phase 4 after Phase 2 (scaffold independent of full wedge, but prefer after T009 workspace wiring)
- Phase 6 last

### Parallel opportunities

- T002–T006 parallel in Phase 1
- T008–T011 parallelizable after T007 starts
- T018–T022 after wedge artifacts exist
- T023–T027 after contracts/core smoke available

### MVP (Spec 006 exit minimum)

T001–T022 + T023–T025 + T028–T031 (Desktop scaffold without v0; PRIVACY_PROOF; CLI wedge).

---

## Task summary counts

| Phase | Tasks |
|---|---|
| 1 Setup | T001–T006 |
| 2 Doctor/CLI foundation | T007–T011 |
| 3 CLI wedge | T012–T017 |
| 4 PRIVACY_PROOF | T018–T022 |
| 5 Desktop/v0 | T023–T027 |
| 6 Polish | T028–T032 |
| **Total** | **T001–T032** |
