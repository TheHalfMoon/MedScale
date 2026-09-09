# v0 UI Integration Contract

## Ownership

**v0 owns:** visual design, component composition, responsive layout, interaction presentation, visual states, and founder-selected UX direction.

**Cursor owns:** Rust trusted core, clinical/source/authority semantics, typed client/IPC contracts, local state wiring, deterministic fixture adapters, accessibility verification, error/loading/empty-state correctness, integration tests, privacy/security qualification, packaging and platform integration.

## Integration rule

v0 output is treated as user-supplied UI source. Cursor must preserve its approved visual intent but must remove or replace any generated backend/database/auth/network assumptions that conflict with MedScale. No frontend component directly reads the canonical database or keys. No UI event silently becomes an authority-changing or external action.

For Desktop, Tauri/WebView remains conditional on privacy qualification. If WebView storage/cache/crash behavior cannot meet the declared privacy claim, Cursor must move the approved UI intent to the safer qualified shell rather than weaken the claim.

For mobile, Swift/Kotlin platform shells may implement the v0 visual intent while sharing Rust semantics. Web UI code is not automatically the mobile runtime.

## v0 workflow

1. Founder creates/iterates visual UI in v0 using synthetic data.
2. v0 may import/sync the GitHub repo on a UI branch.
3. Cursor inventories the diff and records provenance under `imports/v0/`.
4. Cursor rejects generated authority/backend shortcuts, adapts components to typed MedScale contracts, and adds tests.
5. Only integrated, privacy-qualified UI is merged.

Cursor does not wait for final v0 polish to build core product behavior.

## Whole-product qualification refinement (2026-09-09)

The first integration journey is import -> inspect source/identity/coverage -> timeline/Brief
-> export -> close/reopen -> backup/restore. Show unsupported resources, partial dates,
conflicts, unknown/missing evidence and model availability explicitly. Acceptance includes
keyboard-only operation, focus/error recovery, screen-reader labels, accessible contrast,
empty/loading/failure states and source drill-down. UI calls only the Rust authority facade;
no direct vault, model, secret or network access. Backend contracts and synthetic integration
can proceed before final v0 visuals; final visual release remains governed by this contract.
