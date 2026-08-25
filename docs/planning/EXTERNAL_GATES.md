# External Gates

This file records blockers that require a real human/external authority. They do not block unrelated repository work.

| Gate | Current state | Scope | Cursor behavior |
|---|---|---|---|
| REAL_PHI_AUTHORIZATION | `NOT_AUTHORIZED` | Any real patient data | Use synthetic/permitted fixtures; continue. |
| LEGAL_COUNSEL_FLOW_MAPPING | `PENDING_WHEN_REQUIRED` | Deployment-specific PDPL/SFDA/controller/processor/lawful-basis decisions | Implement versioned decision-record capability, do not invent legal conclusions. |
| PUBLIC_SOURCE_LICENSE_CHOICE | `PENDING` | Public SPDX license for MedScale crates/distribution | Keep crates `publish = false` / `UNLICENSED` privately; cargo-deny ignores unpublished workspace members; continue. |
| GATED_MODEL_TERMS | `NOT_GRANTED_BY_DEFAULT` | Model requiring click-through/credential acceptance | Do not accept terms; use admissible public/local comparator and continue. |
| PRODUCTION_CREDENTIALS | `NOT_GRANTED` | Real cloud/provider/partner credentials | Build/test with mocks/synthetic endpoints; continue. |
| PARTNER_EHR_NPHIES_ENDPOINT | `NOT_GRANTED` | Real integration qualification | Use conformance/synthetic harness; continue. |
| APP_STORE_SIGNING_RELEASE | `NOT_GRANTED` | Final public mobile signing/submission | Build unsigned/dev-test artifacts and all pre-release qualification possible. |
| FINAL_V0_UI_ARTIFACT | `USER_SUPPLIED_WHEN_READY` | Final visual implementation | Continue core/contracts/integration shell with deterministic fixture UI; do not invent final visual design. Spec 006 ships CLI + host wiring without blocking on v0. |
| TAURI_WEBVIEW_PRIVACY_QUALIFICATION | `DEFERRED` | Admit Tauri/WebView as Desktop DEPENDENCY | Spec 006 does **not** admit Tauri v2.11.5. Keep SOURCE candidate pin; require WebView cache/crash/log PHI-containment proof before DEPENDENCY admission; otherwise use a safer shell. Record limitations in Spec 006 `PRIVACY_PROOF`. |

Cursor may add rows only when a blocker truly requires external authority. Never use this file for ordinary engineering uncertainty.