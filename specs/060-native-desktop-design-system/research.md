# Research — Spec 060

## Native shell decision

### Slint 1.13.1 — ADMIT
- Native Rust UI toolkit, no WebView.
- Windows, macOS, and Linux desktop support.
- OS accessibility integration is enabled by default and explicit accessibility properties are available.
- Slint 1.13.1 declares Rust MSRV 1.85, matching MedScale workspace `rust-version = 1.85`.
- Declarative markup is suitable for translating the approved high-fidelity visual direction while keeping Rust authority code separate.

### Iced — REJECT FOR FINAL PRODUCT SHELL NOW
Cross-platform and attractive, but stable accessibility integration remains incomplete. This conflicts with final-v0 qualification goals.

### GPUI — REJECT FOR CROSS-PLATFORM PRODUCT SHELL NOW
Excellent performance/design potential, but public framework maturity, Windows story, and accessibility path remain less stable than required for this healthcare desktop launch.

### egui — REFERENCE/TOOLING ONLY
Strong immediate-mode tooling and AccessKit support, but less aligned with the polished structured application surfaces and typography/layout direction selected for MedScale.

### Tauri/WebView — CONTINUE REJECTED
The prior privacy gate remains unnecessary: a native shell is available, so MedScale does not need to weaken privacy by adding a browser engine.

## Design methodology
Impeccable is adopted as a review vocabulary and anti-pattern checklist. We use its shape/critique/audit/polish/harden/optimize sequence but do not install it as product runtime code.
