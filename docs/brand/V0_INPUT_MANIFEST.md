# MedScale V0 Input Manifest

**Status:** FOUNDER_APPROVED_HANDOFF — Spec 093

Use only the canonical inputs below when generating the MedScale UI reference.

## Brand assets

- `assets/brand/medscale-mark-gradient.svg` — primary ScaleFold M.
- `assets/brand/medscale-app-icon.svg` — app icon treatment.
- `assets/brand/medscale-mark-monochrome-dark.svg` — dark-ink monochrome.
- `assets/brand/medscale-mark-monochrome-light.svg` — light-ink monochrome.
- `assets/brand/intro/medscale-intro-light.svg` — light brand/intro reference.
- `assets/brand/intro/medscale-intro-dark.svg` — dark brand/intro reference.

Do not redesign the logo in V0. Treat its geometry and spectrum as fixed.

## Tokens

- `assets/brand/tokens/medscale.tokens.json` — canonical machine-readable source.
- `assets/brand/tokens/medscale.css` — generated CSS variables for reference implementation.

Regenerate/check with `python3 scripts/generate-brand-tokens.py` and `python3 scripts/generate-brand-tokens.py --check`.

## Typography

- `assets/brand/fonts/InstrumentSans-*.ttf`
- `assets/brand/fonts/SourceSerif4-*.ttf`

Use Instrument Sans for UI/product and Source Serif 4 selectively for editorial/evidence reading. Do not add a competing display family.

## Design authority

- `DESIGN.md`
- `docs/brand/BRAND_IDENTITY_SYSTEM.md`
- `docs/brand/SCALEFOLD_LANGUAGE.md`
- `docs/brand/LOGO_SPEC.md`
- `docs/brand/DESIGN_TOKENS.md`
- `docs/brand/TYPOGRAPHY_SYSTEM.md`
- `docs/brand/ICON_SYSTEM.md`
- `docs/brand/PRODUCT_UI_GRAMMAR.md`
- `docs/brand/COMPONENT_SYSTEM.md`
- `docs/brand/PAGE_BLUEPRINTS.md`
- `docs/brand/MOTION_AND_INTERACTION.md`
- `docs/brand/V0_MASTER_PROMPT.md`
- `docs/brand/V0_ACCEPTANCE_CHECKLIST.md`

## Runtime boundary

The V0 output is a visual/reference implementation until separately promoted. It must not add clinical authority, provider/network permissions, model admission, real PHI handling, release claims, or MESC scope.
