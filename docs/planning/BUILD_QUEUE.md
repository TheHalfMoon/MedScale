# BUILD_QUEUE.md — MedScale Autonomous Build Queue

**Queue owner:** repository canonical plan  
**Execution agent:** Cursor  
**Rule:** update this file whenever a unit enters or leaves a canonical state.

**Autonomous stop status (historical V2 scoped closure):** superseded for Trusted V1 follow-on work.
**Live follow-on status (2026-09-10):** Specs **016**–**040** `CLOSED_CANONICAL` READY_BASE on the Trusted V1 residual track (Spec **040** = Windows AppContainer LPAC ReadyBaseMeasured). Spec **041** (macOS App Sandbox entitlements ReadyBaseMeasured) promoted as existing Q09 residual — see `SPEC_041_PROMOTION.md`. Spec **012** remains MESC-blocked. Honesty: `PRIVATE_DATA_READY=false`; `RELEASE_READY=false`; sandbox `platform_qualified=false`. Deferred advanced product work is **042+**.

## 2026-09-10 planning refinement

See Trusted V1 delivery plan. Specs 018–040 READY_BASE closed on residual track; Spec 012 MESC-blocked. Spec **041** App Sandbox entitlements promoted (`EXISTING_Q09_RESIDUAL_ELIGIBLE_FOR_PROMOTION`). Advanced **042+** deferred.

## Historical scoped queue (closures preserved)

| Order | Spec | State | Next action |
|---:|---|---|---|
| 000 | Constitution + Source Authority | `CLOSED_CANONICAL` | Planning V2/source authority already canonicalized; Spec 001 materializes it into the Spec Kit repository structure without reopening founder decisions. |
| 001 | Rust Repository + Spec Kit Bootstrap | `CLOSED_CANONICAL` | Spec Kit + Rust workspace + CI/supply-chain bootstrap merged; continue Spec 002. |
| 002 | Trusted Object / Source / Authority + Process/Text Foundation | `CLOSED_CANONICAL` | Object/authority foundation merged; continue Spec 003. |
| 003 | H0-A Trusted Ingest + Durability | `CLOSED_CANONICAL` | Synthetic FHIR R4 ingest + durability merged; continue Spec 004. |
| 004 | H0-B Trusted Presentation + Coverage | `CLOSED_CANONICAL` | Deterministic timeline/Brief/coverage merged; continue Spec 005. |
| 005 | Local Private Vault + Encryption + Recovery | `CLOSED_CANONICAL` | Encrypted vault + recovery merged; continue Spec 006. |
| 006 | CLI + Desktop Foundation | `CLOSED_CANONICAL` | CLI wedge + non-WebView desktop scaffold merged; continue eligible units. |
| 007 | OpenMed Absorption / Parity Research | `CLOSED_CANONICAL` | OpenMed v2.2 pin + parity matrix/dispositions merged; Spec 008 unblocked for fabric. |
| 008 | Local AI Capability Fabric | `CLOSED_CANONICAL` | Offline Pack v0 + worker ambient-deny merged; OS sandbox PLATFORM_QUALIFIED remains OPEN gate. |
| 009 | Mobile iOS + Android | `CLOSED_CANONICAL` | READY_BASE: doctor/mobile axes, keystore sync forbid, FFI stubs; no apps. |
| 010 | Documents + OCR + Voice | `CLOSED_CANONICAL` | MIME quarantine + OCR/ASR stubs merged; real engines deferred. |
| 011 | Evidence / Retrieval / Medical Intelligence | `CLOSED_CANONICAL` | Lexical retrieval to evidence-only EvaluationRecords; relevance is not authority. |
| 012 | MESC Artifact Integration | `BLOCKED_BY_RELEASED_MESC_ARTIFACT` | Fail-closed admit + doctor axis shipped; Spec **036** verifier READY_BASE; ARTIFACT_IMPORT still blocked (empty MESC release assets). |
| 013 | FHIR / SMART / Network Broker | `CLOSED_CANONICAL` | Fail-closed broker + stub SMART/FHIR adapters merged; continue Spec 014 when workflow evidence ready. |
| 014 | Controlled Actions / NPHIES | `CLOSED_CANONICAL` | READY_BASE: outbox + payload-bound intents; NPHIES remains external gate. |
| 015 | HF + Online Pack Ecosystem | `CLOSED_CANONICAL` | READY_BASE deny path via Network Broker; HF online remains external gate. |
| 016 | Durable Trusted Record (Q02) | `CLOSED_CANONICAL` | Full authority object graph persists across process restart; see evidence/016. |
| 017 | Vault Privacy Qualification (Q03) | `CLOSED_CANONICAL` | Work/WAL wipe + doctor honesty; PRIVATE_DATA_READY remains FALSE. |
| 018 | Host / Client Authority (Q04) | `CLOSED_CANONICAL` | READY_BASE: in-process SessionRegistry; MULTI_CLIENT_RELEASE_READY=false. |
| 019 | Record Semantics (Q06) | `CLOSED_CANONICAL` | READY_BASE: precision-aware MedicalTime, AmendAssertion, missingness, identity unresolved; RELEASE_READY=false. |
| 020 | FHIR Interchange Qualification (Q08) | `CLOSED_CANONICAL` | READY_BASE: honest support matrix + loss-aware export; no full conformance; RELEASE_READY=false. |
| 021 | Minimum Lovable Workflow (Q07) | `CLOSED_CANONICAL` | READY_BASE: synthetic restartable import-review-export-backup journey + disclosure; RELEASE_READY=false. |
| 022 | Release Qualification Prep (Q05) | `CLOSED_CANONICAL` | READY_BASE prep: locked CI, evidence binding, doctor honesty; RELEASE_READY=false; branch protection EXTERNAL_GATES. |
| 023 | Vault Open-Metadata Privacy (Q03 residual) | `CLOSED_CANONICAL` | READY_BASE: SQLCipher page-encrypted EncryptedVault open work; PRIVATE_DATA_READY=false (OS key/swap/snapshot). |
| 024 | Host OS IPC Authority (Q04 residual) | `CLOSED_CANONICAL` | READY_BASE: localhost OS IPC + strict sessions; os_ipc_qualified=true; MULTI_CLIENT_RELEASE_READY=false. |
| 025 | Evidence Corpus Lifecycle (Q10) | `CLOSED_CANONICAL` | READY_BASE: versioned synthetic-lexical corpus + lexical filters; clinical quality / RELEASE_READY=false. |
| 026 | Pack Signer + OS Sandbox (Q09) | `CLOSED_CANONICAL` | READY_BASE: synthetic pack signer/anti-rollback + Linux Landlock measured; platform_qualified=false; WORKER_OS_SANDBOX gate OPEN. |
| 027 | Perf Harness + Package/SBOM Evidence (Q05 remnants) | `CLOSED_CANONICAL` | READY_BASE: perf harness + SBOM scaffold + checksums; budgets not claimed; RELEASE_READY=false. |
| 028 | OS Keyring Custody (Q03 residual) | `CLOSED_CANONICAL` | READY_BASE: OsKeyStore + doctor os_keyring_*; PRIVATE_DATA_READY=false (swap/snapshot). |
| 029 | macOS CI + Accessibility Honesty | `CLOSED_CANONICAL` | READY_BASE: macos_ci_present; accessibility fixture/CLI honesty; macos_qualified=false; RELEASE_READY=false; no WCAG claim. |
| 030 | Windows Job Object Sandbox (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: Job Object ReadyBaseMeasured (`windows_measured`); AppContainer scaffold; platform_qualified=false; WORKER_OS_SANDBOX gate OPEN. |
| 031 | macOS Seatbelt Sandbox (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: Seatbelt sandbox_init ReadyBaseMeasured (`macos_measured`); App Sandbox entitlements scaffold; platform_qualified=false; WORKER_OS_SANDBOX gate OPEN. |
| 032 | Privacy Probes + NOTICE + Perf Binding | `CLOSED_CANONICAL` | READY_BASE: probes_present + NOTICE inventory + perf binding; PRIVATE_DATA_READY=false; rights_license_decision=false; budgets not claimed. |
| 033 | Windows AppContainer FS (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: AppContainer child FS deny (windows_appcontainer_fs_measured); network/LPAC scaffold; platform_qualified=false; WORKER_OS_SANDBOX gate OPEN. |
| 034 | Durable Outbox Restart (Q12 residual) | `CLOSED_CANONICAL` | READY_BASE: SyntheticVault outbox reload + UNKNOWN reconcile; `outbox_restart_qualified`; NPHIES gated. |
| 035 | EncryptedVault Authority Sync (Q02/Q03 residual) | `CLOSED_CANONICAL` | READY_BASE: EncryptedVault persists/reloads authority graph; `encrypted_authority_sync_qualified`; PRIVATE_DATA_READY=false. |
| 036 | MESC Synthetic Verifier (012 residual) | `CLOSED_CANONICAL` | READY_BASE: synthetic manifest verifier + fixtures; `verifier_ready_base=true`; Spec 012 / MESC_RELEASED_ARTIFACT still NOT_AVAILABLE. |
| 037 | Release Prep + Transport Fail Fixtures | `CLOSED_CANONICAL` | READY_BASE: REQUIRED_CHECKS packet + checksum verify + license counsel packet + broker transport fail fixtures; RELEASE_READY=false. |
| 038 | Windows AppContainer Network (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: AppContainer TCP deny (`windows_appcontainer_network_measured`); LPAC scaffold; platform_qualified=false. |
| 039 | Release Honesty Packets | `CLOSED_CANONICAL` | READY_BASE: START_HERE/SPECKIT sync + PHI readiness checklist + signing/provenance prep + unsigned release-manifest scaffold. |
| 040 | Windows AppContainer LPAC (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: LPAC ReadyBaseMeasured (`windows_appcontainer_lpac_measured`); platform_qualified=false; sandbox gate OPEN. |
| 041 | macOS App Sandbox Entitlements (Q09 residual) | `READY` | Entitlements artifact + detection probe ReadyBaseMeasured; enforcement_measured=false (signing external); platform_qualified=false. See `SPEC_041_PROMOTION.md`. |
| 042+ | Advanced deferred work | `DEFERRED_BY_CANONICAL_DESIGN` | Plugins/GraphRAG/replicas/imaging/CUDA/etc. |

## Automatic progression

For the first `READY` unit: create/complete its Spec Kit package, analyze it, implement tasks in dependency order, qualify exact head, converge, merge if all required gates pass, mark `CLOSED_CANONICAL`, recompute this queue, and immediately start the next eligible unit.

Do not stop merely because a PR merged, one milestone passed, or an external optional gate exists.

**Next eligible (honest):** Spec **041** macOS App Sandbox entitlements ReadyBaseMeasured (`READY`). Spec 012 MESC-blocked. After 041: multi-OS PLATFORM_QUALIFIED composition, delivery-plan perf scale, swap/snapshot PRIVATE_DATA_READY, branch protection, SPDX, signing credentials, WCAG/final-v0, MESC assets. Deferred advanced **042+**. Do **not** claim `RELEASE_READY`, `PRIVATE_DATA_READY`, or `MULTI_CLIENT_RELEASE_READY`.
