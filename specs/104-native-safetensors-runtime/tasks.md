# Spec 104 tasks

- [~] T104-01 Dependency admission: `candle-core`, `candle-nn`, `candle-transformers` `=0.9.1`, CPU only. `cargo deny` passes on the workspace (research §3).
- [~] T104-02 `PackArtifactKind::SafetensorsModel` with the 1 GiB bound; pickle stays forbidden.
- [~] T104-03 `candle_runtime.rs`:
  - allowlist: bert, distilbert, roberta, xlm-roberta, deberta-v2, modernbert;
  - biased linear head;
  - document execution;
  - tiny-BERT unit tests.
- [ ] T104-04 Snapshot Pack building and acquisition for `.safetensors`.
- [ ] T104-05 Core: residency and Model Fleet lanes for candle Packs.
- [ ] T104-06 Real-model CI qualification per architecture; compatibility expectations for PyTorch-format rows.
