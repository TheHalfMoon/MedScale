# MedScale Typography System

**Status:** CANONICAL_SPEC_073

## Families

**Product/UI:** Instrument Sans
**Long-form clinical/writing:** Source Serif 4
**Machine/terminal:** platform monospace

The admitted font binaries and licenses live in `assets/brand/fonts/`. Native Slint imports the branded UI and serif assets directly so rendered identity does not depend on a developer-machine font installation.

Geist/Geist Mono are historical Spec 068 choices and are not active MedScale identity fonts. Helvetica is not a MedScale identity font.

## Product principles

- Instrument Sans is the default for navigation, controls, tables, headings, and product metadata.
- Source Serif 4 is selective: narrative clinical/writing passages where reading rhythm benefits from serif text. It is not used for navigation or buttons.
- Platform monospace is used for CLI, commands, digests, hashes, model/repository IDs, evidence IDs, and other machine-readable identifiers.
- Prefer medium/semibold hierarchy to excessive bold.
- Keep clinical narrative comfortably readable; do not shrink it to create artificial density.
- Uppercase micro-labels are reserved for system layers, section groups, and evidence/state vocabulary.

## Desktop reference scale

- Page: 28–30 px, semibold, restrained negative tracking.
- Section: 16–18 px, semibold.
- Subsection: 13–15 px, medium/semibold.
- Body: 13–14 px.
- Dense row: 11–12 px.
- Caption: 10–11 px.
- Micro: 8–9 px, semibold, tracked when uppercase.

## Machine text

Use platform monospace for exact values only. Do not render patient names, narrative explanations, or normal button labels in monospace.

## Licensing

Instrument Sans is carried with its OFL text. Source Serif 4 is carried with its repository license file. License/NOTICE accounting must remain part of normal repository qualification; this document does not replace legal inventory evidence.
