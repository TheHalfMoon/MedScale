# Clarifications — Spec 029

| ID | Question | Resolution |
|---|---|---|
| C01 | Does macos CI imply macos_qualified? | No — `macos_ci_present=true`, `macos_qualified=false` |
| C02 | Claim WCAG? | No — `wcag_conformance_claimed=false` |
| C03 | Final v0 UI required? | No — `final_v0_ui_present=false`; gate FINAL_V0_UI_ARTIFACT remains |
| C04 | RELEASE_READY? | Remains FALSE |
| C05 | Homebrew openssl/perl always? | Install on macOS runners defensively for vendored OpenSSL/SQLCipher |
| C06 | Deferred advanced? | **030+** after this unit closes |
