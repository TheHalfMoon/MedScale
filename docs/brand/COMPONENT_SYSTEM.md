# MedScale Component System

**Status:** FOUNDER_APPROVED_HANDOFF — Spec 093

## Architecture

Build UI in three layers:
1. **Primitives** — behavior, accessibility, focus, sizing, and semantic tokens.
2. **Patterns** — reusable product compositions such as evidence rows, split workspaces, model lanes, and project context.
3. **Pages** — information architecture and task flow only.

Pages must not invent new colors, radii, shadows, or one-off interaction rules.

## Core primitives

Required primitives:
- Button: primary, secondary, quiet, destructive, icon-only.
- IconButton with tooltip and accessible name.
- TextField, SearchField, TextArea, Select, Combobox.
- Checkbox, Radio, Switch, Slider where genuinely needed.
- Tabs and segmented control.
- Tooltip, Popover, Dropdown, ContextMenu.
- Dialog, Drawer, Sheet.
- Toast / inline notification.
- StatusBadge with text + icon/shape, never color alone.
- Progress and skeleton states.

## Structural primitives

- GlobalRail
- RouteSidebar
- ContextBar
- WorkspaceHeader
- SplitPane
- ResizablePane
- InspectorPanel
- Section
- Divider
- ScrollArea
- EmptyState
- ErrorState
- LoadingState

The shell should be capable of 1-pane, 2-pane, and 3-pane work without changing visual grammar.

## Research/product patterns

- ProjectSwitcher and ProjectContext.
- EvidenceRow and EvidenceInspector.
- ProvenanceTrail.
- SourceChip for compact source identity only.
- ModelCard and ModelRuntimeState.
- ModelCompareLane.
- AgentThread and AgentOutcomePane.
- ArtifactRow and ArtifactPreview.
- DatasetRow and DataSourceState.
- MetricSummary and ChartFrame.
- ActivityTimeline.
- ReviewQueue.
- CollaborationPresence and CommentThread.

## Component state contract

Every interactive component must define:
- default;
- hover;
- keyboard focus;
- pressed/active;
- disabled;
- loading where relevant;
- validation/error where relevant.

Every data-bearing pattern must define:
- populated;
- empty;
- loading;
- unavailable;
- stale or unresolved when the domain supports it.

## Density

Use three density bands:
- **Comfortable:** onboarding, brand/reference, empty states.
- **Standard:** default workspace and forms.
- **Compact:** tables, evidence rows, model/runtime metadata.

Density changes spacing and row height, not core typography families or semantic color meaning.

## Shadows and elevation

Prefer borders, surface tone, and overlap before shadow. Use soft low-opacity elevation only for floating layers such as menus, dialogs, command palette, and dragged/resized surfaces. Avoid persistent card-drop-shadow dashboards.

## Brand-system patterns

Reference/marketing implementations may add:
- ScaleFoldSupergraphic;
- ScaleFoldPattern;
- BrandIntro;
- BrandToWorkTransition;
- DisplayLockup.

These are brand patterns, not ordinary workspace primitives. They should not leak gradient or decorative geometry into every product component.
