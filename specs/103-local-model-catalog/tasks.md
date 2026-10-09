# Spec 103 tasks

Status as of 2026-10-10 (after #191–#196). Capability matrix: [`CAPABILITY_MATRIX.md`](CAPABILITY_MATRIX.md). `[x]` done, `[~]` partly done (the gap is stated), `[ ]` not started.

- [x] T103-01 (P1) `catalog.rs`: `CatalogRow`, `CatalogStatus` (10-state ladder), snapshot source binding (repository, commit, SHA-256), strict JSONL parser with required fields validated and duplicates rejected. Unknown fields are ignored, not kept. Merged in #184.
- [~] T103-02 (P1) Filters: task, family, language, format, architecture, license claim, device fit, status, parameter and disk ceilings; stable ordering; offset pagination. **Gap:** no tier filter; offset rather than cursor pagination.
- [x] T103-03 (P1) Size estimator from parameter count and format, labelled `estimated`, or `unknown` when it cannot be derived (#184).
- [~] T103-04 (P1) Tests: synthetic 10,000-row import within budget; malformed, duplicate and oversize input; pagination; filters (#184). **Gap:** no socket-detection test (the module has no network code).
- [x] T103-05 (P1) Core surface `medscale_core::model_catalog` (read-only query, snapshot → Pack), with tests (#184). Dispatched as `Capability::ModelCatalogRead` (#193).
- [x] T103-06 (P2) CLI `medscale packs catalog`, `packs admit-snapshot` (#184) and `packs acquire` (#187).
- [ ] T103-07 (P2) Tauri Models route catalog tab. **Blocked:** needs the Tauri chain (#174 → #177), which is not merge-eligible while F096-T01 is open.
- [~] T103-08 (P3) Governed opt-in acquisition (#187):
  - per-model consent and byte ceiling;
  - Governed Browse transport;
  - Hub host allowlist;
  - pinned-commit files with exact size and digest;
  - atomic staging;
  - hermetic scripted-transport tests.

  File-level resume of interrupted acquisitions from verified staging (#192). **Gap:** no byte-range resume inside one file (the transport sends no `Range`). **The live CLI acquisition is not executed on the founder workstation** (host memory); real snapshots are acquired and verified on CI by the qualification workflow.
- [~] T103-09 (P4) Budgeted LRU residency pool; Core prepared-model cache migrated to it (#185). Pre-download runtime expectation per catalog row from recorded evidence, shown in the Core catalog view and CLI (#190). **Gap:** expectations are architecture-level, not per-model verification.
- [~] T103-10 (P5) Execution evidence for four real OpenMed v3.0.0 models: bert, distilbert, roberta, modernbert (`research.md` §9–10). Card-level rights ledger with base-model lineage: `evidence/103-local-model-catalog/RIGHTS_LEDGER.md`. **Gap:** training-dataset terms (BC5CDR, ANATOMY, PII sets) are not verified.
- [x] T103-11 (P6) Catalog-admitted Packs run as Model Fleet lanes (end-to-end test, #188).
- [~] T103-12 Bare model-count claims are blocked by the identity guard (`apps/desktop-tauri/src/identity.test.ts`, on #179). **Gap:** lands with the Tauri chain.
- [x] T103-13 Whole-document execution without truncation: windows, best-context vote and BIOES entity decoding ported from OpenMed v3.0.0 (`token_windows.rs`, #194), with whitespace-trimmed spans.
- [x] T103-14 DeBERTa-v2 and XLM-R on tract: integer `Sign` with `If`-branch rewrite, symbolic `value_info` relaxation, fp16→fp32 widening at load, degenerate-tokenizer refusal, stage-specific prepare diagnostics (#196). Qualification run 37977100903 executed all six architectures.
- [ ] T103-15 DeBERTa-v2 first prepare takes 9–13 minutes on CI (tract optimisation of the `If`-heavy graph). **Gap:** not interactive. Residency caching hides repeats only.
- [~] T103-16 Runtimes beyond tract ONNX. PyTorch-format weights: Spec 104 native safetensors runtime (#198, in review; 5 architectures executed on CI). Still open: ONNX Runtime (evaluated, not admitted), MLX/Core ML, Android, WebGPU, GLiNER zero-shot, multimodal. Each needs its own dependency-admission spec.
