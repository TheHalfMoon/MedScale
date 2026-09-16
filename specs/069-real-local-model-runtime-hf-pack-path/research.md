# Research — Spec 069

## Runtime selection

`tract-onnx = 0.22.4` is the portable CPU baseline because it remains compatible with MedScale Rust 1.88 and does not require a default runtime-binary download path. `tract-onnx 0.23.x` raises the MSRV beyond the project floor. `ort` remains an accelerated-runtime candidate for Spec 071; its default binary-download behavior is not accepted as the MedScale default.

`tokenizers = 0.23.2` is compiled without `hf-hub` or HTTP acquisition features. Hugging Face is an artifact source, not a runtime authority or network plane.

### Rust 1.88 transitive lock qualification

`tract-onnx 0.22.4` brings `liquid-core 0.26.11`, whose `kstring 2.x` range initially resolved to `kstring 2.0.4`. That release declares Rust 1.96 and correctly failed the repository's Rust 1.88 qualification. MedScale therefore pins the transitive lockfile selection to `kstring 2.0.2`, which declares Rust 1.73 compatibility. This is a lock-level MSRV compatibility pin, not a new direct product dependency. The final candidate passes `cargo +1.88.0 check --workspace --all-targets --locked`. Do not raise the `kstring` lock selection above 2.0.2 without requalifying the workspace MSRV.

## Artifact byte-bound contract

Spec 069 bounds artifact bytes before any model/tokenizer load: ONNX model artifacts are capped at 1 GiB, tokenizer/fixture artifacts at 64 MiB, and model provenance metadata at 1 MiB. Pack admission and direct runtime verification both enforce the same ceilings with bounded reads. The runtime feeds the already-verified tokenizer/model bytes directly into `tokenizers` and tract rather than reopening artifact paths after digest verification, so a post-verification file growth or swap cannot become the bytes that are executed.

## Qualification model

The external qualification model is `onnx-community/bert-base-NER-ONNX` at revision `9faa2f4a2d59b396888b318f596ff719cc893f1e`, file `onnx/model_int8.onnx`, MIT license. It is a general-domain NER qualification model, not a clinical production model.

The original ONNX SHA-256 is `b324e829f1fad3b897f926d1a1d1372803c6d04546831a3a2ed103652b916adf`. The pinned tokenizer SHA-256 is `343989712a36cd8b253efeaf8baf6a08b9d2583f78e395e83832e8ee9f8d8ee1`.

## Shape specialization

The upstream export carries symbolic `batch_size` and `sequence_length` dimensions throughout graph metadata. Tract 0.22.4 cannot infer that graph safely from input facts alone. Spec 069 therefore uses a deterministic metadata-only specialization: `batch_size=1`, `sequence_length=128` across graph inputs, outputs, value-info, and nested graph attributes. No model weights are changed.

With `onnx==1.19.0`, the transform replaces 637 `batch_size` and 680 `sequence_length` symbols and deterministically produces SHA-256 `183057ca3123d7bd09fda062738390b326b1298c58d0eee39f0003f5a8920f14`.

## Honesty boundary

Passing the qualification model proves real local Hugging Face-origin ONNX execution and signed provenance. It does not prove clinical-domain quality, PII/de-identification safety, real-PHI authorization, broad model parity with OpenMed, or accelerated Apple Silicon performance. Those claims remain open for Specs 070–072.
