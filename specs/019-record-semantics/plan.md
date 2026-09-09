# Plan: Spec 019 Record Semantics

1. Contract extensions: MedicalTime, MissingnessKind, AmendmentRecord, identity unresolved types.
2. Core: `amend_assertion` + AmendAssertion facade; durable StoredObject::Amendment; precision-aware timeline sort.
3. Tests: `record_semantics_019` + contracts unit tests.
4. Spec package + evidence + BUILD_QUEUE + SPECKIT_MASTER_ROADMAP_V2.
5. Gates: fmt, clippy -D warnings, workspace test.

## Architecture

- Library helpers in `medscale-contracts` for time/missingness/identity.
- One mutating facade capability `AmendAssertion`.
- Persistence reuses Spec 016 authority_objects JSON rows (`object_class=amendment`).
