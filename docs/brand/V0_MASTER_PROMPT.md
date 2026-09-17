# MedScale V0 UI Master Prompt

**Status:** FOUNDER_APPROVED_HANDOFF — Spec 093
**Purpose:** Generate the visual/reference UI only. Do not create or imply backend authority.

Build a complete MedScale UI reference system from the repository's canonical brand package. The result must feel unmistakably MedScale: premium, research-grade, human, technically serious, light-first, and equally excellent in dark mode.

Do not copy JetBrains, Abridge, Cohere, or any other brand. Their work is reference research only. Use MedScale's own ScaleFold M, token system, typography, icon grammar, evidence-first product language, and information architecture.

## Read first

Use these repository files as authority:
- `docs/brand/BRAND_IDENTITY_SYSTEM.md`
- `docs/brand/LOGO_SPEC.md`
- `docs/brand/DESIGN_TOKENS.md`
- `docs/brand/TYPOGRAPHY_SYSTEM.md`
- `docs/brand/ICON_SYSTEM.md`
- `docs/brand/COMPONENT_SYSTEM.md`
- `docs/brand/PAGE_BLUEPRINTS.md`
- `docs/brand/MOTION_AND_INTERACTION.md`
- `docs/brand/V0_ACCEPTANCE_CHECKLIST.md`
- `docs/brand/V0_INPUT_MANIFEST.md`
- `assets/brand/tokens/medscale.tokens.json`
- `assets/brand/tokens/medscale.css`
- `crates/medscale-desktop/ui/assets/medscale-mark.svg`

## Visual north star

The interface has two intensity modes that belong to one system:
- **Brand mode:** ScaleFold M, MedScale Spectrum, confident editorial typography, bounded gradient fields, and concise storytelling.
- **Work mode:** quiet neutral surfaces, precise hierarchy, restrained solid accents, readable density, and evidence/provenance close to consequential output.

The transition matters. Brand moments can be vivid; the workspace must give attention back to the work.

Light mode is a first-class design, not a washed-out dark mode. Dark mode is layered navy/graphite, not neon black. The logo gradient stays vivid in both.

Use generous whitespace where comprehension benefits, but do not make research tooling artificially sparse. Dense screens should feel ordered rather than empty.

## Typography

Use Instrument Sans for product UI, navigation, headings, metrics, buttons, tables, and wordmark treatment. Use Source Serif 4 selectively for editorial/evidence narrative. Use monospace only for exact technical identifiers.

Headlines should be compact, direct, and tightly composed. Avoid generic giant SaaS hero text inside the product. Marketing/reference pages may use the 36 px display scale and stronger line breaks.

## Color rules

Use the exact canonical tokens. Never invent a replacement palette.
- Spectrum = identity and bounded focal moments.
- Indigo accent = interaction/focus.
- Success/warning/danger = state.
- Gradient must never encode authority, model quality, risk, or clinical meaning.

## Product shell

Design one reusable shell with:
- a narrow obsidian global rail;
- an adaptive named-route sidebar;
- a top context bar for search, command access, project context, theme, and account/runtime posture;
- a continuous workspace canvas;
- optional right inspector only when the active task benefits from it.

The shell must support both current MedScale surfaces and future Research OS V2 surfaces without becoming a crowded mega-navigation.

Use progressive disclosure. Keep primary work visible; reveal advanced governance, provenance, runtime, and configuration details on demand.

## Component behavior

Build reusable primitives first, then patterns, then pages. Every interactive component needs hover, focus, active, disabled, loading, empty, error, and keyboard behavior where applicable.

Do not put every icon inside a colored tile. Do not make every metadata item a pill. Do not nest cards merely to create visual layers. Use whitespace, separators, typography, and alignment before adding containers.

Rounded geometry must encode grouping or interaction. Default controls use 10 px radius, compact controls 8 px, panels 14 px, and large editorial surfaces 22 px.

## Iconography

Use an original coherent 24 px outline family with rounded joins/caps and approximately 1.7–1.8 px optical stroke. Conventional metaphors are fine; competitor-specific silhouettes are not.

## Required pages and states

Create high-fidelity light and dark designs for the following reference surfaces:
1. Brand/landing intro.
2. Home / Research Command Center.
3. Projects and project workspace.
4. Data / datasets and source fabric.
5. MedAgent workspace with split-pane outcome rendering.
6. Model Center and multi-model Compare.
7. Analytics Gate.
8. Knowledge / research canvas.
9. AudioFlow.
10. Governed Browse.
11. Team / collaboration.
12. Evidence / provenance inspection.
13. Settings / privacy / integrations.

For each surface, include at least one honest empty, loading, error, unresolved, or unavailable state. Do not imply that planned Research OS V2 capabilities are already implemented.

Use synthetic/demo content only. Clearly label future or non-authoritative states where needed.

## Research workspace principles

The work is the interface. A user should understand the current project, evidence state, model/runtime state, and next safe action without scanning a dashboard full of decorative cards.

Use split panes when output genuinely benefits from simultaneous inspection: agent conversation + artifact, model A + model B, source + evidence, analysis + visualization, or task + inspector.

Preserve a strong center canvas. Side panels are supporting tools, not permanent visual noise.

## Trust and evidence behavior

Keep provenance, source identity, uncertainty, runtime admission, and review state visually close to consequential output. Prefer precise state language over vague confidence decoration.

Do not present AI output as clinical authority. Do not invent patient outcomes, readiness, benchmark superiority, security certification, or regulatory status.

Useful vocabulary includes `PROVEN`, `SUPPORTED`, `REVIEW REQUIRED`, `UNKNOWN`, `NOT ADMITTED`, `UNMEASURED`, and `SOURCE CONFLICT` when accurate.

## Motion

Motion explains state changes only. Use short restrained transitions, no perpetual decorative animation, no particle fields, and no glowing cursor theater inside the product. Intro motion may be richer but must remain skippable and reduced-motion safe.

## Responsive behavior

Primary target is desktop at 1440×900. Also demonstrate 1280 desktop and narrow/tablet behavior. Collapse the named sidebar before collapsing essential work. Preserve keyboard navigation and visible focus.

## Output requirements

Deliver:
- a reusable token-backed component system;
- light and dark theme parity;
- the required page set;
- component variants and state examples;
- responsive behavior;
- no backend wiring or fabricated runtime behavior;
- no proprietary competitor assets;
- no alternate logo exploration.

Treat the ScaleFold M and canonical tokens as fixed inputs. Improve composition, interaction, hierarchy, and polish around them rather than redesigning the identity.

## V0 reference implementation conventions

Use the normal V0 React/Next.js workflow for the reference implementation. Keep styling token-driven through CSS variables generated from the canonical MedScale tokens. Tailwind utilities are acceptable as implementation mechanics, but raw palette values and arbitrary radii must not become a second design system.

Organize reusable primitives separately from MedScale product patterns and page compositions. Avoid default shadcn visual appearance: primitives may be used structurally, but all geometry, typography, color, spacing, focus, and state treatment must resolve to MedScale rules.

Use static synthetic fixtures for demonstration. Do not create fake production APIs, authentication, database wiring, telemetry, or network behavior.

Prefer local SVG icon components built to the MedScale icon grammar for primary navigation/product concepts. Generic utility symbols may begin from conventional geometry, but the finished reference must feel optically coherent rather than like a mixed icon-pack demo.

Keep assets in a clear `public/brand` or equivalent reference directory and preserve the canonical SVG source unchanged.
