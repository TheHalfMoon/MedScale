# MedScale Logo Specification

**Status:** CANONICAL_SPEC_073 — FOUNDER_APPROVED_SIGNATURE_MARK

## Approved master mark

The MedScale master mark is fixed for Spec 073:
- black circular field;
- soft-white/soft-gray continuous rounded `M`;
- a custom asymmetric inner transition ending in the short **MedScale Shelf** at the center;
- simple geometry with rounded stroke caps and joins;
- monochrome only.

The MedScale Shelf is the recognition cue: the left inner stroke settles into a short measured baseline before the steeper right return. It must remain visible at rail size without becoming a separate symbol, cutout, medical motif, or decorative flourish.

`crates/medscale-desktop/ui/assets/medscale-mark.svg` is the native runtime source of truth. The same `medscale-signature-m` geometry is used by the app icon and reference surfaces. Routine implementation work must not explore alternate logo directions.

## Meaning

The continuous `M` suggests continuity and linked evidence without using a healthcare cliché. The MedScale Shelf adds a measured, ownable rhythm to the center of the letter while preserving immediate `M` recognition. The circle creates a stable compact field that remains legible in rail, window, print, and monochrome contexts.

## Color

Master mark:
- field: near-black (`#0A0A0A` in the runtime SVG);
- mark: soft white (`#F4F4F1` in the runtime SVG).

No state, vendor, model, or clinical semantic color is applied to the logo.

## Sizing and clear space

Preserve clear space of at least 18% of the circle diameter around the mark when it appears in a larger lockup. At small digital sizes, simplify surrounding UI before altering the approved geometry.

## Wordmark

Canonical written name: `MedScale`.

Use Instrument Sans Medium/Semibold for product lockups. Do not split `Med` and `Scale` by color. Lowercase `medscale` is reserved for technical identifiers/package names.

## App icon

`crates/medscale-desktop/ui/assets/medscale-app-icon.svg` uses the same approved monochrome circular treatment. It must not introduce gradients, colored center points, shadows, glow, or additional healthcare/AI symbols.

## Forbidden treatments

Never add purple, gradients, a medical cross, ECG/heartbeat trace, shield, stethoscope, brain, sparkle, mascot, literal octopus imagery, vendor logo combination, 3D treatment, or decorative signal waves.

Any future geometry change requires explicit founder authority plus rendered review at small rail size and app-icon scale. The current signature geometry was explicitly founder-authorized during Spec 073 and supersedes the earlier generic rounded-M geometry.
