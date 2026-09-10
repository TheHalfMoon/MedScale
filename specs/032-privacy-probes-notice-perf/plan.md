# Plan: Spec 032 Privacy Probes + NOTICE + Perf Binding

1. Spec Kit package (specify/clarify/plan/research/ADR/tasks/checklist/analyze/converge).
2. Add `privacy_probes` module in `medscale-storage` (OS file existence probes + leftover/crash helpers).
3. Extend `VaultPrivacyDoctorStatus` with probe honesty fields; keep `private_data_ready=false`.
4. Extend `ReleaseQualificationDoctorStatus` with `notice_inventory_present` / `rights_license_decision=false`.
5. Wire doctor + CLI; tests for probe helpers and doctor honesty.
6. Script `scripts/generate-notice-inventory.ps1`; write inventory under evidence + `docs/legal/NOTICE_INVENTORY.md`.
7. Strengthen `perf_harness_027` binding fields; re-run and commit `perf_harness_latest.json`; update methodology note.
8. Fix stale 017/023 LIMITATIONS / admission MemoryMock wording; EXTERNAL_GATES residual notes.
9. BUILD_QUEUE / roadmap / START_HERE → 032 CLOSED; deferred **033+**.
10. Gates: fmt, clippy `-D warnings`, `cargo test --workspace --locked` (`CARGO_TARGET_DIR=D:\medscale-target`).

## Architecture

```text
doctor
  vault_privacy.probes_present=true
  residual_risk_classes_open = [swap, hibernate, snapshot, pagefile]
  pagefile/hiberfil existence probes (best-effort)
  leftover_work_present / wipe helpers (Spec 017)
  private_data_ready=false  -----> OS_KEYRING_SWAP_SNAPSHOT stays OPEN

release_qualification
  notice_inventory_present=true
  rights_license_decision=false -----> PUBLIC_SOURCE_LICENSE_CHOICE stays PENDING

perf_harness_027
  bind git SHA/tree, rustc, OS, host/CPU, Cargo.lock digest, fixtures
  budgets_claimed_met=false
```
