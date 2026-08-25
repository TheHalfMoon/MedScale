# Implementation Plan: CLI + Desktop Foundation

**Branch**: `spec/006-cli-desktop-foundation` | **Date**: 2026-08-25 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/006-cli-desktop-foundation/spec.md`

## Summary

Ship the first useful product surface—Trusted Local Longitudinal Record—via a **CLI-complete** wedge over the same Rust Core Host / authority facade as future Desktop/SDK clients. Productize `medscale doctor`, vault create/open, synthetic ingest, and timeline/Brief/coverage. Produce typed `PRIVACY_PROOF` evidence. Desktop is a **thin non-WebView scaffold**. **Defer Tauri v2.11.5** admission until WebView privacy proof can pass; document limitations. Integrate v0 only if `imports/v0/` exists—do not block. Synthetic-only; DEFAULT_DENY; no MESC mutation; REAL_PHI unauthorized.

## Technical Context

**Language/Version**: Rust stable per workspace `rust-toolchain.toml`

**Primary Dependencies** (pins in research.md):

- clap **4.6.6**
- anyhow **1.0.104** (CLI edge only)
- Existing workspace: medscale-contracts, medscale-core, medscale-storage, medscale-keys, medscale-fhir, serde/serde_json
- **Not admitted**: tauri **2.11.5** (candidate only)

**Storage**: EncryptedVault via Spec 005 Core Host path only; CLI never opens DB

**Testing**: CLI integration (doctor, wedge E2E), authority non-bypass, secret non-emission, PRIVACY_PROOF schema, Desktop scaffold smoke; Windows+Linux as available

**Target Platform**: Windows + Linux CI; Desktop scaffold builds; no WebView runtime claim

**Project Type**: Rust workspace — extend `medscale-cli`, `medscale-core`, `medscale-contracts`; add thin `medscale-desktop` scaffold

**Performance Goals**: Correctness, fail-closed authority, deterministic presentation; bounded synthetic fixtures

**Constraints**: Synthetic-only; DEFAULT_DENY product network; no MESC mutation; no Tauri admit; no direct CLI/UI DB; REAL_PHI unauthorized; no invented final v0 UI

**Scale/Scope**: First product wedge (CLI + Desktop foundation)—not mobile, not Network Broker, not packs, not REAL_PHI

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Rust-owned trusted core: CLI/Desktop are clients of Core Host facade only
- [x] Local-first / privacy-first: offline wedge; DEFAULT_DENY; PRIVACY_PROOF with limitations
- [x] Source/authority discipline: no privileged CLI bypass; no DB/key handles in client crates
- [x] Evidence-before-claims: PRIVACY_PROOF + doctor freshness; no WebView PASS without proof
- [x] Fail-closed: lease, sync-root, missing vault, missing privacy evidence
- [x] MESC/PHI/network anti-scope honored
- [x] Minimal reversible architecture: in-process host now; IPC envelopes reusable; Tauri re-entry possible later

## Project Structure

### Documentation (this feature)

```text
specs/006-cli-desktop-foundation/
├── spec.md
├── clarifications.md
├── research.md
├── plan.md
├── data-model.md
├── quickstart.md
├── analyze-notes.md
├── tasks.md
├── contracts/
│   ├── cli-authority.md
│   ├── doctor-report.md
│   └── privacy-proof.md
└── checklists/
    └── requirements.md
```

### Source Code (repository root — implement phase only; not this planning package)

```text
crates/
  medscale-cli/           # product CLI (clap); doctor + wedge commands
  medscale-desktop/       # NEW: thin scaffold; no Tauri
  medscale-contracts/     # DoctorReport, PrivacyProof types
  medscale-core/          # doctor aggregation; facade wiring for CLI session
docs/engineering/admissions/
  006-cli-clap-anyhow.md
evidence/
  006-cli-desktop-foundation/
imports/v0/               # only if founder supplies; else EXTERNAL_GATES note
```

**Structure Decision**: Prefer extending existing `medscale-cli`. Add Desktop scaffold crate without WebView. Do not admit Tauri in this unit.

## Implementation approach

1. Write clap/anyhow admission **before** dependency lines.
2. Add DoctorReport + PrivacyProof contracts; core doctor aggregation over vault/claim/keys.
3. Replace Spec 001 CLI bootstrap with clap commands; wire in-process CoreFacade session.
4. Implement vault create/open, ingest, timeline/brief/coverage CLI paths.
5. Add medscale-desktop scaffold smoke.
6. Secret non-emission + authority non-bypass tests.
7. Assemble PRIVACY_PROOF evidence; record Tauri/v0 deferrals.
8. Update EXTERNAL_GATES / BUILD_QUEUE only at converge as required.

## Complexity Tracking

No constitution violations. Tauri deferral is intentional fail-closed privacy posture, not scope escape. v0 absence is non-blocking by contract.
