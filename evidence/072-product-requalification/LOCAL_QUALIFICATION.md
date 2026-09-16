# Spec 072 Local Qualification

## Candidate binding

- Branch: `spec/072-product-requalification`
- Canonical base: `e2031bdef918980cc184aed811f381e41f5dfb8b`
- Qualified implementation commit: `02d339ea7982ad5227468650aa0d3c1cfaa530b8`
- Qualified implementation tree: `4d2cb61f32b0bbfd90b014226f608af6d625ac03`
- Default toolchain: `rustc 1.97.1` / `cargo 1.97.1`
- MSRV qualification toolchain: `rustc 1.88.0` / `cargo 1.88.0`

## Final results

- `cargo fmt --all -- --check` — PASS.
- `git diff --check` — PASS.
- `cargo test -p medscale-core --test product_requalification_072 --locked` — PASS (`5/5`) on the committed implementation head.
- Spec 059 release-closure regression — PASS (`2/2`).
- Spec 067 final-UI regression — PASS (`5/5`).
- Spec 068 product-differentiation regression — PASS (`5/5`) after advancing the living progression guard to Spec 072.
- Spec 069 local-runtime regression — PASS (`6/6`).
- `cargo test -p medscale-desktop --locked` — PASS (`23/23` unit tests; `3/3` runtime-performance tests).
- Native Desktop smoke — PASS (`medscale-desktop smoke ok`, `tauri_admitted=false`, `desktop_shell=slint_native`).
- Native Desktop bounded perf-idle probe — PASS (`medscale-desktop perf-idle-ready`).
- `cargo clippy --workspace --all-targets --locked -- -D warnings` — PASS.
- `cargo +1.88.0 check --workspace --all-targets --locked` — PASS.
- `cargo test --workspace --locked` — PASS on the final source tree after environmental artifact repair.

## Findings resolved during qualification

1. Rebuilt-product review found two stale comparative-superiority phrases on the native UI: `what OpenMed still does better` and `See where OpenMed is ahead`. They were replaced with pinned-evidence / claim-state wording, and Spec 072 now regresses against their return and against unmanifested `PROVEN ADVANTAGE`, `OPENMED AHEAD`, `PARITY`, and `SURPASS` verdict assignments.
2. The historical Spec 068 living progression guard still required `NEXT_PROMOTED_SPEC = 071`; it was advanced to require Spec 072 and reject Spec 073 while 072 is active.
3. The release external-gate pointers for qualified-hardware performance and final accessibility were advanced from the Spec 067 packets to the rebuilt-product Spec 072 packets, including Models/Evidence coverage.

## Environmental events

The authorized host had very low free disk space during qualification. An initial smoke build and two full-workspace attempts failed with `errno=28` / `No space left on device` during compilation or linking; these events were not treated as product failures or passes. Only rebuildable build/cache artifacts were removed. Partial-target binaries left malformed by the ENOSPC attempts (`host_session_018`, `network_broker_013`, `outbox_restart_034`, and one Desktop artifact) were identified by non-executable/data-file state, removed with their fingerprints, rebuilt, and their focused tests passed before the final workspace rerun. The final `cargo test --workspace --locked` completed with exit code 0.

## Honesty boundary

This qualification does not claim WCAG conformance, qualified-hardware performance attainment, production signing/notarization, signed macOS product/App Sandbox qualification, private-data readiness, multi-client release readiness, real-PHI authorization, OpenMed parity/surpass, or release readiness. The release doctor continues to expose exactly five external release evidence classes. Spec 012/MESC remains optional, deferred, untouched, and non-blocking.
