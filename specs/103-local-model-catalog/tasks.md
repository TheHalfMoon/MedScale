# Spec 103 tasks

- [ ] T103-01 (P1) `catalog.rs`: `CatalogRow`, `CatalogStatus` (10-state ladder), `CatalogSnapshot { source, commit, sha256, rows }`, strict JSONL parser (unknown fields kept, required fields validated, duplicates rejected).
- [ ] T103-02 (P1) Filters and search: task, language, format, architecture, tier, parameter range, license claim, estimated size, device fit; stable ordering; cursor pagination.
- [ ] T103-03 (P1) Size estimator from parameter count and format, marked `estimated`; `unknown` when not derivable.
- [ ] T103-04 (P1) Tests: synthetic 10,000-row manifest (generated in-test); malformed, duplicate and oversize rows; no network (the test fails if any socket opens); performance budget.
- [ ] T103-05 (P1) Core capability `ListModelCatalog` (read-only), plus authority tests.
- [ ] T103-06 (P2) CLI `medscale models catalog import|list|search|show`.
- [ ] T103-07 (P2) Tauri Models route catalog tab, after the productization stack merges.
- [ ] T103-08 (P3) Acquisition design record and Network Broker extension; consent UX; resume and quota; negative tests against a local fixture server.
- [ ] T103-09 (P4) ONNX architecture compatibility predicates; LRU and budgets.
- [ ] T103-10 (P5) Rights ledger and execution evidence for 3–5 real Apache-2.0 ONNX models.
- [ ] T103-11 (P6) Model Fleet lanes for admitted catalog models.
- [ ] T103-12 Status-scoped claim lint for README and release notes (shared with #182).
