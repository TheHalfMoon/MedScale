# External Gates

This file records blockers that require a real human/external authority. They do not block unrelated repository work.

| Gate | Current state | Scope | Cursor behavior |
|---|---|---|---|
| REAL_PHI_AUTHORIZATION | `NOT_AUTHORIZED` | Any real patient data | Use synthetic/permitted fixtures; continue. Spec **039** `docs/legal/PHI_READINESS_CHECKLIST.md` is engineering prep only and does **not** authorize PHI. |
| LEGAL_COUNSEL_FLOW_MAPPING | `PENDING_WHEN_REQUIRED` | Deployment-specific PDPL/SFDA/controller/processor/lawful-basis decisions | FlowDecisionRecord schema shipped (PendingCounsel default); do not invent legal conclusions ? counsel fills records. |
| OS_KEYRING_SWAP_SNAPSHOT_PRIVATE_DATA | `OPEN` | OS key custody (not MemoryMock), swap/hibernate, and snapshot artifact proof required for `PRIVATE_DATA_READY` | Spec **028** delivers OsKeyStore READY_BASE + doctor `os_keyring_*` but does **not** clear this gate (swap/hibernate/snapshot still open). Spec **032** adds existence/leftover privacy **probes** (`probes_present=true`; residual classes listed open) but does **not** clear this gate. Spec **043** adds honesty classification (`swap_snapshot_honesty_present`; existence vs protection; never `protection_measured` for these surfaces) but does **not** clear this gate. Spec 023 SQLCipher likewise did not clear it. Continue independent work; do not claim private-data readiness. |
| PUBLIC_SOURCE_LICENSE_CHOICE | `PENDING` | Public SPDX license for MedScale crates/distribution | Keep crates `publish = false` / `UNLICENSED` privately; cargo-deny ignores unpublished workspace members; Spec **032** NOTICE inventory is attribution prep only; Spec **037** `docs/legal/LICENSE_COUNSEL_PACKET.md` reduces the founder/legal choice to the smallest SPDX option set without choosing. Continue. |
| REPO_BRANCH_PROTECTION_REQUIRED_CHECKS | `NOT_CONFIGURED_OWNER_SETTINGS` | GitHub branch protection, required status checks, and related repository rulesets for `main` | Spec 022 documents the gate. Spec **037** shipped the inventory; Spec **051** refreshes `evidence/022-release-qualification-prep/REQUIRED_CHECKS.md` to live CI names (rust ubuntu/windows/macos + `perf delivery-plan scale (windows)` + cargo-deny + supply-chain policy present) and `evidence/051-required-checks-sync/OWNER_BRANCH_PROTECTION_ACTION.md`. Do **not** change repository settings via API. Owner must configure protection/required checks; Cursor continues independent engineering and must not claim RELEASE_READY. |
| GATED_MODEL_TERMS | `NOT_GRANTED_BY_DEFAULT` | Model requiring click-through/credential acceptance | Do not accept terms; use admissible public/local comparator and continue. |
| PRODUCTION_CREDENTIALS | `NOT_GRANTED` | Real cloud/provider/partner credentials | Build/test with mocks/synthetic endpoints; continue. |
| PARTNER_EHR_NPHIES_ENDPOINT | `NOT_GRANTED` | Real integration qualification (live partner FHIR/SMART/EHR/NPHIES hosts) | Spec 013 ships stub adapters + fixture receipts only; refuse LivePartner mode; continue. |
| SMART_LIVE_PARTNER_AUTHORIZATION | `NOT_GRANTED` | Live SMART-on-FHIR app registration / authorize/token against real partners | Use FixtureStub SMART adapter + synthetic envelopes in Spec 013; do not register live SMART apps or contact partner hosts. |
| APP_STORE_SIGNING_RELEASE | `NOT_GRANTED` | Final public mobile signing/submission | Build unsigned/dev-test artifacts and all pre-release qualification possible. Spec **039** `evidence/022-release-qualification-prep/SIGNING_PROVENANCE_PREP.md` documents per-platform sign/notarize verification without credentials. |
| FINAL_V0_UI_ARTIFACT | `USER_SUPPLIED_WHEN_READY` | Final visual implementation | FixtureUiViewModel adapters shipped; preserve v0 visuals when supplied; do not invent final design. |
| TAURI_WEBVIEW_PRIVACY_QUALIFICATION | `DEFERRED` | Admit Tauri/WebView as Desktop DEPENDENCY | Spec 006 does **not** admit Tauri v2.11.5. Keep SOURCE candidate pin; require WebView cache/crash/log PHI-containment proof before DEPENDENCY admission; otherwise use a safer shell. Record limitations in Spec 006 `PRIVACY_PROOF`. |
| TERMINOLOGY_SNOMED_LICENSE | `OPEN` | SNOMED CT Pack rights for MedScale distribution | Spec 007 starts track; do not copy tables from OpenMed; continue research/impl without Pack until counsel/pack ready. |
| TERMINOLOGY_LOINC_LICENSE | `OPEN` | LOINC Pack rights | Same as SNOMED track discipline. |
| TERMINOLOGY_ICD_LICENSE | `OPEN` | ICD Pack rights | Same as SNOMED track discipline. |
| TERMINOLOGY_ATC_LICENSE | `OPEN` | ATC Pack rights | Same as SNOMED track discipline. |
| TERMINOLOGY_UMLS_LICENSE | `OPEN` | UMLS/Athena Pack rights | Same as SNOMED track discipline. |
| WORKER_OS_SANDBOX_PLATFORM_QUALIFIED | `OPEN` | Landlock / AppContainer / Seatbelt PLATFORM_QUALIFIED for workers | Spec 026: Linux Landlock **ReadyBaseMeasured**. Spec 030: Windows Job Object **ReadyBaseMeasured**. Spec 031: macOS Seatbelt **ReadyBaseMeasured**. Spec **033**: Windows AppContainer FS **ReadyBaseMeasured**. Spec **038**: Windows AppContainer network **ReadyBaseMeasured**. Spec **040**: Windows AppContainer LPAC **ReadyBaseMeasured**. Spec **041**: macOS App Sandbox entitlements artifact/probe (`macos_app_sandbox_entitlements_measured`; enforcement still signing-gated). Spec **044**: composition inventory present (`composition_inventory_present`) listing residuals; does **not** clear this gate. Multi-OS PLATFORM_QUALIFIED + signed App Sandbox enforcement still required. Do **not** treat ReadyBaseMeasured as gate close. |
| MESC_RELEASED_ARTIFACT | `NOT_AVAILABLE` | Spec 012 immutable MESC artifact (hash/rights/SBOM) | Re-check historically: MESC main `4b193c01â¦` TRAINING_CODE_READY=YES but RELEASE_STATUS=BLOCKED; v0.1.0 release_id 352847712 assets=[]; v0.2.0 tag has no Release. See evidence/012-mesc-artifact-integration/GATE_CHECK.md. Spec **036** ships synthetic verifier READY_BASE (`verifier_ready_base=true`; fixtures under evidence/012â¦/fixtures) but does **not** clear this gate. Fail-closed `MescArtifactAdmit` remains; residual = `BLOCKED_BY_UPSTREAM_MESC_RELEASE_ASSETS`. |
| SPEC_014_WORKFLOW_EVIDENCE | `PENDING` | Selected NPHIES/workflow profiles + terminology evidence | Spec 014 READY_BASE closed; NphiesInvoke remains ExternalGateRequired until this gate + PARTNER_EHR_NPHIES_ENDPOINT. |
| HF_ONLINE_PACK_DISTRIBUTION | `NOT_GRANTED` | Hugging Face / online pack publish+fetch credentials and terms | Spec 015 READY_BASE deny path closed; OnlinePackAcquire remains ExternalGateRequired; offline Pack v0 only. |

Cursor may add rows only when a blocker truly requires external authority. Never use this file for ordinary engineering uncertainty.

## Whole-product qualification refinement (2026-09-09)

Observed 2026-09-09: MESC v0.1.0 has no release assets; a v0.2.0 tag is insufficient.
Spec 012 remains blocked; see [acceptance contract](MESC_ARTIFACT_ACCEPTANCE.md).
OS sandbox/key/ownership qualification includes ordinary engineering work; lack of proof
blocks the affected readiness claim, not independent specification work. Repository required
checks/protection need owner settings authority: observed main unprotected and rulesets empty.
Recorded as `REPO_BRANCH_PROTECTION_REQUIRED_CHECKS` (Spec 022). This review does not authorize
settings changes or bypass exact-head review and CI.
