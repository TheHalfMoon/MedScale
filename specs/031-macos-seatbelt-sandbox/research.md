# Research — Spec 031 macOS Seatbelt sandbox READY_BASE

## Decision: sandbox_init SBPL network-deny; App Sandbox entitlements scaffold

- Full App Sandbox requires entitlements, codesigning, and containerization — not available for ordinary cargo test binaries on GitHub Actions `macos-latest` without a signed app bundle.
- Apple still ships `sandbox_init(3)` in libSystem (deprecated in headers; used in production by Chromium and multiple CLI sandboxes). Custom SBPL strings apply with `flags=0` (not `SANDBOX_NAMED`).
- Measurable ambient deny without entitlements: `(version 1) (allow default) (deny network*)` — socket/connect/bind fail with EPERM / `PermissionDenied`.
- Honesty: this is **not** App Sandbox container FS isolation, Landlock path-beneath, or Windows AppContainer LPAC.

## Alternatives rejected for READY_BASE

| Approach | Why not now |
|---|---|
| App Sandbox entitlements + signed bundle | CI cargo tests are not entitlement-bearing apps |
| Named `pure-computation` only | Named profiles vary by OS version; custom SBPL is explicit and measurable |
| File allowlist-only SBPL | Higher profile complexity for dyld/system reads; network deny is sufficient ambient proof |
| Claim PlatformQualified | EXTERNAL_GATES + Windows AppContainer FS residual forbid |

## Dependency / FFI

- No new crate: link `sandbox_init` / `sandbox_free_error` from libSystem.
- Workspace `unsafe_code` lint is `deny` (not `forbid`) so `macos_seatbelt` may `#![allow(unsafe_code)]`.
- Provenance: `docs/engineering/admissions/031-macos-sandbox-init-ffi.md`.
