# Research: Tauri release packaging (Spec 101)

All facts below were read on 2026-10-07 from the exact upstream source tag that matches the locked CLI. `apps/desktop-tauri/package-lock.json` pins `@tauri-apps/cli` **2.12.0**; source tag `tauri-cli-v2.12.0`, `crates/tauri-bundler`.

## Build-time downloads by bundle target

| Target | Tool | Exact source | Verification in bundler | Decision |
|---|---|---|---|---|
| Windows NSIS | NSIS 3.11 | `https://github.com/tauri-apps/binary-releases/releases/download/nsis-3.11/nsis-3.11.zip` | SHA-1 `EF7FF767E5CBD9EDD22ADD3A32C9B8F4500BB10D` (`windows/nsis/mod.rs`) | Admitted for engineering packages. The job also records the archive's SHA-256. SHA-1 is a weak hash, so this is a recorded residual, not a supply-chain PASS. |
| Windows NSIS | `nsis_tauri_utils` 0.5.3 | `https://github.com/tauri-apps/nsis-tauri-utils/releases/download/nsis_tauri_utils-v0.5.3/nsis_tauri_utils.dll` | SHA-1 `75197FEE3C6A814FE035788D1C34EAD39349B860` | Same as above. |
| Windows MSI | WiX 3.14.1 | `https://github.com/wixtoolset/wix3/releases/download/wix3141rtm/wix314-binaries.zip` | SHA-256 `6ac824e1642d6f7277d0ed7ea09411a508f6116ba6fae0aa5f2c7daa2ff43d31` (`windows/msi/mod.rs`) | Deferred (MSI out of scope). |
| Linux AppImage | linuxdeploy, AppRun, `linuxdeploy-plugin-appimage` | `.../releases/download/continuous/linuxdeploy-plugin-appimage-{arch}.AppImage` (floating `continuous`) | none found in `linux/appimage/linuxdeploy.rs` | **Rejected.** A floating, unverified download fails pinned supply-chain policy. |
| Linux deb | none | system `dpkg-deb` semantics implemented in Rust | n/a | Admitted. |
| macOS app/dmg | none | system `hdiutil` plus the bundled `bundle_dmg.sh` | n/a | Admitted. |

Tools are cached under the user cache directory (`<cache>/tauri/NSIS`, `<cache>/tauri/WixTools314`) and reused when the hash still matches. CI may cache that directory; the cache key must include the CLI version.

## WebView2 install mode

`WebviewInstallMode::default()` is `DownloadBootstrapper { silent: true }` (`tauri-utils/src/config.rs`). With that default, the installer downloads the WebView2 bootstrapper at **install** time, from `https://go.microsoft.com/fwlink/p/?LinkId=2124703`, when the runtime is missing.

| Mode | Install-time network | Size | Decision |
|---|---|---|---|
| `downloadBootstrapper` (default) | yes, if the runtime is missing | small | rejected for engineering packages (hidden install-time network) |
| `embedBootstrapper` | yes, the bootstrapper downloads the runtime | +~1.8 MB | rejected for the same reason |
| `offlineInstaller` | no | +~127 MB, a Microsoft-licensed redistributable | deferred; redistribution terms review needed |
| `skip` | no | none | **chosen.** The WebView2 Evergreen Runtime is a documented prerequisite, shipped with Windows 11 and serviced on supported Windows 10. A missing runtime fails at launch rather than downloading silently. |

## Runtime egress (F096-T05)

The installer does not change runtime behavior. The packaged app keeps Tauri's default WebView2 arguments (`--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection`, wry 0.57.0). The F096-T05 investigation already separates `MEDSCALE_APP_EGRESS` (none observed) from `EMBEDDED_PLATFORM_RUNTIME_EGRESS` (observed, owned by the Microsoft runtime).

## Signing

Authenticode, Apple Developer ID plus notarization, and distro repository keys all need paid or organizational identities that are not granted (`DESKTOP_RELEASE_SIGNING_PROVENANCE = NOT_GRANTED`). The packages are therefore unsigned.

- **Windows:** SmartScreen may warn users on first run.
- **macOS:** Gatekeeper blocks an unsigned, un-notarized app by default; users must explicitly override it. Ad-hoc signing (`codesign -s -`) is free and gives no publisher identity. It may be used only to satisfy Apple Silicon's executable-signature requirement, and is recorded as ad-hoc, never as signed.

## Relation to existing contracts

- Spec 058 manifest fields are reused: source SHA, tree SHA, `Cargo.lock` SHA-256, payload SHA-256 list, platform, and the `signed`, `notarized`, `release_ready` and `reproducible_binary_build` flags. Spec 101 adds the frontend lockfile, the second `Cargo.lock` (`src-tauri`), installer and executable hashes, SBOM and NOTICE digests, and toolchain versions.
- The Spec 054 SBOM generator is reused unchanged. The Tauri crate is a separate Cargo workspace, so its dependency inventory is shipped as `rust-dependencies.json` (as the Spec 096 package already does) until a governed SBOM extension covers it.
