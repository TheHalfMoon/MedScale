# MedScale Master Build Plan V2

**Date:** 2026-08-25  
**Prepared after:** GLM 5.3 adversarial review + live-source reconciliation  
**Status:** `PLAN_READY_CANONICAL`  
**Repository state:** `PLANNING_ARTIFACTS_CANONICAL / PRODUCT_IMPLEMENTATION_NOT_STARTED`  
**Implementation authorization:** NO  
**MESC mutation authorization from this plan:** NO

## 1. Product mandate

MedScale is a Rust-owned, local-first, privacy-first medical intelligence **platform**, not a chatbot/model wrapper/FHIR database/OpenMed fork. Its first durable advantage is trusted local longitudinal medical truth: exact source custody, explicit identity/time, Proposal-vs-Assertion authority, deterministic coverage/conflict semantics, and one policy model across CLI/Desktop/mobile.

OpenMed is the capability floor, not the domain model. MedScale must reach phase-scoped parity for relevant OpenMed capabilities and may claim superiority only with pinned comparative evidence.

## 2. Constitutional invariants

```text
PRIMARY_LANGUAGE = RUST
TRUSTED_CORE_LANGUAGE = RUST
LOCAL_FIRST = CONSTITUTIONAL
PRIVACY_FIRST = CONSTITUTIONAL
NETWORK_EGRESS = DEFAULT_DENY
NO_CLOUD_ACCOUNT_REQUIRED_FOR_USEFUL_CORE
NO_REMOTE_MODEL_REQUIRED_FOR_USEFUL_CORE
NO_HIDDEN_TELEMETRY
SOURCE_BYTES != DERIVED_REPRESENTATION
SOURCE_IDENTITY != CONTENT_HASH
PROPOSAL != CLINICAL_ASSERTION
PROJECTION != TRUTH
AI_OUTPUT != AUTHORITY
FHIR_R4_4.0.1 = INITIAL_INTERCHANGE_BASELINE
MESC = ARTIFACT_FIRST + EXCEPTIONAL_SANDBOXED_SERVICE
UNKNOWN_EXTERNAL_ACTION != AUTO_RETRY
```

Durable classes remain distinct: SourceRecord, DerivedSourceArtifact, Proposal, ClinicalAssertion, EvaluationRecord, Action/AuditRecord, Projection. IdentityAssertion is explicit; no silent merge. Realm and opaque `authority_scope_id` remain explicit. Purpose/legal basis/consent are versioned flow decisions; law is not invented in code.

## 3. Trusted Rust responsibilities

Rust owns object/schema contracts; source/raw-byte identity; identity/time/realm/authority; proposal/promotion; effect/retry; audit/evidence; canonical visibility; key lifecycle interfaces and zeroization policy; storage contracts; FHIR intake/acceptance; projection contracts; Network Broker; Pack admission; worker process supervision + IPC/capability enforcement; external action state machine; CLI authority facade.

Non-Rust and native code may be best-in-class but never obtains product authority. Engine placement follows P0/P1/P2/P3 risk classes from the V2 OSS matrix.

## 4. Process topology

One Core Host owns each desktop vault’s writable canonical store and active vault keys. CLI/Desktop/SDK call the same local IPC authority facade. A per-vault lease prevents independent writers. Workers receive capability-scoped handles/data only, never ambient canonical DB/master keys/network/unrestricted filesystem. Mobile runs the shared Rust authority core inside the app sandbox with native Keychain/Keystore and platform lifecycle adapters.

## 5. Source / text / terminology semantics

Raw source bytes are immutable evidence. Any normalization/OCR/transcript/tokenization is a DerivedSourceArtifact with explicit transformation/loss/version metadata. Spans are tagged with representation and coordinate system; raw-byte offsets are preferred for source drill-down, Unicode-scalar coordinates are used for derived text where appropriate, and Swift/Kotlin/JS conversions are explicit and tested.

Terminology is a first-class pack/right/version problem, not a string lookup helper. Terminology packs record code system/version/snapshot digest/rights/language/crosswalk provenance/calibration state. Licensed content is never silently bundled.

## 6. H0 / pre-PHI program

### H0-A / Spec 003
Synthetic FHIR R4 only. Trusted ingest, lexical safety, exact source bytes, validator evidence, identity evidence, blob-first canonical visibility, projection rebuild, crash/fault tests, synthetic backup interface. No real PHI/model/OpenMed/MESC/OCR/ASR/vector/network/action.

Durability additionally proves blob digest integrity, GC/promotion races, restore completeness, migration interruption and claim-scoped filesystem behavior.

### H0-B / Spec 004
Deterministic timeline and narrow LLM-free Brief from promoted canonical resources, honest coverage/unknown/absence/conflict/time precision, source drill-down and deterministic rebuild. Bounded typed extractors replace a general FHIRPath engine in H0.

### Pre-PHI / Spec 005
Encryption at rest; KeyProvider and key class separation; recovery/key-loss; backup/restore; retention/destruction; cloud-sync/remote-filesystem vault policy; snapshot implications; logs/crash privacy; platform custody; counsel/flow-specific PDPL/SFDA mapping. Completion does not itself authorize real PHI.

## 7. First product wedge

**Trusted Local Longitudinal Record — Desktop + CLI.**

A user can import permitted local/synthetic data, inspect exact sources, see a deterministic timeline/Brief/coverage/conflicts, verify provenance, and operate fully offline. Desktop and CLI are two views over the same Rust authority path, not separate products. This wedge is valuable before any model is installed and demonstrates the founder’s local-first/privacy-first product thesis.

## 8. OpenMed parity / donor program

Pinned baseline: OpenMed v2.2.0 / `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837`.

Spec 007 owns the comparator corpus, capability rows, donor/code-admission plan, terminology/licensing track, Saudi/Arabic benchmark design, and stage-promotion pattern. It does not force a runtime into the trusted core.

Critical newly represented v2.2 capability families: terminology grounding/calibration; structured privacy/SDC; FHIR R4 summaries/documents/profile validation/integrity; OMOP/OpenEHR as later/useful parity; document-intake breadth; Unicode/cross-format offsets; Android network/privacy evidence; MCP/security patterns; staged model promotion.

No raw model-count race is a product goal. Exact model counts are dated evidence only.

## 9. Local AI capability fabric / Packs

Spec 008 defines:

- `Pack v0` content-addressed manifest, rights, SBOM, artifact hashes, runtime requirements, benchmark/evidence links;
- offline-only initial import from local path/media, with no network requirement;
- candidate/current/last-green/canary promotion semantics;
- workload-specific runtime adapters, never a single prematurely preferred engine;
- 008S worker confinement qualification per OS;
- fail-closed model-format admission: no pickle/code-bearing `.bin` by default, no unsigned ONNX custom-op libraries, bounded tokenizer metadata.

Online acquisition does not exist until Spec 013 supplies the Network Broker and Spec 015 uses it.

## 10. Desktop / CLI privacy contract

CLI has no special privilege. Exports/destruction/effects are classified, authorized and receipted. PHI-to-stdout is denied by default or requires explicit high-friction opt-in in the future claim scope.

Tauri remains a prototype candidate. Spec 006 must show that PHI markers do not persist into WebView caches/crash artifacts outside declared storage boundaries. If it cannot, MedScale changes the desktop shell rather than weakening privacy.

`medscale doctor` reports vault location, sync/remote-root risk, key-store availability, pack/runtime state, privacy evidence freshness and supported filesystem/OS claim status.

## 11. Mobile

Shared Rust semantics, native Swift/Kotlin UI/lifecycle, Keychain/Keystore providers, no silent Apple key synchronization, hardware-backed Android keys where available, offline Files/SAF pack sideload, Android 16KB native-binary compatibility, pack chunking/per-arch/resume rules and battery/background-operation budgets. Mobile in-process native runtimes are platform-confinement exceptions that need explicit evidence.

## 12. Network / actions

Spec 013 creates the sole product online egress abstraction: Network Broker with explicit destination/purpose/data-class/authorization/receipt. No worker/provider SDK/HF client may bypass it.

Spec 014 adds durable external action intents and exact state machine `PENDING -> SENT -> CONFIRMED / FAILED / UNKNOWN`; `UNKNOWN` is reconciled before retry. NPHIES is implemented only for selected, evidence-backed workflows and exact versioned profiles/terminology.

## 13. Supply chain

Initial CI/security program: locked Rust toolchain; cargo-deny; RustSec; OSV; cargo-vet policy; cargo-auditable; Rust fuzzing for hostile paths; native donor sanitizer/fuzz evidence; provenance/SBOM for Rust + C/C++ + model assets; exact vendor build flags/ownership/ABI; one SQLite copy per process; offline signing root/threshold/rotation/revocation metadata; TUF-compatible offline verification; Sigstore optional online attestations.

## 14. Evidence classes

Existing release classes remain (`LOCAL_PASS`, `CI_PASS`, `EXTERNAL_CONFORMANCE_PASS`, `PLATFORM_QUALIFIED`, `RELEASE_QUALIFIED`) and every artifact binds repository/tree, toolchain/lock, OS/hardware, fixture/corpus hashes, exact commands/config, raw report digests, limitations and claim scope.

Add `PRIVACY_PROOF` as a typed evidence artifact: attributable network observation, network-capability audit, crash/log/cache marker scans, mobile entitlements/manifests, vault/sync-root evidence and explicit limitations.

## 15. Canonical Spec Kit roadmap

```text
000 Constitution + Source Authority
001 Rust Repository + Spec Kit Bootstrap
002 Trusted Object / Source / Authority + Process/Text Foundation
003 H0-A Trusted Ingest + Durability
004 H0-B Trusted Presentation + Coverage
005 Local Private Vault + Encryption + Recovery
006 CLI + Desktop Foundation
007 OpenMed Capability Absorption / Parity Research
008 Local AI Capability Fabric + Pack Admission + Worker Confinement
009 Mobile iOS + Android
010 Documents + OCR + Voice
011 Evidence / Retrieval / Medical Intelligence
012 MESC Artifact Integration
013 FHIR / SMART / Network Broker
014 Controlled Actions / NPHIES
015 Hugging Face + Online MedScale Pack Ecosystem
016+ Deferred Research / Site / Imaging / Genomics / Browser / Advanced Workflows
```

Critical foundation: `000 -> 001 -> 002 -> 003 -> 004 -> 005 -> 006`.

After 004, Spec 007 planning/research can proceed in parallel with 005 without models/PHI/runtime. Capability implementation: `005 + 006 + 007 -> 008`. Online distribution: `008 + 013 -> 015`.

## 16. Do not build in V2 core

No general FHIRPath engine for H0; no custom cryptography; no OpenMed/Python inference port just to make it Rust; no cloud-provider model adapters; no REST/gRPC/GraphQL product plane; no early plugin/Wasm ecosystem; no vector DB server; no graph DB; no browser/WebGPU surface; no watchOS/visionOS; no imaging/genomics/site/sync/CRDT; no full terminology server; no telemetry/crash-upload infrastructure; no MESC direct Python import/shared DB/key.

These may be reconsidered only with later product evidence and an owning spec.

## 17. Planning closure and authorization

The GLM findings have been reconciled into canonical V2 planning artifacts. The architecture is internally coherent after the corrections above. Remaining work such as exact OS sandbox APIs, dependency versions, terminology licences and runtime choices is intentionally owned by research/qualification gates in the relevant specs rather than left as hidden assumptions.

```text
V2_PLANNING_STATUS = PLAN_READY_CANONICAL
MEDSCALE_REPOSITORY_PLANNING_FINALIZATION = COMPLETE
MEDSCALE_PRODUCT_IMPLEMENTATION = NOT_STARTED / NOT_AUTHORIZED
MESC_MUTATION = NOT_AUTHORIZED_BY_THIS_PLAN
```

The canonical V2 planning artifacts are now placed in `TheHalfMoon/MedScale`. The next executable gate is explicit founder authorization to begin Spec 000/001 bootstrap work. Repository planning finalization does not itself authorize Cargo initialization, dependency installation, product code, model execution, PHI access, or `/speckit.implement`.
