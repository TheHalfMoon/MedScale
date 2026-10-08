# Spec 103 plan

Phases are bounded PRs with exact-head CI and normal merges. Each phase extends existing crates; there is no rewrite.

| Phase | Scope | Where | Exit evidence |
|---|---|---|---|
| **P1 Catalog core** | `ModelCatalog` types (row, status ladder, filters, page/search); importer for a local `models.jsonl` snapshot with SHA-256 and commit binding; size and RAM estimator labelled `estimated` | new module in `crates/medscale-pack` (`catalog.rs`) plus a Core capability `ListModelCatalog` (read-only) | unit and property tests; import of a **synthetic 10,000-row** manifest under a time budget; malformed and duplicate rows rejected; zero-network test |
| **P2 Catalog surfaces** | CLI `medscale models catalog …`; Tauri Models route catalog tab (filters, per-status counts, pagination) | `crates/medscale-cli`, `apps/desktop-tauri` (after the stack merges) | CLI tests; frontend tests; native screenshots; no network observed |
| **P3 Opt-in acquisition** | per-artifact consent; Network Broker transfer of pinned revision files; per-file SHA-256; resume; quota; Pack creation through existing admission | `medscale-broker`, `medscale-pack` | tampered, partial, gated and oversize negative tests against a local fixture server; no Hub contact in tests |
| **P4 Runtime breadth** | tract ONNX compatibility predicates per architecture; LRU with RAM/disk budgets; unload and cancellation | `onnx_runtime.rs`, `runtime.rs` | per-architecture synthetic ONNX fixtures; eviction tests; CPU-only timing budgets on CI |
| **P5 Representative real models** | 3–5 small Apache-2.0 ONNX token classifiers (for example one English NER, one multilingual PII, one DeBERTa), selected with card-level rights; executed locally | evidence only, no bundled weights | `EXECUTED_TESTED` evidence per model and platform; status counts recorded |
| **P6 Fleet integration** | admitted catalog models as Model Fleet lanes | Spec 078 code paths | fleet tests with synthetic models |

Constraints that apply to every phase:
- offline default;
- Core is the sole authority;
- no PHI;
- no paid services;
- dependency admission for any new crate;
- status-scoped claims only.
