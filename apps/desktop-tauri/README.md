# MedScale Tauri presentation preview

Synthetic-only, preparatory Spec 096. Slint remains the current functional reference. Core IPC is **not connected**: the only native command, `get_shell_status`, reports that limitation. Route bodies show unavailable states and contain no fabricated patient or evidence records.

From this directory:

```text
npm ci --ignore-scripts
npm test
npm run build
npm run tauri -- dev
```

The isolated Rust workspace requires Rust 1.90 or newer; CI uses 1.97.1. Native builds require the platform Tauri/WebView development dependencies. The founder workstation's existing broken MSVC toolchain is an open local build gate. Do not begin duplicate cold native builds there.

## Capability and navigation policy

The main local window can call only the generated `allow-get-shell-status` permission. No Tauri plugin or built-in filesystem, HTTP, shell, process, clipboard or window control permission is granted. The command is a shell diagnostic and exposes no privileged Core operation. Navigation accepts the exact packaged application origins; debug builds also accept the exact loopback Vite origin. New windows are denied. Production CSP has no remote connection origin; loopback HMR exists only in development CSP. Native window decorations preserve platform conventions.

WebView incognito mode is enabled as a mitigation. It does not establish cache/crash/log/storage containment; that requires native measurement. Theme choice is ephemeral for this session. Fonts, icons and application assets are bundled locally. The logo/icon asset is deterministically rasterized from the approved first-party SVG with the locked Tauri CLI; no mobile app is added by this scaffold.

## Qualification status

Strict frontend build and two route tests pass locally. Native compilation, physical keyboard/focus, screenshots, performance, cross-platform behavior and privacy are not qualified. Exact MPL-2.0 license exceptions are documented with source obligations; cargo-deny license checks pass, and the native preview package must carry verified source archives/notices. The Linux `glib` 0.18.5 RUSTSEC-2024-0429 finding remains open. No advisory exception was added. Jev and Alibaba OCR remain blocked by the zero-cost constraint. Spec 095 dependency is still open. Do not infer product or private-data readiness from the preview.
