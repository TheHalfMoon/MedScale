# Analyze Notes: Spec 006 CLI + Desktop Foundation

**Date**: 2026-08-25  
**Command**: `/speckit.analyze` equivalent (planning package consistency)  
**Package**: `specs/006-cli-desktop-foundation/`

## Artifacts reviewed

| Artifact | Present |
|---|---|
| spec.md | yes |
| clarifications.md | yes |
| research.md | yes |
| plan.md | yes |
| data-model.md | yes |
| contracts/cli-authority.md | yes |
| contracts/doctor-report.md | yes |
| contracts/privacy-proof.md | yes |
| quickstart.md | yes |
| checklists/requirements.md | yes |
| tasks.md | yes |
| analyze-notes.md | yes (this file) |

## Alignment with roadmap / constitution

| Gate / invariant | Coverage | Status |
|---|---|---|
| First useful product surface; same Rust core | US1/US3, FR-001/004, D1 | OK |
| CLI authority; no privileged bypass; no direct DB | US1, FR-001/002, cli-authority.md, T012–T013 | OK |
| Desktop shell prototype; WebView prove-or-reject | US5, FR-006/007, D2 — **reject/defer Tauri** | OK |
| `medscale doctor` axes | US2, FR-003, D5, doctor-report.md | OK |
| `PRIVACY_PROOF` typed evidence | US4, FR-005, D6, privacy-proof.md | OK |
| v0 integrate when available; else do not block | US5, FR-008, D7, T025 | OK |
| Synthetic-only; REAL_PHI unauthorized | FR-009, SC-007 | OK |
| DEFAULT_DENY network; no MESC mutation | FR-010 | OK |
| Depends on Spec 005 CLOSED | Assumptions, T001, implement gate | OK |
| clap/anyhow pins | D4, FR-012, T002 | OK |

## Cross-document consistency checks

1. **SPECKIT_MASTER_ROADMAP 006 row**: CLI authority, Desktop prototype, WebView prove-or-reject, doctor, PRIVACY_PROOF, no UI/CLI DB bypass — all present; WebView resolved as **defer/reject for this unit**.
2. **MASTER_BUILD_PLAN §7/§10/§14**: Longitudinal wedge; CLI privacy contract; doctor; PRIVACY_PROOF — covered.
3. **V0_UI_INTEGRATION_CONTRACT**: Non-blocking without artifact; no invented final UI; Tauri conditional — OK.
4. **SOURCE_ACQUISITION Tauri v2.11.5**: Candidate only; admission explicitly withheld — OK.
5. **Clarifications**: Zero NEEDS CLARIFICATION; autonomous defaults — OK.
6. **Tasks**: T001–T032 Spec 006 only; no REAL_PHI flip; no Tauri DEPENDENCY — OK.
7. **Existing `medscale-cli`**: Bootstrap replaced/extended — OK.

## Contradictions / gaps

| Item | Severity | Disposition |
|---|---|---|
| Roadmap “Desktop shell prototype” vs deferred Tauri | resolved | Thin scaffold = foundation prototype; WebView rejected until privacy proof |
| Multi-process IPC vs in-process host | residual | D3: in-process for 006; envelopes reusable |
| Pack/runtime doctor fields | expected | Placeholder until Spec 008 |
| Exact safer native shell tech post-Tauri | expected | Out of 006; documented policy only |
| Spec 005 CLOSED at implement time | gate | BUILD_QUEUE currently shows CLOSED; T001 reconfirms |

**Unresolved design blockers inside Spec 006 package: NONE.**

**Implementation readiness blocker: NONE if Spec 005 remains `CLOSED_CANONICAL`.**

## Entry / exit readiness

```text
ENTRY: Spec 005 CLOSED_CANONICAL
PACKAGE_STATE: COMPLETE_SPEC_KIT_PACKAGE
ANALYZE_RESULT: PASS_NO_UNRESOLVED_DESIGN_BLOCKERS
ANALYZE_QUALIFICATION: QUALIFIED
IMPLEMENT_BLOCKED_UNTIL: Spec 005 must remain CLOSED_CANONICAL
READY: YES
IMPLEMENTATION: AUTHORIZED_TO_START (no Rust in this planning turn)
TAURI_ADMITTED: NO
V0_REQUIRED_FOR_CLOSE: NO
```

## Recommendation

**QUALIFIED for implementation.** Proceed with `/speckit.implement` on branch `spec/006-cli-desktop-foundation` using `tasks.md`. Prefer CLI-complete wedge + Desktop scaffold. Do **not** admit Tauri until WebView privacy proof passes; record PRIVACY_PROOF limitations. Do not treat missing v0 artifact, REAL_PHI gate, MESC, or counsel legal sign-off as READY blockers.
