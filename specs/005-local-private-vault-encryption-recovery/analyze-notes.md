# Analyze Notes: Spec 005 Local Private Vault + Encryption + Recovery

**Date**: 2026-08-25  
**Command**: `/speckit.analyze` equivalent (planning package consistency)  
**Package**: `specs/005-local-private-vault-encryption-recovery/`

## Artifacts reviewed

| Artifact | Present |
|---|---|
| spec.md | yes |
| clarifications.md | yes |
| research.md | yes |
| plan.md | yes |
| data-model.md | yes |
| contracts/vault-api.md | yes |
| contracts/key-lifecycle.md | yes |
| quickstart.md | yes |
| checklists/requirements.md | yes |
| tasks.md | yes |
| analyze-notes.md | yes (this file) |

## Alignment with roadmap / constitution

| Gate / invariant | Coverage | Status |
|---|---|---|
| Pre-PHI encryption / key / recovery / backup / migration / log / privacy tech qual | US1–US7, FR-001–015, SC-001–008 | OK |
| Vault defaults to local app-data | US1, FR-002, D9, T012 | OK |
| Sync/remote filesystem detection (extend Spec 003) | US2, FR-003, D9, T018 | OK |
| Single-writer + key lifecycle | US3, FR-004/005, D7/D10, key-lifecycle.md | OK |
| Snapshot / retention / key-loss / restore proofs | US4–US5, SC-004/005, T022–T029 | OK |
| REAL_PHI separately unauthorized | FR-011, SC-008, C8, EXTERNAL_GATES | OK |
| SQLCipher DEPENDENCY/FFI candidate + pin | D2–D4, T002/T014 | OK |
| keyring-core + selected stores pinned | D5, T003/T011 | OK |
| medscale-keys crate | FR-014, D13, T005 | OK |
| EncryptedVault trait + fail-closed / SQLCipher prefer + AES-GCM exit | D1/D4, FR-013, T041 | OK |
| One-way migration from Spec 003 | US6, D12, T030–T034 | OK |
| No product network; no MESC mutation; synthetic-only | FR-011/012, anti-scope | OK |

## Cross-document consistency checks

1. **Roadmap 005 row**: app-data, sync detection, single-writer/key lifecycle, snapshot/retention/key-loss/restore; REAL_PHI separate — all present.
2. **MASTER_BUILD_PLAN Pre-PHI / Spec 005**: encryption, KeyProvider, recovery, backup, retention, sync policy, logs — covered; counsel PDPL/SFDA explicitly non-blocking technical-only (C11).
3. **SOURCE_ACQUISITION**: SQLCipher v4.17.0 + keyring-core pins in research; admission tasks before code.
4. **GLM F-03/F-05/F-10/F-12**: app-data + sync refuse; Core Host keys; encryption before PHI; keyring-core stores — OK.
5. **Spec 003 admission**: SQLCipher not yet admitted — T002 creates 005 admission; does not mutate 003 package beyond consumption.
6. **Clarifications**: Zero NEEDS CLARIFICATION; autonomous defaults only — OK.
7. **Tasks**: Numbered T001–T045 for Spec 005 only; no Spec 006 product UI / REAL_PHI flip — OK.

## Contradictions / gaps

| Item | Severity | Disposition |
|---|---|---|
| Spec 004 CLOSED_CANONICAL | resolved | BUILD_QUEUE shows CLOSED; implement gate open |
| Exact embedded SQLCipher SHA inside rusqlite vendor tree vs tag 4.17.0 | residual | Resolve at T014 with admission amend if needed (research D3) |
| rusqlite 0.37.0 vs 0.40.2 CI bump | residual | Prefer 0.37.0; allow 0.40.2 per D3 without founder ask |
| Apple keyring / mobile Keystore depth | expected | Optional macOS CI pin; Spec 009 owns mobile |
| Full PRIVACY_PROOF / medscale doctor | expected | Spec 006; 005 supplies foundations only |
| Legal counsel PDPL/SFDA mapping | expected | External/planning; not code blocker |

**Unresolved design blockers inside Spec 005 package: NONE.**

**Implementation readiness blocker: NONE (Spec 004 is `CLOSED_CANONICAL`).**

## Entry / exit readiness

```text
ENTRY: Spec 004 CLOSED_CANONICAL
PACKAGE_STATE: COMPLETE_SPEC_KIT_PACKAGE
ANALYZE_RESULT: PASS_NO_UNRESOLVED_DESIGN_BLOCKERS
ANALYZE_QUALIFICATION: QUALIFIED
IMPLEMENT_BLOCKED_UNTIL: NONE
READY: YES
IMPLEMENTATION: AUTHORIZED_TO_START
```

## Recommendation

**QUALIFIED for implementation.** Proceed with `/speckit.implement` on branch `spec/005-local-private-vault-encryption-recovery` using `tasks.md`. Do not treat absence of REAL_PHI authorization, Spec 006 UI, MESC, or counsel legal sign-off as READY blockers. Prefer SQLCipher; use AES-GCM EncryptedVault exit only if CI proves SQLCipher non-viable after documented fix attempts.
