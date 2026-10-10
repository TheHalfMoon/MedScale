# Spec 103 research: exact-source gap analysis (OpenMed v3.0.0 vs MedScale)

**Date:** 2026-10-08. **Issue:** #183. **Method:** read-only metadata of the public upstream repository. No model weights were downloaded.

## 1. New comparator snapshot (does not replace the frozen v2.2.0 pin)

| Field | Value |
|---|---|
| Repository | `maziyarpanahi/openmed` (Apache-2.0 SDK) |
| Tag / commit | `v3.0.0` = `ea920f36fadd7b45935247d639f0ffa1ef493b23` (also `master` HEAD on 2026-10-08) |
| `models.jsonl` | 2,266 lines, SHA-256 `805e79f7db5514cf308c6deffa67d72c4ce9a59bf4e54487d2783eecbb1e34ab` |
| Frozen historical comparator | `v2.2.0` = `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837` (Spec 071). It is unchanged and is not retroactively updated |

## 2. What the catalog actually contains (measured from `models.jsonl`)

- **2,266 rows, 2,266 unique `repo_id`s**, all under the `OpenMed/` Hugging Face organization.
- **Families:** NER 1,093; PII 1,018; ZeroShot 143; General 9; Vision 3.
- **Tasks:** token-classification 2,254 (99.5%); text-generation 6; visual-question-answering 3; unknown 3.
- **Formats (a row may list several):** pytorch 1,512; onnx 753; mlx-fp 652; mlx-8bit 7; mlx-4bit 2; gguf 0.
- **Architectures:** bert 754, xlm-roberta 366, deberta-v2 304, modernbert 227, roberta 209, distilbert 204, gliner 91, eurobert 33, qwen 30, clinical-longformer 10, others.
- **Parameter counts (2,249 known):** 33M min, 184M median, 3.0B max.
- **Languages:** 35 distinct. Examples: en 1,310; de, es, fr, it, nl 106 each; hi 103; te 102; pt 99; tr 96; vi 27; ja 14; ar 13.
- **Licenses (row field):** apache-2.0 2,255; mit 4; other 4; missing 3. A row license is a **claim**; each model card and its weights must be checked before admission.
- **Hardware metadata is mostly absent:** only 5 rows carry `disk_mb` / `download_mb`, and 3 carry `peak_ram_mb`. Sizes must be estimated from `param_count` and format, and marked as estimates.
- **Integrity:** every row has `reproducibility_hash` (`sha256:…`). Its exact preimage (weights, config or card) is **not yet verified** and must not be treated as an artifact digest until it is.

Statements this supports: *"the OpenMed v3.0.0 manifest lists 2,266 entries"*.
Statements it does **not** support:
- "2,266 models run locally";
- "2,266 qualified models";
- any clinical-validation claim.

## 3. Upstream loading behavior (source-read at `ea920f3`)

- `openmed/core/model_registry.py` (1,897 lines):
  - `load_manifest_rows()` turns manifest rows into `ModelInfo` with category, display name, entity types, `recommended_confidence`, size category and languages;
  - `estimate_model_sizes()` / `_estimated_peak_ram_mb()` estimate sizes;
  - manifest signature verification runs when a signature is present (`verify_manifest_signature_if_present`).
- `openmed/core/models.py` (1,043 lines), `ModelLoader`:
  - `load_model(…, require_integrity=…)` loads through Transformers pipelines;
  - an explicit offline mode uses `network_blocked_if_offline` and `local_files_only=True`;
  - `load_local_sequence_classifier()` requires a 40-hex pinned revision and sets `trust_remote_code=False`;
  - a model cache with `unload_model()` / `unload_all_models()` and CUDA/MPS cache release;
  - device resolution and model suggestions.
- Other runtimes in the tree: an MLX backend (`openmed/mlx/*`, Apple Silicon); ONNX export for Android and WebGPU; ONNX Runtime Web (`js/openmedkit-web`); an Android `OnnxTokenClassifier`, `ModelCache` and `ModelDownloader`. Each is format- and platform-specific.

## 4. MedScale baseline (`main` 37f5ae8)

| Area | Current state | Spec |
|---|---|---|
| Local runtime | `tract_onnx_token_classification_v1` (`crates/medscale-pack/src/onnx_runtime.rs`): offline, pure-Rust ONNX token classification; bounded input (64 KiB), labels (512), fixed sequence length; proposal-only output | 069 |
| Pack admission | `PackStore::admit/promote`; signed Pack manifests, anti-rollback (Spec 026), `ModelSourceProvenance` | 026, 069 |
| Online acquisition | READY_BASE deny path through the Network Broker; HF online distribution is an external gate | 015 |
| Model Center UI | lists session-admitted Packs separately from qualification references | 070 |
| Comparison | Model Fleet independent lanes; factual comparison, never a ranking | 078 |
| Evidence | pinned OpenMed v2.2.0 baseline; fail-closed parity claims | 071 |

## 5. Gap matrix

| # | Capability (founder target) | OpenMed v3.0.0 | MedScale today | Gap → Spec 103 phase |
|---|---|---|---|---|
| 1 | Large searchable offline catalog | `models.jsonl`, 2,266 rows | none (admitted Packs only) | **P1**: read-only catalog store, pagination, search |
| 2 | Filters: task, language, format, device, license, RAM | partial metadata; RAM mostly missing | none | **P1**: filters; estimated sizes labelled as estimates |
| 3 | Explicit optional acquisition through the governed boundary | Hub download, optional offline mode | deny path only (015) | **P3**: opt-in acquisition via the Network Broker, consent per artifact |
| 4 | Immutable revision and digest verification | pinned revision for the local classifier; integrity flag | Pack signature and provenance | **P3**: 40-hex revision pin plus per-file SHA-256 before admission |
| 5 | Local admission via signed Packs | n/a | yes | **P3**: a catalog entry becomes a Pack only through existing admission |
| 6 | Runtime adapters per format | PyTorch, MLX, ONNX (web and mobile) | tract ONNX token classification | **P4**: broaden tract ONNX coverage (bert, roberta, deberta, xlm-r, distilbert, modernbert where tract supports the ops); evaluate ORT, GGUF and MLX later |
| 7 | Resource-aware load and unload | unload, device caches | single prepared runtime | **P4**: LRU with RAM/disk budgets, cancellation |
| 8 | Model Fleet integration | n/a | lanes (078) | **P5**: Fleet lanes accept admitted catalog models |
| 9 | Status separation | not modelled | admitted vs qualification references | **P1**: status ladder below |
| 10 | No hidden cloud or PHI movement | offline mode optional | default deny | invariant in every phase |

## 6. Status ladder (never conflated)

`DISCOVERABLE` → `RIGHTS_PENDING` → `DOWNLOADABLE_OPT_IN` → `CACHED` → `VERIFIED` → `ADMITTED` → `RUNTIME_COMPATIBLE` → `EXECUTED_TESTED` → `TASK_QUALIFIED` → `CLINICALLY_VALIDATED`.

Rules:
- A row only moves up on evidence.
- `CLINICALLY_VALIDATED` is not reachable inside this spec (`CLINICAL_VALIDATION_STATUS=NOT_PERFORMED`).
- Counts are reported per status.

## 7. Risks and constraints

- **Rights.** Row license fields can differ from model cards and base-model terms (for example `other` or missing). Admission requires a card-level check.
- **Size.** Thousands of weights are tens to hundreds of GB. Only metadata is ingested by default, and weights are never bulk-downloaded.
- **Executable artifacts.** `trust_remote_code`, pickled PyTorch `.bin` files and custom code are never loaded. Only safetensors and ONNX are admitted, under worker isolation.
- **Platform.** MLX is Apple-only. CPU-only must remain the baseline.
- **Supply chain.** New runtime crates (for example ORT bindings or llama.cpp) need full dependency admission and are not assumed.

## 8. Phase 1 evidence (local, 2026-10-08)

`ModelCatalog::import` was run on the real `models.jsonl` at `ea920f3` (metadata only), using the ignored test `catalog_snapshot_evidence`:

```text
CATALOG_EVIDENCE sha256=805e79f7db5514cf308c6deffa67d72c4ce9a59bf4e54487d2783eecbb1e34ab rows=2266
  status={Discoverable: 2259, RightsPending: 7} cpu_portable=2265 onnx=753 arabic=13
```

- The counts match an independent count of the same file.
- The run found 9 rows with `architecture: null`, which the first parser rejected. The parser now normalises them to `unknown`, with a regression test.
- These are **listing** counts only. No row is downloaded, admitted, runnable or qualified.

## 9. Execution evidence: first real OpenMed model (local, 2026-10-08)

**Path:** catalog row → Hub metadata → reproducibility-hash binding → per-file digests → signed Pack → `admit_pack_dir` → `prepare` → `run`, all in pure Rust (`tract-onnx`), on local CPU in a release build.

| Item | Value |
|---|---|
| Catalog row | `OpenMed/OpenMed-NER-DiseaseDetect-ElectraMed-33M-v1-onnx-android` (NER, en, bert, 33M params, `apache-2.0` claim) |
| Reproducibility hash | `sha256:5f22790e…14b0`, recomputed from Hub metadata: **match** |
| Pinned commit | `54c9cc119d325ddad23300a1e9108bafa9643a06` |
| `model.onnx` | 133,100,723 bytes, SHA-256 `a61a8493b191e44f…` = LFS record |
| `tokenizer.json`, `config.json` | Git blob SHA-1 = Hub `blobId` |
| Pack | `hf-openmed-ner-diseasedetect-electramed-33m-v1-onnx-android-static128`, synthetic qualification trust root |
| Timing | prepare 1,848 ms; warm run 384 ms (128 tokens) |
| Synthetic input | "The synthetic patient was diagnosed with type 2 diabetes mellitus and chronic kidney disease." |
| Output | `type 2 diabetes mel ##lit ##us` → I-DISEASE; `chronic` → B-DISEASE, `kidney disease` → I-DISEASE; everything else O |

Runtime fixes this required, both general and both covered by the existing Spec 069 tests:

1. `normalize_static_shape_ops`: `tract-onnx` types ONNX `Shape` and `Cast(INT64)` as symbolic `TDim`, and analysis then failed at the position-id `Range` ("Impossible to unify TDim with I64"). With all inputs fixed to `[1, L]`, these are rewritten to `i64`.
2. The tokenizer's embedded padding and truncation are disabled. OpenMed `tokenizer.json` pads to 512; the runtime owns padding and refuses over-length input.

Status for this one model: `EXECUTED_TESTED` on Windows x64 CPU. It is **not** `TASK_QUALIFIED` (no labelled evaluation run) and **not** `CLINICALLY_VALIDATED`.

Known limits:
- tokenizers without `[PAD]` (for example XLM-R `<pad>`) are still refused by the runtime;
- int8, fp16 and other architectures are not yet executed.

## 10. Representative architecture matrix (local, 2026-10-08)

Each model went through the same path: catalog row, then reproducibility-hash binding, per-file digests, signed Pack, admission, `prepare`, and `run`. All runs were on Windows x64 CPU (release build), with no network during verification or execution and with synthetic sentences. Weights were fetched once at the pinned commit for qualification and are not vendored.

| Architecture | Catalog row (OpenMed v3.0.0) | Commit | Result | Prepare / warm run | Non-O output on a synthetic sentence |
|---|---|---|---|---|---|
| bert (ElectraMed) | `OpenMed-NER-DiseaseDetect-ElectraMed-33M-v1-onnx-android` | `54c9cc11` | **EXECUTED_TESTED** | 1.8 s / 0.38 s | `type 2 diabetes mellitus`, `chronic kidney disease` → DISEASE |
| distilbert | `OpenMed-NER-AnatomyDetect-TinyMed-65M-v1-onnx-android` | `ad5e79cb` | **EXECUTED_TESTED** | 3.7 s / 0.61 s | `left knee`, `right ankle` → Anatomy |
| roberta (`<pad>`) | `OpenMed-NER-AnatomyDetect-TinyMed-82M-v1-onnx-android` | `95f0bf74` | **EXECUTED_TESTED** | 6.0 s / 0.65 s | `left knee`, `right ankle` → Anatomy |
| modernbert | `OpenMed-NER-AnatomyDetect-ModernClinical-149M-v1-onnx-android` | `66525717` | **EXECUTED_TESTED** | 12.0 s / 1.8 s | `knee`, `ankle` → Anatomy |
| deberta-v2 | `OpenMed-PII-Dutch-SuperClinical-Small-44M-v1-onnx-android` | `e41c8009` | **NOT RUNTIME_COMPATIBLE** | prepare fails | `tract` `Sign` does not support `I64` (relative-position buckets). One occurrence sits inside an `If` subgraph that the public `tract` API cannot rewrite |
| xlm-roberta | `OpenMed-NER-AnatomyDetect-BigMed-278M-v1-onnx-android` | `cf00a087` | **NOT ADMITTED / NOT RUNTIME_COMPATIBLE** | — | fp32 `model.onnx` (1.11 GB) exceeds the 1 GiB Pack bound (Spec 069 policy, unchanged); `model_int8.onnx` (855 MB) admits but fails `prepare` (quantized operators) |

The pad token now comes from the tokenizer (BERT `[PAD]`, RoBERTa/XLM-R `<pad>`), which was needed for RoBERTa.

**Catalog impact.** These four architectures cover 1,394 catalog rows (bert 754, distilbert 204, roberta 209, modernbert 227). They are still `DISCOVERABLE`: only the four executed models are `EXECUTED_TESTED`.

**Not runnable yet:**
- deberta-v2 (304 rows) and xlm-roberta (366 rows);
- gliner, eurobert, qwen and the remaining families;
- MLX-only rows (Apple);
- the generative and vision rows.

**Next options:**
- **XLM-R fp32:** needs a governed decision to raise the ONNX bound, or external-data ONNX.
- **DeBERTa:** needs operator support upstream in `tract`, or an ONNX Runtime backend, which requires dependency admission.

No row is `TASK_QUALIFIED` (no labelled evaluation) or `CLINICALLY_VALIDATED`.

## 11. DeBERTa-v2 and XLM-R on tract; CI qualification (2026-10-10)

This section supersedes the "next options" in §10. Neither ONNX Runtime nor a raised Pack bound was needed:
- **DeBERTa-v2:** `tract_onnx::ops::logic::If` exposes `then_body` and `else_body` publicly. The prepare-time rewrite replaces `Sign` with an integer-capable op in both branches.
- **XLM-R:** the fp16 export (555 MB) fits the 1 GiB bound. It is widened to fp32 in memory, because tract's fp16 `LayerNormalization` declares f16 but computes f32. Symbolic `value_info` dimensions are relaxed first.

Real models are now qualified on free GitHub runners by `.github/workflows/model-qualification.yml` (`qual/**` branches only, synthetic text, weights never kept). The founder workstation could not hold the release build plus a 566 MB model in memory: the local DeBERTa run was paged out and was stopped. Run 37977100903 executed all six architectures; per-model figures are in [`CAPABILITY_MATRIX.md`](CAPABILITY_MATRIX.md) §3.

Findings:
- The XLM-R **NER** exports ship a degenerate tokenizer (BPE, 250,002 entries, zero merges; run 37974743874). MedScale refuses it; the PII exports (Unigram) run.
- DeBERTa-v2 first prepare takes 9–13 minutes on CI (T103-15).
- Spans from SentencePiece/byte-level tokenizers carried the word-start space. Decoded spans are trimmed (#194).
- Outputs on single synthetic sentences are partial (`Vries` and `María` missed). This is execution evidence, not task quality.
