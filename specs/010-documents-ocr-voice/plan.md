# Plan: Spec 010

Branch `spec/010-documents-ocr-voice`. Contracts + core document intake module + facade capabilities; FixtureStubs only; reuse 008 worker_policy and storage claim/quarantine.

## Decisions
| ID | Decision |
|---|---|
| D1 | No native OCR/ASR DEPENDENCY in this unit |
| D2 | MIME deny-by-default except synthetic allowlist |
| D3 | Workers = policy proofs + in-process stubs (P1 placement declared) |
| D4 | Output = DerivedSourceArtifact + Proposal only |
