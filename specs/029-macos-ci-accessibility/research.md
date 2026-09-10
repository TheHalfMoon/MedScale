# Research — Spec 029 macOS CI + Accessibility Honesty

## R1: macOS CI vs product qualification

| Layer | Meaning | Spec 029 |
|---|---|---|
| CI matrix runner | `cargo fmt/clippy/test --locked` on `macos-latest` | **In scope** → `macos_ci_present=true` |
| Product PLATFORM_QUALIFIED | Packaging, notarization, UX, privacy shell claims on macOS | **Out of scope** → `macos_qualified=false` |

Missing evidence class becomes `macos_platform_product_qualification` (CI presence alone is not enough for RELEASE_READY OS matrix).

## R2: SQLCipher / OpenSSL on macOS

Workspace uses rusqlite `bundled-sqlcipher-vendored-openssl`. Vendored OpenSSL typically needs Perl to build (Windows already installs StrawberryPerl). GitHub `macos-latest` usually has Perl, but Homebrew `openssl@3` + `perl` is a defensive, documented step matching founder guidance.

## R3: Accessibility READY_BASE scope

Delivery plan: accessibility acceptance starts at each boundary. Spec 029 covers **engineering honesty** for existing CLI/fixture paths:

- Stable FixtureUi surface titles / accessible labels
- CLI `--help` exposes named subcommands/flags operators can navigate without GUI
- Disclosure / doctor honesty fields remain clear (`synthetic_only`, `release_ready=false`)

Explicitly **not**: WCAG 2.x audit, contrast measurement, screen-reader product testing, final v0 UI.

## R4: Doctor field placement

New top-level `accessibility` axis (like `workflow`) rather than burying flags only under release_qualification, so operators see a11y posture independently of OS matrix.
