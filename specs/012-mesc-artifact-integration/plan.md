# Plan: Spec 012

| ID | Decision |
|---|---|
| D1 | Admit/closeout blocked until `MESC_RELEASED_ARTIFACT` closes with qualifying assets |
| D2 | Disposition = `ARTIFACT_IMPORT` / Pack only — never Python import |
| D3 | Fail-closed `MescArtifactAdmit` + doctor axis may ship while gate is open |
| D4 | Refresh `GATE_CHECK.md` whenever MESC publishes release assets |

## Current evidence
2026-08-26 re-check: TheHalfMoon/MESC `v0.1.0` has `assets: []`; `v0.2.0` tag has no Release — gate remains NOT_AVAILABLE.
