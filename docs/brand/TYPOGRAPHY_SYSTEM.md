# MedScale Typography System

**Status:** CANONICAL_DRAFT_SPEC_068

## 1. Reference finding

Abridge's current public marketing CSS uses `Avantt` across multiple weights plus a separate internal `Abridge Font`. This is a craft reference only. MedScale does not copy, vendor, or redistribute those font assets.

## 2. MedScale families

**Primary branded family:** Geist Sans
**Technical/monospace family:** Geist Mono

Geist is licensed under SIL Open Font License 1.1. Font binaries are admitted separately with license/NOTICE evidence before they become packaged runtime assets.

Native builds without the admitted branded asset use the platform UI family as fallback. Typography metrics and hierarchy remain canonical even when fallback rendering is active.

## 3. Typography principles

- Prefer medium and semibold hierarchy over excessive bold.
- Use display weight for one dominant idea, not every section title.
- Keep body copy neutral and highly legible.
- Use mono only for machine-readable material.
- Use uppercase micro-labels sparingly for system layers, evidence states, and navigation grouping.
- Never use typography merely as decoration.
## 4. Product type scale

Desktop reference scale at 1×:
- `Display / 36`: 36 px, 600, line-height 42, tracking -0.035em.
- `Page / 30`: 30 px, 600, line-height 36, tracking -0.030em.
- `Section / 18`: 18 px, 600, line-height 24, tracking -0.015em.
- `Subsection / 15`: 15 px, 600, line-height 21, tracking -0.010em.
- `Body / 14`: 14 px, 430-500, line-height 20.
- `Dense / 12`: 12 px, 450-520, line-height 17.
- `Caption / 11`: 11 px, 500, line-height 15.
- `Micro / 9`: 9 px, 600, line-height 12, tracking 0.10em when uppercase.

Do not reduce clinical narrative body text below 13 px at normal desktop scaling.

## 5. Evidence and machine text

Use Geist Mono or the platform monospace fallback for:
- SHA-256 digests;
- model/repository identifiers;
- source IDs and evidence IDs;
- command examples and CLI syntax;
- exact runtime/version strings.

Do not render patient names, clinical prose, or long explanations in monospace.

## 6. Numeric behavior

Use tabular figures for tables, measurements, durations, benchmark output, and aligned counts when the rendering stack supports them. Do not use oversized KPI numerals as visual decoration.
