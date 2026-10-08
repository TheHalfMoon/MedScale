# Pre-release identity and provenance audit (issue #182)

**Date:** 2026-10-08. **Branch audited:** the productization stack top (`claude/tauri-release-packaging`, PR #179), which contains the current desktop.
**Rule:** product-facing identity is MedScale's own. Technical source provenance, licenses and history are kept, accurate and separate. No history rewrite. No removal of required notices.

## 1. Product-facing surfaces

| Surface | Finding | Disposition |
|---|---|---|
| `README.md` intro | Led with agent instructions and a status block containing `UI_VISUAL_SOURCE = v0`; described OpenMed as "capability floor/strategic donor" | **Changed**:<br>• a MedScale-first product description, with a clear engineering-preview / synthetic-only status;<br>• the engineering and agent sections moved below it, unchanged in content;<br>• `UI_VISUAL_SOURCE = v0` removed (superseded by Specs 095–101);<br>• OpenMed described as a **pinned comparator** under "Comparisons and provenance" |
| Desktop window / product name | `MedScale` (`tauri.conf.json`, `index.html` title "MedScale — local clinical intelligence") | Kept; guarded by test |
| Bundle metadata | publisher `MedScale`; copyright "MedScale contributors, Apache-2.0"; short description says synthetic-only engineering build; identifier `org.medscale.desktop.preview` | Kept. The `.preview` identifier is intentional until a release decision; changing it would orphan app data |
| App icons (`src-tauri/icons`) | Generated from the approved paired-triangle mark (commit `a023526`) | Kept. First-party |
| Desktop UI copy | No template text, external URLs or generator boilerplate. The Evidence route's "MedScale vs OpenMed" ledger is the Spec 071 measured comparison, not branding | Kept |
| About page | Lists MedScale Apache-2.0 and the font licenses (Inter, JetBrains Mono, OFL-1.1) | Kept |
| Release bundles | No source maps emitted (Vite default; 0 `.map` files in `dist`); NOTICE, SBOM, licenses and MPL sources in every release set (Spec 101 verifier) | Kept |
| Brand colors and mark | Black/white plus sky blue `#70B8C7` and orange `#FB905A` (2026-10-07); superseded #125 identity not present | Guarded by test (`ScaleFold`) |

## 2. Provenance retained (not product branding)

| Record | Status |
|---|---|
| `LICENSE` (Apache-2.0) | Retained |
| `third_party/provenance/*` (identity vectors, fonts with exact source revisions and SHA-256) | Retained; font digests re-verified 2026-10-08 |
| `assets/brand/fonts/*-OFL.txt`, `FONT_NOTICE.md` | Retained (license requirement) |
| Generated `NOTICE.md`, `SBOM.cdx.json`, `licenses/`, `source-archives/` (MPL) | Retained; required in each release set |
| Planning records (`OSS_CODE_ABSORPTION_MATRIX_V2`, `OPENMED_PARITY_SURPASS_MATRIX_V2`, `SOURCE_ACQUISITION_AND_COPY_PLAN`, `V0_UI_INTEGRATION_CONTRACT`) | Retained as engineering provenance. The README labels the v0 contract historical |
| Specs, evidence, captures, PR descriptions and bot summaries | Historical records. **Not altered**; exempt from the identity guard |

The founder states that permission exists to reuse or adapt donor source code. That permission does not remove license or notice obligations. Source-specific licenses remain in the records above. Any separately granted written permission must be added to `third_party/provenance` before it is relied on for an attribution exception. No such exception is applied in this audit.

## 3. Automated guard

`apps/desktop-tauri/src/identity.test.ts` runs in every `tauri-preview` job and checks two things:

1. **Product-facing surfaces** (README intro, desktop UI source, HTML shell, bundle metadata) contain no:
   - stale `v0` visual-source marker;
   - "strategic donor" or "capability floor" positioning;
   - generator boilerplate;
   - superseded #125 identity;
   - placeholder text;
   - bare model-count claim.

   The product name and window title must be MedScale.
2. **Required records exist:** `LICENSE`, `third_party/provenance`, the font license texts, and the NOTICE/SBOM generators.

The test was verified to fail when a stale phrase is inserted into the README intro.

## 4. Open items

- A final native visual QA pass and screenshots for release material are part of the release checklist. Release remains blocked by its gates (`RELEASE_READY=false`).
- The similarity/copy audit of UI code against donor templates:
  - the current Tauri UI was written in this repository from the MedScale design canvas;
  - no template files are present;
  - a formal tool-based similarity scan is not performed (no zero-cost tool admitted).
