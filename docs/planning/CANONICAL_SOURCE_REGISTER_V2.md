# MedScale Canonical Source Register V2

**Date:** 2026-08-25  
**Status:** `CANONICAL_PLANNING_V2`  
**Implementation authorization:** NO

## Register invariants

- Founder decisions and frozen MedScale architecture control MedScale semantics.
- Live GitHub truth controls mutable repository facts.
- External `AUTHORITATIVE` classification is scoped to the external source's subject only.
- Donor disposition is separate from source classification.
- Mutable releases/versions must be reverified before material implementation or benchmarking.
- The original 185 external URLs are retained exactly once; none is silently discarded.

## Source classes

```text
AUTHORITATIVE
EVIDENCE
CANDIDATE
HISTORICAL
REJECTED
SUPERSEDED
```

## Corpus files

- [Original 185 — Part A](source-register/ORIGINAL_185_PART_A.md)
- [Original 185 — Part B](source-register/ORIGINAL_185_PART_B.md)
- [V2 additions](source-register/V2_ADDITIONS.md)

## Current mutable baselines

```text
TheHalfMoon/MedScale = current product repository; planning artifacts canonical
TheHalfMoon/MESC = independent scientific/model program; always re-read live state before integration decisions
OpenMed parity baseline = v2.2.0 @ 59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837
```

## Explicit supersessions

- OpenMed v2.1.0 release pins remain provenance but are superseded for parity by v2.2.0.
- Old pinned MESC project-truth URLs remain provenance but are superseded for current mutable state.
- `github.com/IamShehri/MedScale` is historical; `TheHalfMoon/MedScale` is current.
- Legacy keyring-rs single-dependency assumptions are superseded by `keyring-core` plus explicitly selected credential-store crates.
- Raw OpenMed model counts are dated/manifest-bound evidence, never a stable architectural target.

## Corpus accounting

```text
ORIGINAL_URL_COUNT = 185
ORIGINAL_URLS_DROPPED = 0
V2_ADDITIONS = 25
TOTAL_REGISTERED_POINTERS = 210
```
