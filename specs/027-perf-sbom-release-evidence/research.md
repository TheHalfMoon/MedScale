# Research — Spec 027

## Delivery-plan budgets (measure; do not claim)

From `TRUSTED_V1_DELIVERY_PLAN.md`:
- timeline for 10,000 events p95 ≤250 ms
- lexical search over 10,000 local records p95 ≤300 ms
- 1 MiB bounded FHIR ingest p95 ≤500 ms (excluding external validation)
- Method: 30 runs after explicit warmup; report p50/p95 + hardware/toolchain notes

## Harness choice

| Option | Pros | Cons | Decision |
|---|---|---|---|
| Criterion `#[bench]` | Industry standard | Extra dep / CI complexity; deny review | Defer |
| Deterministic integration test | No new runtime dep; CI-safe | Not full Criterion stats | **Select** |

READY_BASE scale may be smaller than 10k events/records; evidence must state scale honesty and `budgets_claimed_met=false`.

## SBOM approach

Prefer `cargo metadata` → minimal CycloneDX 1.5-like JSON without admitting heavy cargo-cyclonedx as a CI-required tool. Document limitations: no native binaries, no model weights, not a signed release SBOM.

## Package checksums

sha256 over listed workspace crate source trees + `Cargo.lock` into an evidence manifest. Not a reproducible binary package or signed artifact.
