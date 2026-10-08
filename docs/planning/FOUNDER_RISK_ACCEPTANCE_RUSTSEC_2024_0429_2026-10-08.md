# Founder Risk Acceptance: RUSTSEC-2024-0429 in the Tauri desktop (2026-10-08)

**Status:** `ACTIVE`, time-limited (revisit on every Tauri / wry / webkit2gtk upgrade; expires when a Tauri release admits `glib >= 0.20`).
**Authority:** explicit founder decision, 2026-10-08, given in the founder session after the analysis below was presented.
**Finding:** F096-T01 (`specs/096-tauri-foundation/findings.md`). It **stays OPEN**.
**Scope:** merging the productization stack (#174, #175, #177, #179) into `main` while the `tauri dependency policy` job fails **only** on this advisory. Nothing else is accepted.

## Advisory

- `glib` 0.18.5, RUSTSEC-2024-0429 (informational: `unsound`), patched in `>= 0.20.0`.
- Affected functions: `VariantStrIter` iterator methods (`next`, `nth`, `last`, `next_back`, `nth_back`). They pass `&p` instead of `&mut p` to a C out-argument, which can cause a NULL dereference in optimized builds.
- `glib` enters only through Tauri 2.x's Linux GTK3 / WebKitGTK stack. It is not compiled into the Windows or macOS builds.

## Why it cannot be remediated now

The latest `webkit2gtk` (2.0.2) still requires `glib ^0.18` / `gtk ^0.18`, and Tauri 2.12 / wry 0.57 depend on it. No stable Tauri release admits a patched `glib` (rechecked 2026-10-08 on crates.io).

## Reachability analysis (2026-10-08)

The analysis used the versions locked in `apps/desktop-tauri/src-tauri/Cargo.lock`: tauri 2.12.0, tauri-runtime-wry 2.12.0, wry 0.57.0, tao 0.37.1, muda 0.20.0, tray-icon 0.25.1, webkit2gtk 2.0.2, gtk/gdk 0.18.2, gio 0.18.4, glib 0.18.5, libappindicator 0.9.0, soup3 0.5.0, javascriptcore-rs 1.1.2.

- In `glib` 0.18.5, `VariantStrIter` is constructed only by the public `Variant::array_iter_str()`, plus `glib`'s own tests.
- A source search of every crate in the local registry cache found **no caller** of `array_iter_str` or `VariantStrIter` outside `glib` itself.
- MedScale's code does not call `glib` at all.

Conclusion: the unsound code is not reachable from MedScale's dependency graph as locked. This is a static source search, not a proof, and it must be repeated on every dependency update.

## Conditions of the acceptance

1. No advisory ignore is added, and cargo-deny is not weakened. The `tauri dependency policy` job keeps failing visibly on this advisory.
2. F096-T01 stays `OPEN`. Its PR, queue and completion records cite this acceptance.
3. Any dependency update in the Tauri graph repeats the reachability search. If a caller appears, the acceptance lapses.
4. The acceptance does not change `PRIVATE_DATA_READY=false`, `RELEASE_READY=false`, `PLATFORM_QUALIFIED=false`, signing (`NOT_GRANTED`), or any other gate.
5. Any other advisory, or any other failure of the dependency-policy job, is **not** covered and blocks the merge.
