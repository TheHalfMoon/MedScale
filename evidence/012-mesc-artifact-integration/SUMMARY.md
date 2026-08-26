# Spec 012 evidence summary

## Live gate (2026-08-26 independent re-check)
- MESC `main`: `4b193c01f5f94447afd359b0420640647b449a69` / tree `3f48f3a9af89d8e82f04c0501019b09084ff860a`
- `TRAINING_CODE_READY = YES`; `REAL_TRAINING_AUTHORIZED = NO`; `RELEASE_STATUS = BLOCKED`
- TheHalfMoon/MESC `v0.1.0` (release_id `352847712`) assets: **empty**
- `v0.2.0` tag: **no GitHub Release**
- Gate `MESC_RELEASED_ARTIFACT` remains `NOT_AVAILABLE`
- `MEDSCALE_SPEC_012_ADMISSION_READINESS = NOT_READY`

## Delivered while gated
- `MescArtifactAdmit` → `ExternalGateRequired(MESC_RELEASED_ARTIFACT)`
- Requires `pack_path_required=true` (ARTIFACT_IMPORT / Pack boundary)
- Doctor `mesc_artifact` axis (`artifact_admitted=false`)
- Architecture refuse MESC Python dependency test

## Not delivered
- Actual Pack admission of MESC bytes
- CLOSED_CANONICAL for Spec 012
