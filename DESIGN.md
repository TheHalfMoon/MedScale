# MedScale Design System

This document is the canonical visual and interaction direction for the Desktop + CLI product. The founder explicitly rejected the earlier multicolor direction on 2026-09-15 as derivative and below the product bar. Spec 068 supersedes that visual direction with the MedScale Signal system below. Impeccable is used as a review discipline (`shape`, `critique`, `audit`, `polish`, `harden`, `optimize`), not as a runtime dependency.

## 1. Brand

### Mark
Use the MedScale Signal mark: a compact geometric `M` contained in a rounded signal-blue field. It must remain legible at 16 px, app-icon scale, and monochrome. Do not use the previous double-arch mark or mimic another AI/healthcare brand.

### MedScale Signal palette
MedScale is monochrome-first with one product accent. Color is functional, not decorative.
- `Signal`: `#0A66FF` — selected state, focus, primary navigation/action emphasis.
- `Signal Hover`: `#0056D6`.
- `Signal Strong`: `#0047B3` — accessible signal-colored text on light surfaces.
- `Signal Soft`: `#EAF2FF`.
- `Signal Glow`: `#B8D5FF` — restrained dark-surface support only.
- `Obsidian`: `#0A0E14` — primary navigation shell.
- `Graphite`: `#111822` and `Graphite Raised`: `#17212E`.
- `Ink`: `#0D1420`; `Ink Subtle`: `#455468`; `Ink Quiet`: `#68778A`.
- `Border`: `#DCE3EB`; `Border Strong`: `#C7D0DC`.
- `Canvas`: `#F3F6F9`; `Surface`: `#FFFFFF`; `Surface Raised`: `#F8FAFC`.
- Semantic-only states: `Success #0B8F69`, `Warning #B87500`, `Danger #C64242`.

Do not build a supporting rainbow palette. Do not use blurple/coral/lavender/pine as brand language. Charts may introduce additional colors only when the data genuinely requires distinct series and every distinction remains accessible without color.

### CLI dark palette
- `CLI Canvas`: `#0B1220`.
- `CLI Surface`: `#101A2C`.
- `CLI Raised`: `#152238`.
- `CLI Text`: `#E8ECFA`.
- `CLI Muted`: `#91A0BD`.
- `CLI Border`: `#24334D`.

## 2. Typography
The branded target family is **Geist**, with **Geist Mono** for machine-readable evidence. Runtime font assets require explicit license/NOTICE admission before packaging; platform UI and monospace families are the fallback when the branded assets are absent.

The canonical type scale, tracking, mono rules, and Abridge-reference boundary are defined in `docs/brand/TYPOGRAPHY_SYSTEM.md`.

## Brand authority documents

- `docs/brand/BRAND_IDENTITY_SYSTEM.md`
- `docs/brand/LOGO_SPEC.md`
- `docs/brand/TYPOGRAPHY_SYSTEM.md`
- `docs/brand/VOICE_AND_COPY.md`
- `docs/brand/PRODUCT_UI_GRAMMAR.md`

These documents are normative for Spec 068+ product work.

## 3. Spacing and geometry
Base spacing: 4 px. Preferred steps: 4, 8, 12, 16, 24, 32, 48.
Radii: 8 controls, 12 compact panels, 16 primary surfaces, 20 hero/assistant surfaces. Avoid making every element a card. Prefer grouping by whitespace and separators.

## 4. Layout
Desktop launch target: 1440x900. Functional minimum: 1024x700.
- Global left rail/sidebar: 216-232 px.
- Top command/search bar: 52-60 px.
- Main content uses a 12-column mental grid.
- Right contextual rail appears only when it adds immediate value.
- Dense clinical views favor rows and sections over nested cards.

## 5. Navigation
Canonical Desktop navigation:
Home, Patients, Insights, Models, Evidence, Workflows, Messages, Tasks, Documents, Audit Trail, Exports, Integrations, Settings.

`Models` is the product-visible runtime/provenance surface. `Evidence` is the product-visible comparative proof ledger, including explicit OpenMed gaps. Neither may be hidden in Settings or developer documentation.

Global command palette: `Cmd/Ctrl+K`.
Global search must accept patients/subjects, notes/documents, commands, and approved assistant queries.
Keyboard navigation is first-class; every primary action must have a visible focus treatment.

## 6. Core surfaces
1. **Home / Command Center** — authority posture, model/runtime posture, evidence state, safe next actions; no fake business analytics.
2. **Patient Workspace** — overview, timeline, care plan, labs, medications, documents, messages, source drill-down.
3. **Population Insights** — evidence-aware cohorts/distributions without invented clinical ranking.
4. **Model Center** — installed/admitted models, source, task, runtime, hardware, provenance, trust, benchmark, current/last-green state.
5. **Evidence Center** — MedScale × OpenMed capability ledger using exact evidence states; unsupported superiority is forbidden.
6. **Workflow Studio** — action library, typed workflow graph, inspector, validation, run history.
7. **Documents / Audit / Exports / Integrations / Settings** — utility-first surfaces with strong provenance and privacy clarity.
8. **CLI** — command palette quality in the terminal: discoverable commands, structured output, JSON mode, copy/paste-safe syntax, equivalent authority.

## 7. Components
Build a small reusable system: NavItem, TopBar, SearchField, CommandPalette, SectionHeader, Metric, StatusPill, DataTable, TimelineRow, EmptyState, ErrorState, AssistantPanel, PrivacyStatus, ActionButton, SplitPane, Inspector, Toast, Modal, Tooltip, Chart primitives.

Do not create wrapper components that exist only to add another border/background. Components should encode behavior, semantics, tokens, or reuse.

## 8. State language
- Loading: quiet skeleton/progress, never ambiguous spinner-only for long work.
- Empty: explain why empty and the next safe action.
- Error: state what failed, whether anything changed, and recovery action.
- Conflict: explicitly say no authority-changing action was silently applied.
- Success: brief confirmation; do not flood the interface with green.
- Offline/local: explicit, reassuring, not alarming.

## 9. AI interaction
AI is contextual, not the entire product shell.
- Assistant suggestions must be visibly suggestions.
- Source/evidence links remain available.
- Care-plan generation and similar consequential actions require explicit review/commit boundaries.
- Never visually imply an AI response is clinical authority.

## 10. Accessibility
Every interactive element needs OS-exposed role, label, focus, and action semantics. Slint accessibility remains enabled.
- Keyboard-only operation for all primary paths.
- Minimum 44x32 effective interactive target in dense Desktop UI; 44x44 for primary touch-like controls.
- Visible focus ring: 2 px Signal blue with sufficient offset/contrast.
- Never encode risk/status by color alone.
- Respect OS reduce-motion where feasible.
- Final WCAG/assistive-technology qualification is evidence work, not a design claim.

## 11. Motion
Use motion sparingly to explain state changes.
- 120-180 ms for hover/focus/selection.
- 180-240 ms for panels and route transitions.
- Ease-out / smooth cubic motion; no bounce or elastic easing.
- Avoid decorative perpetual animation.

## 12. Impeccable review gates
For each major surface:
1. `shape`: define user goal, hierarchy, default state, failure state, keyboard path.
2. `critique`: review clarity, hierarchy, density, emotional tone, and generic-AI-design tells.
3. `audit`: accessibility, responsive/min-size behavior, performance, text overflow.
4. `polish`: align tokens, spacing, component consistency, copy.
5. `harden`: error/loading/empty/conflict, long text, localization pressure, slow operations.
6. `optimize`: startup, input latency, rendering, memory, unnecessary allocations/assets.

Explicit anti-patterns: excessive gradients; purple everywhere; gray text on saturated color; nested cards; decorative icon tiles above every heading; unnecessary glassmorphism; giant marketing headings inside work surfaces; fake analytics; hidden destructive actions; animation without purpose.

## 13. Performance budget direction
The final shell must be measured, not assumed. Product targets remain those in release methodology, including final UI interaction response <=100 ms on qualified hardware. UI implementation should avoid a browser engine and large asset/runtime dependencies.

## 14. Founder product-bar status
The earlier 2026-09-15 multicolor visual direction is **SUPERSEDED_BY_SPEC_068** after explicit founder rejection. It is historical context only and must not be used as visual authority.

Current product authority:
- MedScale Signal identity and mark;
- graphite/obsidian navigation shell with one signal-blue product accent;
- visible Model Center and Evidence Center;
- no fake operational/business analytics;
- no chatbot-first product shell;
- no visual imitation of Cohere, OpenMed, or another AI/healthcare product;
- comparative superiority only when evidence is bound and inspectable.

Future visual changes must preserve product truth and may raise the quality bar further; they must not restore the superseded palette merely for compatibility.
