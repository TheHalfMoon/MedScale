# MedScale Design Tokens

**Status:** CANONICAL_SPEC_093

## Single source of truth

`assets/brand/tokens/medscale.tokens.json` is the canonical machine-readable source for MedScale brand colors, light/dark semantic surfaces, geometry, and type scale.

Generated artifacts:
- `crates/medscale-desktop/ui/theme_tokens.slint`
- `assets/brand/tokens/medscale.css`

Regenerate:

```bash
python3 scripts/generate-brand-tokens.py
```

Verify no drift:

```bash
python3 scripts/generate-brand-tokens.py --check
```

Generated files must never be edited manually.

## Color architecture

### Signature spectrum

The brand spectrum runs from Azure through Iris/Violet to Pink. It is reserved for:
- the ScaleFold M;
- app icon and intro/onboarding moments;
- bounded brand illustrations or a single focal accent;
- marketing/reference surfaces where color is not confused with product state.

The spectrum is not used to encode success, warning, danger, provenance, model admission, or clinical meaning.

### Product themes

Light mode uses cool near-white canvas, white work surfaces, midnight ink, quiet blue-gray metadata, and a solid indigo interaction color.

Dark mode uses deep navy-black canvas, layered graphite/navy surfaces, soft white ink, cool gray metadata, and a lighter indigo interaction color.

The permanent rail may remain deep obsidian in both themes so the gradient mark has a stable recognition field.

### Semantic state

Success, warning, and danger have independent accessible colors in each theme. Every semantic color must be paired with text, an icon, shape, or explicit state label.

## Geometry

- control radius: 10 px;
- compact radius: 8 px;
- panel radius: 14 px;
- large brand/editorial surface radius: 22 px;
- focus ring: 2 px.

Pills are for statuses, compact filters, and intentionally capsule-shaped controls only. Cards should not become nested rounded containers by default.

## Type scale

The canonical product scale is machine-readable in the token source. Page and section hierarchy should use the scale rather than inventing one-off sizes where practical.

Display typography is allowed in marketing/reference/onboarding surfaces, not as an excuse for oversized headings inside dense research workspaces.

## Spacing and layout

The token source also defines the shared spacing rhythm and shell geometry. Use spacing tokens instead of arbitrary gaps where practical.

Reference shell values:
- global rail: 58 px;
- named-route sidebar: 224 px reference width;
- optional inspector: 320 px reference width;
- context/top bar: 52 px;
- primary workspace padding: 32 px.

These values are defaults, not hard constraints for every responsive width.

## Motion

Canonical motion durations are stored in `motionMs`. They cover hover/control response, pane changes, dialogs, and bounded intro timing. Reduced-motion mode may shorten or remove non-essential transforms while preserving state clarity.
