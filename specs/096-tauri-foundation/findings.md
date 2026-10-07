# Open findings — Spec 096

| ID | Finding | State / owner |
|---|---|---|
| F096-T01 | `glib` 0.18.5 remains in the Tauri 2.12 Linux graph and is affected by RUSTSEC-2024-0429 / GHSA-wrw7-89jp-8q8g (`VariantStrIter` unsoundness; upstream fix requires >=0.20). The GTK3 stack does not resolve that version directly. | OPEN. Do not ignore the advisory or call Linux security qualification PASS. Investigate a bounded upstream backport or supported graph before shipping. |
| F096-T02 | Unmodified MPL-2.0 `cssparser` 0.37.0, `cssparser-macros` 0.7.1, `dtoa-short` 0.3.5, `option-ext` 0.2.0 and `selectors` 0.38.0 require source availability and notices. | Exact-version admission recorded; cargo-deny licenses passed. All three native packaging jobs passed at 76e9fe3. Windows package verification checked 1,027 file sizes/digests with zero mismatches, including five original source archives and complete MPL text. Rebind receipts at the final head; global license allow list unchanged. |
| F096-T03 | The only native IPC command reported shell/Core availability; actual Core Host/session/authorization DTO integration was absent. | ADDRESSED IN PR #177 (unmerged): Rust owns an in-process Core `CliSession`; 67 narrow named commands are validated in Rust and authorized by Core; the WebView holds no vault key, passphrase or Core handle. Synthetic-only. Closes only when #177 merges with its gates. |
| F096-T04 | Browser source/build does not qualify native visual fidelity or focus/keyboard behavior. | Windows evidence captured and inspected at 76e9fe3: both Home themes/sizes, palette and unavailable Patients; see evidence/096-tauri-foundation/native/76e9fe3. Current-head binding, independent visual review, Figma comparison and macOS/Linux visual acceptance remain OPEN. |
| F096-T05 | WebView incognito/CSP/local assets do not prove privacy containment across cache/crash/log/storage. | OPEN. Synthetic-only; private-data readiness remains false. |
| F096-T06 | Required Jev/Alibaba OCR independent provider reviews have no established zero-cost path; 095 is also unmerged. | BLOCKED_BY_ZERO_COST / dependency. Do not merge or claim review PASS. |
| F096-T07 | Canonical CI used checkout's PR merge-ref default. The 342dd28 Ubuntu job actually checked out `03cc7519a58b5cb9e686678caa5aa232645b0dbc`, a generated merge into the remote 095 base. Run metadata alone does not bind the tested tree to the submitted head. | Forward repair sets all canonical checkouts to the PR head (main runs use the event SHA). Exact-head execution still requires verification from fresh job logs and package receipts. Historical CI results remain recorded without being rewritten as head-tree tests. |

### F096-T01 upgrade-path investigation (2026-10-07, PR #177)

Checked against crates.io at that date, with `cargo tree -i glib@0.18.5 --target x86_64-unknown-linux-gnu` on the PR #177 lockfile:

- Path into the graph: `tauri 2.12.0` → `gtk 0.18.2` / `webkit2gtk 2.0.2` / `tao 0.37.1` / `muda 0.20.0` / `wry 0.57.0` → `glib 0.18.5` (plus `gio`, `pango`, `cairo-rs`, `soup3 0.5`, `javascriptcore-rs 1.1.2` from the same 0.18 family).
- Patched upstream exists only outside Tauri's pinned range: `gtk 0.19.0` (GTK3 bindings) depends on `glib ^0.22`, and `javascriptcore-rs 2.0.0` on `glib ^0.22`.
- No supported Tauri release adopts it:
  - `tauri 2.12.1` (latest stable) requires `gtk ^0.18`, `muda ^0.20`, `webkit2gtk ^2`.
  - `tauri-runtime-wry 2.12.1` requires `gtk ^0.18`, `webkit2gtk =2.0`, `wry ^0.57`, `tao ^0.37`.
  - `webkit2gtk 2.0.2` (latest), `wry 0.57.0` (latest), `tao 0.37.1` (latest), `muda 0.21.0` (latest) all require `gtk ^0.18`.
  - `tauri 3.0.0-alpha.4` still requires `gtk ^0.18` (adds `gtk4` alongside); it is a pre-release major migration and out of scope.
- A `[patch]` to `glib >= 0.20` is not API-compatible with the `gtk 0.18` family and would mean forking the whole GTK3/WebKitGTK binding stack under Tauri. That is an unsupported, out-of-scope migration and is not done.
- Reachability of `glib::VariantStrIter` from MedScale code is not claimed either way; MedScale code does not call it directly.

Result: F096-T01 stays OPEN and the `tauri dependency policy` (cargo-deny advisories) gate stays honestly FAILED. No advisory ignore is added (`deny.toml` keeps `ignore = []`). Re-check when a stable Tauri 2.x release moves `webkit2gtk`/`wry`/`tao`/`muda` to `gtk >= 0.19`.

The first 2.12 all-platform cargo-deny run reported advisories FAILED, licenses FAILED, bans/sources OK. A subsequent license-only check passed after exact-version admissions. The advisory finding remains a failing qualification gate. Duplicate-version warnings remain informational. The six canonical CI jobs and new native preview checks still need a final committed head.
