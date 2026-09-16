# Plan — Spec 070

## Slice A — authority-backed inventory
- Project `PacksList` manifests into Model Center rows.
- Preserve Core ownership; Desktop must not depend on `medscale-pack`.
- Render explicit session-local empty state and persistence boundary.

## Slice B — local operator admission
- Open a Desktop `CliSession` only for the interactive UI path.
- Admit an absolute local Pack path via `PacksInstallLocal`.
- Surface refusal/error state and refresh Core inventory after successful admission.

## Slice C — truthful runtime presentation
- Separate `SESSION INVENTORY`, `QUALIFICATION REFERENCE`, and `CAPABILITY GAP`.
- Expose digest, runtime, device, trust, benchmark and promotion fields.
- Remove stale `FIXTURE`/pre-069 copy.

## Slice D — qualification
- Add deterministic VM regressions for empty and admitted Pack state.
- Verify Slint compilation, Desktop smoke/perf behavior, Rust 1.88, Clippy and workspace tests.
- Run exact-range review, required exact-head CI, protected merge and post-main verification.
