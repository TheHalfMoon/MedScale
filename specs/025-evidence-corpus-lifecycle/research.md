# Research — Spec 025 Evidence Corpus Lifecycle

## Inputs
- TRUSTED_V1_DELIVERY_PLAN Q10 (P1; deps Q02/Q06 closed)
- Spec 011 lexical retrieval + EvaluationRecord evidence-only path
- Pack v0 admission pattern (Spec 008) for manifest/digest/rights
- WHOLE_PRODUCT_REVIEW: three-document synthetic corpus is PARTIAL

## Decisions
1. **MedScale-owned synthetic corpus only** — no licensed corpora; rights fail-closed to `synthetic_owned`.
2. **Pack-like local corpus manifest** — not a model Pack; same digest/rights discipline.
3. **Source identity ≠ content digest** — `corpus_id@version` vs SHA-256 over sorted doc digests.
4. **Retracted excluded by default** — `include_retracted` opt-in marks hits.
5. **Markers are evidence metadata** — never ClinicalAssertion / authority.
6. **Doctor honesty** — READY_BASE without RELEASE_READY or clinical quality.

## Alternatives rejected
- Vector/embeddings (deferred 026+)
- OpenMed corpus copy (not authorized for this unit; synthetic-owned only)
- Treating relevance rank as clinical quality
