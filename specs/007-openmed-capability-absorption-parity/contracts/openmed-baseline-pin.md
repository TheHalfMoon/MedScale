# Contract: OpenMed Baseline Pin

**Spec**: 007  
**Status**: Canonical for research  
**Related**: [data-model.md](../data-model.md) `OpenMedBaselinePin`, SOURCE_ACQUISITION_AND_COPY_PLAN §2

## Frozen identity

| Field | Value |
|---|---|
| Upstream URL | `https://github.com/maziyarpanahi/openmed` |
| Baseline tag | `v2.2.0` |
| Baseline commit | `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837` |
| Baseline tree | `1c949e35b2b8f2ea69da4284b370074fc4bf84ab` |

## Rules

1. Every Spec 007 parity/absorption artifact MUST cite this commit as the floor.
2. Comparing MedScale to a moving OpenMed branch as the **floor** is a contract violation.
3. Post-v2.2 tip notes MAY be appended under `informational_deltas[]` with `authoritative: false`.
4. Verification SHOULD record method (`git_clone` preferred when network/public clone available for research tooling).
5. Failure to verify locally does not invalidate the documented pin from SOURCE_ACQUISITION; evidence must state `documented_pin_only` until verified.

## Evidence path

`evidence/007-openmed-capability-absorption-parity/BASELINE.md` and optional `PIN_VERIFICATION.md`.
