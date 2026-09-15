# MedScale Brand Identity System

**Status:** CANONICAL_DRAFT_SPEC_068
**Owner:** MedScale product identity
**Product category:** Clinical Intelligence OS

## 1. Brand thesis

MedScale turns clinical data, model output, and workflow state into inspectable evidence before anything becomes authority.

**Brand promise:** Evidence-native clinical intelligence.
**Operating line:** Evidence first. Action second.
**Internal design test:** The work is the interface.

MedScale is not an AI scribe, generic healthcare dashboard, chatbot shell, or EHR skin. It is an intelligence and authority layer for clinical work.

## 2. Character

MedScale must feel:
- precise, calm, and technically serious;
- premium without luxury theatrics;
- clinical without hospital-corporate visual clichés;
- intelligent without AI spectacle;
- inspectable rather than magical;
- fast, dense when useful, and quiet when not.
## 3. Positioning boundaries

MedScale may claim a capability only when the product can point to its evidence.

Do not use:
- revolutionary, magical, effortless, or generic AI-superiority language;
- unsupported "better than" competitor claims;
- fake activity, fake impact metrics, or fabricated patient outcomes;
- healthcare clichés such as heartbeat marks, crosses, shields, or brain/sparkle AI icons.

Competitive references are allowed only in the Evidence Center and research/evidence documents with `PROVEN`, `PARTIAL`, `NOT_YET_PROVEN`, or equivalent evidence states.

## 4. Brand architecture

- **MedScale** — company/product master brand.
- **Core Host** — canonical local authority path; technical product term, not a sub-brand.
- **Model Fabric** — model/runtime/provenance layer; technical product term.
- **Model Center** — user-facing model inspection surface.
- **Evidence Center** — user-facing proof and comparative evidence surface.
- **Evidence Ledger** — inspectable evidence record, not a separate brand.

Do not create colored sub-brand logos for these concepts.
## 5. Visual system

MedScale is monochrome-first with one product signal color.

### Product colors
- `Signal Blue` `#0A66FF` — primary focus, selected state, and intentional action emphasis.
- `Signal Hover` `#0056D6`.
- `Signal Strong` `#0047B3` — accessible signal text on light surfaces.
- `Signal Soft` `#EAF2FF`.
- `Obsidian` `#0A0E14` — navigation and high-authority dark surfaces.
- `Graphite` `#111822` and `Graphite Raised` `#17212E`.
- `Ice` `#F3F6F9` — working canvas.
- `Paper` `#FFFFFF` — primary reading/work surface.
- `Ink` `#0D1420`, `Ink Subtle` `#455468`, `Ink Quiet` `#68778A`.

Semantic green, amber, and red are status colors only. They are not brand accents.

No purple-to-blue gradients, supporting rainbow palette, glassmorphism, neon glow, decorative blobs, or colored icon tiles.

## 6. Composition grammar

The default surface is a workspace, not a dashboard.

Priority order:
1. current clinical/work context;
2. source and evidence state;
3. intelligence/model state;
4. reviewable next action;
5. governance detail on demand.
Avoid page structures dominated by KPI grids, nested cards, or generic SaaS hero areas. Prefer one dominant work surface with secondary context around it.

### Navigation
Navigation is grouped by user intent:
- Workspace
- Intelligence
- Operations
- Governance

Models and Evidence are first-class Intelligence routes. They may not be buried in Settings.

## 7. Typography direction

The branded target family is **Geist Sans**, with **Geist Mono** for hashes, model IDs, evidence IDs, digests, commands, and other machine-readable strings.

Abridge is a craft reference, not a typeface donor. Its public marketing CSS currently uses Avantt; MedScale intentionally does not copy or redistribute that family.

Until a tracked font-asset admission step is completed, native application builds may use platform UI fallback. The type scale, weight hierarchy, line height, and tracking remain mandatory regardless of fallback family.

Typography details are canonical in `docs/brand/TYPOGRAPHY_SYSTEM.md`.

## 8. Product signature

MedScale should be recognizable without its logo through:
- dark structured navigation against an ice/paper workspace;
- Signal Blue used sparingly for focus and intent;
- source/evidence metadata placed close to AI output;
- compact uppercase micro-labels only for system layers and state;
- restrained geometry and thin structural dividers;
- explicit state language such as `PROVEN`, `REVIEW REQUIRED`, `RUNTIME GAP`, and `UNKNOWN`.
## 9. Reference discipline

Abridge contributes workflow lessons: keep the clinician's work central, keep source material close to generated output, and organize around the care journey rather than software modules.

OpenMed contributes capability lessons: models, providers, runtime choices, and technical state should be inspectable rather than hidden.

Impeccable contributes review discipline: shape, critique, distill, typeset, polish, harden, optimize, and deterministic anti-pattern detection.

None of these sources is visual authority for MedScale. MedScale must remain recognizable as itself.

## 10. Quality gates

Every primary surface must pass:
- **Shape:** one primary user goal and clear hierarchy.
- **Critique:** no generic AI/SaaS tells; no competitor mimicry.
- **Distill:** remove elements that do not change understanding or action.
- **Typeset:** deliberate hierarchy, measure, rhythm, and scanability.
- **Polish:** consistent spacing, alignment, interaction, and copy.
- **Harden:** long data, empty/error/conflict states, localization pressure, keyboard access.
- **Optimize:** responsive interaction, bounded memory, and no unnecessary runtime weight.

A green compiler or test run does not establish visual acceptance. Rendered review is mandatory for primary surfaces.
