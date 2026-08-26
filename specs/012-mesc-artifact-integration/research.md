# Spec 012 research notes

## Disposition
`ARTIFACT_IMPORT` exclusively (SOURCE_ACQUISITION_AND_COPY_PLAN §6).

## Missing for eligibility
1. Downloadable immutable release asset(s)
2. Exact content hashes
3. Rights/NOTICE for redistribution into MedScale Pack
4. SBOM / provenance
5. Evaluation evidence bound to artifact identity

## Live check (2026-08-26 re-check)
- `gh api repos/TheHalfMoon/MESC/releases/latest` → tag `v0.1.0`, `asset_count: 0`, `assets: []`
- Tag `v0.2.0` exists; `/releases/tags/v0.2.0` → **404** (no Release)
- Code search `*.sha256` in MESC → `total_count: 0`
- Gate decision: **do not clear** `MESC_RELEASED_ARTIFACT`

## Fail-closed MedScale path (ships while gated)
- Capability `MescArtifactAdmit` + request requiring `pack_path_required=true`
- Authority returns `ExternalGateRequired(MESC_RELEASED_ARTIFACT)`
- Doctor axis `mesc_artifact` (`artifact_admitted=false`)
- Architecture test refuses MESC Python runtime dependency
- No Python import, shared DB/keys, or ambient service authority
