# Clarification Closeout: Spec 006

**Date**: 2026-08-25  
**Command**: `/speckit.clarify` equivalent (autonomous defaults)

No founder questions. Ambiguities resolved via `IMPLEMENTATION_DECISION_DEFAULTS.md`, constitution, MASTER_BUILD_PLAN_V2 §7/§10/§14, SPECKIT_MASTER_ROADMAP_V2 Spec 006 row, V0_UI_INTEGRATION_CONTRACT, SOURCE_ACQUISITION Tauri row, GLM F-05/F-17, Spec 005 foundations, and EXTERNAL_GATES. Details live in `research.md`.

| ID | Ambiguity | Resolution |
|---|---|---|
| C1 | Admit Tauri in this PR vs defer | **Defer Tauri.** CLI-complete wedge + Desktop thin scaffold first. Tauri **v2.11.5** remains SOURCE candidate only; not DEPENDENCY-admitted until WebView PHI containment/privacy proof passes. PRIVACY_PROOF limitations document deferral |
| C2 | WebView reject vs qualify | Spec 006 **rejects WebView for product Desktop** in this unit (cannot close privacy proof in one PR with CLI wedge). Safer shell policy: change shell later rather than weaken privacy |
| C3 | IPC topology for 006 | **In-process Core Host** (CLI/Desktop scaffold as TransientHostOwner / same-process facade). Multi-process local IPC socket deferred; same authority envelopes |
| C4 | CLI command set for wedge | **doctor**, **vault create/open**, **ingest** (synthetic), **timeline**, **brief**, **coverage**, plus version/help. Packs/network remain placeholders |
| C5 | clap / anyhow pins | **clap 4.6.6**, **anyhow 1.0.104** (crates.io current at package date); freeze in admission + Cargo.lock at implement |
| C6 | v0 UI absence | **Do not block.** Record FINAL_V0_UI_ARTIFACT / deferred visual integration; ship CLI + host wiring with fixture/scaffold UI only |
| C7 | doctor pack/runtime fields | Report **not_implemented** / unavailable until Spec 008; still required axes present |
| C8 | PRIVACY_PROOF claim scope | Typed evidence with limitations; **no** system-wide zero-packet claim; CLI/Core Host attributable scans only in 006 |
| C9 | PHI-to-stdout | Synthetic fixtures may print structured presentation; future real PHI stdout remains deny-by-default / high-friction (out of 006 authorization) |
| C10 | REAL_PHI / network / MESC | Unchanged: NOT_AUTHORIZED / DEFAULT_DENY / no MESC mutation |
| C11 | Desktop crate shape | Thin `medscale-desktop` scaffold (or `apps/desktop` stub) calling facade; no WebView runtime |
| C12 | Spec 005 dependency | Implement gated on Spec 005 `CLOSED_CANONICAL` |

**Outstanding NEEDS CLARIFICATION markers in spec.md**: none

**External gates**: REAL_PHI NOT_AUTHORIZED; FINAL_V0_UI_ARTIFACT USER_SUPPLIED_WHEN_READY; TAURI_WEBVIEW_PRIVACY_QUALIFICATION DEFERRED (new row). MESC mutation NO. Product runtime network DEFAULT_DENY.
