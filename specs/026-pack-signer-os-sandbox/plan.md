# Plan: Spec 026 Pack Signer + OS Sandbox READY_BASE

1. Spec Kit package (specify/clarify/plan/research/ADR/tasks/checklist/analyze/converge).
2. Contracts: PackManifestV0 signer/epoch fields; PackAdmitReason; OsSandboxQualification::ReadyBaseMeasured; doctor axes.
3. `medscale-keys` ed25519 verify/sign helpers + synthetic trust root; admit Pack signatures in `medscale-pack`.
4. PackStore anti-rollback on insert/admit.
5. Linux Landlock apply path behind ReadyBaseMeasured; Windows/macOS NotPlatformQualified.
6. Doctor honesty; tests; evidence SUMMARY/LIMITATIONS/LINUX_LANDLOCK_MEASURED.
7. Admissions for ed25519-dalek (+ landlock on Linux); deny.toml if needed.
8. BUILD_QUEUE / roadmap / START_HERE → 026 CLOSED; deferred **027+**.
9. Gates: fmt, clippy `-D warnings`, `cargo test --workspace --locked` (`CARGO_TARGET_DIR=D:\medscale-target`).

## Architecture

```text
pack.manifest.json --verify digests/rights/sbom/sig--> PackManifestV0
                              |
                              v
                     PackStore anti-rollback
                              |
Doctor: pack_signer READY_BASE (release_ready=false)

OsSandboxPlan(Linux, ReadyBaseMeasured, allow_paths)
                              |
              cfg(linux) Landlock restrict_self
                              |
Doctor: os_sandbox linux_measured=true, platform_qualified=false
EXTERNAL_GATES WORKER_OS_SANDBOX_PLATFORM_QUALIFIED = OPEN
```
