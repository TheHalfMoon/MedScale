# Research — Spec 062

## Existing trusted inputs
- `SubjectBriefV1`: LLM-free identity/vitals/conditions plus coverage summary.
- `SubjectCoverageV1`: closed concept slots preserving all coverage statuses.
- `LexicalRetrieveResult`: evidence-only retrieval result with corpus/source identity and explicit `relevance_is_not_authority`.
- `RetrievalHit`: score/snippet plus retracted/freshness/conflict/corpus-version metadata; relevance only.

## Population consequence
There is no canonically qualified clinical risk-score contract. Population UI may aggregate trusted presentation state, but it must not silently invent clinical risk. Evidence-review attention and coverage gaps are directly derivable and therefore safe first-class signals. Present condition distribution counts only Present facts.

## Assistant consequence
Spec 062 does not authorize provider-backed or autonomous clinical reasoning. The assistant is an evidence navigator: explain trusted display state, surface retrieval metadata, explain unknown/conflicting evidence, and point toward review. Retrieval text never becomes an assertion.

## Design consequence
Founder-approved Population Insights visual intent is preserved with metrics, distribution rows, cohort exploration, contextual assistant, and recommendation panels. Risk and trend areas remain explicit non-claims rather than fake analytics until trustworthy contracts exist.
