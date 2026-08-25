# MedScale OSS / Code Absorption Matrix V2

**Date:** 2026-08-25  
**Status:** `CANONICAL_PLANNING_V2`  
**Trusted authority boundary:** Rust-owned  
**Repository state:** `PLANNING_ARTIFACTS_CANONICAL / PRODUCT_IMPLEMENTATION_NOT_STARTED`  
**Implementation authorization:** NO

## 1. Placement rule (reconciles GLM F-01)

The V1 mistake was treating `FFI` as sufficient confinement for some complex hostile-input engines. The opposite blanket rule — “every parser of untrusted bytes must be out-of-process” — is also rejected because MedScale’s trusted Rust core must parse bounded FHIR/JSON and its own signed metadata.

Canonical engine-placement classes:

| Class | Placement | Criteria |
|---|---|---|
| `P0_TRUSTED_CORE_SAFE` | in Rust trusted core | narrow deterministic format; memory-safe implementation on the hostile path; strict size/depth bounds; fuzz/property tests; no ambient network; no third-party native parser needed |
| `P1_ISOLATED_REQUIRED` | sandboxed worker on desktop | complex hostile formats or model/codecs/runtimes involving C/C++/unsafe/native parsers, large attack surface, auto-loading, plugins/custom ops, or unproven confinement |
| `P2_PLATFORM_CONFINED_EXCEPTION` | mobile in app sandbox only after qualification | platform does not permit useful process isolation or the qualified adapter requires in-process execution; no canonical DB handle/master key; bounded inputs/outputs; explicit per-platform evidence |
| `P3_TRUSTED_NATIVE_INFRA` | narrow in-process native FFI | infrastructure such as SQLCipher where inputs are MedScale-controlled, ABI/ownership is hardened, and the dependency is inside a deliberately trusted storage/crypto boundary |

`Rust memory safety != sandbox`. `Rust implementation != automatic P0`; a complex hostile PDF/DICOM/model parser can still be `P1`.

## 2. V2 matrix

| Donor / source | Capability | V2 disposition | Placement / phase | Canonical decision |
|---|---|---|---|---|
| OpenMed v2.2.0 | NER, PII, de-ID, terminology, document/FHIR/privacy patterns, mobile tests, model ecosystem | `REFERENCE + ABSORB selected algorithms/tests/contracts + PORT_TO_RUST where semantics belong to MedScale`; `FORK` only per bounded subsystem if sustained divergence justifies it | 007 research/admission; 008+ capability implementations | Never fork the whole product by default; never inherit OpenMed authority/domain model |
| ONNX Runtime | specialist inference | `VENDOR` candidate | `P1` desktop worker; `P2` mobile only after platform evidence; 008 | no direct canonical DB/key/network access; custom op libraries denied by default |
| llama.cpp | GGUF inference | `VENDOR` benchmark candidate | `P1` desktop worker; `P2` platform exception only after evidence; 008 | not the automatic preferred runtime |
| mistral.rs | Rust model runtime | `BENCHMARK / CANDIDATE` | isolated unless/until its exact hostile-input and unsafe surface proves P0; 008 | **demoted** from V1 “preferred” status; built-in agent features never grant authority |
| Candle | Rust ML runtime | `BENCHMARK / CANDIDATE` | workload-specific; 008 | no blanket preference merely because Rust-native |
| burn | Rust-native ML | `BENCHMARK / CANDIDATE` | 008+ | new comparator; admission only by measured workload fit |
| MLX / mlx-lm | Apple acceleration | `PLATFORM ADAPTER / CANDIDATE` | macOS/iOS; P1/P2 depending exact adapter | no Python sidecar as a default product dependency |
| hf tokenizers | tokenizer runtime | `VENDOR` candidate | bounded worker or audited library depending model/input path | pin exact version; regex/size/normalization limits; no remote fetch |
| `ort` | Rust ORT bindings | `VENDOR` candidate | wrapper only; ORT placement still governs | exact version + ABI compatibility must be pinned |
| SQLCipher | encrypted metadata store | `VENDOR/FFI PROTOTYPE` | `P3`; 005 | one SQLite implementation per process; durability/key behavior must be proven |
| keyring-core + selected stores | OS secret-store abstraction | `VENDOR` candidate | trusted key adapter; 005/009 | replaces old umbrella keyring-rs integration assumption; choose minimal per-platform stores |
| Tauri | desktop shell | `PROTOTYPE CANDIDATE` | 006 | rejected if PHI cache/crash containment evidence cannot close; UI never opens DB directly |
| docling.rs | document parsing | `VENDOR/ABSORB` candidate | `P1` worker despite Rust; 010 | complex document inputs make language alone insufficient for trust |
| Docling | document reference/fallback | `SIDECAR / CONFORMANCE COMPARATOR` | `P1`; 010 | no authority; no ambient network/FS |
| PaddleOCR | OCR | `COMPARATOR / SIDECAR OR NATIVE MODEL` | `P1`; 010 | prefer bounded ONNX/native artifact if quality/rights justify; no auto-download |
| Presidio | PII patterns/tests | `REFERENCE + selective ABSORB` | 007/010 | stewardship transition recorded; not a trusted runtime requirement |
| medSpaCy | deterministic clinical context | `REFERENCE + selective ABSORB rules/tests` | 010/011 | Proposal context only; not H0-B load-bearing engine |
| scispaCy / QuickUMLS | NLP/terminology comparators | `REFERENCE` | 007/011 | rights/quality gate; not product authority |
| sherpa-onnx | ASR/VAD/speaker | `VENDOR` candidate | `P1` desktop; P2 mobile if qualified; 010 | subsystem interfaces remain split |
| whisper.cpp | local ASR | `BENCHMARK / VENDOR` candidate | `P1`; 010 | Arabic/code-switch quality and resource gates |
| DeepFilterNet | enhancement | `CANDIDATE` | isolated capability; 010 | optional; never conflated with transcript truth |
| ICU4X | Unicode/locale semantics | `VENDOR` candidate | trusted deterministic text utility after audit; 002 | MedScale text contract remains authoritative |
| Tantivy | lexical search | `VENDOR` candidate | projection only; 011 | relevance never authority |
| USearch | vector similarity | `VENDOR` candidate | isolated/native projection; 011 | benchmark and memory-safety gate |
| sqlite-vec | vector projection | `EXPERIMENTAL / REFERENCE` | 011 | **demoted** from V1 VENDOR candidate; no pre-v1 freeze and no second SQLite copy |
| LanceDB | vector/data layer | `REFERENCE / DEFER` | 011+ | only if simpler projections fail |
| Qdrant | vector server | `REJECT current core / REFERENCE later` | 016+ site/server only | no local server dependency in V2 product core |
| Synthea | synthetic data | `REFERENCE + FIXTURE DONOR` | 003/004/013 | synthetic only; not Saudi conformance proof |
| HL7 Java Validator / HAPI core | FHIR external validation oracle | `PINNED SIDECAR / EXTERNAL EVIDENCE` | 003/013 | output is evidence, never authority |
| Inferno | SMART/FHIR conformance | `EXTERNAL CONFORMANCE` | 013 | evidence only |
| CQL / cqframework | clinical logic | `REFERENCE / SIDECAR later` | 011/013 | clinical logic != authorization |
| Snowstorm | terminology service | `REFERENCE / OPTIONAL ADAPTER` | 007/011/013 | do not build or require a terminology server for local core |
| SNOMED / LOINC / UCUM / ICD / ATC / UMLS | terminology assets/standards | `STANDARD/PACK INPUT` | licensing starts 007; use later by owning specs | rights/version/checksum are first-class; licensed content never assumed bundled |
| MCP Rust SDK | tool protocol | `VENDOR candidate later` | 014+ | adapter only behind capability/authority gateway |
| Wasmtime | WASM | `DEFER` | 016+ | no plugin platform in V2 |
| Cedar | authorization engine | `DEFER / CANDIDATE later` | 012+ or later if needed | MedScale authorization semantics remain core-owned |
| OPA / OpenFGA | external IAM/policy engines | `REJECT current core / REFERENCE` | later enterprise/site only | no authority split in V2 |
| Extism | plugin runtime | `DEFER` | 016+ | no early plugin ecosystem |
| iroh | sync/network substrate | `DEFER` | 016+ | conflicts with current local-only scope unless future product evidence selects sync |
| OpenCR | identity matching | `REFERENCE` | later | may generate match evidence/Proposals only; never silent merge |
| Statig | state machines | `REFERENCE / DEFER` | 002+ | use only if it materially simplifies explicit effect/action states |
| cargo-deny + RustSec + OSV | dependency policy | `MANDATORY CI TOOLSET` | 001 | lock/policy/vulnerability evidence |
| cargo-vet | human dependency review | `MANDATORY CANDIDATE` | 001 | review policy must be practical and documented |
| cargo-auditable | binary dependency metadata | `MANDATORY CANDIDATE` | 001 | evidence complement, not sole SBOM |
| cargo-fuzz | Rust fuzzing | `MANDATORY CANDIDATE` | 001+ | hostile-input Rust paths get fuzz targets/SLOs; native donors require equivalent upstream/native fuzz evidence |
| CycloneDX + Syft | SBOM | `RELEASE TOOL` | 001/015 | include Rust, native libraries, model/pack assets |
| TUF / tough | offline-verifiable update trust | `VENDOR CANDIDATE` | 008 pack admission / 015 online ecosystem | offline root/threshold/rotation/revocation design required |
| Sigstore | attestations | `OPTIONAL ONLINE ATTESTATION` | 015 | never a requirement for air-gapped verification |
| Trivy | artifact/container scanner | `DEFER / CI CANDIDATE` | when container/image artifacts exist | not required for initial local Rust core |

## 3. Mandatory native/FFI contract

Every admitted native dependency records: exact source/revision; build flags; ABI; ownership/freeing rules; handle thread-affinity; callback/unwind rules; typed error mapping; allocator assumptions; target architectures; sandbox placement; fuzz/sanitizer evidence; SBOM; update/rollback/exit strategy.

Rust panics must never unwind across `extern "C"`. Use boundary-specific containment/abort policy; do not impose a blanket workspace `panic=abort` without a spec decision.

Exactly one SQLite implementation may be linked into any process that opens the canonical metadata database.

## 4. Explicit donor-delta log

The V1/deep-research donor set is no longer allowed to silently lose entries. Every donor is retained with a disposition: medSpaCy re-added; keyring integration updated; Presidio stewardship transition recorded; sqlite-vec and mistral.rs demoted; Statig/OpenCR/iroh/Extism/OPA/OpenFGA/Qdrant/Trivy explicitly dispositioned.
