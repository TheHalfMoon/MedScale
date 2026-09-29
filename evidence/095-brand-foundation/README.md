# Spec 095 qualification evidence

**Status:** implementation candidate; native visual and exact-head CI qualification pending.

## Live base

- Repository main: `1e2b7d94e970256b38bda15fa91f62bc397e825a`.
- Main run `36480424698`: six required jobs passed.
- Specs 068–094 stay closed; draft PRs124/125 remain separate.
- New program: `docs/planning/PRODUCTIZATION_PROGRAM.md`.

## Requirement traceability

| Requirement | Implementation/evidence | Qualification status |
|---|---|---|
| R095-01/02 approved geometry, inversion, size/space | assets/brand SVGs, LOGO_SPEC, approved-identity provenance; brand_foundation_095 numeric/asset tests | proof sheet rendered at1200×1120 and inspected; actual native logo contexts pending |
| R095-03 free fonts/rights | Inter/JB Mono NL exact binary/license hashes, native typography admission, FONT_NOTICE; builder/verifier/ZIP refusal probes | bytes/official rights verified; native integration/CI packaging pending |
| R095-04 tokens | theme.slint, components.slint, typography/brand/state docs, web reference | live-token text/focus/selection contrast guard passed; native rendering pending |
| R095-05 eleven distinct states | StateBadge literal names/unique markers/accessible text; brand_foundation_095 tests | executable integration pending; no color-only state |
| R095-06 actual Home application | command-center.slint bound to existing patient and runtime VM, scrollable current route dock | route/Home truth guards passed; rendered fit pending |
| R095-07 controlled snapshots | visual_evidence.rs, capture-native-ui.py, quickstart | bounded argument/export tests implemented; candidate captures pending |
| R095-08 qualification | deterministic current guards, this record, exact-range/final CI/rendered records | partial checks below; final head/merge/post-main pending |

## Local checks performed

2026-09-29, Windows, Rust toolchain1.97.1:

- `rtk cargo fmt --all -- --check`: passed after forward formatting repair.
- `rtk git diff --check`: passed.
- `rtk proxy pwsh -NoProfile -File scripts/check-dependency-direction.ps1`: passed.
- Spec066 standard-library regression executable: 5 passed, including contrast measured from actual Theme values.
- Spec068 standard-library current brand/product binding executable: 5 passed.
- The two std-only tests were compiled with `rtk proxy rustc --edition2024 --test` and CARGO_MANIFEST_DIR pointing at medscale-core. This avoided a cold workstation workspace build; it is not a claim that cargo workspace tests ran locally.
- All16 SVGs parsed; exact inverse/runtime master consistency is additionally guarded by the new numeric tests. The proof sheet was genuinely rendered and visually inspected. Its geometry/Inter type/inverses/clear-space/small-scale samples had no clipping. This qualifies the proof layout only.

## Scope and limits

No new Cargo dependency or lockfile mutation. No Rust Core/storage/network/contract authority source changed. Current presentation regression guards move forward under new founder authority; closed spec/evidence artifacts are not rewritten. Minimal font assets are explicitly admitted and their redistribution gap is repaired. Native capture is opt-in, isolated synthetic-only and refuses existing outputs.

Use CI artifacts for the actual native qualification to protect workstation memory. Native screenshots and six exact-head/post-main jobs must be recorded before closure. No WCAG, release/privacy/multi-client readiness, clinical model qualification or real-PHI claim is made. Full launch UI freeze remains owned by100.
