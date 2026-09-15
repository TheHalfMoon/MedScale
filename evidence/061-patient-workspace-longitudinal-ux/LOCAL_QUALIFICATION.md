# Local Qualification — Spec 061

Date: 2026-09-15
Platform: macOS development host (not qualified release hardware)

## Results
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS after correcting one new `redundant_closure` lint without changing behavior.
- `cargo test -p medscale-core --test native_patient_workspace_061 --locked`: 4/4 PASS.
- `cargo test -p medscale-desktop --locked`: PASS, including patient view-model and runtime performance regressions.
- `cargo test --workspace --locked`: PASS.
- `cargo +1.88.0 check --workspace --all-targets --locked`: PASS.
- `medscale-desktop --smoke`: PASS; reports `tauri_admitted=false` and `desktop_shell=slint_native`.
- `medscale-desktop --perf-idle-ms 50`: PASS without opening a window.
- `git -c core.whitespace=cr-at-eol diff --check`: PASS.

Clean rebuilds were used after deleting only the untracked `target/` cache to recover local disk space. One combined final rerun reached local ENOSPC during linking after fmt/clippy had passed; no product/test failure occurred. The cache was deleted again, `cargo test --workspace --locked` then passed from a clean build, and the Rust 1.88 all-target check passed in a separate clean cache. No repository content or evidence was discarded. The generated Spec 027 perf scratch artifact was restored and is not part of this change.

Baseline main verification before the 061 branch: run `34913680562` on `a0142c20fd1870ba2f0eccc0b29e709d382a8593` completed successfully with all six required jobs. Exact-head 061 CI and cross-platform package qualification remain required before merge.
