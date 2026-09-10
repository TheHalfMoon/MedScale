# Plan: Spec 029 macOS CI + Accessibility Honesty

1. Spec Kit package (specify/clarify/plan/research/ADR/tasks/checklist/analyze/converge).
2. Expand `.github/workflows/ci.yml` rust matrix with `macos-latest`; add macOS Homebrew openssl/perl step when needed; keep `--locked`.
3. Add `AccessibilityDoctorStatus` in contracts; wire into `DoctorReport` + CLI doctor display + notes.
4. Extend `ReleaseQualificationDoctorStatus` with `macos_ci_present=true`; keep `macos_qualified=false` / `release_ready=false`; replace `qualified_os_matrix_macos` missing class with `macos_platform_product_qualification`.
5. FixtureUi accessible labels + CLI help label/code test (no WCAG claim).
6. Evidence under `evidence/029-macos-ci-accessibility/`; LIMITATIONS honest.
7. BUILD_QUEUE / roadmap / START_HERE → 029 CLOSED; deferred **030+**.
8. Gates: fmt, clippy `-D warnings`, `cargo test --workspace --locked` (`CARGO_TARGET_DIR=D:\medscale-target`).

## Architecture

```text
.github/workflows/ci.yml
  matrix.os: [ubuntu-latest, windows-latest, macos-latest]
        |
        v
ReleaseQualificationDoctorStatus
  macos_ci_present=true
  macos_qualified=false   # baseline CI ≠ product PLATFORM_QUALIFIED
  release_ready=false

AccessibilityDoctorStatus (READY_BASE)
  fixture_cli_labels_checked
  cli_keyboard_path_documented
  disclosure_clarity_checked
  wcag_conformance_claimed=false
  final_v0_ui_present=false
  release_ready=false
        |
        v
FixtureUiViewModel.accessible_label + CLI --help codes
  (honesty tests only; not a WCAG audit)
```
