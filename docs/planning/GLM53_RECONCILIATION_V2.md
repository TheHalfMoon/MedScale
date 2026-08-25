# MedScale GLM 5.3 Claim-by-Claim Reconciliation V2

**Date:** 2026-08-25  
**Status:** `CANONICAL_RECONCILIATION_V2`  
**Repository state:** `PLANNING_ARTIFACTS_CANONICAL / PRODUCT_IMPLEMENTATION_NOT_STARTED`  
**Implementation authorization:** NO

## Verdict

GLM 5.3’s `PLAN_READY_WITH_REQUIRED_CORRECTIONS` is accepted as the review outcome, but individual recommendations are not adopted automatically. The two material defects are closed at planning level as follows: (1) engine placement is corrected without banning safe bounded Rust parsing in the trusted core; (2) OpenMed parity is repinned to v2.2.0 / `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837` and the matrix is rebuilt.

## Findings reconciliation

| ID | Decision | Canonical V2 correction |
|---|---|---|
| F-01 | **MODIFY / ACCEPT CORE** | In-process FFI is not confinement. Introduce P0/P1/P2/P3 engine-placement classes. Complex hostile native/unsafe parsers/runtimes are P1 workers on desktop. Bounded memory-safe Rust FHIR/JSON parsing can remain P0. Mobile P2 exception is explicit and evidenced. |
| F-02 | **ACCEPT** | Pin OpenMed v2.2.0 exact commit; add terminology, structured privacy/SDC, document breadth, FHIR/OMOP, Unicode offsets, staged promotion, mobile/security rows; regrade Documents/FHIR/privacy “surpass” to parity-first/surpass-pending-evidence. |
| F-03 | **ACCEPT WITH NARROWING** | Canonical vault defaults to local app-data, not Documents/Desktop. Detect/refuse known remote/network/sync roots by default and surface `doctor` warnings. Do not overclaim OneDrive-specific corruption without direct proof; SQLite’s filesystem/locking requirements are the governing technical reason. |
| F-04 | **ACCEPT WITH NARROWING** | Add terminology rights/version/checksum program (SNOMED, LOINC, UCUM, ICD, ATC, UMLS). Absorb OpenMed snapshot-manifest semantics where suitable. Snowstorm is reference/optional adapter, not mandatory local server. NPHIES uses terminology bindings including UCUM/SNOMED in specific profiles; do not claim universal SNOMED requirement. |
| F-05 | **ACCEPT / DECIDE NOW** | Choose one Core Host per vault, single writer, local IPC clients, no direct UI/CLI DB access, no DB/key handle transfer. Mobile uses app-owned Rust core through same authority facade. |
| F-06 | **ACCEPT WITH CORRECTIONS** | Versioned native ABI, opaque handles, thread affinity, typed errors, no unwind across C, ownership tables, one SQLite copy/process. Android 16KB support is a mobile CI requirement; current Play enforcement date must be read from current Android docs, not frozen from GLM text. |
| F-07 | **ACCEPT** | Offline signed local pack import belongs to 008; online acquisition/publishing belongs to 015 and is gated on 013 Network Broker. |
| F-08 | **ACCEPT WITH STRONGER SPAN MODEL** | Raw bytes remain source identity; spans are tagged by representation/coordinate system. Prefer raw byte spans for source drilldown and Unicode-scalar indexes for derived text, with explicit UTF-16/grapheme conversions. Normalization is versioned derived data, never silent source mutation. |
| F-09 | **ACCEPT WITH PROTOTYPE GATE** | Tauri/WebView is a candidate only. Spec 006 must prove PHI does not persist outside the vault in caches/crash artifacts. If the platform cannot meet evidence, reject WebView for PHI surface rather than weakening privacy. |
| F-10 | **ACCEPT** | 003 proves synthetic-scope backup/restore interface and crash behavior; 005 proves production encryption/recovery/backup/key-loss/rotation UX before PHI. |
| F-11 | **ACCEPT** | sqlite-vec -> EXPERIMENTAL/REFERENCE until 011 maturity/performance/security/linkage proof. |
| F-12 | **ACCEPT** | Donor delta log mandatory. keyring integration updated to keyring-core + selected stores; Presidio stewardship transition recorded; medSpaCy restored as reference; Statig/OpenCR/iroh/Extism/OPA/OpenFGA/Qdrant/Trivy explicitly dispositioned. |
| F-13 | **ACCEPT DEMOTION / REJECT NEW DEFAULT** | mistral.rs is no longer preferred. GLM’s proposal to automatically make llama.cpp preferred is also rejected. ONNX/llama.cpp/mistral.rs/Candle/MLX/burn are workload-specific benchmark candidates. |
| F-14 | **ACCEPT** | H0-B gets typed per-resource extractors + coverage accounting; no general FHIRPath engine in H0. Revisit in 011/013 only with real requirement. |
| F-15 | **ACCEPT** | 008S worker-confinement qualification: Linux Landlock + syscall/network/resource confinement as appropriate; Windows AppContainer + Job Object/brokered handles; macOS App Sandbox/Seatbelt/XPC pattern as appropriate. Exact OS composition must be reverified at spec time. |
| F-16 | **ACCEPT WITH TOOL-SCOPING** | cargo-vet/auditable/fuzz added. Rust hostile paths use cargo-fuzz; native donors need equivalent upstream/native fuzz/sanitizer evidence. Pack format denies code-bearing pickle/.bin by default; ONNX custom-op libraries are separate signed/sandbox-admitted code. TUF is offline-verifiable trust basis; Sigstore optional online attestation. |
| F-17 | **ACCEPT WITH CLAIM-SCOPING** | Define `PRIVACY_PROOF`: MedScale-attributable egress capture, dependency capability audit, crash/log/cache marker scan, platform entitlement/config evidence. Do not claim “zero packets system-wide.” |
| F-18 | **ACCEPT** | Pack format anticipates mobile chunking/per-arch/resume/hash; Files/SAF sideload; non-synchronizing Apple Keychain policy; Android hardware-backed keys where available; 16KB support in CI. |
| F-19 | **ACCEPT WITH USABILITY POLICY** | CLI routes through same authority facade. Export is an egress-class boundary with exact destination/payload receipt. PHI stdout is denied by default or requires explicit high-friction opt-in. Elevated execution grants no extra MedScale authority; exact refusal policy is spec-tested, not assumed. |
| F-20 | **ACCEPT** | Re-pin OpenMed sources to v2.2.0, make model counts dated/manifest-bound, and absorb candidate/current/last-green/canary promotion pattern into Pack lifecycle. |

## Additional accepted attacks from GLM storage review

- Blob GC requires a crash-safe mark/tombstone/sweep algorithm and a race test against concurrent promotion.
- Canonical blob retrieval/restore verifies digest/size; corruption is quarantined and evidenced.
- Restore closure checks that every visible metadata reference has its required blob and digest.
- Cross-authority-scope plaintext dedup is not assumed. Default is separate ciphertext/key scope so deletion/revocation boundaries remain real.
- Filesystem atomicity/fsync claims are qualified per OS/filesystem; unsupported filesystems stay outside the release claim rather than being waved through.
- SQLCipher remains a candidate, not a constitutional dependency.

## Product correction

MedScale must not become “OpenMed plus more tools.” The first product wedge after H0/H1 is the **Trusted Local Longitudinal Record**: import synthetic/local records, preserve exact sources, show deterministic timeline + narrow Brief + coverage/conflict/provenance, and expose the same trust path through Desktop + CLI. AI/OpenMed capabilities attach later as bounded Proposals/Evaluation artifacts.

This is the measurable translation of the founder’s integrated-product ambition: fewer duplicated semantics, strong default UX, local operation, inspectable provenance, and one authority model across surfaces.
