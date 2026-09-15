# BUILD_QUEUE.md â€” MedScale Autonomous Build Queue

**Queue owner:** repository canonical plan  
**Execution agent:** Cursor  
**Rule:** update this file whenever a unit enters or leaves a canonical state.

**Autonomous stop status (historical V2 scoped closure):** superseded for Trusted V1 follow-on work.
**Live follow-on status (2026-09-15):** Specs **016**–**067** are `CLOSED_CANONICAL`; Spec **012** is `DEFERRED_BY_CANONICAL_DESIGN` as optional MESC integration. Trusted V1 and the separately promoted Desktop+CLI implementation phase are repository-complete pending external gates. Honesty: `PRIVATE_DATA_READY=false`; `RELEASE_READY=false`; `budgets_claimed_met=false`; sandbox `platform_qualified=false`.

## 2026-09-11 planning refinement

See Trusted V1 delivery plan. Specs 018–067 are closed; Spec 012 remains optional/deferred. Specs 060–067 formed the promoted Desktop+CLI product-launch implementation phase and are now closed.

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
| 012 | MESC Artifact Integration (optional) | `DEFERRED_BY_CANONICAL_DESIGN` | Fail-closed admit + doctor optional-integration axis shipped; Spec **036** verifier READY_BASE; ARTIFACT_IMPORT lane gated on a real upstream release; MESC absence never blocks core, completion, or release. |
| 013 | FHIR / SMART / Network Broker | `CLOSED_CANONICAL` | Fail-closed broker + stub SMART/FHIR adapters merged; continue Spec 014 when workflow evidence ready. |
| 014 | Controlled Actions / NPHIES | `CLOSED_CANONICAL` | READY_BASE: outbox + payload-bound intents; NPHIES remains external gate. |
| 015 | HF + Online Pack Ecosystem | `CLOSED_CANONICAL` | READY_BASE deny path via Network Broker; HF online remains external gate. |
| 016 | Durable Trusted Record (Q02) | `CLOSED_CANONICAL` | Full authority object graph persists across process restart; see evidence/016. |
| 017 | Vault Privacy Qualification (Q03) | `CLOSED_CANONICAL` | Work/WAL wipe + doctor honesty; PRIVATE_DATA_READY remains FALSE. |
| 018 | Host / Client Authority (Q04) | `CLOSED_CANONICAL` | READY_BASE: in-process SessionRegistry; MULTI_CLIENT_RELEASE_READY=false. |
| 019 | Record Semantics (Q06) | `CLOSED_CANONICAL` | READY_BASE: precision-aware MedicalTime, AmendAssertion, missingness, identity unresolved; RELEASE_READY=false. |
| 020 | FHIR Interchange Qualification (Q08) | `CLOSED_CANONICAL` | READY_BASE: honest support matrix + loss-aware export; no full conformance; RELEASE_READY=false. |
| 021 | Minimum Lovable Workflow (Q07) | `CLOSED_CANONICAL` | READY_BASE: synthetic restartable import-review-export-backup journey + disclosure; RELEASE_READY=false. |
| 022 | Release Qualification Prep (Q05) | `CLOSED_CANONICAL` | READY_BASE prep: locked CI, evidence binding, doctor honesty; RELEASE_READY=false; branch protection was external at closure and is now CONFIGURED by ruleset 23259329. |
| 023 | Vault Open-Metadata Privacy (Q03 residual) | `CLOSED_CANONICAL` | READY_BASE: SQLCipher page-encrypted EncryptedVault open work; PRIVATE_DATA_READY=false (OS key/swap/snapshot). |
| 024 | Host OS IPC Authority (Q04 residual) | `CLOSED_CANONICAL` | READY_BASE: localhost OS IPC + strict sessions; os_ipc_qualified=true; MULTI_CLIENT_RELEASE_READY=false. |
| 025 | Evidence Corpus Lifecycle (Q10) | `CLOSED_CANONICAL` | READY_BASE: versioned synthetic-lexical corpus + lexical filters; clinical quality / RELEASE_READY=false. |
| 026 | Pack Signer + OS Sandbox (Q09) | `CLOSED_CANONICAL` | READY_BASE: synthetic pack signer/anti-rollback + Linux Landlock measured; platform_qualified=false; WORKER_OS_SANDBOX gate OPEN. |
| 027 | Perf Harness + Package/SBOM Evidence (Q05 remnants) | `CLOSED_CANONICAL` | READY_BASE: perf harness + SBOM scaffold + checksums; budgets not claimed; RELEASE_READY=false. |
| 028 | OS Keyring Custody (Q03 residual) | `CLOSED_CANONICAL` | READY_BASE: OsKeyStore + doctor os_keyring_*; PRIVATE_DATA_READY=false (swap/snapshot). |
| 029 | macOS CI + Accessibility Honesty | `CLOSED_CANONICAL` | READY_BASE: macos_ci_present; accessibility fixture/CLI honesty; macos_qualified=false; RELEASE_READY=false; no WCAG claim. |
| 030 | Windows Job Object Sandbox (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: Job Object ReadyBaseMeasured (`windows_measured`); AppContainer scaffold; platform_qualified=false; WORKER_OS_SANDBOX gate OPEN. |
| 031 | macOS Seatbelt Sandbox (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: Seatbelt sandbox_init ReadyBaseMeasured (`macos_measured`); App Sandbox entitlements scaffold; platform_qualified=false; WORKER_OS_SANDBOX gate OPEN. |
| 032 | Privacy Probes + NOTICE + Perf Binding | `CLOSED_CANONICAL` | READY_BASE: probes_present + NOTICE inventory + perf binding; PRIVATE_DATA_READY=false; rights license was undecided at closure and is now Apache-2.0; budgets not claimed. |
| 033 | Windows AppContainer FS (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: AppContainer child FS deny (windows_appcontainer_fs_measured); network/LPAC scaffold; platform_qualified=false; WORKER_OS_SANDBOX gate OPEN. |
| 034 | Durable Outbox Restart (Q12 residual) | `CLOSED_CANONICAL` | READY_BASE: SyntheticVault outbox reload + UNKNOWN reconcile; `outbox_restart_qualified`; NPHIES gated. |
| 035 | EncryptedVault Authority Sync (Q02/Q03 residual) | `CLOSED_CANONICAL` | READY_BASE: EncryptedVault persists/reloads authority graph; `encrypted_authority_sync_qualified`; PRIVATE_DATA_READY=false. |
| 036 | MESC Synthetic Verifier (012 residual) | `CLOSED_CANONICAL` | READY_BASE: synthetic manifest verifier + fixtures; `verifier_ready_base=true`; MESC artifact NOT_AVAILABLE with no core/completion/release impact (Spec 012 optional). |
| 037 | Release Prep + Transport Fail Fixtures | `CLOSED_CANONICAL` | READY_BASE: REQUIRED_CHECKS packet + checksum verify + license counsel packet + broker transport fail fixtures; RELEASE_READY=false. |
| 038 | Windows AppContainer Network (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: AppContainer TCP deny (`windows_appcontainer_network_measured`); LPAC scaffold; platform_qualified=false. |
| 039 | Release Honesty Packets | `CLOSED_CANONICAL` | READY_BASE: START_HERE/SPECKIT sync + PHI readiness checklist + signing/provenance prep + unsigned release-manifest scaffold. |
| 040 | Windows AppContainer LPAC (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: LPAC ReadyBaseMeasured (`windows_appcontainer_lpac_measured`); platform_qualified=false; sandbox gate OPEN. |
| 041 | macOS App Sandbox Entitlements (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: entitlements artifact + detection probe; `macos_app_sandbox_entitlements_measured`; enforcement_measured=false; platform_qualified=false. |
| 042 | Perf Delivery-Plan Scale (Q05 residual) | `CLOSED_CANONICAL` | READY_BASE: near-delivery-plan harness (CI 1000 events + 1MiB FHIR) + Windows CI job; host may set TIMELINE_EVENTS=10000; budgets_claimed_met=false; lexical 10k still limited. |
| 043 | Swap/Snapshot Honesty (Q03 residual) | `CLOSED_CANONICAL` | READY_BASE: classified swap/pagefile/hibernate/snapshot/core-dump existence vs protection; PRIVATE_DATA_READY=false. |
| 044 | Sandbox Composition Honesty (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: composition inventory present; residuals listed; platform_qualified=false; sandbox gate OPEN. |
| 045 | Lexical 10k Corpus (Q10/Q05 residual) | `CLOSED_CANONICAL` | READY_BASE: procedural synthetic-lexical-scale@10k + harness wiring; clinical_quality/budgets/RELEASE_READY false. |
| 046 | SBOM Lock Binding (Q05 residual) | `CLOSED_CANONICAL` | Cargo.lock digest in SBOM scaffold; sbom_lock_bound; not full release SBOM. See `SPEC_046_PROMOTION.md`. |
| 047 | Release Dry-Run + Verifier (Q05 residual) | `CLOSED_CANONICAL` | READY_BASE: dry-run binds source/tree/lock + build env + native-deps honesty; cross-verifier present; RELEASE_READY=false. See `SPEC_047_PROMOTION.md`. |
| 048 | Migration/Recovery Release-Bar (Q05 residual) | `CLOSED_CANONICAL` | READY_BASE: interrupt fail-closed + backup/restore recovery + encrypted backup closure; migration_recovery_ready_base; RELEASE_READY=false. See `SPEC_048_PROMOTION.md`. |
| 049 | Package Upgrade/Rollback Scaffold (Q05 residual) | `CLOSED_CANONICAL` | READY_BASE: unsigned dry-run upgrade/rollback scaffold; real portable package lifecycle proof later closed by Spec 058. See `SPEC_049_PROMOTION.md`. |
| 050 | Host Perf Measurement Path (Q05 residual) | `CLOSED_CANONICAL` | READY_BASE: operator host-bound perf path + binding sidecar; host_perf_measurement_path_present; budgets not claimed. See `SPEC_050_PROMOTION.md`. |
| 051 | REQUIRED_CHECKS Live CI Sync (Q05 residual) | `CLOSED_CANONICAL` | READY_BASE: owner packet lists six live CI jobs incl. perf delivery-plan scale; required_checks_packet_synced. Post-close gate now CONFIGURED by active ruleset 23259329 targeting main; PR #96 enforcement observed. See `SPEC_051_PROMOTION.md`. |
| 052 | Linux Landlock Composition (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: Landlock FS allowlist + TCP deny + RLIMIT_NOFILE composition measured (`linux_landlock_composition_measured`); seccomp still open; platform_qualified=false; sandbox gate OPEN. See `SPEC_052_PROMOTION.md`. |
| 053 | Linux Seccomp Composition (Q09 residual) | `CLOSED_CANONICAL` | READY_BASE: seccomp-bpf strict allowlist + child SIGSYS deny measured (`linux_seccomp_composition_measured`); x86_64/aarch64 tables; platform_qualified=false; sandbox gate OPEN. See `SPEC_053_PROMOTION.md`. |
| 054 | Native / Full Release SBOM Qualification (Q05 residual) | `CLOSED_CANONICAL` | READY_BASE: deterministic CycloneDX 1.5 + verifier (tamper/stale/missing fail closed); `release_sbom_qualified`; reproducible_build=unproven; RELEASE_READY=false. See `SPEC_054_PROMOTION.md`. |
| 055 | Perf Attainment Dossier (Q05 residual) | `CLOSED_CANONICAL` | READY_BASE dossier: PERFORMANCE_NON_ATTAINMENT bound to source/tree/lock; budgets not claimed. See `SPEC_055_PROMOTION.md`. |
| 056 | Fixture A11y Semantics (a11y residual) | `CLOSED_CANONICAL` | READY_BASE deepening: role/keyboard/announcement semantics on every fixture view model; no WCAG claim. See `SPEC_056_PROMOTION.md`. |
| 057 | Release Qualification Residual Integrity (Q05 residual) | `CLOSED_CANONICAL` | Fresh audit: complete cold-launch/idle-memory coverage + immutable Action pin enforcement; budgets/RELEASE_READY remain false. See `SPEC_057_PROMOTION.md`. |
| 058 | Portable Release Package Qualification (Q05 residual) | `CLOSED_CANONICAL` | Deterministic unsigned portable package + real install/upgrade/rollback lifecycle proof qualified on Windows/Linux/macOS; signing/native installers/RELEASE_READY remain open. See `SPEC_058_PROMOTION.md`. |
| 059 | Final Release Closure Audit (Q05 terminal residual) | `CLOSED_CANONICAL` | Repository-owned material findings cleared; remaining release residuals are mapped external-only. See `SPEC_059_PROMOTION.md`. |
| 060 | Native Desktop Design System + Command Center | `CLOSED_CANONICAL` | Native Slint shell/design system qualified on exact-head run `34909664814`, merged as `cb5dce0e…`, and post-merge run `34911182508` passed all six required jobs. |
| 061 | Patient Workspace + Longitudinal UX | `CLOSED_CANONICAL` | Exact-head `ee9e076f…` passed run `34916956439`, merged as `1438460e…`, and post-merge main run `34917999834` passed all six required jobs. |
| 062 | Population Insights + Assistant UX | `CLOSED_CANONICAL` | Exact-head `4c113b0b…` passed run `34920024022`, PR #106 merged as `dd3f6bd1…`, and post-merge main run `34920946581` passed all six required jobs. |
| 063 | Workflow Studio + Tasks/Messages | `CLOSED_CANONICAL` | Exact-head `375de163…` passed run `34922310943`, PR #107 merged as `26bc3cfe…`, and post-merge main run `34922935845` passed all six required jobs. |
| 064 | Audit + Exports + Settings + Integrations | `CLOSED_CANONICAL` | Exact-head `086891bf…` passed run `34924373669`, PR #108 merged as `eb8eec1e…`, and post-merge main run `34925062638` passed all six required jobs. |
| 065 | CLI Product Experience + Capability Parity | `CLOSED_CANONICAL` | Exact-head `06be6d09…` passed run `34933002914`, PR #109 merged as `11a9652c…`, and post-merge main run `34933737357` passed all six required jobs. |
| 066 | Desktop + CLI Hardening | `CLOSED_CANONICAL` | Exact-head `ed5cbbb…` passed run `34934782208`, PR #110 merged as `31e7c6c8…`, and post-merge main run `34935549696` passed all six required jobs. OpenCodeReview delegation supplemented manual Slint/docs review. |
| 067 | Final UI Product Qualification | `CLOSED_CANONICAL` | Exact-head `9b4eb259…` passed run `34936519051`, PR #111 merged as `92ff3773…`, and post-merge main run `34937549663` passed all six required jobs. OpenCodeReview delegation supplemented manual docs review. |
| 068 | Product Differentiation Rebuild | `IN_REVIEW` | Founder-reopened product bar: original MedScale identity, Models route, Evidence route, truthful OpenMed comparison. |
| 069 | Real Local Model Runtime + HF Pack Path | `PROMOTED_PENDING_068` | Admit real bounded inference behind worker/authority boundaries; explicit HF acquisition through Network Broker/pack provenance. |
| 070 | Model Center + Runtime UX | `PROMOTED_PENDING_069` | Make installed/available models, tasks, devices, provenance, trust, benchmark and promotion state visible. |
| 071 | OpenMed Evidence Center | `PROMOTED_PENDING_070` | Productize exact comparative evidence; prove or refuse every parity/surpass claim. |
| 072 | Product Requalification | `PROMOTED_PENDING_071` | Re-run product, accessibility, performance and release-candidate qualification on the rebuilt product. |
| 073+ | Advanced deferred work | `DEFERRED_BY_CANONICAL_DESIGN` | Plugins/GraphRAG/replicas/imaging/CUDA/mobile-after-launch/etc.; require fresh promotion. |

## Automatic progression

For the first `READY` unit: create/complete its Spec Kit package, analyze it, implement tasks in dependency order, qualify exact head, converge, merge if all required gates pass, mark `CLOSED_CANONICAL`, recompute this queue, and immediately start the next eligible unit.

Do not stop merely because a PR merged, one milestone passed, or an external optional gate exists.

**Mobile remains deferred until Desktop+CLI launch.** No mobile application implementation is authorized by Specs 068-072.

**Next eligible (honest):** Spec **068** is `IN_REVIEW` after explicit founder rejection of the prior product UI/positioning. Release qualification is paused. Follow 068 -> 069 -> 070 -> 071 -> 072 before forming a new release candidate. `RELEASE_READY`, `PRIVATE_DATA_READY`, and `MULTI_CLIENT_RELEASE_READY` remain false. Spec 012/MESC remains optional/deferred and untouched.

