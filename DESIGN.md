# MedScale Design System

This document is the canonical visual and interaction direction for the Desktop + CLI product phase. It normalizes the founder-approved visual concepts supplied on 2026-09-15 into implementable rules. Impeccable is used as a review discipline (`shape`, `critique`, `audit`, `polish`, `harden`, `optimize`), not as a runtime dependency.

## 1. Brand

### Mark
Use an original rounded lowercase-arch `M` symbol. It may be inspired by the friendly arch rhythm of the founder's Kraken reference, but it must not copy Kraken geometry. The mark must remain recognizable at 16 px, 24 px, app-icon scale, and monochrome.

### Primary palette
- `MedScale Blurple`: `#5B5CF6` — primary actions, selection, focus, charts.
- `Blurple Hover`: `#4F50E8`.
- `Blurple Soft`: `#EEEDFF`.
- `Pine`: `#2F4F46` — privacy/trust and secondary brand moments.
- `Coral`: `#FF785C` — attention, warmth, selected risk accents.
- `Lavender`: `#D8B4FE` — secondary data series and quiet decorative moments.
- `Mint`: `#47C98B` — positive/healthy/complete state.
- `Amber`: `#F3A94B` — caution/follow-up.
- `Danger`: `#E85B64` — destructive/high-risk only.

### Neutrals
All neutrals are slightly blue/violet-tinted; avoid pure black and flat gray.
- `Ink`: `#17182B`.
- `Ink Subtle`: `#596078`.
- `Ink Quiet`: `#8C93A8`.
- `Border`: `#E7E9F2`.
- `Canvas`: `#F7F8FC`.
- `Surface`: `#FFFFFF`.
- `Surface Raised`: `#FBFBFE`.

### CLI dark palette
- `CLI Canvas`: `#0B1220`.
- `CLI Surface`: `#101A2C`.
- `CLI Raised`: `#152238`.
- `CLI Text`: `#E8ECFA`.
- `CLI Muted`: `#91A0BD`.
- `CLI Border`: `#24334D`.

## 2. Typography
Use the platform's high-quality system UI font stack for application chrome and data. This is a deliberate native-performance decision, not an unconsidered default. Use weight, size, tracking, and hierarchy for personality. CLI uses the platform monospace font.

Desktop scale:
- Display: 32/38, 700.
- Page title: 28/34, 700.
- Section title: 16/22, 650.
- Body: 14/20, 450-500.
- Caption: 12/16, 500.
- Dense data: 12-13/18, tabular numbers where available.

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
Home, Patients, Insights, Workflows, Messages, Tasks, Documents, Audit Trail, Exports, Settings.

Global command palette: `Cmd/Ctrl+K`.
Global search must accept patients/subjects, notes/documents, commands, and approved assistant queries.
Keyboard navigation is first-class; every primary action must have a visible focus treatment.

## 6. Core surfaces
1. **Home / Command Center** — quick actions, operational metrics, active queue, recent activity, reminders, assistant.
2. **Patient Workspace** — overview, timeline, care plan, labs, medications, documents, messages, source drill-down.
3. **Population Insights** — risk distribution, care gaps, cohorts, trends, recommendations.
4. **Workflow Studio** — action library, typed workflow graph, inspector, validation, run history.
5. **Documents / Audit / Exports / Settings** — utility-first surfaces with strong provenance and privacy clarity.
6. **CLI** — command palette quality in the terminal: discoverable commands, structured output, JSON mode, copy/paste-safe syntax, equivalent authority.

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
- Visible focus ring: 2 px blurple with sufficient offset/contrast.
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

## 14. Founder-approved visual artifact status
The founder supplied and approved visual direction on 2026-09-15 for:
- brand/design system board;
- Desktop patient dashboard;
- Home/Command Center;
- Patient Timeline;
- Population Insights;
- Workflow Studio;
- CLI dashboard/showcase/configuration views.

These are visual authority. Implementation may adapt layout to real data/authority constraints but must preserve the approved visual intent.
