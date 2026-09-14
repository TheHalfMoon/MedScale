# Admission 060 — Slint Native UI Toolkit

**Decision:** ADMITTED for `medscale-desktop` product UI.
**Version line:** `1.13.1`, pinned exactly in Cargo.
**Reason:** native, non-WebView, Windows/macOS/Linux, declarative UI, OS accessibility integration, and Rust 1.85 compatibility.

## License
Slint is offered under multiple licenses. MedScale uses the royalty-free Desktop/Mobile/Web application license path and must retain the required Slint attribution in an About surface or qualifying public distribution page. MedScale remains Apache-2.0; this admission does not relicense MedScale source.

## Security / authority boundary
- Slint is presentation only.
- UI code does not open MedScale canonical storage, keys, network clients, or partner endpoints directly.
- All authority-changing operations remain behind typed Core Host/facade boundaries.
- Tauri/WebView remains absent.

## Update policy
Stay pinned to 1.13.1 until MedScale's workspace MSRV is intentionally raised and a dependency/release qualification unit admits a newer Slint line.

## Cargo-deny scoped transitive exceptions
Slint's platform graph introduces narrowly scoped platform licenses that are not part of MedScale's global allowlist:
- `dwrote@0.11.5` — `MPL-2.0` (Windows text backend)
- `clipboard-win@5.4.1` — `BSL-1.0`
- `error-code@3.4.0` — `BSL-1.0`

`deny.toml` permits these licenses only for the exact crates above. It does not globally allow MPL-2.0 or BSL-1.0.
