# Plan: Spec 011

In-memory deterministic lexical ranker in `medscale-core`; contracts for RetrievalHit/EvidenceSet; facade `RetrieveLexical`. No new crate.

## Decisions
| ID | Decision |
|---|---|
| D1 | Substring/token overlap scorer — no Tantivy in MVP |
| D2 | Results → EvaluationRecord only |
| D3 | Corpus = synthetic fixtures registered in-process |
