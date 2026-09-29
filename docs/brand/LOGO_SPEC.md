# MedScale Logo Specification

**Status:** CANONICAL_SPEC_093 — FOUNDER_APPROVED_SCALEFOLD_M

## Master mark

The MedScale master mark is the **ScaleFold M**.

It uses three vertical geometric forms:
1. a left folded pillar;
2. a second folded pillar with the same shoulder logic and slightly shifted rhythm;
3. a shorter closing pillar.

The repeated diagonal shoulder creates a recognizable family rhythm while the shorter closing form makes the mark asymmetric and memorable. The mark must still read clearly between 20 px and app-icon scale.

Runtime source of truth:
- `crates/medscale-desktop/ui/assets/medscale-mark.svg`
- `crates/medscale-desktop/ui/assets/medscale-app-icon.svg`

Monochrome brand assets:
- `assets/brand/medscale-mark-monochrome-dark.svg`
- `assets/brand/medscale-mark-monochrome-light.svg`

## Meaning

The mark is intentionally abstract. It may suggest scale, structured growth, translation from source to action, and repeated evidence layers, but no one metaphor is required for recognition.

The logo is not a chart, hospital symbol, ECG trace, brain, shield, or AI sparkle.

## Color

The primary mark uses the MedScale Spectrum:
- Azure `#0B73FF`
- Cobalt `#3D5BFF`
- Iris `#6B4EFF`
- Violet `#9C4DFF`
- Magenta `#E12DEA`
- Pink `#FF2D78`

The exact runtime SVG gradient may interpolate between these anchors. Monochrome dark and light variants are required for print, legal, low-color, and contrast-constrained use.

## Clear space

Preserve clear space of at least one quarter of the mark's overall height around the standalone mark. In a wordmark lockup, preserve at least the width of the short closing pillar between the mark and `MedScale`.

## Minimum size

- standalone digital mark: 20 px minimum;
- preferred navigation/rail size: 28–36 px;
- app icon: use the dedicated app-icon asset rather than placing the bare mark on an arbitrary tile.

If detail becomes unclear, simplify the surrounding UI before altering the geometry.

## Wordmark

Canonical written name: `MedScale`.

Use Instrument Sans Semibold/Bold for large brand lockups and Medium/Semibold for compact product lockups. Keep `MedScale` one color in the product UI; the gradient belongs to the mark, not to individual wordmark letters.

Lowercase `medscale` remains reserved for technical identifiers/package names.

## App icon

The app icon uses the ScaleFold M on a deep navy-black rounded field with a restrained hairline edge. No extra symbol, glossy badge, fake 3D depth, or medical motif is added.

## Intro treatment

Intro/onboarding may use the full spectrum and enlarged abstract echoes of ScaleFold geometry. The brand intro must not block useful startup longer than actual initialization requires and must respect reduced-motion preferences when animation is introduced.

Reference assets:
- `assets/brand/intro/medscale-intro-dark.svg`
- `assets/brand/intro/medscale-intro-light.svg`

## Forbidden treatments

Do not:
- redraw the mark as a conventional typed `M`;
- add a circle merely because a prior identity used one;
- recolor each pillar as semantic status;
- add medical crosses, ECG lines, brains, shields, sparkles, stethoscopes, mascots, or vendor marks;
- stretch, skew, rotate, outline, bevel, or apply arbitrary glow to the master mark;
- copy a competitor logo construction or product icon.

Any future geometry change requires explicit founder authority plus review at favicon, rail, app-icon, light, dark, and monochrome scales.
