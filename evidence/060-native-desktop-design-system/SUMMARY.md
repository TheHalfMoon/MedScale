# Spec 060 Evidence Summary

State: `IN_REVIEW` pending exact-head CI / merge / post-merge verification.

Founder-approved visual direction supplied on 2026-09-15 is normalized into `PRODUCT.md` and `DESIGN.md`. MedScale Desktop now has a real native Slint 1.16.1 shell with the original MedScale M, canonical palette, navigation, global search, command palette, Home/Command Center, synthetic operational data, local/privacy posture, About/Slint attribution, keyboard focus semantics, and safe route-only shell actions.

`medscale-desktop --smoke` and `--perf-idle-ms` remain headless. The live doctor reports `desktop_shell=slint_native`, `final_v0_ui_present=true`, `tauri_admitted=false`, `wcag_conformance_claimed=false`, and `release_ready=false`.

Local full fmt/clippy/workspace tests and macOS native-window launch passed; see `LOCAL_QUALIFICATION.md`. Exact-head three-OS CI is still required before canonical closure.

The first PR exact-head run exposed and rejected `RUSTSEC-2026-0253` in the Slint 1.13.1 femtovg path. The candidate was hardened to Slint 1.16.1 / MSRV 1.88, which removes the vulnerable `lru` package from the active all-features/all-target graph without disabling the GPU renderer or weakening `cargo-deny`.
