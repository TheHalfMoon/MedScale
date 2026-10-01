# Findings — Spec 097

## Initial bounded Patients implementation

Shared sidebar, navigation, icons, header, status, search, table, row, empty state,
section, button and runtime state are extracted. The first-party outline family
uses one 24-unit grid, 18px rendering and two-unit strokes. Tabs will be added
with the supported Patient Detail slice; no inactive tab control is invented.

The Patients view has four prospective supported columns, an unknown total,
disabled search, compact unavailable row and an inline explanation. The reusable
PatientRow is not populated: no roster query has run and Sarah's Slint display
fixture is not imported. No Core, Rust command, capability, CSP or dependency
change is included in this slice.

Local verification: six frontend tests pass; strict TypeScript and Vite build
pass. The first build caught the missing search/commands icon union, repaired
before the successful build. No skipped test or native success is inferred.

Browser inspection covered Patients in both themes at 1440×900 and 1100×720,
keyboard Enter opening/closing the availability explanation, Ctrl+K opening the
palette and Escape restoring focus to its trigger. At 1280×800 the table is 956px
wide with no page-wide horizontal overflow. Home retains one sidebar and a
compact workspace notice. These are development observations, not native final
evidence or assistive-technology qualification.

Open: native captures and exact-revision receipts; populated row; governed Core
adapter; Patient Detail/Evidence; full provider/dependency/privacy qualification.
Jev and provider-backed Alibaba OCR are BLOCKED_BY_ZERO_COST. The inherited glib
advisory policy failure is unresolved. No merge or CLOSED_CANONICAL claim.
