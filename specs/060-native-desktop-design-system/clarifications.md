# Clarifications — Spec 060

- Founder visual direction is now supplied and approved; the old “do not invent final UI” blocker no longer applies to visual direction.
- Slint is selected over WebView/Tauri, Iced, GPUI, and egui for this phase because it is native, cross-platform, compatible with Rust 1.85 at the admitted 1.13.1 release, and has OS accessibility integration.
- Slint is admitted under its royalty-free desktop/mobile/web license path; attribution requirements must be preserved in distribution documentation/About surface. No Slint source is copied into MedScale.
- Impeccable is used as design-review methodology only; it is not a runtime dependency and is not vendored into the application.
- 060 is a vertical slice, not a claim that all approved screens are complete.
- Synthetic/demo data in UI is explicitly non-production and non-PHI.
- CLI remains first-class and will receive product-experience parity work before mobile begins.
