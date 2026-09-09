# Requirements checklist — Spec 026

- [x] Signer + synthetic trust root required for Pack admit
- [x] Anti-rollback on epoch/version
- [x] Digests/rights/sbom still required
- [x] Linux ReadyBaseMeasured Landlock apply with measured deny evidence
- [x] Windows/macOS remain NotPlatformQualified
- [x] EXTERNAL_GATES WORKER_OS_SANDBOX_PLATFORM_QUALIFIED stays OPEN
- [x] Doctor honesty: linux_measured / !platform_qualified / !release_ready
- [x] No RELEASE_READY / PRIVATE_DATA_READY / MULTI_CLIENT_RELEASE_READY / real models
- [x] Deferred advanced work labeled 027+
