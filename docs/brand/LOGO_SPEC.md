# MedScale Logo Specification

**Status:** CANONICAL_DRAFT_SPEC_068

## 1. Core idea

The MedScale mark is a geometric `M` built as a continuous clinical-intelligence rail: two stable outer rails joined through one central decision point.

The mark expresses:
- structure without a shield cliché;
- intelligence without sparkles or a brain icon;
- continuity without a heartbeat line;
- authority without institutional heraldry.

The core mark must work without a container. A rounded app-icon container is a packaging treatment, not the logo itself.

## 2. Primary mark construction

Use a 48 × 48 reference grid.
- Optical left/right rails align around x=8 and x=40.
- The center decision point sits near x=24, y=27.
- Stroke ends and joins are rounded.
- Primary stroke weight is approximately 4.5 units at 48 px.
- The mark must remain legible at 16 px without auxiliary detail.

Do not add a baseline, heartbeat trace, cross, shield, or decorative signal waves to the primary mark.
## 3. Color variants

Approved primary variants:
- Signal Blue mark on Obsidian.
- Obsidian mark on Paper/Ice.
- White mark on Obsidian for constrained monochrome contexts.
- Single-color black or white for legal, print, embossing, or accessibility constraints.

Do not use multi-color mark variants.

## 4. App icon

The application icon may place the core mark inside an Obsidian or Signal Blue rounded-square field. The container must never be treated as part of the master logo geometry.

App-icon rules:
- one background color;
- one foreground mark color;
- no gradients;
- no inner shadow, glow, glass, or 3D treatment;
- no text inside the icon;
- preserve at least 18% clear space around the mark.

## 5. Wordmark

The canonical written name is `MedScale` with capital `M` and `S`.

Preferred wordmark behavior:
- Geist Sans Medium/Semibold once the branded font asset is admitted;
- tight but not compressed tracking;
- no custom ligatures that reduce readability;
- no color split between `Med` and `Scale`.

`medscale` may be used only as a technical identifier, package/repository name, or deliberate campaign treatment, never as the default product wordmark.
## 6. Clear space and sizing

Minimum clear space equals the visual width of one outer rail around all sides of the mark.

Minimum practical sizes:
- core mark: 16 px digital;
- mark + wordmark lockup: 96 px wide digital;
- app icon: platform minimums, using the simplified mark only.

At small sizes, remove optional visual detail before increasing stroke weight or changing geometry.

## 7. Forbidden treatments

Never:
- place the mark in a rainbow or multi-accent system;
- add a medical cross, ECG trace, shield, stethoscope, brain, or sparkle;
- stretch, skew, rotate, outline twice, or add a drop shadow;
- mimic the Abridge arch, OpenMed identity, Cohere mark/color language, or another healthcare/AI logo;
- combine the mark with model/vendor logos to imply ownership or endorsement.

## 8. Governance

`crates/medscale-desktop/ui/assets/medscale-mark.svg` is the runtime source of truth after Spec 068 qualification. Any geometry change requires rendered review at 16 px, 24 px, 48 px, and app-icon scale plus monochrome inspection.

## 9. App-icon asset

`crates/medscale-desktop/ui/assets/medscale-app-icon.svg` is the canonical app-icon treatment. It may use an Obsidian container because operating systems require an icon surface, but that container is not part of the master corporate mark.

The master mark and app icon must never be conflated in brand, web, documentation, or product-header use.
