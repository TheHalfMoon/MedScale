# MedScale Design System

**Canonical authority:** Spec 073 — Final UI Polish

Spec 073 supersedes the Spec 068 active visual identity while preserving Spec 068 as historical product-differentiation evidence. The approved identity is monochrome, light-first, evidence-native, and deliberately quiet. Impeccable remains a review discipline (`shape`, `critique`, `audit`, `distill`, `typeset`, `polish`, `harden`, `optimize`), not a runtime dependency.

## 1. Brand

### Approved mark
The approved MedScale mark is a black circular field containing a soft-white rounded continuous `M`. Its signature recognition cue is the **MedScale Shelf**, a short measured baseline in the inner center of the letter before the steeper right return. The mark is monochrome. It is not a place for product-state color.

Forbidden logo treatments include purple, gradients, medical crosses, ECG/heartbeat traces, shields, brains, sparkles, mascots, and literal octopus imagery. The founder-authorized Spec 073 signature geometry is fixed; it must not be redesigned inside routine product implementation work.

### Product color
Brand identity is black, white, and gray. Product color communicates state or interaction:
- **Mist Blue** — primary interaction, selection, and focus.
- **Sage** — positive/qualified semantics.
- **Amber** — warning/review semantics.
- **Red** — danger/failure semantics.

Color is never used as decoration or as the only carrier of meaning.

## 2. Theme

Light mode is the primary/default design target. The application follows the operating-system color scheme through Slint where supported.

Light surfaces use warm off-white canvas, quiet paper work surfaces, neutral ink, and subtle structural borders. Dark mode uses layered charcoal surfaces rather than pure black everywhere. The permanent icon rail and deliberate high-authority panels may use the black/charcoal brand range in either mode.

No gradients, neon glow, glassmorphism, purple-dashboard language, or decorative colored icon tiles.

## 3. Typography

- **Instrument Sans** — native product UI and wordmark treatment.
- **Source Serif 4** — selected long-form clinical/writing surfaces where reading rhythm benefits from serif text.
- **Platform monospace** — CLI, commands, hashes, model IDs, evidence IDs, and other machine-readable identifiers.

The native Desktop package imports the admitted Instrument Sans and Source Serif 4 assets from `assets/brand/fonts/`. Geist and Helvetica are not MedScale identity fonts.

Detailed rules are in `docs/brand/TYPOGRAPHY_SYSTEM.md`.

## 4. Layout and navigation

Desktop launch target: 1440×900. Functional minimum remains 1100×720 unless a separately qualified change is required.

The shell uses a dual-dock composition:
1. a narrow black icon rail for stable product areas and global commands;
2. a refined adaptive sidebar for named routes and local runtime posture;
3. a continuous work canvas for the active clinical/operator task.

Canonical routes remain: Home, Patients, Documents, Insights, Models, Evidence, Workflows, Tasks, Messages, Audit Trail, Exports, Integrations, Settings, About.

Models and Evidence remain first-class Intelligence routes. Keyboard navigation, explicit focus, and `Cmd/Ctrl+K` command access remain first-class.

## 5. Workspace grammar

The work is the interface. Prefer whitespace, alignment, sections, rows, and separators over card grids.

Priority order:
1. current context/work;
2. source and evidence state;
3. intelligence/model context;
4. reviewable next action;
5. governance detail on demand.

Home is a clinical workspace, not a KPI dashboard. Patient and evidence surfaces are provenance-forward. AI output remains visibly inspectable and non-authoritative. No fake clinical metrics, fake patient authority, or unsupported AI claims.

## 6. Components and geometry

Controls use restrained rounding; primary panels use approximately 10 px, compact controls approximately 7–8 px. A rounded boundary must encode an actual interaction or semantic grouping, not merely decoration.

Shared primitives should encode behavior, accessibility, semantics, or reuse. Avoid wrapper components that only add another background/border.

Status chips are reserved for states that benefit from compact semantic labeling. Do not convert every piece of metadata into a pill.

## 7. Icon grammar

MedScale uses a small original monochrome navigation icon family under `crates/medscale-desktop/ui/assets/icons/`.

Icons use consistent line weight, rounded joins/caps, a 24×24 view box, and no colored tile container. They support navigation recognition; text labels remain the semantic authority in the secondary dock and accessibility tree.

## 8. Copy and truth

Preferred language is precise, calm, evidence-linked, and explicit about uncertainty. `Evidence first. Action second.` remains an operating line, not a readiness claim.

Never convert feature presence into release readiness, WCAG conformance, clinical authority, production-model authority, or comparative superiority. MESC is a separate project and is not a MedScale release/completion condition.

## 9. CLI

The CLI shares the MedScale voice without becoming decorative terminal UI. Human help/output may use restrained headings and spacing. Machine JSON contracts, field names, exit codes, stdout/stderr roles, and script-safe behavior remain stable.

ANSI color is not required for identity. Platform monospace remains the terminal typography authority.

## 10. Web reference boundary

`docs/brand/web-reference/` is a non-production design contract/prototype only. It exists to show how the Spec 073 identity maps to a future web surface. It is not a web application, runtime, build target, release artifact, or capability claim.

## 11. Accessibility and motion

Every interactive control requires an exposed role/label/action and visible keyboard focus. Status must not rely on color alone. Final assistive-technology/WCAG qualification remains external evidence work.

Motion, when used, must explain a state transition; no decorative perpetual animation.

## 12. Impeccable review gates

For each primary surface:
1. **Shape** — one primary user goal and honest default/failure state.
2. **Critique** — challenge hierarchy, density, generic-SaaS/AI tells, and competitor mimicry.
3. **Audit** — inspect accessibility semantics, theme contrast, overflow, focus behavior, truth boundaries, and implementation integrity.
4. **Distill** — remove elements that do not change understanding or action.
5. **Typeset** — verify hierarchy, measure, rhythm, and reading comfort.
6. **Polish** — align tokens, spacing, controls, and copy.
7. **Harden** — test long text, empty/error/conflict states, keyboard paths, minimum size, and theme changes.
8. **Optimize** — protect native startup, interaction latency, memory, and asset weight.

A green compiler or empty automated detector result does not establish visual acceptance. Rendered native review is mandatory for Spec 073 closure.

## 13. Authority history

- Pre-Spec-068 multicolor direction: `SUPERSEDED_BY_SPEC_068`.
- Spec 068 Signal/Geist active visual identity: `SUPERSEDED_BY_SPEC_073`.
- Current active identity: Spec 073 monochrome circular signature M with the MedScale Shelf, Instrument Sans / Source Serif 4, OS-adaptive light-first surfaces, Mist Blue/Sage/amber/red semantic product color, and dual-dock native shell.
