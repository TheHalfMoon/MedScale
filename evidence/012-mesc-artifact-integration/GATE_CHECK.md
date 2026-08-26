# Spec 012 MESC artifact gate check

**Checked**: 2026-08-26  
**Against**: `TheHalfMoon/MESC` live GitHub truth  

## Finding

No qualifying **immutable MESC artifact package** is available for `ARTIFACT_IMPORT`.

| Requirement | Status |
|---|---|
| Released immutable artifact bundle (downloadable assets) | **MISSING** — `v0.1.0` release has `assets: []` |
| Exact content hashes for admitted bytes | **MISSING** |
| Rights / license NOTICE for artifact redistribution | **MISSING** (repo is research Python package; not an artifact Pack) |
| SBOM / provenance for artifact | **MISSING** |
| Evaluation evidence bound to artifact identity | **MISSING** |
| Pack admission path in MedScale | **NOT STARTED** (blocked) |

## Non-admission notes

- `TheHalfMoon/MESC` currently presents as a Python research-intelligence repository (README branded MedScale research foundation), not a MedScale-admissible MESC evaluation artifact feed.
- MedScale must not copy/import MESC Python package, DB, keys, or internal runtime.
- MedScale must not mutate `TheHalfMoon/MESC`.

## Gate

Keep `MESC_RELEASED_ARTIFACT = NOT_AVAILABLE` and Spec 012 `BLOCKED_BY_RELEASED_MESC_ARTIFACT` until a released immutable artifact with hashes/rights/SBOM/evaluation is published for ARTIFACT_IMPORT.
