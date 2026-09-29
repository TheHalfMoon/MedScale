# MedScale Design System

**Canonical authority:** Spec 093 — Brand Identity System V2

Spec 093 is the founder-approved visual successor to Spec 073. Spec 073 remains historical evidence for the prior identity and its product-safety work. Spec 093 changes visual identity, tokens, typography hierarchy, icon grammar, and presentation rules only; it does not alter clinical authority, privacy, model/runtime authority, network policy, release claims, or evidence semantics.

## 1. Brand

The approved mark is the **ScaleFold M**: three measured folded geometric forms with repeated diagonal shoulders and a shorter final pillar. Its geometry is the trademark cue; do not add medical or AI clichés to explain it.

The signature MedScale Spectrum runs from Azure through Cobalt/Iris/Violet to Magenta/Pink. Use it for the mark, app icon, intro/onboarding, and bounded focal brand moments.

Brand gradient is identity, not product state. Success, warning, danger, provenance, runtime admission, and clinical/evidence status use independent semantic tokens plus explicit language.

Canonical detail:
- `docs/brand/BRAND_IDENTITY_SYSTEM.md`
- `docs/brand/LOGO_SPEC.md`
- `assets/brand/tokens/medscale.tokens.json`

## 2. Theme

Light and dark are equal first-class themes sharing one component grammar.

Light uses cool near-white canvas, white work surfaces, midnight ink, blue-gray metadata, and restrained indigo interaction. Dark uses deep navy-black canvas, layered graphite/navy surfaces, soft white ink, and cool gray metadata.

The permanent global rail may remain obsidian in both themes so the ScaleFold M has one stable home. Avoid neon-everywhere dark UI, glassmorphism, decorative glow, or white-page-with-random-gradient-card light UI.

## 3. Typography

- **Instrument Sans** — product UI, navigation, headings, metrics, controls, and wordmark treatment.
- **Source Serif 4** — selective evidence/editorial/long-form reading surfaces.
- **Platform monospace** — CLI, commands, hashes, model IDs, evidence IDs, and exact machine-readable values.

Do not add another display font for novelty. Distinctiveness comes from composition, weight, tracking, color discipline, and the ScaleFold M.

## 4. Product shell

Desktop launch target remains 1440×900 with 1100×720 functional minimum unless a separately qualified change is required.

The preferred shell is:
1. narrow obsidian global rail;
2. adaptive named-route sidebar;
3. continuous primary workspace;
4. optional inspector only when the active task benefits.

Use split panes for genuine simultaneous inspection such as agent + artifact, model A + model B, source + evidence, or analysis + visualization. Do not create permanent side panels without task value.

## 5. Workspace grammar

The work is the interface. Prefer hierarchy, whitespace, alignment, rows, tables, and separators before cards.

Priority order:
1. current project/task/context;
2. source/evidence state;
3. intelligence/model/runtime context;
4. reviewable next action;
5. governance detail on demand.

## 6. Components and geometry

Canonical geometry comes from `assets/brand/tokens/medscale.tokens.json`:
- compact radius: 8 px;
- control radius: 10 px;
- panel radius: 14 px;
- large editorial surface radius: 22 px;
- focus ring: 2 px.

A rounded boundary must encode grouping or interaction. Pills are reserved for states, filters, and true capsule controls. Avoid card-within-card dashboards.

Reusable primitives own accessibility, focus, interaction state, spacing, and theme behavior. Pages compose primitives and product patterns rather than inventing a local design system.

## 7. Icon grammar

MedScale uses an original outline family on a 24×24 grid with rounded joins/caps and approximately 1.7–1.8 px optical stroke. Icons should have distinguishable silhouettes and remain legible at 16–20 px.

Do not wrap every icon in a colored tile. Active state comes primarily from the surrounding control state. Marketing/onboarding may use a soft tint or one spectrum accent when it improves grouping.

## 8. Copy and truth

Preferred language is precise, calm, evidence-linked, and explicit about uncertainty. `Evidence first. Action second.` remains an operating line, not a readiness claim.

Never convert feature presence into clinical authority, production readiness, WCAG conformance, model superiority, regulatory status, or release readiness. Planned Research OS V2 surfaces must be clearly treated as planned/reference until implemented and qualified.

## 9. Motion

Motion explains state transitions and preserves spatial understanding. No perpetual decorative animation. Respect reduced-motion preferences. Intro motion may be richer than product motion but must not delay useful startup.

## 10. V0 design handoff

V0 is authorized to implement the visual/reference UI from the Spec 093 package. It is not authorized to invent backend behavior, evidence, runtime state, clinical authority, network permissions, or release claims.

Start with `docs/brand/V0_MASTER_PROMPT.md` and validate against `docs/brand/V0_ACCEPTANCE_CHECKLIST.md`.

The Web/V0 output is a design/reference implementation unless separately promoted into production. Native Desktop authority remains governed by the Rust/Slint repository and the active implementation spec.

## 11. Impeccable review gates

For every major surface apply: shape → critique → audit → distill → typeset → polish → harden → optimize.

Rendered review is required for visual acceptance. A compiler, linter, or token drift check does not establish design quality by itself.

## 12. Reference discipline

JetBrains, Abridge, Cohere, and other high-craft products may inform transferable principles. Their proprietary logos, icons, illustrations, layouts, typography assets, and visual systems are not MedScale assets and must not be copied.

## 13. Authority history

- Pre-Spec-068 multicolor direction: historical.
- Spec 068 Signal/Geist identity: superseded by Spec 073.
- Spec 073 monochrome circular signature M: superseded visually by founder-approved Spec 093.
- Current identity direction: ScaleFold M + MedScale Spectrum + token-governed light/dark system + Instrument Sans / Source Serif 4 + original icon grammar.

MESC remains a separate project and is excluded from MedScale identity, execution, completion, and release criteria.
