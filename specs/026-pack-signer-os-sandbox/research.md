# Research — Spec 026

## Pack trust

- Spec 008 admitted Packs by digest/rights/sbom without signer or anti-rollback.
- Q09 requires signer/trust/rollback before native engines.
- Decision: synthetic MedScale trust root (ed25519) for fixture Packs only; no gated model terms.
- Crypto placement: `ed25519-dalek` admitted under Spec 026 in `medscale-keys` (central crypto), consumed by `medscale-pack`.

## Anti-rollback

- Track last admitted `(pack_epoch, version)` per `pack_id`.
- Reject when `new.epoch < last.epoch` or (`new.epoch == last.epoch` and `new.version < last.version` via simple dotted numeric compare).

## OS sandbox

- Spec 008 left `try_apply_os_sandbox` always `NotPlatformQualified`.
- Platform qualification is engineering + measured evidence; EXTERNAL_GATES stays OPEN until multi-OS measured.
- Decision: `ReadyBaseMeasured` for Linux Landlock apply only; Windows/macOS remain scaffolds.
- Landlock crate is Linux-only (`target_os = "linux"` dependency).
- Measured proof: thread-scoped apply + deny open outside allowlist (documented evidence).

## Honesty

- Never set `PlatformQualified` for all targets.
- Never clear `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED`.
- Doctor: `linux_measured=true`, `platform_qualified=false`, `release_ready=false`.
