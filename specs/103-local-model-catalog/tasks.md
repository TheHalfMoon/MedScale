# Spec 103 tasks

Status as of 2026-10-09. `[x]` done, `[~]` partly done (the gap is stated), `[ ]` not started.

- [x] T103-01 (P1) `catalog.rs`: `CatalogRow`, `CatalogStatus` (10-state ladder), snapshot source binding (repository, commit, SHA-256), strict JSONL parser with required fields validated and duplicates rejected. Unknown fields are ignored, not kept. Merged in #184.
- [~] T103-02 (P1) Filters: task, family, language, format, architecture, license claim, device fit, status, parameter and disk ceilings; stable ordering; offset pagination. **Gap:** no tier filter; offset rather than cursor pagination.
- [x] T103-03 (P1) Size estimator from parameter count and format, labelled `estimated`, or `unknown` when it cannot be derived (#184).
- [~] T103-04 (P1) Tests: synthetic 10,000-row import within budget; malformed, duplicate and oversize input; pagination; filters (#184). **Gap:** no socket-detection test (the module has no network code).
- [~] T103-05 (P1) Core surface `medscale_core::model_catalog` (read-only query, snapshot → Pack), with tests (#184). **Gap:** exposed as Core functions, not as a dispatched `Capability`.
- [x] T103-06 (P2) CLI `medscale packs catalog`, `packs admit-snapshot` (#184) and `packs acquire` (#187).
- [ ] T103-07 (P2) Tauri Models route catalog tab. **Blocked:** needs the Tauri chain (#174 → #177), which is not merge-eligible while F096-T01 is open.
- [~] T103-08 (P3) Governed opt-in acquisition (#187):
  - per-model consent and byte ceiling;
  - Governed Browse transport;
  - Hub host allowlist;
  - pinned-commit files with exact size and digest;
  - atomic staging;
  - hermetic scripted-transport tests.

  **Gap:** no resume of interrupted downloads, and **the live acquisition of a real model is not yet executed** (host memory).
- [~] T103-09 (P4) Budgeted LRU residency pool; Core prepared-model cache migrated to it (#185). **Gap:** no per-architecture compatibility predicates; incompatibility surfaces at `prepare`.
- [~] T103-10 (P5) Execution evidence for four real OpenMed v3.0.0 models: bert, distilbert, roberta, modernbert (`research.md` §9–10). **Gap:** licenses come from the Hub card claim; a card-level rights review is not recorded. DeBERTa-v2 and XLM-R are not runtime-compatible.
- [x] T103-11 (P6) Catalog-admitted Packs run as Model Fleet lanes (end-to-end test, #188).
- [~] T103-12 Bare model-count claims are blocked by the identity guard (`apps/desktop-tauri/src/identity.test.ts`, on #179). **Gap:** lands with the Tauri chain.
