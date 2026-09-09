# MedScale Definition of Done

A task is not done because code exists. A material unit closes only when its exact acceptance criteria, tests, security/privacy/evidence gates, documentation, migration/rollback concerns, and current-head review are satisfied.

A spec becomes `CLOSED_CANONICAL` only after:

- complete Spec Kit artifacts and `/speckit.analyze` consistency;
- all dependency/admission records resolved for its scope;
- implementation tasks complete in dependency order;
- required unit/integration/property/fuzz/fault/conformance/benchmark tests pass;
- exact-head evidence is recorded with commit/tree/toolchain/lock/platform/fixtures/commands/results/limitations;
- security/privacy gates for the spec pass;
- docs/contracts/source provenance are synchronized;
- no unresolved material review finding remains;
- merge lands without bypassing required checks;
- post-merge canonical state is verified;
- `BUILD_QUEUE.md` is updated and the next eligible unit starts.

## Project completion

The current V2 product program is complete when Specs 000–015 are each `CLOSED_CANONICAL` or explicitly `DEFERRED_BY_CANONICAL_DESIGN`, all release-qualified claims are evidence-backed, and any remaining blockers are only external gates recorded in `EXTERNAL_GATES.md`.

If code is complete but final UI/app-store/partner/legal/real-PHI gates remain, report `IMPLEMENTATION_COMPLETE_PENDING_EXTERNAL_GATES` rather than pretending those gates passed. Spec 016+ is not required for V2 completion unless later promoted by canonical evidence.

## Whole-product qualification refinement (2026-09-09)

Scoped CLOSED_CANONICAL means the owning scope passed its checks. It does not mean a fixture
is a live integration or that product release, privacy, clinical, mobile or interoperability
readiness is established. Report these axes separately. Trusted V1 release requires the
measured workflow, storage/recovery, platform and artifact gates in
[delivery plan](TRUSTED_V1_DELIVERY_PLAN.md). No PARITY or SURPASS claim without matched-task evidence.
