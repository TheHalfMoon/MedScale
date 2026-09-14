# Research — Spec 060

## Native shell decision

### Slint 1.16.1 — ADMIT
- Native Rust UI toolkit, no WebView.
- Windows, macOS, and Linux desktop support.
- OS accessibility integration is explicit and remains enabled.
- Slint 1.16.1 declares Rust MSRV 1.88; MedScale raises workspace `rust-version` from 1.85 to 1.88 for this admitted UI runtime.
- Declarative markup maps well to the founder-approved high-fidelity direction while keeping Rust authority code separate.
- The 1.16.1 femtovg renderer uses femtovg 0.23.2 with a `swash` path and no active `lru` dependency.

### Why not Slint 1.13.1
The first exact-head PR #103 run exposed `RUSTSEC-2026-0253` through `femtovg 0.17 -> lru 0.16.4`. MedScale rejected an advisory ignore. The software renderer removed the advisory but increased sampled macOS RSS materially. Slint 1.16.1 provides the preferred GPU-backed renderer without the vulnerable `lru` path, at the bounded cost of raising MSRV to 1.88.

### Iced — REJECT FOR FINAL PRODUCT SHELL NOW
Cross-platform and attractive, but stable accessibility integration remains incomplete relative to final-v0 qualification goals.

### GPUI — REJECT FOR CROSS-PLATFORM PRODUCT SHELL NOW
Excellent performance/design potential, but public framework maturity, Windows support, and accessibility path remain less stable than required for this healthcare desktop launch.

### egui — REFERENCE/TOOLING ONLY
Strong immediate-mode tooling and AccessKit support, but less aligned with the polished structured application surfaces and typography/layout direction selected for MedScale.

### Tauri/WebView — CONTINUE REJECTED
A native shell is available, so MedScale does not need to weaken privacy by adding a browser engine.

## Design methodology
Impeccable is adopted as a review vocabulary and anti-pattern checklist. We use its shape/critique/audit/polish/harden/optimize sequence but do not install it as product runtime code.
