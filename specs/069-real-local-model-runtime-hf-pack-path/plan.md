# Plan — Spec 069

## Slice A — portable runtime admission
- Pin a Rust-1.88-compatible portable ONNX engine.
- Keep model execution inside `medscale-pack`; Desktop and CLI must not own runtime dependencies.
- Return evidence/proposal output only.

## Slice B — Core authority integration
- Add a typed local-pack evaluation request/response.
- Require Core lease/session enforcement and synthetic-only use.
- Re-admit and digest-check pack bytes on every invocation.
- Cache only prepared model plans keyed by signed content digest; never cache user input.

## Slice C — signed model provenance
- Add signed model-source metadata to Pack content.
- Bind repository, exact revision, original file SHA-256, transform, runtime, task, license, and rights URI.
- Keep online acquisition denied behind the existing Network Broker gate.

## Slice D — real Hugging Face qualification
- Use a pinned MIT Hugging Face ONNX NER model as external qualification input only.
- Do not vendor model weights into Git.
- Apply deterministic graph-wide static-shape specialization for the portable runtime.
- Prove semantic output for PERSON and LOCATION labels.

## Slice E — qualification and product truth
- Separate correctness evidence from release-profile performance evidence.
- Update Models/Evidence product truth without promoting the qualification model as a clinical production model.
- Run Rust 1.88 all-targets, Clippy, focused tests, OpenCodeReview, exact-head CI, merge, and post-main verification.
