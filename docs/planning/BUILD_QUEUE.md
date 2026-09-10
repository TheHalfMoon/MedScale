# BUILD_QUEUE.md — MedScale Autonomous Build Queue

**Queue owner:** repository canonical plan  
**Execution agent:** Cursor  
**Rule:** update this file whenever a unit enters or leaves a canonical state.

**Autonomous stop status (historical V2 scoped closure):** superseded for Trusted V1 follow-on work.
**Live follow-on status (2026-09-10):** Spec **016**–**030** `CLOSED_CANONICAL`. Spec **030** closes Q09 residual Windows Job Object OS sandbox READY_BASE (`windows_measured=true`; AppContainer still scaffold; `platform_qualified=false`; EXTERNAL_GATES `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED` remains OPEN — macOS Seatbelt still NotPlatformQualified). Spec **029** closes macOS CI matrix expansion + CLI/fixture accessibility honesty as READY_BASE (`macos_ci_present=true`; `macos_qualified=false`; accessibility READY_BASE without WCAG; `RELEASE_READY=false`). Spec **028** closes Q03 residual OS keyring custody as READY_BASE (`os_keyring_available` / `os_keyring_used`; `PRIVATE_DATA_READY=false`; swap/hibernate/snapshot still open). Spec **027** closes Q05 release-evidence remnants: perf harness + SBOM scaffold + package checksums READY_BASE (`perf_harness_present` / `sbom_scaffold_present`; budgets not claimed; `RELEASE_READY=false`). Spec **026** closes Q09 Pack signer/trust/anti-rollback + Linux OS sandbox READY_BASE (`pack_signer` / `os_sandbox` doctor axes; `linux_measured=true`; `platform_qualified=false`; EXTERNAL_GATES `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED` remains OPEN). Spec **025** closes Q10 source-versioned synthetic lexical evidence corpus as READY_BASE (`evidence_corpus.ready_base=true`; clinical quality / `RELEASE_READY` still FALSE). Spec **024** closes Q04 residual host OS IPC authority as READY_BASE (`os_ipc_qualified=true`; `MULTI_CLIENT_RELEASE_READY=false`). Spec **023** closes Q03 residual open-metadata privacy (SQLCipher EncryptedVault) as READY_BASE; `PRIVATE_DATA_READY` remains FALSE. Spec 022 is READY_BASE **prep** for Q05 (`RELEASE_READY = FALSE`). Spec 012 remains MESC-blocked. Deferred advanced work is **031+**. Do **not** claim `RELEASE_READY`, `PRIVATE_DATA_READY`, or `MULTI_CLIENT_RELEASE_READY`. Q03 is **not** fully complete for PRIVATE_DATA_READY (swap / snapshot still open after Spec 028 OS keyring READY_BASE).

## 2026-09-10 planning refinement

See Trusted V1 delivery plan. Specs 018–030 READY_BASE closed (022 = Q05 prep only; 023 = Q03 open-metadata SQLCipher; 024 = Q04 OS IPC; 025 = Q10 evidence corpus; 026 = Q09 pack signer + Linux sandbox; 027 = Q05 perf/SBOM/checksum scaffolds; 028 = Q03 OS keyring custody; 029 = macOS CI + accessibility honesty; 030 = Q09 Windows Job Object sandbox). Spec 012 MESC-blocked. Advanced **031+** deferred.

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
| 012 | MESC Artifact Integration | `BLOCKED_BY_RELEASED_MESC_ARTIFACT` | Fail-closed admit + doctor axis shipped; ARTIFACT_IMPORT still blocked (empty MESC release assets). |
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
| 031+ | Advanced deferred work | `DEFERRED_BY_CANONICAL_DESIGN` | Plugins/GraphRAG/replicas/imaging/CUDA/etc. |

## Automatic progression

For the first `READY` unit: create/complete its Spec Kit package, analyze it, implement tasks in dependency order, qualify exact head, converge, merge if all required gates pass, mark `CLOSED_CANONICAL`, recompute this queue, and immediately start the next eligible unit.

Do not stop merely because a PR merged, one milestone passed, or an external optional gate exists.

**Next eligible (honest):** Spec 012 MESC-blocked. PRIVATE_DATA_READY still blocked by swap / hibernate / snapshot qualification (OS keyring READY_BASE landed in Spec 028). Branch protection / required checks need owner settings. macOS Seatbelt OS sandbox + product PLATFORM_QUALIFIED and WCAG/final-v0 accessibility remain open after Spec 029/030. Deferred advanced **031+**. Do **not** claim `RELEASE_READY`, `PRIVATE_DATA_READY`, or `MULTI_CLIENT_RELEASE_READY`. Remaining ordinary Trusted V1 in-repo work may still include measured OS privacy follow-ons (swap/snapshot) and macOS sandbox qualification; do not treat Q03 or Q09 multi-OS as fully exhausted.
