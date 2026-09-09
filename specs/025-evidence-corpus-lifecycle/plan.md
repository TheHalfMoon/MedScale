# Plan: Spec 025 Evidence Corpus Lifecycle

1. Spec Kit package (specify/clarify/plan/research/ADR/tasks/checklist/analyze/converge).
2. Contracts: `EvidenceCorpusManifest`, document lifecycle, hit metadata, doctor axis.
3. Admit Pack-like `corpus.manifest.json` (hex digests, synthetic-owned rights fail-closed).
4. Embed/admit default `synthetic-lexical@1.0.0` fixture; replace hardcoded retrieval corpus.
5. Lexical retrieve with retraction filter + freshness/conflict markers on hits/EvaluationRecord.
6. Doctor `evidence_corpus` READY_BASE honesty.
7. Tests + evidence SUMMARY/LIMITATIONS.
8. BUILD_QUEUE / roadmap / START_HERE → 025 CLOSED; deferred **026+**.
9. Gates: fmt, clippy `-D warnings`, `cargo test --workspace --locked` (`CARGO_TARGET_DIR=D:\medscale-target`).

## Architecture

```text
corpus.manifest.json  --admit-->  EvidenceCorpusManifest
                                      |
                                      v
CoreFacade.RetrieveLexical --> lexical rank/filter --> EvaluationRecord (evidence_only)
                                      |
                                      v
DoctorReport.evidence_corpus (READY_BASE; clinical_quality=false; release_ready=false)
```

Source identity = `{corpus_id}@{version}` (never content digest).
