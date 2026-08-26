# Spec 012 MESC artifact gate check

**Checked**: 2026-08-26 (re-check from live GitHub truth)  
**Against**: `TheHalfMoon/MESC`  
**MedScale main**: `c4184b0e19eca495b8eb6246126d1d8544235fa8`

## Live release observation

| Item | Observation |
|---|---|
| Latest GitHub Release | `v0.1.0` — “MedScale v0.1.0 — Research Foundation Release” |
| Release assets | **`assets: []`** (zero downloadable files) |
| Tag `v0.2.0` | Exists (`d2e651a…`) but **no** GitHub Release (`404` on `/releases/tags/v0.2.0`) |
| Org container packages | Not available / not found |
| Filename search `*.sha256` in MESC | `total_count: 0` via code search |

## Qualifying ARTIFACT_IMPORT requirements

| Requirement | Status |
|---|---|
| Released immutable artifact bundle (downloadable assets) | **MISSING** |
| Exact content hashes for admitted bytes | **MISSING** |
| Rights / license NOTICE for artifact redistribution into MedScale Pack | **MISSING** as Pack-bound artifact |
| SBOM / provenance bound to artifact identity | **MISSING** |
| Evaluation evidence bound to artifact identity | **MISSING** |
| Pack admission of MESC bytes | **BLOCKED** (gate) |

## Non-admission notes

- `TheHalfMoon/MESC` presents as a Python research-intelligence repository, not a MedScale-admissible MESC evaluation artifact feed.
- In-repo `data/` trees (litdb/bench) are **not** a released immutable ARTIFACT_IMPORT package with hashes/rights/SBOM for MedScale Pack.
- MedScale must not copy/import MESC Python package, DB, keys, or internal runtime.
- MedScale must not mutate `TheHalfMoon/MESC`.

## Gate decision

**Do not clear** `MESC_RELEASED_ARTIFACT`.  
Keep Spec 012 `BLOCKED_BY_RELEASED_MESC_ARTIFACT` for admit/closeout tasks (T002/T003/T005).

Fail-closed admit contracts + doctor axis may ship while the gate remains open (ExternalGateRequired).
