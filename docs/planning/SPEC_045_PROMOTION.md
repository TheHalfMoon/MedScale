# Spec 045 promotion — synthetic lexical 10k corpus (Q10/Q05 residual)

**Classification:** `EXISTING_Q10_Q05_RESIDUAL_ELIGIBLE_FOR_PROMOTION`  
**Not:** deferred advanced product.

## Authority

Spec 025 shipped 5-doc builtin READY_BASE. Spec 042 honesty explicitly left lexical at builtin scale. BUILD_QUEUE listed lexical-10k as optional Trusted V1 residual.

## Scope

- Procedural deterministic `synthetic-lexical-scale@10k.0.0` (10_000 docs); no fixture dump.
- Wire into retrieval + perf harness via `MEDSCALE_027_LEXICAL_SCALE=10000` / delivery-plan scale.
- Doctor `scale_corpus_generator_present=true`.
- Keep `clinical_quality_claimed=false`, `budgets_claimed_met=false`, `RELEASE_READY=false`.

## Numbering

Spec **045** is this residual. Advanced deferred product renumbered **046+**.
