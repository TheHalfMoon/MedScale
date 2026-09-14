# Local Qualification — Spec 060

Date: 2026-09-15
Platform: macOS development host (not qualified release hardware)

## Gates
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS.
- `cargo test --workspace --locked`: PASS.
- `git -c core.whitespace=cr-at-eol diff --check`: PASS.
- `cargo test --locked -p medscale-core --test native_desktop_ui_060`: 3/3 PASS.
- Native `MedScale` window observed through macOS System Events: PASS.
- `medscale-desktop --smoke`: PASS without opening UI.
- `medscale-desktop --perf-idle-ms 50`: PASS without opening UI.

## Development measurements
Current Slint 1.13.1 + Winit + femtovg/OpenGL build:
- release binary: 9,207,168 bytes (~8.78 MiB);
- live RSS sample after window appeared: 130,112 KiB (~127 MiB).

Renderer comparison on the same host/surface:
- software-renderer experiment: 8,871,520-byte binary, 143,792 KiB RSS;
- femtovg was retained: ~3.8% larger binary in that experiment but lower sampled RSS and a GPU-backed path better aligned with polished desktop motion/rendering.

These are development-host observations only. They do not clear `QUALIFIED_RELEASE_PERFORMANCE_HARDWARE` and do not claim final UI latency or memory-budget attainment.

## Accessibility observation
Slint accessibility is enabled and primary custom actions use `FocusScope`, keyboard Enter/Space activation, visible focus borders, accessible roles/labels, and default accessibility actions. macOS exposed the native window and accessibility elements to System Events. This is engineering evidence only, not screen-reader or WCAG conformance qualification.

## Visual review limitation
The remote macOS process did not have Screen Recording permission, so `screencapture` could not produce a visual artifact. Native window existence and title were observed through WindowServer/System Events. Final visual/a11y qualification remains owned by later product slices and Spec 067.
