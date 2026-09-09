# Plan: Spec 027 Perf Harness + Package/SBOM Release Evidence

1. Spec Kit package (specify/clarify/plan/research/ADR/tasks/checklist/analyze/converge).
2. Extend `ReleaseQualificationDoctorStatus` with `perf_harness_present` / `sbom_scaffold_present`; keep `release_ready=false`; refresh missing classes.
3. Implement `crates/medscale-core/tests/perf_harness_027.rs`: synthetic timeline / lexical / FHIR ingest timings; warmup + 30 runs; p50/p95 JSON evidence; assert harness ran only.
4. Add `scripts/generate-sbom-scaffold.ps1` (cargo metadata → CycloneDX-like JSON) and `scripts/generate-package-checksums.ps1` (sha256 manifest).
5. Commit evidence under `evidence/027-perf-sbom-release-evidence/` (SUMMARY, LIMITATIONS, sample SBOM, checksums, harness methodology).
6. Wire CLI doctor display for new honesty fields; update release_qualification_022 / CLI tests.
7. BUILD_QUEUE / SPECKIT_MASTER_ROADMAP_V2 / START_HERE → 027 CLOSED; deferred **028+**.
8. Gates: fmt, clippy `-D warnings`, `cargo test --workspace --locked` (`CARGO_TARGET_DIR=D:\medscale-target`).

## Architecture

```text
synthetic fixtures
        |
        v
perf_harness_027 (warmup + N) --> evidence JSON (p50/p95; budgets_not_claimed)
        |
Doctor: release_qualification.perf_harness_present=true, release_ready=false

Cargo.lock + crate sources
        |
        +--> generate-sbom-scaffold.ps1 --> CycloneDX-like scaffold (crates only)
        +--> generate-package-checksums.ps1 --> sha256 manifest (not signed)
        |
Doctor: sbom_scaffold_present=true; missing: full release SBOM / signing / package bar
```
