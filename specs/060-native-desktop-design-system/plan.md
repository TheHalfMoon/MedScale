# Plan — Spec 060

1. Normalize founder-approved product/design truth into `PRODUCT.md` + `DESIGN.md`.
2. Record Slint admission and update final-v0 gate status from absent to supplied/implementation-in-progress.
3. Add Slint 1.13.1 + `slint-build` to the native Desktop crate.
4. Create an original MedScale M SVG and reusable native UI tokens/components.
5. Preserve existing headless smoke/perf code paths; normal launch enters the native UI.
6. Implement Home/Command Center shell with deterministic synthetic fixture data.
7. Add keyboard/focus/accessibility semantics and design-system regression tests.
8. Run local fmt/clippy/workspace tests and manually launch on macOS for visual/runtime smoke.
9. Push PR; require exact-head six-check CI; close canonically only after post-merge main verification.
10. Promote Spec 061 only after 060 closure.
