# Spec 104 research and dependency admission

## 1. Why

OpenMed v3.0.0 `models.jsonl` (`ea920f36`), measured:

| Format | Rows |
|---|---|
| `pytorch` | 1,512 |
| `onnx` | 753 |
| `mlx-*` | 661 |

Rows without ONNX, by architecture:

| Architecture | Rows |
|---|---|
| bert | 502 |
| xlm-roberta | 244 |
| deberta-v2 | 219 |
| roberta | 139 |
| distilbert | 136 |
| modernbert | 113 |
| gliner | 91 |
| other | 75 |

The PyTorch rows publish `model.safetensors`. For example, `OpenMed/OpenMed-NER-AnatomyDetect-ElectraMed-33M` at `3c914174` ships `model.safetensors` (66 MB), `config.json` and `tokenizer.json`.

OpenMed runs these models through Python Transformers/PyTorch (`core/backends.py` `HuggingFaceBackend`). MedScale has no Python runtime, and embedding one would add an interpreter, a package supply chain and an executable-code surface. A native Rust reader of safetensors weights runs the same checkpoints without any of that.

## 2. Candidates

| Candidate | Result |
|---|---|
| Python worker (Transformers/PyTorch) | Rejected: interpreter plus pip supply chain plus code execution; contradicts R103-06 |
| ONNX conversion of PyTorch weights | Rejected: needs Python tooling, and produces derived artifacts with no upstream digest |
| `candle` 0.11.0 | Rejected: `candle-core` depends on `tokenizers 0.22.2` (C++ `esaxx-rs` build, duplicate of the workspace `tokenizers 0.23.2`) and `zip 8` |
| `candle` 0.10.2 | Rejected: same `tokenizers 0.22.2` / `esaxx-rs` dependency |
| **`candle` 0.9.1** (`candle-core`, `candle-nn`, `candle-transformers`; `default-features = false`) | **Admitted**: pure Rust, CPU, no tokenizer dependency, encoders for all six target architectures |

## 3. Admission evidence for `candle` 0.9.1

- **Licenses:** `MIT OR Apache-2.0` (Hugging Face). Its 128-package transitive tree contains only licenses already allowed by `deny.toml`.
- **`cargo deny --all-features check advisories licenses bans sources`** against the repository's `deny.toml`, on the isolated tree (2026-10-10, cargo-deny 0.20.2): `advisories ok, bans ok, licenses ok, sources ok`. The only warnings are duplicate versions (`gemm` 0.17/0.18 families, `syn`, `bitflags`, `pulp`, `raw-cpuid`), which the policy sets to `warn`.
- **No build-time network or binary download:** crates.io sources only, no git dependencies, no `build.rs` fetching. `default-features = false` excludes the `cuda`, `cudnn`, `metal`, `mkl` and `accelerate` features.
- **Notable transitive crates:**
  - `gemm` (CPU matrix kernels);
  - `safetensors` 0.4.5 (the format parser);
  - `half`;
  - `rayon`;
  - `memmap2` (unused: MedScale loads from verified in-memory bytes);
  - `zip` 1.1.4 (candle's `.npz` reader; MedScale never calls it);
  - `ug` 0.4 (kernel IR used by optional GPU paths).
- **Encoders used:** `bert::BertModel`, `distilbert::DistilBertModel`, `xlm_roberta::XLMRobertaModel` (also used for `roberta`), `debertav2::DebertaV2Model`, `modernbert::ModernBert`.

## 4. Implementation notes from source reading

- `candle-transformers` 0.9.1 `DebertaV2NERModel` loads its classifier with `linear_no_bias`. Hugging Face `DebertaV2ForTokenClassification` has a bias, so MedScale builds every head itself as `linear(hidden, labels)` with bias at `classifier`.
- DistilBERT's attention mask is inverted (nonzero means masked; `masked_fill` with `-inf`). Other encoders take the usual 1 = attend mask.
- ModernBERT token classification: `model.*` (encoder), `head.dense` / `head.norm` (`ModernBertHead`), then `classifier`.
- Hugging Face prefixes: `bert.`, `distilbert.`, `roberta.`, `deberta.`; ModernBERT uses `model.`.
- `VarBuilder::from_buffered_safetensors(bytes, DType::F32, &Device::Cpu)` converts bf16/fp16 tensors to f32 on access.

## 5. What the PyTorch-format rows actually ship (sampled 2026-10-10)

- **MLX repositories:** OpenMed v3.0.0 lists them (`*-mlx`) with formats `["mlx-fp", "pytorch"]`, but they publish MLX weights only. This covers 527 of the 1,109 PyTorch-format rows of the five evidenced architectures, and 122 of the 244 XLM-R rows. They are not candidates for this runtime, and the compatibility expectation treats any MLX format as "no runnable artifact".
- **Non-MLX rows:** 15 of the 15 sampled ship `model.safetensors`, giving 582 candidates for the five architectures. A pickle-only repository would be refused at acquisition.
- **GLiNER zero-shot repositories** (91 rows, for example `OpenMed-ZeroShot-NER-Anatomy-Tiny-60M` at `65f3fb2f`) ship only `pytorch_model.bin` (pickle). They are refused by R103-06 / R104-02 until upstream publishes safetensors.
