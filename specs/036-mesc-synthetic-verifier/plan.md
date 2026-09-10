# Plan — Spec 036

1. Extend `medscale-contracts` MESC types (manifest, verify request/report, doctor flag).
2. Implement `medscale-pack::verify_mesc_release_dir` + `MescEpochStore`.
3. Wire `Capability::MescArtifactVerify` through envelopes + CoreFacade.
4. Add synthetic fixtures + `mesc_verifier_036` tests; keep admit gate-blocked.
5. Update MESC acceptance doc, EXTERNAL_GATES note, BUILD_QUEUE row; evidence SUMMARY/LIMITATIONS.
