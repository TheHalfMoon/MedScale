# Contract: Parity Corpus Registry

**Spec**: 007  
**Status**: Canonical for research design  
**Related**: [data-model.md](../data-model.md) `ParityCorpusDescriptor`, OPENMED_PARITY_SURPASS_MATRIX_V2

## Purpose

Define synthetic-first corpora and metric suites that Spec 008+ will execute. Spec 007 owns **design and registry**; measurement may complete later.

## Required corpus families (minimum)

| corpus_id | capability_ids | Languages | Notes |
|---|---|---|---|
| `om-ner-synthetic-en-v0` | `clinical-ner` | en | Span F1 / P / R; Proposal-only outputs later |
| `om-pii-synthetic-multi-v0` | `pii-detection` | en, ar | High-risk PHI recall; multilingual traps |
| `om-deid-synthetic-v0` | `de-identification` | en | Span fidelity; transformation record; no hidden egress |
| `om-multilingual-smoke-v0` | `multilingual-medical-nlp` | multi | Supported-language registry honesty |
| `om-saudi-arabic-design-v0` | `saudi-arabic-pii-clinical-ner` | ar, ar-SA, en+ar | Design + seed traps; measure in 008 |
| `om-terminology-contract-v0` | `terminology-grounding` | n/a | Fixtures for candidate/rank/abstain—**no** licensed tables |
| `om-unicode-offset-handoff-v0` | `unicode-offset-semantics` | ar, en | Adversarial offset cases for 002/010 |

## Metric rules

- REQUIRED_PARITY rows must list `required_for_parity: true` metrics.
- Anti-metrics MUST include `raw_model_count` unless a dated product exception is recorded.
- Latency/RSS fields are placeholders in 007; Spec 008 fills hardware-bound measurements.

## Anti-scope

- No REAL_PHI corpora admitted under Spec 007.
- No OpenMed model weights required to close Spec 007 research.
- Corpus “ready” ≠ SURPASS claim.

## Evidence path

`evidence/007-openmed-capability-absorption-parity/CORPUS_REGISTRY.md` (+ optional JSON mirror under `docs/matrices/`).
