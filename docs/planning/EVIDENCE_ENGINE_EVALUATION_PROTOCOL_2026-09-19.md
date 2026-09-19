# Evidence Engine Evaluation Protocol — 2026-09-19

**Status:** `PLANNING_CANDIDATE_ONLY`  
**Purpose:** define how MedScale must evaluate evidence-grounded medical answers, literature retrieval, evidence quality and citation behavior before making parity, superiority or clinical-quality claims.

## 1. Evaluation principle

An evidence answer is a chain:

```text
question
 -> interpretation/decomposition
 -> retrieval
 -> source selection
 -> claim generation
 -> citation assignment
 -> evidence appraisal
 -> applicability
 -> contradiction/uncertainty
 -> final synthesis
```

Evaluate each stage separately.

A fluent answer with real citations can still be wrong.

## 2. Benchmark families

Build versioned benchmark Packs covering:

- diagnosis;
- treatment;
- screening;
- prevention;
- prognosis;
- adverse effects;
- drug interactions;
- pregnancy/pediatrics;
- renal/hepatic adjustment;
- conflicting guidelines;
- rare disease;
- multimorbidity;
- rapidly changing evidence;
- insufficient evidence;
- retracted/corrected literature;
- jurisdiction-specific questions;
- patient-context questions;
- questions that should trigger abstention/clarification.

Use only sources/data permitted by benchmark rights.

## 3. Question representation

Where appropriate, preserve:
- original clinician question;
- structured PICO/PICOTS elements;
- patient-context fields used;
- jurisdiction;
- timeframe;
- desired evidence type;
- ambiguity/clarification state.

Do not silently infer missing patient details.

## 4. Retrieval metrics

Measure:
- recall@k against governed relevant-source sets;
- precision@k;
- nDCG or appropriate ranking measure;
- guideline/source-family coverage;
- source diversity;
- recency;
- retraction exclusion;
- rights/access-policy compliance;
- retrieval latency/resource use.

Report lexical, graph, vector and reranking contributions separately where possible.

## 5. Citation identity

Citation existence verification should check available:
- DOI;
- PMID/PMCID;
- title;
- authors;
- journal;
- publication year/date;
- version/correction/retraction state.

Possible outcomes:

```text
VERIFIED_IDENTITY
PARTIAL_METADATA_MATCH
AMBIGUOUS
NOT_FOUND
RETRACTED
CORRECTED
SOURCE_UNAVAILABLE
```

## 6. Claim-citation support

For every material claim with a citation, determine whether the cited source:

```text
SUPPORTS
PARTIALLY_SUPPORTS
CONTRADICTS
DOES_NOT_ESTABLISH
UNRESOLVED
```

Measure:
- citation support precision;
- unsupported cited-claim rate;
- contradiction miss rate;
- citation placement accuracy.

The verifier itself must be benchmarked; model-assisted verification is not ground truth.

## 7. Unsupported claims

Classify uncited/material synthesis claims:

- common contextual wording;
- inference clearly labeled;
- recommendation;
- numeric effect;
- safety statement;
- contraindication;
- dosing;
- diagnostic threshold;
- epidemiologic prevalence;
- policy/guideline statement.

High-consequence categories require stronger source linkage.

## 8. Evidence-quality evaluation

Compare MedScale evidence profiles against expert-curated references where available.

Dimensions:
- study design;
- risk of bias;
- consistency;
- directness;
- precision;
- magnitude/effect uncertainty;
- population applicability;
- intervention/comparator match;
- outcome match;
- recency;
- guideline/jurisdiction relevance;
- contradiction.

Do not claim formal GRADE equivalence unless the exact implementation has been validated for that claim.

## 9. Applicability

For patient-context questions, evaluate:
- age/sex/pregnancy relevance when supplied;
- disease severity/stage;
- comorbidities;
- renal/hepatic status;
- intervention availability;
- jurisdiction/formulary/guideline;
- trial inclusion/exclusion similarity.

The output should distinguish:
- evidence strength;
- patient applicability;
- action authority.

These are not one score.

## 10. Guideline conflict

Benchmark cases where:
- US and non-US guidelines differ;
- guideline versions changed;
- specialty societies disagree;
- evidence changed after a guideline publication.

Expected behavior:
- identify the conflict;
- preserve organization/version/jurisdiction;
- avoid collapsing disagreement into one hidden recommendation.

## 11. Retraction/correction

Include:
- retracted papers;
- corrected papers;
- expressions of concern;
- superseded guidelines.

Measure whether the system:
- recognizes status;
- avoids relying on invalid evidence;
- explains prior saved-answer snapshot versus current state.

## 12. Deep Consult / extended synthesis

A longer-running evidence workflow should:
1. expose a research plan;
2. record search/retrieval queries;
3. show source set;
4. identify gaps;
5. synthesize by subquestion;
6. record contradictions;
7. produce an evidence profile;
8. bind the final report to an immutable evidence snapshot.

Evaluate deep synthesis separately from fast Q&A.

## 13. Clinical-trial matching

Benchmark:
- exact eligibility criteria extraction;
- patient-to-criterion mapping;
- unknown/unavailable criteria;
- temporal eligibility;
- site/location status;
- study status/freshness;
- false eligibility rate.

The output is `CANDIDATE_MATCH`, never enrollment eligibility authority.

## 14. Patient education

For clinician-reviewable patient handouts, evaluate:
- factual consistency with approved clinician plan;
- source linkage;
- reading level;
- language quality;
- prohibited unsupported personalization;
- warning/follow-up preservation;
- omission of critical instructions.

## 15. Voice evidence mode

Evaluate:
- speech recognition of medical questions;
- interruption;
- answer latency;
- spoken answer concision;
- preservation of full citations in visible written form;
- no degradation in evidence standards because output is spoken.

## 16. Failure behavior

Test:
- no relevant evidence;
- only weak evidence;
- conflicting sources;
- source behind unavailable rights boundary;
- network unavailable;
- local Evidence Pack stale;
- citation metadata conflict;
- retrieval worker crash;
- model unavailable.

Expected behavior is explicit degradation/abstention, not fabricated support.

## 17. Rights/access tests

The same query under different source permissions must not leak:
- licensed full text;
- restricted snippets beyond allowed use;
- institution-only data;
- another user's imported paper.

Caches/indexes include permission scope.

## 18. Performance

Record:
- retrieval latency;
- synthesis latency;
- source count;
- index size;
- CPU/GPU/RAM;
- offline Pack behavior;
- cold/warm cache separately.

Performance never justifies omitting rights/provenance checks.

## 19. Human expert review

When authorized:
- use qualified clinicians/researchers;
- predefined rubric;
- blinded system/version where feasible;
- adjudication;
- inter-rater agreement;
- question-domain matching.

Assess separately:
- factuality;
- completeness;
- evidence support;
- uncertainty;
- usefulness;
- unsafe overstatement.

## 20. Comparative evaluation

Abridge/OpenEvidence/other product comparisons require:
- same dated use case;
- same question set where terms permit;
- comparable source-access assumptions;
- explicit differences in available licensed content;
- no automated scraping that violates terms;
- no “winner” inference from unmatched conditions.

## 21. Evidence packet

Each evaluation records:
- code SHA;
- Evidence Pack revision;
- model/runtime Packs;
- retrieval/index versions;
- benchmark revision;
- hardware/environment;
- source-access scope;
- metrics;
- failures;
- limitations;
- adjudication protocol;
- exact claim supported by the evaluation.

No single aggregate score is sufficient for an evidence-engine release decision.
