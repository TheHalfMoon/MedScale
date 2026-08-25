# MedScale Source Acquisition and Copy Plan

**Status:** `CANONICAL_EXECUTION_INPUT`  
**Purpose:** tell Cursor exactly where capabilities come from and whether to copy, port, depend, vendor, isolate, reference, or reject them.

## 1. Operation vocabulary

```text
COPY_BOUNDED        Copy a deliberately small implementation/test/rule fragment with provenance.
PORT_TO_RUST        Reimplement bounded behavior/contracts/tests in Rust; preserve upstream provenance/evidence.
DEPENDENCY          Consume normal pinned package/crate/library; do not copy source by default.
VENDOR_SNAPSHOT     Vendor exact upstream source/binary only after owning spec justifies vendoring.
FFI                 Native library behind a Rust-owned ABI/lifetime/thread/error boundary.
SIDECAR             Separate constrained process; no ambient authority.
REFERENCE_ONLY      Read ideas/spec/tests; no runtime/code inheritance.
ARTIFACT_IMPORT     Admit immutable data/model artifact through MedScale Pack/evidence path.
USER_SUPPLIED_UI    v0/founder-provided UI source integrated through UI contract.
DO_NOT_COPY         Explicitly forbidden source/asset or architecture.
```

**No repository is copied wholesale.** Permission to use code is treated as authorization to choose the best bounded operation, not as permission to create an untracked fork pile.

Before the first copied/vendored line enters MedScale, create `third_party/provenance/<donor>-<component>.md` (or the owning spec's equivalent) containing upstream URL, exact commit/tag, original path(s), original content hash, license/NOTICE/permission evidence, local target, operation, modifications, transitive dependencies, security placement, tests, update strategy and exit strategy. This record is mandatory and does not require another founder approval when the row below authorizes the operation.

## 2. OpenMed — primary strategic donor and competitive floor

```text
UPSTREAM = https://github.com/maziyarpanahi/openmed
BASELINE_TAG = v2.2.0
BASELINE_COMMIT = 59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837
BASELINE_TREE = 1c949e35b2b8f2ea69da4284b370074fc4bf84ab
```

Cursor must inspect the exact baseline and the listed integration PR/diff before copying. Prefer copying **tests, fixtures, deterministic algorithms and contract ideas**, then porting authority-adjacent behavior to Rust. Never import the OpenMed Python package as MedScale trusted runtime.

| Capability family | Exact upstream loci / evidence | Operation | Owning spec | MedScale target intent |
|---|---|---|---|---|
| Terminology candidate/grounding contracts | baseline `openmed/clinical/grounding/`; PR #1911; PR #2042; PR #2685 | `COPY_BOUNDED` tests/fixtures + `PORT_TO_RUST` | 007/011 | terminology Pack/index/candidate/abstention contracts |
| Terminology snapshot/cache/provenance/expiry | PR #2224, #2474, #2540, #2680 | `COPY_BOUNDED` tests/manifest ideas + `PORT_TO_RUST` | 007/011 | checksum/version/rights/provenance manifests |
| Terminology conflict/ranking/calibration | PR #2542; normalization/ranking work including #1915; v2.2 integration #2685 | `COPY_BOUNDED` deterministic tests + `PORT_TO_RUST` | 007/011 | explicit conflict/rank/calibration evidence; never hidden authority |
| Terminology binding to FHIR | baseline `openmed/interop/terminology_binding.py`; PR #2154 | `REFERENCE_ONLY` + selected test `COPY_BOUNDED`; MedScale adapter in Rust | 011/013 | CodeableConcept binding with exact terminology provenance |
| Grounding/audit provenance | `openmed/clinical/grounding/provenance.py`, `openmed/core/audit.py`; PR #1920 | `COPY_BOUNDED` test patterns + `PORT_TO_RUST` | 007/011 | PHI-safe provenance/evidence envelope |
| FHIR R4 exchange/profile/integrity/SDC/Bulk patterns | integration PR #2686 and its included PRs #2044/#2046/#2133/#2140/#2149 | `COPY_BOUNDED` synthetic tests/fixtures + `PORT_TO_RUST`/`REFERENCE_ONLY` | 007/013 | parity evidence and external adapter behavior; never canonical DB model |
| FHIR-to-OMOP 5.4 mapping | PR #2048, #2162, integration #2686 | `REFERENCE_ONLY` initially; selected mapping tests may be `COPY_BOUNDED` | 007/016+ unless product evidence promotes | assistive export/mapping, no authority |
| Document intake/MIME/quarantine/form/KV/table/office-format patterns | integration PR #2687; PR #2548 and #2549 | `COPY_BOUNDED` fixtures/tests/policy + port/adapt behind P1 workers | 007/010 | parity corpus and hostile-document contracts |
| PII/NER/de-identification behavior and eval taxonomy | v2.2 baseline clinical/privacy/eval code; use exact changed-file inventory from relevant v2.2 PRs before import | `COPY_BOUNDED` tests/taxonomy/rules; `PORT_TO_RUST` contracts; model runtimes remain Pack capabilities | 007/008 | parity benchmark + Rust-owned redaction transformation/provenance |
| Structured privacy / release-risk controls | integration PR #2689; consent revocation #2541 | `COPY_BOUNDED` tests/policy patterns + `PORT_TO_RUST` where selected | 007/011/015 | dataset/release evidence; no silent legal conclusions |
| Service security / MCP / GraphQL / mTLS/HMAC patterns | integration PR #2690 | `REFERENCE_ONLY` | 013/014+ | security test ideas only; do not import OpenMed service plane |
| Unicode/cross-format offset projection | PR #2549 and baseline tests | `COPY_BOUNDED` adversarial/golden tests + `PORT_TO_RUST` | 002/010 | tagged coordinate conversions and source alignment |
| Local model cache quota/eviction | PR #2550 | `REFERENCE_ONLY` + selected tests | 008 | bounded Pack/runtime cache; storage authority remains MedScale-owned |
| Timeline provenance | PR #2551 | `COPY_BOUNDED` value-free tests/patterns only | 004/007 | compare with MedScale stronger source-backed timeline |
| CPU INT8 fast-path and performance ideas | PR #2237 | `REFERENCE_ONLY` | 007/008 | benchmark input; do not inherit Python runtime |
| Hard-negative and multilingual traps | PR #2243 and #2682 | `COPY_BOUNDED` synthetic benchmark-generation/test ideas with provenance | 007 | parity/adversarial benchmark corpus |
| Release evidence/promotion (`latest`/`last_green`/`canary`) | OpenMed v2.2 release evidence, `release-source.json`, signed/SBOM assets, model-pointer behavior documented in release | `REFERENCE_ONLY` + `PORT_TO_RUST` lifecycle semantics | 007/008/015 | MedScale Pack candidate/current/last-green/canary state |
| Swift/OpenMedKit and Android offline/privacy patterns | v2.2 baseline Swift/Android trees and v2.2 mobile docs/tests | `REFERENCE_ONLY`; selectively port tests/contracts | 007/009 | mobile parity/privacy patterns; native UI remains MedScale/v0-driven |

**OpenMed forbidden interpretations:** no wholesale fork as MedScale product; no Python clinical domain model in trusted core; no copying restricted terminology; no assumption that code permission covers weights/data/assets; no service framework becoming the Network Broker; no OpenMed model output becoming ClinicalAssertion.

For a row that says `COPY_BOUNDED`, Cursor is authorized to freeze the exact changed-file list from the cited baseline/PR during the owning spec, record it in provenance, and copy only the minimum needed files/fragments/tests without asking the founder.

## 3. Core dependencies / native runtimes — do not manually copy by default

| Source | Baseline/pin from corpus | Operation | Spec | Rule |
|---|---|---|---|---|
| ONNX Runtime | `https://github.com/microsoft/onnxruntime` / v1.29.0 | `DEPENDENCY` or `VENDOR_SNAPSHOT` + `FFI` | 008/009 | benchmark per workload; P1/P2 placement as applicable |
| llama.cpp | `https://github.com/ggml-org/llama.cpp` / b10430 | `DEPENDENCY`/`VENDOR_SNAPSHOT` + `FFI` | 008 | not preferred by fiat; qualify against alternatives |
| mistral.rs | `https://github.com/EricLBuehler/mistral.rs` / v0.9.0 corpus pin | `DEPENDENCY` | 008 | candidate only; no agent authority |
| Candle | `https://github.com/huggingface/candle` | `DEPENDENCY`; pin exact tested rev in Spec 008 | 008 | owned specialist-runtime candidate |
| Burn | official Burn repository from V2 delta | `DEPENDENCY`; pin at Spec 008 | 008 | Rust-native comparator |
| MLX / mlx-lm | `https://github.com/ml-explore/mlx`, `/mlx-lm` | `DEPENDENCY`/platform adapter | 008/009 | Apple accelerator only; no product authority |
| sherpa-onnx | `https://github.com/k2-fsa/sherpa-onnx` / v1.13.5 | `DEPENDENCY`/`VENDOR_SNAPSHOT` + `FFI` | 010 | ASR/VAD/speaker candidate; P1/P2 |
| whisper.cpp | `https://github.com/ggerganov/whisper.cpp` | `DEPENDENCY`/`VENDOR_SNAPSHOT` + `FFI`; pin in 010 | 010 | Arabic/code-switch comparator |
| DeepFilterNet | `https://github.com/Rikorose/DeepFilterNet` / v0.5.6 | `DEPENDENCY` | 010 | enhancement separate from ASR semantics |
| docling.rs | `https://github.com/docling-project/docling.rs` / v1.10.0 | `DEPENDENCY` or justified vendor | 010 | hostile-input benchmark required; Rust does not automatically mean P0 |
| Docling | `https://github.com/docling-project/docling` / v2.120.1 | `SIDECAR`/conformance comparator | 010 | Python never trusted core |
| PaddleOCR | `https://github.com/PaddlePaddle/PaddleOCR` | `SIDECAR`/native vendor; pin exact rev in 010 | 010 | benchmark Arabic medical OCR and asset rights |

## 4. Storage, search, security and platform donors

| Source | Pin/status | Operation | Spec | Rule |
|---|---|---|---|---|
| Tauri | v2.11.5 corpus pin | `DEPENDENCY` candidate | 006 | keep only if WebView privacy proof passes |
| SQLCipher | v4.17.0 | `DEPENDENCY`/`FFI` | 005 | candidate, not constitutional; durability/key tests mandatory |
| keyring-core + selected stores | choose exact current compatible pin in Spec 005 research | `DEPENDENCY` | 005/009 | select only needed platform stores; no ambient secret API |
| Tantivy | 0.26.1 | `DEPENDENCY` | 011 | projection only |
| sqlite-vec | 0.1.9 | `REFERENCE_ONLY`/experimental dependency | 011 | no load-bearing use without explicit qualification |
| USearch | v2.26.0 | `DEPENDENCY`/`FFI` candidate | 011 | projection only; benchmark against alternatives |
| LanceDB / Qdrant | pin only if 011 evidence requires | `REFERENCE_ONLY` initially | 011+ | avoid server/vector stack by default |
| Cedar | v4.12.0 corpus pin | `DEPENDENCY` candidate later | 014+ | MedScale authorization semantics remain canonical |
| Wasmtime | current exact pin at owning spec | `DEPENDENCY` only if plugin need proven | 016+ | no early plugin system |
| TUF/tough | tough v0.24.0 corpus pin | `DEPENDENCY` | 015 | offline-verifiable update/Pack trust |
| sigstore-rs | v0.14.0 corpus pin | `DEPENDENCY` optional | 015 | optional online attestation, not sole trust root |
| cargo-cyclonedx | 0.5.9 corpus pin | `DEVELOPMENT_TOOL` | 001/015 | SBOM evidence |
| cargo-deny / RustSec / OSV / cargo-vet / cargo-auditable | exact versions pinned by 001 lock/tool policy | `DEVELOPMENT_TOOL` | 001 | supply-chain gates |
| Trivy / Syft | exact version pin at use | `DEVELOPMENT_TOOL`/release tool | 001/015 | native/container artifact evidence where applicable |

## 5. Standards, fixtures and reference-only sources

- **HL7 FHIR R4 4.0.1 / SMART / CQL / NPHIES / PDPL / SFDA:** `REFERENCE_ONLY` authoritative-in-scope standards/regulatory evidence. Never copy them into a new MedScale authority model.
- **HL7 Java Validator / HAPI core 6.10.2:** pinned external validation oracle/sidecar, not clinical authority.
- **Inferno core v1.4.2 / SMART test kit:** external conformance evidence.
- **Synthea v4.0.0:** `REFERENCE_ONLY` + synthetic fixture generation; not Saudi or clinical conformance evidence.
- **SNOMED CT, LOINC, UCUM, ICD, ATC, UMLS/Athena:** rights/version/checksum-controlled terminology Packs. Licensed/restricted content is caller-supplied or separately licensed. **Never copy terminology tables from OpenMed.**
- **Presidio 2.2.364 corpus pin:** `REFERENCE_ONLY` plus small bounded rules/tests may be copied with provenance when 007/010 proves value; do not import its authority/service architecture.
- **medSpaCy:** `REFERENCE_ONLY` clinical NLP pattern/test source unless later evidence justifies more.
- **Statig/OpenCR/iroh/Extism/OPA/OpenFGA/Microsoft Agent Governance Toolkit/MCP SDK:** reference/candidate sources only in owning later specs; no early framework adoption.
- **dicom-rs/OHIF/MONAI/noodles/GA4GH VRS/DuckDB/Polars/Arrow/DataFusion/Perspective/etc.:** deferred 016+ unless formally promoted.

## 6. MESC and v0

**MESC:** `ARTIFACT_IMPORT` only. MedScale may admit released immutable MESC artifacts with exact hashes, rights, SBOM and evaluation evidence in Spec 012. Do not copy/import MESC Python package, DB, keys or internal runtime into MedScale; do not mutate `TheHalfMoon/MESC`.

**v0:** `USER_SUPPLIED_UI`. Use `V0_UI_INTEGRATION_CONTRACT.md`; copy/integrate only the founder-approved visual/component artifact. Generated backend/database/network code is reviewed as untrusted donor code and usually discarded/replaced by typed MedScale interfaces.

## 7. No-question rule for source choice

If a row names a fixed revision, use it for baseline comparison/admission. If it says `pin in Spec N`, Cursor must research current upstream, choose and record the exact tested revision using the decision defaults, then continue without asking the founder. A new donor not listed here requires an owning-spec research entry and must beat or fill a gap in the existing candidate set; adding a donor is not itself a founder question unless it changes a frozen architecture invariant or requires external legal acceptance.