# External Gates

This file records blockers that require a real human/external authority. They do not block unrelated repository work.

| Gate | Current state | Scope | Cursor behavior |
|---|---|---|---|
| REAL_PHI_AUTHORIZATION | `NOT_AUTHORIZED` | Any real patient data | Use synthetic/permitted fixtures; continue. Spec **039** `docs/legal/PHI_READINESS_CHECKLIST.md` is engineering prep only and does **not** authorize PHI. |
| LEGAL_COUNSEL_FLOW_MAPPING | `PENDING_WHEN_REQUIRED` | Deployment-specific PDPL/SFDA/controller/processor/lawful-basis decisions | FlowDecisionRecord schema shipped (PendingCounsel default); do not invent legal conclusions ? counsel fills records. |
| OS_KEYRING_SWAP_SNAPSHOT_PRIVATE_DATA | `OPEN` | OS key custody (not MemoryMock), swap/hibernate, and snapshot artifact proof required for `PRIVATE_DATA_READY` | Spec **028** delivers OsKeyStore READY_BASE + doctor `os_keyring_*` but does **not** clear this gate (swap/hibernate/snapshot still open). Spec **032** adds existence/leftover privacy **probes** (`probes_present=true`; residual classes listed open) but does **not** clear this gate. Spec **043** adds honesty classification (`swap_snapshot_honesty_present`; existence vs protection; never `protection_measured` for these surfaces) but does **not** clear this gate. Spec 023 SQLCipher likewise did not clear it. Continue independent work; do not claim private-data readiness. |
| PUBLIC_SOURCE_LICENSE_CHOICE | `DECIDED` | Public SPDX license for MedScale crates/distribution | Founder selected `Apache-2.0` effective 2026-09-14 with `NOTICE_REQUIRED_IN_PACKAGE=true`. Keep crates `publish = false`; bind the license into metadata, NOTICE generation, SBOM/release manifests, and qualification evidence. This decision alone does **not** establish `RELEASE_READY`. |
| REPO_BRANCH_PROTECTION_REQUIRED_CHECKS | `CONFIGURED` | GitHub branch protection, required status checks, and related repository rulesets for `main` | Founder-authorized ruleset `protect-main` (`id=23259329`) is active and targets `refs/heads/main`; GitHub reports `main protected=true`. All six exact required checks are configured, deletion/non-fast-forward are blocked, PR + conversation resolution are required, and bypass actors are empty. PR #96 was observed `BLOCKED` while required checks were pending. This closes this gate only; `RELEASE_READY` remains false. |
| GATED_MODEL_TERMS | `NOT_GRANTED_BY_DEFAULT` | Model requiring click-through/credential acceptance | Do not accept terms; use admissible public/local comparator and continue. |
| PRODUCTION_CREDENTIALS | `NOT_GRANTED` | Real cloud/provider/partner credentials | Build/test with mocks/synthetic endpoints; continue. |
| PARTNER_EHR_NPHIES_ENDPOINT | `NOT_GRANTED` | Real integration qualification (live partner FHIR/SMART/EHR/NPHIES hosts) | Spec 013 ships stub adapters + fixture receipts only; refuse LivePartner mode; continue. |
| SMART_LIVE_PARTNER_AUTHORIZATION | `NOT_GRANTED` | Live SMART-on-FHIR app registration / authorize/token against real partners | Use FixtureStub SMART adapter + synthetic envelopes in Spec 013; do not register live SMART apps or contact partner hosts. |
| DESKTOP_RELEASE_SIGNING_PROVENANCE | `NOT_GRANTED` | Production desktop artifact/SBOM/checksum signing and provenance | Spec 058 unsigned portable packages + checksum/SBOM verification are complete. External action: owner provisions Authenticode/Apple/Linux signing identity fingerprints in a private store; no production secrets in repo. This maps `release_sbom_signing_provenance` and `checksums_provenance_signing_verification`. |
| QUALIFIED_RELEASE_PERFORMANCE_HARDWARE | `NOT_PROVISIONED` | Trusted V1 budget attainment on declared qualified Windows/Linux/macOS hardware | Spec 057 provides full measurement coverage; hosted CI remains a feasibility signal only. External action: owner declares/provisions hardware matrix, then run the locked 30-run release-profile harness and commit bound p50/p95/peak evidence. This maps `perf_budgets_attained_on_qualified_hardware`. |
| MACOS_SIGNED_PRODUCT_QUALIFICATION | `NOT_GRANTED` | Final macOS product qualification, signing/notarization and measured App Sandbox enforcement | Entitlements/probes and portable package are ready; Developer ID/notary credentials and final product shell are absent. External action: provide signing/notarization identity and final shell, then run codesign/notary/App Sandbox/product privacy qualification. This maps `macos_platform_product_qualification`. |
| FINAL_V0_UI_ACCESSIBILITY_QUALIFICATION | `IMPLEMENTATION_IN_PROGRESS` | Final native UI accessibility / WCAG qualification | Founder-approved visual direction was supplied on 2026-09-15 and Spec 060 begins the native Slint implementation. Keep `wcag_final_v0_ui_accessibility_qualification` open until keyboard, screen-reader, contrast, text-overflow, and final-product assistive-technology evidence is qualified. |
| APP_STORE_SIGNING_RELEASE | `NOT_GRANTED` | Final public mobile signing/submission | Build unsigned/dev-test artifacts and all pre-release qualification possible. Spec **039** `evidence/022-release-qualification-prep/SIGNING_PROVENANCE_PREP.md` documents per-platform sign/notarize verification without credentials. |
| FINAL_V0_UI_ARTIFACT | `SUPPLIED_2026_09_15` | Final visual direction | Founder supplied and approved MedScale brand, Desktop, and CLI visual direction; `PRODUCT.md` + `DESIGN.md` normalize it for native implementation. Spec 060+ may implement it without inventing a conflicting visual system. |
| TAURI_WEBVIEW_PRIVACY_QUALIFICATION | `DEFERRED` | Admit Tauri/WebView as Desktop DEPENDENCY | Spec 006 does **not** admit Tauri v2.11.5. Keep SOURCE candidate pin; require WebView cache/crash/log PHI-containment proof before DEPENDENCY admission; otherwise use a safer shell. Record limitations in Spec 006 `PRIVACY_PROOF`. |
| TERMINOLOGY_SNOMED_LICENSE | `OPEN` | SNOMED CT Pack rights for MedScale distribution | Spec 007 starts track; do not copy tables from OpenMed; continue research/impl without Pack until counsel/pack ready. |
| TERMINOLOGY_LOINC_LICENSE | `OPEN` | LOINC Pack rights | Same as SNOMED track discipline. |
| TERMINOLOGY_ICD_LICENSE | `OPEN` | ICD Pack rights | Same as SNOMED track discipline. |
| TERMINOLOGY_ATC_LICENSE | `OPEN` | ATC Pack rights | Same as SNOMED track discipline. |
| TERMINOLOGY_UMLS_LICENSE | `OPEN` | UMLS/Athena Pack rights | Same as SNOMED track discipline. |
| WORKER_OS_SANDBOX_PLATFORM_QUALIFIED | `OPEN` | Landlock / AppContainer / Seatbelt PLATFORM_QUALIFIED for workers | Spec 026: Linux Landlock **ReadyBaseMeasured**. Spec **052**: Linux Landlock FS+TCP+rlimit **composition** ReadyBaseMeasured (`linux_landlock_composition_measured`). Spec **053**: Linux seccomp-bpf strict allowlist composition **ReadyBaseMeasured** (`linux_seccomp_composition_measured`; x86_64/aarch64 child-measured SIGSYS deny; parent/worker-spawn confinement still open). Spec 030: Windows Job Object **ReadyBaseMeasured**. Spec 031: macOS Seatbelt **ReadyBaseMeasured**. Spec **033**: Windows AppContainer FS **ReadyBaseMeasured**. Spec **038**: Windows AppContainer network **ReadyBaseMeasured**. Spec **040**: Windows AppContainer LPAC **ReadyBaseMeasured**. Spec **041**: macOS App Sandbox entitlements artifact/probe (`macos_app_sandbox_entitlements_measured`; enforcement still signing-gated). Spec **044**: composition inventory present (`composition_inventory_present`) listing residuals; does **not** clear this gate. Multi-OS PLATFORM_QUALIFIED + signed App Sandbox enforcement still required. Do **not** treat ReadyBaseMeasured as gate close. |
| MESC_RELEASED_ARTIFACT | `OPTIONAL_NOT_AVAILABLE` | Optional Spec 012 MESC artifact lane (hash/rights/SBOM); never a MedScale completion or release blocker | Re-check historically: MESC main `4b193c01â¦` TRAINING_CODE_READY=YES but RELEASE_STATUS=BLOCKED; v0.1.0 release_id 352847712 assets=[]; v0.2.0 tag has no Release. See evidence/012-mesc-artifact-integration/GATE_CHECK.md. Spec **036** ships synthetic verifier READY_BASE (`verifier_ready_base=true`; fixtures under evidence/012â¦/fixtures) but does **not** clear this gate. Fail-closed `MescArtifactAdmit` remains; residual = `BLOCKED_BY_UPSTREAM_MESC_RELEASE_ASSETS`. |
| SPEC_014_WORKFLOW_EVIDENCE | `PENDING` | Selected NPHIES/workflow profiles + terminology evidence | Spec 014 READY_BASE closed; NphiesInvoke remains ExternalGateRequired until this gate + PARTNER_EHR_NPHIES_ENDPOINT. |
| HF_ONLINE_PACK_DISTRIBUTION | `NOT_GRANTED` | Hugging Face / online pack publish+fetch credentials and terms | Spec 015 READY_BASE deny path closed; OnlinePackAcquire remains ExternalGateRequired; offline Pack v0 only. |

Cursor may add rows only when a blocker truly requires external authority. Never use this file for ordinary engineering uncertainty.

## Optional integration gates (never MedScale release-blocking)

| Gate | Current state | Impact |
|---|---|---|
| MESC_ARTIFACT_AVAILABLE | `false` | Optional MESC integration unavailable; MedScale core impact = none; release impact = none. |

`MEDSCALE_RELEASE_READY` depends on MedScale-owned requirements only. `MESC_ARTIFACT_AVAILABLE=false` must not force `MEDSCALE_RELEASE_READY=false`.

## Whole-product qualification refinement (2026-09-09)

Observed 2026-09-09: MESC v0.1.0 has no release assets; a v0.2.0 tag is insufficient.
Spec 012 is DEFERRED_BY_CANONICAL_DESIGN as an optional integration; MESC absence never blocks MedScale completion or release; see [acceptance contract](MESC_ARTIFACT_ACCEPTANCE.md).
OS sandbox/key/ownership qualification includes ordinary engineering work; lack of proof
blocks the affected readiness claim, not independent specification work. Repository required
checks/protection are now configured by founder-authorized ruleset `23259329`; `main` is protected,
all six required checks are enforced, and bypass actors are empty. Historical reviews that observed
an unprotected branch remain historical evidence only; no required gate may be bypassed.
