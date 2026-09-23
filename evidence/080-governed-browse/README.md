# Evidence — Spec 080 Governed Browse

Required set (see `specs/080-governed-browse/plan.md`): README, LIVE_TRUTH,
CONTRACT_QUALIFICATION, NETWORK_SSRF_QUALIFICATION,
STORAGE_MIGRATION_RECOVERY, CORE_AUTHORITY_QUALIFICATION, CLI_QUALIFICATION,
DESKTOP_QUALIFICATION, SECURITY_ADVERSARIAL, EXACT_RANGE_REVIEW,
EXACT_HEAD_QUALIFICATION, POST_MERGE_VERIFICATION, CLOSURE.

Hermetic tests use a scripted, socket-free transport. GitHub Actions CI is
the authoritative qualification path (the local Windows toolchain cannot
link, and the local WSL distro has been unavailable since 2026-09-23).
