# Clarifications: Spec 053

**External gates**: none opened. REAL_PHI NOT_AUTHORIZED. MESC mutation NO.
Product runtime network DEFAULT_DENY. WORKER_OS_SANDBOX_PLATFORM_QUALIFIED
stays OPEN (signed App Sandbox enforcement + multi-OS qualification external).

**Failure modes**: old kernels without seccomp yield honest ApplyFailed
(child exit 1), never a PASS claim. Non-Linux yields NotReadyOnThisHost.
