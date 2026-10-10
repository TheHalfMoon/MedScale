# Spec 104 tasks

- [~] T104-01 Dependency admission: `candle-core`, `candle-nn`, `candle-transformers` `=0.9.1`, CPU only. `cargo deny` passes on the workspace (research §3).
- [~] T104-02 `PackArtifactKind::SafetensorsModel` with the 1 GiB bound; pickle stays forbidden.
- [~] T104-03 `candle_runtime.rs`:
  - allowlist: bert, distilbert, roberta, xlm-roberta, deberta-v2, modernbert;
  - biased linear head;
  - document execution;
  - tiny-BERT unit tests.
- [~] T104-04 Snapshot Pack building (`model.safetensors` plus `config.json`, candle runtime declared) and acquisition consent for `.safetensors`; end-to-end test `tests/safetensors_pack_104.rs`.
- [x] T104-05 Core: `PacksEvaluateLocal` dispatches candle Packs to their own residency pool (2 GiB, keyed by content digest); MedAgent / Model Fleet lanes run the Pack's declared runtime through `medscale_pack::evaluate_admitted_pack` (test `candle_runtime_core_104.rs`).
- [x] T104-06 Real-model CI qualification: run 37986892863 executed bert, distilbert, roberta, deberta-v2 and modernbert from PyTorch-format rows (`model.safetensors`). Scores match the tract ONNX runs of the same models to about four decimals. Pre-download expectation `expected_runnable_safetensors` for those architectures; PyTorch-format XLM-R is known unsupported (1.11 GB > 1 GiB bound).
