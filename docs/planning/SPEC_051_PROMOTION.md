# Spec 051 promotion — REQUIRED_CHECKS live CI sync

## Promotion decision

```text
UNIT = SPEC_051_REQUIRED_CHECKS_LIVE_CI_SYNC
CLASS = EXECUTABLE_NOW
AUTHORITY = TRUSTED_V1 residual (Q05 / Spec 022–037 owner packet honesty)
PROMOTION = APPROVED_FOR_SPEC_KIT_PACKAGE
```

## Residual proof

After Spec 042, live `.github/workflows/ci.yml` includes job
`perf delivery-plan scale (windows)`.

`evidence/022-release-qualification-prep/REQUIRED_CHECKS.md` (Spec 037 packet)
still lists only five jobs and omits the perf job. Owner branch-protection
instructions must match live workflow job names exactly.

## Bound

- Update REQUIRED_CHECKS packet + owner instructions for the six live CI jobs.
- Record that branch protection remains `NOT_CONFIGURED_OWNER_SETTINGS`.
- Do not configure GitHub settings from Cursor.
- Do not claim RELEASE_READY.

## Out of scope

- Enabling branch protection (owner setting).
- Changing CI job behavior.
- Spec 012 MESC / signing / legal.
