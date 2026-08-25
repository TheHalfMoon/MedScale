# MedScale OpenMed Parity / Surpass Matrix V2

**Date:** 2026-08-25  
**Pinned OpenMed baseline:** `v2.2.0` / `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837`  
**Status:** `CANONICAL_PLANNING_V2`  
**Repository state:** `PLANNING_ARTIFACTS_CANONICAL / PRODUCT_IMPLEMENTATION_NOT_STARTED`

## 1. Gate semantics

Parity is phase-scoped. MedScale does **not** have to clone every OpenMed feature before the H0 product exists. A row becomes blocking when its owning MedScale spec reaches qualification. Deferred rows require explicit, documented scope waivers and do not count as silently passed.

Classifications:

- `REQUIRED_PARITY` — must meet the relevant OpenMed floor in the owning spec.
- `REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE` — parity first; superiority cannot be claimed until measured.
- `USEFUL_PARITY` — valuable but not a blocker to the current product claim.
- `REQUIRED_ABSORB_PATTERN` — adopt/test the architectural pattern, not necessarily the code/API.
- `MEDSCALE_SUPERSEDES_CONCEPT` — MedScale’s product goal requires a stronger category of semantics; still benchmark adjacent OpenMed behavior where relevant.
- `DEFER` — deliberately outside V2 claim envelope.

## 2. V2 matrix

| Capability | V2 classification | Owning spec | OpenMed v2.2 significance | MedScale qualification target |
|---|---|---:|---|---|
| Clinical NER | REQUIRED_PARITY | 007/008 | core OpenMed model family | task F1/precision/recall + latency/RSS + span/provenance contract |
| PII detection | REQUIRED_PARITY | 007/008 | core strength | high-risk PHI recall, false-negative analysis, multilingual traps |
| De-identification | REQUIRED_PARITY | 007/008 | core strength | deterministic transformation record, span fidelity, reversal policy, no hidden egress |
| Multilingual medical NLP | REQUIRED_PARITY | 007/008 | broad language coverage | supported-language registry + per-language evidence + honest unsupported behavior |
| Saudi/Arabic PII + clinical NER | REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE | 007/008 | multilingual baseline exists | Saudi clinical corpus, code-switch/dialect/entity/number traps; surpass only after exact-head benchmark |
| Terminology grounding | REQUIRED_PARITY | 007/011 | v2.2 adds checksum snapshots, ranked grounding, calibration/abstention | signed/checksummed terminology packs, rights metadata, exact/ranked grounding, conflict/abstention evidence |
| Unicode/offset semantics | REQUIRED_PARITY | 002/007/010 | OpenMed publishes cross-format offset/parity behavior | canonical tagged coordinate systems; byte/source mapping; Unicode-scalar derived spans; Swift/Kotlin conversion tests; Arabic normalization as versioned transform |
| Local/offline useful operation | REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE | all | OpenMed is local/on-device | no cloud/model hub account required; scripted offline workload + privacy proof |
| CPU inference | REQUIRED_PARITY | 008 | supported | quality parity + low/mid hardware latency/RSS/throughput |
| Apple Silicon / MLX | REQUIRED_PARITY | 008/009 | strong OpenMed native story | cold start/memory/latency/offline + key/privacy behavior |
| Android local inference | REQUIRED_PARITY | 008/009 | v2.2 includes Android network-denial guarantees | native build, 16KB compatibility, no unintended network capability, model lifecycle |
| iOS local capability | REQUIRED_PARITY | 008/009 | OpenMedKit competitive surface | binary size, battery, Keychain non-sync policy, offline/model install |
| NVIDIA GPU | USEFUL_PARITY | 008 | available via OpenMed stack | workload-specific only; not required for useful local core |
| Browser/WebGPU | DEFER | 016+ | OpenMed supports browser | explicit V2 waiver; no browser surface until product/privacy evidence justifies it |
| Python API | USEFUL_PARITY | later | OpenMed’s large developer API | Rust SDK is canonical; bindings only when demand exists; no authority bypass |
| CLI | REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE | 006 | OpenMed has tooling surfaces | first-class Rust CLI through the same authority path as desktop; no privileged bypass |
| REST/gRPC/GraphQL service plane | DEFER | 016+ | OpenMed broad service surface | not a core V2 requirement; future site adapter only |
| MCP / Agent Skills | USEFUL_PARITY | 014+ | v2.2 has MCP boundaries; skills ecosystem is broad | later adapter behind authority/effect gateway; no skill/tool grants authority |
| Model catalogue breadth | USEFUL_PARITY | 015 | OpenMed has 2,200+ ecosystem scale | **do not chase raw model count**; qualify task/language/hardware coverage and pack trust instead |
| Model packaging/provenance | REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE | 008/015 | OpenMed has signed/release evidence and promotion pointers | signed MedScale Pack, exact hashes, rights, SBOM, revocation, offline verification, reproducibility |
| Staged model promotion | REQUIRED_ABSORB_PATTERN | 008/015 | `latest` / `last_green` / canary pattern | candidate -> qualified -> current + last-green rollback; signed gates; fail-closed readiness |
| Structured privacy / SDC | REQUIRED_PARITY | 011/013 | k/l/t, membership-inference tests, aggregate DP, SDC controls | research/export release-gates with exact algorithms/parameters/evidence; not forced into H0 product |
| FHIR R4 summaries/documents/profile validation/integrity | REQUIRED_PARITY | 013 (with H0 foundations in 003/004) | v2.2 materially broadens FHIR | raw-byte custody + MedScale acceptance + profile/integrity evidence; OpenMed capability parity before any “surpass” claim |
| OMOP CDM 5.4 | USEFUL_PARITY | 016+ / research adapter | v2.2 bridge | later research projection; no canonical truth replacement |
| OpenEHR export | USEFUL_PARITY | 016+ | v2.1+ export path | deferred adapter; explicit waiver from core parity gate |
| Document format breadth | REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE | 010 | v2.2 adds PDF table, XLSX/PPTX/ODT, HL7v2, X12, MIME quarantine | hostile-input sandbox + fidelity/source-span/loss evidence; parity first |
| Deterministic clinical document processing | REQUIRED_PARITY | 010/011 | structured extraction, temporal/coreference and span-grounded recall | Proposal-only outputs, deterministic context/negation/section tests, source mapping; specialty logic phase-scoped |
| Radiology specialty helpers | DEFER | 016+ | BI-RADS/Lung-RADS style capability | imaging workflow not in V2 |
| National-ID validation | REQUIRED_PARITY for Saudi/Gulf; USEFUL_PARITY globally | 007/010 | broad validators | Saudi/Gulf Arabic PII traps first; global breadth does not block V2 |
| Voice / ASR | REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE where OpenMed comparable; otherwise MedScale target | 010 | not OpenMed’s defining center | Arabic/code-switch WER + critical-number/name error + source-time alignment |
| watchOS / visionOS | DEFER | 016+ | OpenMed has targets | explicit waiver |
| Source custody | MEDSCALE_SUPERSEDES_CONCEPT | 002/003 | OpenMed has provenance but MedScale goal is broader | raw source identity, immutable bytes, explicit transformations/loss |
| Identity | MEDSCALE_SUPERSEDES_CONCEPT | 002/003 | not OpenMed category center | explicit IdentityAssertion; no silent merge; wrong-patient adversarial tests |
| Time semantics | MEDSCALE_SUPERSEDES_CONCEPT | 002/004 | timeline features exist but MedScale freezes richer semantics | effective/recorded/acquired/precision/approximation without false causality |
| Longitudinal truth/conflict | MEDSCALE_SUPERSEDES_CONCEPT | 004/011 | OpenMed has timeline/provenance | versioned assertions, conflict/absence/freshness without silent resolution |
| Proposal/promotion authority | MEDSCALE_SUPERSEDES_CONCEPT | 002+ | OpenMed outputs are capability results | exact authorized promotion only; AI confidence never authority |
| Privacy observability | REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE | 006/009 | OpenMed already has concrete Android/network/diagnostic controls | formal `PRIVACY_PROOF`; superiority only after cross-platform release evidence |
| Cross-surface consistency | REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE | 006/009 | OpenMed has multiple SDKs | one Rust contract corpus across CLI/Desktop/iOS/Android; no duplicated clinical truth |
| Controlled external actions | MEDSCALE_SUPERSEDES_CONCEPT | 014 | OpenMed has agent/integration surfaces | durable intent + effect/retry + UNKNOWN reconciliation; no blind retry |

## 3. Benchmark manifest

Every parity/surpass result binds: exact OpenMed commit/release; exact MedScale commit/tree; exact artifact/model hash; fixture/corpus hash; rights; hardware/OS/runtime; quality metrics; latency/cold-start/throughput; RAM/VRAM; network/privacy observation; failure behavior; limitations; benchmark harness revision.

Raw model counts are never a parity metric unless bound to a dated manifest and a product reason. Capability coverage, quality, trust, portability and reproducibility matter more than matching catalogue cardinality.

## 4. `PRIVACY_PROOF`

A release-grade privacy evidence artifact includes, per supported OS: MedScale-attributable egress capture under a scripted workload; dependency/network-capability audit; crash/log marker scan; WebView/cache marker scan where applicable; mobile manifest/entitlement review; vault-location/sync-root check; exact claim scope and limitations. It does **not** claim zero packets system-wide outside MedScale’s attributable process/app boundary.
