# Open findings — Spec 096

| ID | Finding | State / owner |
|---|---|---|
| F096-T01 | `glib` 0.18.5 remains in the Tauri 2.12 Linux graph and is affected by RUSTSEC-2024-0429 / GHSA-wrw7-89jp-8q8g (`VariantStrIter` unsoundness; upstream fix requires >=0.20). The GTK3 stack does not resolve that version directly. | OPEN. Do not ignore the advisory or call Linux security qualification PASS. Investigate a bounded upstream backport or supported graph before shipping. |
| F096-T02 | Unmodified MPL-2.0 `cssparser` 0.37.0, `cssparser-macros` 0.7.1, `dtoa-short` 0.3.5, `option-ext` 0.2.0 and `selectors` 0.38.0 require source availability and notices. | Exact-version admission recorded; cargo-deny licenses passed. Native packaging verification remains OPEN: archive digest/source/notice receipts must accompany artifacts. Global license allow list unchanged. |
| F096-T03 | The only native IPC command reports shell/Core availability; actual Core Host/session/authorization DTO integration is absent. | OPEN. No clinical data is invented; implement narrow existing read paths. |
| F096-T04 | Browser source/build does not qualify native visual fidelity or focus/keyboard behavior. | OPEN. Capture and inspect actual Tauri frames and interactions at both target sizes/themes. |
| F096-T05 | WebView incognito/CSP/local assets do not prove privacy containment across cache/crash/log/storage. | OPEN. Synthetic-only; private-data readiness remains false. |
| F096-T06 | Required Jev/Alibaba OCR independent provider reviews have no established zero-cost path; 095 is also unmerged. | BLOCKED_BY_ZERO_COST / dependency. Do not merge or claim review PASS. |

The first 2.12 all-platform cargo-deny run reported advisories FAILED, licenses FAILED, bans/sources OK. A subsequent license-only check passed after exact-version admissions. The advisory finding remains a failing qualification gate. Duplicate-version warnings remain informational. The six canonical CI jobs and new native preview checks still need a final committed head.
