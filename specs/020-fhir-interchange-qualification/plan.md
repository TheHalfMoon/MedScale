# Plan: Spec 020 FHIR / Interchange Qualification

1. Contracts: `FhirSupportMatrix`, `ResourceSupportStatus`, axis levels, loss-aware export types reusing `LossClass`.
2. Canonical READY_BASE matrix factory reflecting Spec 003/004 extractors (Patient/Observation/Condition only).
3. Doctor axis + matrix embed; CLI text/JSON surfacing.
4. Facade: `GetFhirSupportMatrix`, `ExportFhirLossAware`.
5. Tests: matrix honesty + loss-aware export stub.
6. Spec package + evidence + BUILD_QUEUE + SPECKIT_MASTER_ROADMAP_V2 (021 = Q07 READY).
7. Gates: fmt, clippy `-D warnings`, workspace test with `CARGO_TARGET_DIR=D:\medscale-target` when needed.

## Architecture

- Matrix is a typed contract, not a live CapabilityStatement server.
- Validator attachments remain EvaluationRecord evidence only.
- Export stub is synthetic/local: emits subset JSON + explicit field loss inventory; never claims lossless FHIR round-trip.
- Clinical interpretation axis stays Unsupported for all resources in READY_BASE.
