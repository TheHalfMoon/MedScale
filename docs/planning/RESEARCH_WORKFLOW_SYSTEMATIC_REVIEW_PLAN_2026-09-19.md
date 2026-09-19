# Research Workflow + Systematic Review Plan — 2026-09-19

**Status:** `PLANNING_CANDIDATE_ONLY`  
**Purpose:** make MedScale a serious end-to-end research workspace rather than only an evidence search engine or analytics dashboard.

## 1. Research lifecycle

Target lifecycle:

```text
Question
 -> Protocol
 -> Search strategy
 -> Literature corpus
 -> Screening
 -> Extraction
 -> Bias / quality appraisal
 -> Synthesis / meta-analysis
 -> Dataset / analysis
 -> Figures / tables
 -> Interpretation
 -> Manuscript / report
 -> Reproducible Research Pack
```

Every stage is a Project artifact with provenance and revision history.

## 2. Protocol artifact

A research protocol should capture:

- research question;
- PICO/PICOTS where relevant;
- objectives/hypotheses;
- inclusion/exclusion criteria;
- planned sources;
- date range/language restrictions;
- outcomes;
- analysis plan;
- screening process;
- bias/quality methods;
- registration/reference ID when external;
- amendments and reasons.

Protocol changes after data/results are visible must remain explicit amendments, not silent edits.

## 3. Literature library

Support:
- DOI;
- PMID/PMCID;
- BibTeX;
- RIS;
- CSL JSON;
- PDF/document attachment;
- duplicate detection;
- canonical citation identity;
- tags/collections;
- notes;
- source/rights metadata;
- retraction/correction state.

Zotero interoperability should prioritize import/export and stable citation formats before any code reuse.

## 4. Search strategy artifact

A systematic-review search should preserve:

- database/source;
- exact query;
- filters;
- date;
- retrieved count;
- source API/version where relevant;
- deduplication method;
- inclusion in review snapshot.

AI may propose a query; the executed query remains explicit and reviewable.

## 5. Screening workflow

Required stages:
- title/abstract;
- full text;
- included/excluded;
- exclusion reason;
- unresolved;
- duplicate.

Team mode should support:
- independent dual review;
- blinded decisions when configured;
- conflict queue;
- adjudication;
- reviewer identity;
- revision history.

No AI screening decision silently excludes a study.

AI may rank or propose `INCLUDE`, `EXCLUDE`, `UNCERTAIN`, but human review rules are explicit.

## 6. PRISMA-style flow

Generate a reproducible review-flow artifact from actual screening events:

- records identified;
- duplicates removed;
- screened;
- excluded;
- full texts assessed;
- exclusions by reason;
- included.

Do not fabricate flow counts from a final bibliography.

## 7. Data extraction

Support structured extraction tables for:
- study design;
- population;
- sample size;
- intervention/comparator;
- outcomes;
- follow-up;
- effect estimates;
- adverse events;
- study limitations;
- funding/conflicts where available.

Every extracted cell can link to a paper page/span when possible.

AI extraction remains a proposal with source linkage and review state.

## 8. Risk-of-bias / quality appraisal

Provide versioned appraisal templates, not one universal score.

Candidate frameworks may include:
- RoB 2;
- ROBINS-I;
- QUADAS-2;
- Newcastle-Ottawa or other domain methods where appropriate;
- guideline/review-specific checklists.

Framework rights/version must be checked before bundling.

Appraisal answers link to source evidence and reviewer identity.

## 9. Evidence certainty

MedScale can assist GRADE-like workflows but must not claim formal GRADE equivalence without validation.

Keep:
- risk of bias;
- inconsistency;
- indirectness;
- imprecision;
- publication bias;
- upgrade/downgrade rationale;
- reviewer/adjudication state.

## 10. Meta-analysis

Analytics/R Workspace should support reproducible:
- effect-size calculation;
- fixed/random effects as appropriate;
- confidence intervals;
- heterogeneity;
- subgroup analysis;
- sensitivity analysis;
- forest plots;
- funnel plots where appropriate;
- meta-regression only with adequate design/data.

Every synthesis binds exact extracted data and method/runtime.

Statistical methods need independent oracle fixtures.

## 11. Living review

A review may be refreshed against new evidence.

Refresh produces:
- new search receipt;
- new corpus snapshot;
- new candidate records;
- screening delta;
- synthesis delta;
- changed conclusion flags.

Old review versions remain reproducible.

No automatic new evidence silently changes a published conclusion.

## 12. Research Canvas integration

Canvas blocks may reference:
- protocol;
- paper;
- evidence claim;
- extraction table;
- risk-of-bias item;
- dataset;
- analysis;
- figure;
- hypothesis;
- decision;
- manuscript section.

All resolve to stable artifact IDs.

## 13. Manuscript / report

Support:
- structured sections;
- linked citations;
- live references to figures/tables;
- artifact provenance;
- change history;
- export to Markdown/DOCX/PDF where qualified;
- machine-readable research manifest.

A report should be reproducible from exact artifact versions, not only stored as a final PDF.

## 14. Citation workflow

Prefer open standards:
- CSL JSON;
- BibTeX;
- RIS;
- DOI/PMID identities.

Allow Zotero-compatible import/export and citation-key preservation where possible.

Do not build citation formatting from scratch if a qualified standards implementation can be reused safely.

## 15. Trial registry integration

ClinicalTrials.gov and other admitted registries can serve:
- literature/research discovery;
- protocol context;
- trial matching;
- study status tracking.

Registry entries are external sources with timestamp/version provenance.

## 16. Research integrity

Explicitly support:
- preregistered versus post-hoc analyses;
- protocol amendments;
- exclusion reasons;
- data transformations;
- source/version lineage;
- analysis failures;
- negative/null results;
- model-assisted contribution logs.

Do not hide failed analyses or post-hoc changes when they matter to interpretation.

## 17. Team/reviewer roles

Research collaboration may distinguish:
- investigator;
- screener;
- adjudicator;
- statistician;
- data curator;
- reviewer;
- observer.

These are workflow roles, not blanket access to patient/clinical data.

## 18. Candidate source/reference tools

Reference only unless exact donor qualification permits more:
- Zotero for citation/library interoperability patterns;
- ASReview for AI-assisted screening concepts;
- mature systematic-review/statistical packages in R/Python for method validation/external-tool integration.

MedScale should own the artifact/provenance model.

## 19. Research OS ownership

- 076 — team screening/review/adjudication;
- 080 — literature/registry acquisition and search receipts;
- 082 — extraction tables, statistical synthesis and reproducible figures;
- 083 — library, evidence ledger, Research Canvas, protocol/manuscript artifacts;
- 085 — isolated Python jobs where needed;
- 086 — R/meta-analysis workspace;
- 089 — method/evidence/review-template Packs;
- 092 — end-to-end reproducibility and human research workflow validation.

No candidate is promoted by this plan.

## 20. Qualification

Before calling the systematic-review workflow qualified, prove an end-to-end synthetic/rights-cleared review:

```text
protocol
 -> frozen search
 -> source retrieval
 -> dedup
 -> dual screening
 -> conflict adjudication
 -> extraction
 -> risk-of-bias
 -> synthesis
 -> figure
 -> manuscript section
 -> export
 -> restart/replay
```

The replay must preserve counts, decisions, inputs and analysis identity.
