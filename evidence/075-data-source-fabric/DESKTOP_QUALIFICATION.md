# DESKTOP_QUALIFICATION — Spec 075

## Surface

Native Slint `Data` route (`crates/medscale-desktop/ui/app.slint`):
sources (active project), snapshots, typed schema header, row grid,
provenance detail, import/refresh/schema-ack/grid-view/transform actions,
explicit empty/loading/error/denied/conflict/corrupt/partial/unavailable
states via `data-status`. Light/dark through the shared `Theme` tokens;
keyboard/focus through shared components (`FocusableClickArea`,
`AdaptiveLineEdit`, `ToolbarAction`, `QuickAction`); no color-only truth
states (`StatusPill` carries text labels).

## Authority

All state arrives through `data_workbench.rs` view-models backed exclusively
by `CliSession` (Core). No direct storage/driver/network access from UI.
Covered by in-module session tests (`workbench_flows_through_real_core_session`,
`workbench_input_parsers_reject_gracefully`).

## Renders

`DATA_WORKBENCH_LIGHT.png` / `DATA_WORKBENCH_DARK.png`: deterministic
renders of the Data route. Local rendering is unavailable on this
workstation (no display/link toolchain); renders are captured from the
qualified build path at close, or this file records the exact limitation.
A screenshot alone never substitutes for the session-assertion tests above.

```text
RESULT = PENDING (exact-head CI on PR #129 branch spec/075-data-source-fabric)
```
