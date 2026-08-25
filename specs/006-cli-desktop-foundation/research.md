# Research: Spec 006 CLI + Desktop Foundation

**Date**: 2026-08-25  
**Spec**: `006-cli-desktop-foundation`

## Decisions

### D1 — CLI-complete wedge first; Desktop thin scaffold

- **Decision**: Spec 006 ships a **CLI-complete** Trusted Local Longitudinal Record wedge (`doctor`, vault create/open, synthetic ingest, timeline/brief/coverage) over the same Rust Core Host / authority facade. Desktop is a **thin scaffold** (compile + facade smoke) without a privacy-qualified interactive WebView shell.
- **Alternatives**: (a) Full Tauri Desktop + CLI in one PR; (b) Desktop-only first.
- **Rationale**: Roadmap requires both surfaces over one core; MASTER_BUILD_PLAN prefers proving the wedge offline; WebView privacy cannot close in the same unit without risking claim weakening. User-authorized preference: CLI-complete first.

### D2 — Defer Tauri admission (reject WebView for Spec 006 product Desktop)

- **Decision**:
  1. SOURCE_ACQUISITION pin remains **Tauri v2.11.5** as `DEPENDENCY` **candidate**.
  2. Spec 006 does **not** add `tauri` to the workspace.
  3. Document WebView PHI containment as **unproven**; PRIVACY_PROOF limitations + EXTERNAL_GATES `TAURI_WEBVIEW_PRIVACY_QUALIFICATION = DEFERRED`.
  4. If a later unit admits Tauri, it MUST pass WebView cache/crash/log marker containment; else choose a safer shell (native, non-WebView) and preserve v0 visual intent per V0_UI_INTEGRATION_CONTRACT.
- **Alternatives**: Admit Tauri now with provisional privacy notes; custom egui/iced shell now.
- **Rationale**: “Keep only if WebView privacy proof passes”; cannot pass in CLI-first PR; never weaken privacy to ship a shell.

### D3 — In-process Core Host for Spec 006

- **Decision**: CLI (and Desktop scaffold) use **in-process** `CoreFacade` / TransientHostOwner semantics: one process owns the vault lease and keys for the command. No separate IPC daemon required for 006 exit.
- **Alternatives**: Unix/Windows named-pipe Core Host daemon in 006.
- **Rationale**: Spec 002 allows CLI to spawn/own transient host; reduces scope; envelopes stay identical for later multi-process IPC.

### D4 — CLI dependency pins

| Crate | Version | Purpose |
|---|---|---|
| `clap` | **4.6.6** | CLI parsing (`derive` feature as needed) |
| `anyhow` | **1.0.104** | CLI-edge error reporting only |
| `serde` / `serde_json` | workspace | DoctorReport / presentation JSON output |
| `thiserror` | workspace | Prefer typed errors in libraries; anyhow at bin edge |

Admit via `docs/engineering/admissions/006-cli-clap-anyhow.md` before first use. Do **not** admit Tauri.

### D5 — `medscale doctor` report axes

- **Decision**: Typed `DoctorReport` MUST include at least:
  - product identity + version
  - `local_only` / network posture (`DEFAULT_DENY`)
  - vault location (resolved or policy default class)
  - sync/remote-root risk (`Ok` \| `Refused` \| `Unknown` \| `NotConfigured`)
  - key-store availability (`Available` \| `Unavailable` \| `Mock` \| `Unknown`)
  - privacy evidence freshness (`Fresh` \| `Stale` \| `Missing` + path/id)
  - filesystem / OS claim status (from Spec 003/005 claim notes)
  - packs/runtime: `not_implemented` until Spec 008
  - network_broker: `not_implemented` until Spec 013
- **Rationale**: MASTER_BUILD_PLAN §10; extends Spec 001 bootstrap doctor.

### D6 — PRIVACY_PROOF schema (006 scope)

- **Decision**: Introduce contracts type + evidence JSON/markdown bundle:
  - attributable network observation: **none expected** (DEFAULT_DENY; no product clients)
  - dependency/network-capability audit: Cargo deps + no network client crates in product path
  - crash/log/cache marker scans: CLI + Core Host paths; synthetic markers
  - WebView/cache scan: **N/A — Tauri deferred**
  - vault/sync-root evidence: doctor + claim tests
  - explicit limitations list (required)
- **Rationale**: F-17 claim-scoping; MASTER_BUILD_PLAN §14; OPENMED_PARITY PRIVACY_PROOF definition.

### D7 — v0 UI integration posture

- **Decision**: If `imports/v0/` artifact absent → continue; update/confirm EXTERNAL_GATES `FINAL_V0_UI_ARTIFACT`; do not invent final visual UI. If present → inventory provenance, strip backend shortcuts, wire to typed facade only.
- **Rationale**: V0_UI_INTEGRATION_CONTRACT; Cursor does not wait for final v0 polish.

### D8 — CLI command surface (MVP)

```text
medscale --version | version
medscale help
medscale doctor [--json]
medscale vault create [--passphrase-env ...]
medscale vault open
medscale ingest <path>          # synthetic FHIR only
medscale timeline --subject <id> [--json]
medscale brief --subject <id> [--json]
medscale coverage --subject <id> [--json]
```

Optional later (out of MVP unless cheap): `vault close`, `vault status`. Packs/network commands stay help-listed as not implemented or omitted.

### D9 — Crate layout

- **Decision**:
  - Extend `crates/medscale-cli` (replace bootstrap stub).
  - Add thin `crates/medscale-desktop` (or `apps/desktop`) scaffold binary/lib with facade smoke—**no** Tauri.
  - Extend `medscale-contracts` with `DoctorReport`, `PrivacyProof` types.
  - Extend `medscale-core` with doctor aggregation helpers over vault/claim/key status.
- **Rationale**: One authority path; CLI already exists from Spec 001.

### D10 — Anti-scope (binding)

- No REAL_PHI authorization
- No product runtime network clients / Network Broker
- No MESC mutation or OpenMed runtime
- No models/packs product install path (008)
- No OCR/ASR (010)
- No mobile shells (009)
- No Tauri/WebView DEPENDENCY admission
- No inventing final v0 visual design

### D11 — Evidence archive layout

```text
evidence/006-cli-desktop-foundation/
  BASELINE.md
  PRIVACY_PROOF.json   # or .md + schema id
  doctor-sample.txt
  wedge-transcript.*
  limitations.md
```

### D12 — Relationship to Spec 005

- Consumes EncryptedVault, KeyProvider, claim/sync refusal, log hygiene foundations.
- Does not reopen SQLCipher/keyring pins.
- Doctor surfaces 005 claim/key-store status; does not re-implement crypto.

## Alternatives considered (summary)

| Topic | Chosen | Rejected |
|---|---|---|
| Shell | CLI + thin Desktop scaffold | Full Tauri Desktop in 006 |
| Host | In-process facade | Daemon IPC in 006 |
| UI | Deferred v0 / fixture | Invented final Desktop UX |
| Privacy | Typed PRIVACY_PROOF + limitations | Absolute zero-packet claim |

## Open residuals (non-blocking)

- Exact multi-process IPC transport (later hardening).
- Safer native Desktop shell technology choice when Tauri remains rejected.
- PHI-to-stdout high-friction policy productization (post-REAL_PHI gate).
- Pack/runtime doctor fields (Spec 008).
