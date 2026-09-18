# MedScale Product UI Grammar

**Status:** CANONICAL_SPEC_093

## The work is the interface

Primary screens begin with the user's active research/clinical task, its project context, evidence, and next safe action. Do not lead with vanity metrics, decorative feature inventory, or AI spectacle.

## Brand mode vs Work mode

Brand mode may use the ScaleFold M, full MedScale Spectrum, confident editorial type, and bounded graphic fields. Work mode uses neutral surfaces, solid interaction/state colors, precise hierarchy, and evidence-native density.

A screen should not remain in Brand mode while the user is doing dense work.

## Product shell

Preferred desktop shell:
1. narrow obsidian global rail;
2. adaptive named-route sidebar;
3. flexible workspace canvas;
4. optional inspector for context/provenance/settings.

Use split panes only when simultaneous inspection improves the task. Preserve the central workspace as the dominant region.

## Surface hierarchy

Default priority:
1. current project/task/context;
2. source/evidence state;
3. intelligence/model/runtime context;
4. reviewable action;
5. governance/configuration detail on demand.

## Containers

Use a card only when a boundary matters. Prefer whitespace, alignment, rows, tables, separators, and pane structure. Avoid nested rounded cards, KPI walls, status-chip soup, and decorative icon tiles.

## AI/model grammar

AI output stays inspectable and non-authoritative. Model source, runtime admission, task, device, digest, rights, and benchmark evidence are distinct concepts and should remain distinguishable in UI.

Parallel model lanes must not imply consensus or a winner without explicit user-defined evidence.

## Evidence grammar

Keep provenance readable in place. Prefer source snippets, timestamps, identifiers, freshness, supporting/contradicting state, and explicit limitations over generic source footers.

## Theme and color

Use the canonical semantic tokens. Gradient is identity; interaction accent is control state; success/warning/danger are semantic state. Never encode status by color alone.

## Interaction grammar

Primary actions use clear verb + object copy. Secondary/quiet actions should recede without becoming invisible. Destructive or externally consequential actions require explicit context and confirmation appropriate to risk.

## Planned surfaces

Design references for future Research OS V2 features may exist before runtime implementation. Such surfaces must be labeled as reference/planned/demo and must not create implementation or release claims.

## Brand-to-work transition

MedScale intentionally moves from Brand mode to Work mode. Intro, onboarding, report covers, and selected empty states may use ScaleFold supergraphics and the full Spectrum. Dense research/clinical work returns to neutral surfaces and semantic solid colors.

A screen should not stay in Brand mode merely to look distinctive. The signature is strongest when the product knows when to become quiet.
