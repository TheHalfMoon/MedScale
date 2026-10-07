# Consistency analysis: Spec 101

Spec 101 adds packaging only. It does not change Core semantics, Tauri commands, capabilities, CSP, plugins, network policy, vault ownership or the synthetic-only scope (FR-001, FR-005, Anti-scope). The bundle identifier is unchanged, so app-local data does not move (FR-007).

**Requirements to tasks:**
- FR-001: T101-01
- FR-002: T101-02
- FR-003: T101-02 and research
- FR-004: T101-04/05
- FR-005: T101-07/08/09
- FR-006: T101-10
- FR-007: T101-01 test
- FR-008: T101-03

**User stories to tasks:**
- US1: T101-01/02/04/05/07
- US2: T101-02/04/05/08
- US3: T101-02/04/05/09
- US4: T101-10/11/12

**Consistency checks:**
- The research table matches the locked CLI version (2.12.0) and the source tag it was read from.
- The plan's per-OS `--bundles` values match the spec's targets.
- The `webviewInstallMode=skip` decision appears identically in the spec, research, config and config test.
- Signing appears only as `NOT_GRANTED`.
- The F096-T01 and F096-T05 carry-forward matches `specs/096-tauri-foundation/findings.md`.

**Open items that block closure, not drafting:**
- Every unchecked checklist line.
- The weak-digest residual for the NSIS tools.
- The dependency on #177 and the scoped review gate on the Tauri chain.
