# Spec 051 plan

1. Diff `REQUIRED_CHECKS.md` against `.github/workflows/ci.yml` job `name:` values.
2. Update packet + OWNER_BRANCH_PROTECTION_ACTION.md with the missing perf job.
3. Add evidence note under `evidence/051-required-checks-sync/`.
4. Add doctor readiness flag `required_checks_packet_synced` = true while keeping
   `release_ready = false`.
5. Contract test asserts packet includes `perf delivery-plan scale (windows)`.
6. Do not modify GitHub repository settings.
