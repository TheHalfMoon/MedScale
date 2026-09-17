# MedScale Product UI Grammar

**Status:** CANONICAL_SPEC_073

## The work is the interface

Primary screens begin with the clinician/operator task and its evidence, not product metrics or decorative modules.

## Dual-dock shell

The native shell has two navigation layers:
1. a narrow black icon rail for stable areas/global commands;
2. an adaptive named-route dock for precise navigation and local runtime posture.

The custom MedScale icon family is monochrome and supportive. Text/accessibility labels remain semantic authority.

## Surface hierarchy

Default priority:
1. current context and work;
2. source/evidence state;
3. intelligence/model context;
4. reviewable action;
5. governance detail.

If every layer has equal visual weight, the surface is not shaped yet.

## Containers

Use a card only when a boundary matters. Prefer whitespace, alignment, rows, and separators. Avoid nested cards, rounded icon tiles, giant work-surface marketing headings, KPI grids, and status-chip soup.

## Care-flow grammar

Use `Prepare → Understand → Act` when a workflow genuinely spans context, intelligence, and consequence. The sequence is conceptual; it does not require a three-column dashboard layout.

## AI/model grammar

AI/model output stays inspectable and non-authoritative. Keep source/runtime/evidence context near consequential output. Model source does not equal runtime authority, and model presence does not equal production admission.

## Evidence grammar

Keep provenance readable in place. Prefer source snippets, timestamps, identifiers, and explicit evidence state over generic source footers. Comparative claims require bound evidence and explicit limitations.

## Theme and color

Light is the primary product target; dark follows the OS. Mist Blue is interaction/focus, Sage positive semantics, amber review/warning, red danger/failure. Never encode status by color alone.
