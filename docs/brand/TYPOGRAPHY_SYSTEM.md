# MedScale Typography System

**Status:** CANONICAL_SPEC_093

## Families

**Product/UI:** Instrument Sans
**Long-form evidence/editorial:** Source Serif 4
**Machine/terminal:** platform monospace

The admitted font binaries and licenses remain under `assets/brand/fonts/`. Native Slint imports the product and serif assets directly so identity does not depend on a developer-machine font installation.

## Why this combination

Instrument Sans is compact enough for dense research tooling but has enough human shape to avoid sterile enterprise UI. Source Serif 4 introduces editorial rhythm where evidence, narrative, or long-form reading benefits from it. Monospace is reserved for exact machine values.

The system deliberately does not add another display font merely to look branded. Distinctiveness comes from weight, scale, tracking, composition, and the ScaleFold M.

## Hierarchy

- **Display:** 36 px reference; bold; tight tracking; marketing/reference/onboarding only.
- **Page:** 28 px; bold/semibold; concise product page titles.
- **Section:** 16 px; semibold.
- **Body:** 13 px; regular/medium.
- **Dense:** 12 px; tables, compact controls, metadata with adequate line height.
- **Caption:** 11 px; supporting detail.
- **Micro:** 10 px; sparse uppercase system labels only.

The canonical numeric scale is stored in `assets/brand/tokens/medscale.tokens.json`.

## Headline behavior

Large MedScale headings should feel direct rather than promotional:
- prefer two to eight words;
- use strong line breaks only when they improve meaning;
- use negative tracking sparingly at large sizes;
- do not set entire product pages in display-scale type;
- do not center dense research/workflow headings by default.

## Long-form behavior

Source Serif 4 may be used for evidence summaries, editorial explainers, research notes, and narrative context. It is not used for navigation, buttons, tables, or state labels.

## Machine text

Use platform monospace for hashes, commands, code, model identifiers, version pins, evidence IDs, and exact values. Never use monospace merely to signal that something is "technical."

## Licensing

Instrument Sans is carried with its OFL text. Source Serif 4 is carried with its repository license file. This design document does not replace NOTICE/SBOM/license qualification.

## ScaleFold Display treatment

ScaleFold Display is not a fourth font family. It is a rare compositional treatment built from Instrument Sans outlines and the geometry rules in `SCALEFOLD_LANGUAGE.md`.

Allowed uses include launch phrases, event titles, report covers, and limited brand hero text. Measured diagonal cuts, steps, gaps, or asymmetric endings may be introduced only when legibility remains immediate.

Never apply ScaleFold Display treatment to navigation, forms, tables, clinical/evidence content, normal headings, or the canonical MedScale wordmark.
