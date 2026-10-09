# OpenMed ↔ MedScale model-execution capability matrix (Spec 103, issue #183)

**Baseline:** OpenMed v3.0.0, `maziyarpanahi/openmed@ea920f36fadd7b45935247d639f0ffa1ef493b23`, source-read. Newer upstream changes are not part of this baseline.
**MedScale:** `main` plus the open Spec 103 PRs named per row. A row's status changes only with merged code and recorded evidence.
**Evidence base:** real-model qualification on free GitHub runners (Linux x64 CPU, static window 128, synthetic text), run [37977100903](https://github.com/TheHalfMoon/MedScale/actions/runs/37977100903). Each model's catalog binding, per-file digests, signed Pack and admission were verified before execution.

Status vocabulary:
- **IMPLEMENTED:** merged with tests;
- **IN REVIEW:** implemented in an open PR;
- **PARTIAL:** the gap is stated;
- **NOT IMPLEMENTED:** no MedScale code;
- **BLOCKED:** the named external constraint applies.

## 1. Lifecycle (DISCOVER → INSPECT → ACQUIRE → VERIFY → ADMIT → LOAD → RUN → COMPARE → UNLOAD)

| Stage | OpenMed v3.0.0 (source) | MedScale | Status |
|---|---|---|---|
| Discover / search | `models.jsonl` (2,266 rows), `core/model_registry.py`, `core/model_search.py` | Bounded JSONL import, 10-state status ladder, filters, stable order, pagination (`medscale-pack/src/catalog.rs`). Core capability `ModelCatalogRead` (#193). CLI `packs catalog` | IMPLEMENTED (no tier filter; offset pagination) |
| Inspect before download | model cards, size estimates | Size estimate labelled `estimated`. Pre-download `runtime_expectation` per row from recorded evidence (`compatibility.rs`), including known-defective exports | IMPLEMENTED (expectations are architecture-level) |
| Acquire | `huggingface_hub.snapshot_download`, `core/hf_hub.py`; Android `ModelDownloader.kt` | Per-model consent with a byte ceiling, Governed Browse transport, Hub host allowlist, pinned commit, atomic staging (#187). File-level resume of interrupted acquisitions (#192) | IMPLEMENTED (#187); resume IN REVIEW (#192). No byte-range resume. The live CLI acquisition is not exercised on the founder workstation (memory) |
| Verify | `core/model_integrity.py`, pinned revisions | OpenMed reproducibility hash ported; per-file LFS SHA-256 or git-blob SHA-1; exact sizes; gated and private repositories refused (`hf_snapshot.rs`) | IMPLEMENTED |
| Admit | n/a (OpenMed loads directly) | Signed MedScale Pack (synthetic trust root), anti-rollback, provenance, `PacksInstallLocal` | IMPLEMENTED (production signing `NOT_GRANTED`) |
| Load / prepare | `onnx/inference.py` `OnnxModel.from_pretrained`; `core/backends.py` | tract static-shape plan. Normalisations: `Shape`/`Cast→TDim`, integer `Sign`, `If` branches, symbolic `value_info` relaxation, fp16→fp32 widening. Degenerate tokenizers refused (#196) | IMPLEMENTED for 4 architectures; 2 more IN REVIEW (#196) |
| Run | `OnnxModel.predict_batch_detailed` (windows, best-context vote, entity decoding) | Single-window `run`. `run_document`: whole-document windows, best-context vote and BIOES entity decoding, ported from OpenMed (#194) | Single window IMPLEMENTED; documents IN REVIEW (#194). No multi-window batching |
| Compare | n/a | Model Fleet lanes over catalog-admitted Packs (#188); factual comparison, never a ranking | IMPLEMENTED |
| Unload / residency | `core/model_cache_policy.py`, `ModelLoader.unload_model`, `onnx/ram_budget.py` | `ResidencyPool`: budgeted LRU, pinning, plan-before-evict (#185); Core prepared-model cache on it (2 GiB) | IMPLEMENTED (no RSS probe; budget by artifact size) |

## 2. Runtimes and platforms

| Runtime / platform | OpenMed v3.0.0 | MedScale | Status |
|---|---|---|---|
| ONNX on CPU | ONNX Runtime `CPUExecutionProvider`; fp32 / int8 variants; threads | tract 0.22.4 (pure Rust, no native runtime download). fp32 and fp16 (widened) | IMPLEMENTED. int8 exports not prepared (quantized operators) |
| ONNX Runtime (native) | yes | Evaluated, not admitted: tract covers all six qualified architectures. ORT adds a native binary to the supply chain; it would mainly help DeBERTa prepare time and int8 | NOT IMPLEMENTED by decision. Revisit if int8 or prepare time becomes a product requirement |
| PyTorch / Transformers | `HuggingFaceBackend` | No Python runtime in MedScale. An isolated Python worker would need dependency admission and an executable-code policy (`trust_remote_code` stays refused) | NOT IMPLEMENTED |
| Apple MLX | `openmed/mlx/*` (bert, deberta-v2, modernbert, longformer, gliner heads) | — | NOT IMPLEMENTED. Needs a dependency-admission spec plus Apple Silicon qualification (macOS CI runners are arm64) |
| Core ML | export only (`coreml` extra) | — | NOT IMPLEMENTED |
| OpenVINO / TensorRT / GGUF | exporters and sessions | — | NOT IMPLEMENTED |
| Android | `android/openmedkit` (ORT Mobile, accelerator fallback, model cache/downloader) | No Android surface | NOT IMPLEMENTED |
| Browser / WebGPU | `js/openmedkit-web` (ORT Web, WebGPU session) | No browser runtime; the Tauri desktop is blocked (F096-T01) | NOT IMPLEMENTED |
| Backend auto-selection | `get_backend`: MLX → HF → ONNX by availability | A single backend. Pre-download expectations replace availability probing | PARTIAL |
| Offline mode | `core/offline.py` (`local_files_only`, socket blocking) | Default-deny egress everywhere. The runtime has no network code; acquisition is the only egress and needs consent | IMPLEMENTED (stricter by default) |

## 3. Architectures (ONNX token classification, measured)

| Architecture | Catalog rows | Model qualified (commit) | Prepare / warm run (CI) | Status |
|---|---|---|---|---|
| bert | 754 | DiseaseDetect-ElectraMed-33M (`54c9cc11`) | 0.9 s / 0.36 s | EXECUTED_TESTED |
| distilbert | 204 | AnatomyDetect-TinyMed-65M (`ad5e79cb`) | 0.8 s / 0.17 s | EXECUTED_TESTED |
| roberta | 209 | AnatomyDetect-TinyMed-82M (`95f0bf74`) | 1.8 s / 0.17 s | EXECUTED_TESTED |
| modernbert | 227 | AnatomyDetect-ModernClinical-149M (`66525717`) | 3.6 s / 0.9 s | EXECUTED_TESTED |
| deberta-v2 | 304 | PII-Dutch-SuperClinical-Small-44M (`e41c8009`) | 679 s / 0.42 s | EXECUTED_TESTED on CI (#196). First prepare is not interactive |
| xlm-roberta | 366 | PII-Spanish-BigMed-Large-278M fp16 (`da3fec60`) | 3.0 s / 0.15 s | EXECUTED_TESTED on CI (#196). The **65 XLM-R NER exports ship a degenerate tokenizer** (BPE, 0 merges) and are refused |
| gliner, eurobert, qwen, others | 91, 33, 30, … | — | — | NOT IMPLEMENTED (span heads, generative and vision models need different runtimes) |

"Catalog rows" are listing counts. Only the named models are `EXECUTED_TESTED`. No row is `TASK_QUALIFIED` (no labelled evaluation) or `CLINICALLY_VALIDATED`.

## 4. Task families

| Task | OpenMed v3.0.0 | MedScale | Status |
|---|---|---|---|
| NER (token classification) | `ner/*`, ONNX/HF/MLX pipelines | Runtime above; evidence-only proposals | IMPLEMENTED |
| PII token classification | PII models plus `core/pii.py`, entity merger, validators, anonymizer | The PII models run as token classifiers (DeBERTa and XLM-R evidence). OpenMed's merger, validators and anonymizer are not ported; MedScale's privacy gate (Specs 079–082) is separate | PARTIAL |
| Zero-shot (GLiNER) | `ner/families/gliner*.py`, `mlx/models/gliner_*` | — | NOT IMPLEMENTED |
| Multimodal | `openmed/multimodal/*` | — | NOT IMPLEMENTED |

## 5. External constraints (precise)

- **F096-T01:** the Tauri dependency advisory (`glib`, Linux). The founder has **not** accepted it. It blocks the Tauri chain (#174 → #179) and therefore the Models UI (T103-07).
- **The OpenMed XLM-R NER exports** need upstream republishing of their tokenizers.
- **Production signing** is `NOT_GRANTED`; **real PHI** is `NOT_AUTHORIZED`; **clinical validation** is `NOT_PERFORMED`.
- **Training-dataset terms** (BC5CDR, ANATOMY, PII sets) are unverified ([rights ledger](../../evidence/103-local-model-catalog/RIGHTS_LEDGER.md)).
