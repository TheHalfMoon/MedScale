# Medical Evidence Source and Rights Strategy — 2026-09-19

**Status:** `PLANNING_CANDIDATE_ONLY`  
**Purpose:** define a rights-aware source strategy for the MedScale Evidence Copilot and Research Packs without assuming that access to an article, guideline, code set or API grants redistribution rights.

## 1. Evidence-source principle

MedScale needs two separate questions for every evidence source:

1. **Can the user/system retrieve or access it?**
2. **May MedScale store, index, transform, redistribute or package the content?**

A technical fetch is not a rights decision.

The local-first product should work well with open and user-owned evidence while supporting institution-authorized licensed content through explicit adapters.

## 2. Source classes

### Class A — Open metadata

Examples:
- PubMed metadata;
- Crossref metadata;
- OpenAlex metadata;
- DOI/PMID identifiers;
- journal/article bibliographic metadata where terms permit.

Typical use:
- citation identity;
- author/title/journal/year;
- publication links;
- deduplication;
- citation existence checks.

### Class B — Open full text

Examples:
- PMC Open Access subset;
- publisher or repository content carrying a license that permits the intended use;
- open guidelines/public reports under suitable terms.

Typical use:
- local full-text indexing;
- evidence spans;
- Research Packs when redistribution is permitted.

The exact item license remains part of the artifact metadata.

### Class C — Publicly accessible but restricted/licensed full text

Examples may include publisher pages, guideline content or institution-entitled material.

Typical use:
- metadata/link/reference;
- user/institution-authorized access through a connector;
- local ephemeral or persistent storage only when terms permit.

MedScale must not repackage or redistribute this content merely because a user can view it.

### Class D — Institution-owned sources

Examples:
- local clinical guidelines;
- formulary;
- protocols;
- internal policy;
- library-licensed collections.

Typical use:
- institution-scoped local evidence Packs;
- explicit access control;
- organization-specific retention and redistribution policy.

### Class E — User-provided literature

PDFs/documents supplied by the user.

Typical use:
- local indexing and analysis subject to the user's rights and MedScale policy;
- provenance records the import source and user-supplied status;
- no assumption that the file can be redistributed to teammates or packaged publicly.

## 3. Canonical source identity

Each literature/guideline artifact should preserve available identifiers:

```text
doi
pmid
pmcid
publisher identifier
journal/source
title
authors
publication date
version/correction state
retrieval URL
retrieval time
content digest
access class
license/rights metadata
retraction/correction state
```

Do not use URL alone as durable identity.

## 4. Acquisition strategy

Preferred order:

1. project-local already admitted artifacts;
2. open metadata APIs;
3. open full-text repositories;
4. user-authorized direct source;
5. institution-authorized licensed adapter;
6. browser capture only when the source terms and active policy permit it.

All external acquisition passes through the existing Network Broker / future Governed Browse authority.

## 5. Candidate source adapters

### PubMed / NCBI
Use for biomedical bibliographic discovery and identifiers.

Candidate operations:
- search;
- fetch metadata;
- resolve PMID/PMCID;
- related/cited-by metadata where available.

### PubMed Central
Use only the appropriate open/full-text subsets according to article-level rights.

### Crossref
Use for DOI metadata resolution and citation identity.

### OpenAlex
Use as an optional metadata/discovery graph, not as a clinical authority.

### ClinicalTrials.gov
Use as a candidate structured trial registry source for local clinical-trial matching and research discovery.

Trial matching remains a proposal:
- criteria may be incomplete or stale;
- eligibility requires authoritative study-site review;
- no enrollment action is automatic.

### Retraction/correction sources
Use publisher/Crossref/PubMed or other admitted authoritative metadata to track correction/retraction state. One source should not silently override another when they conflict.

## 6. Publisher and guideline partnerships

Current commercial products demonstrate the product value of deeply licensed evidence libraries. OpenEvidence publicly highlights content partnerships with NEJM, JAMA Network, NCCN, Nature and Cochrane, while Abridge has announced NEJM and JAMA partnerships for context-aware clinical evidence.

References:
- https://takehome.openevidence.com/
- https://www.abridge.com/press-release/abridge-integrates-nejm-jama

MedScale must not assume equivalent rights.

Product architecture should support:

```text
Open Evidence Pack
Institution-Licensed Evidence Pack
User-Provided Evidence Pack
MedScale Metadata-Only Connector
```

so commercial/licensed content can be added later without redesigning the evidence engine.

## 7. Guideline model

A guideline is not just a PDF.

Preserve:
- issuing organization;
- title;
- version/date;
- jurisdiction;
- intended population;
- recommendation identifier/section;
- recommendation strength/certainty when explicitly published;
- superseded status;
- source text/span;
- rights/access class.

MedScale-generated applicability is separate from the publisher's stated recommendation strength.

## 8. Evidence quality model

For each synthesized claim, record separate dimensions where evidence supports evaluation:

- study design;
- risk of bias;
- consistency;
- directness;
- precision;
- population applicability;
- intervention/comparator applicability;
- outcome applicability;
- recency;
- jurisdiction/guideline applicability;
- retraction/correction state;
- contradiction state.

Do not make one letter/number the only representation of evidence strength.

## 9. Claim-support verification

Citation verification has two stages.

### Stage 1 — Citation existence
Verify the source exists and the bibliographic identity is consistent.

### Stage 2 — Claim support
Verify whether the cited source materially supports, contradicts or does not establish the specific generated claim.

Possible states:

```text
SUPPORTS
PARTIALLY_SUPPORTS
CONTRADICTS
DOES_NOT_ESTABLISH
UNRESOLVED
SOURCE_UNAVAILABLE
```

This verifier can itself be model-assisted, but its result remains evidence/inference with its own run identity and benchmark.

## 10. Evidence snapshotting

Saved answers and research artifacts bind an evidence snapshot:

- exact retrieved sources;
- exact versions/digests where obtainable;
- query/retrieval plan;
- retrieval date;
- source access class;
- ranking/reranking version;
- synthesis model/run;
- evidence-profile result.

When sources update, create a new evaluation rather than rewriting the old answer.

## 11. Evidence Pack rules

A Research/Evidence Pack must declare:

- source list/manifest;
- rights and redistribution state;
- exact versions/digests;
- index/build recipe;
- parser/chunker versions;
- terminology dependencies;
- jurisdiction/language;
- update schedule;
- retraction/correction refresh behavior;
- expiry/staleness policy;
- license/NOTICE artifacts;
- allowed install scope.

A Pack may be local-only and non-redistributable if the institution/user has rights but MedScale does not.

## 12. Clinical-trial matching

Abridge's 2026 product direction includes clinical-trial matching as part of its patient-centered intelligence platform.

MedScale target:

```text
patient context
 -> explicit eligible fields
 -> de-identification/egress policy
 -> local registry Pack or governed ClinicalTrials.gov query
 -> candidate trials
 -> criteria-to-patient evidence mapping
 -> unresolved eligibility items
 -> clinician/research-coordinator review
```

No model may claim a patient is eligible without mapping every material criterion or marking it unresolved.

## 13. Education/CME boundary

OpenEvidence demonstrates that evidence lookup can integrate professional education and credits.

MedScale may implement:
- learning history;
- saved learning activities;
- quizzes/reflections;
- evidence reading log;
- exportable education record.

Actual CME/MOC/CE credit issuance remains an external accreditation gate.

## 14. Source-quality monitoring

Track:
- fetch/access failures;
- metadata conflicts;
- stale sources;
- retractions/corrections;
- Pack update lag;
- unsupported claims;
- evidence coverage;
- source diversity;
- jurisdiction mismatch.

Do not use source count as a proxy for evidence quality.

## 15. Qualification target

Before claiming the Evidence Copilot is production-ready, prove:

- citation identity accuracy;
- claim-support accuracy;
- retrieval recall on a governed benchmark;
- contradiction handling;
- retraction/correction handling;
- rights/access enforcement;
- source deletion/staleness behavior;
- local/offline Pack use;
- no unauthorized source redistribution;
- clinician/researcher usability.

Detailed metric design lives in `EVIDENCE_ENGINE_EVALUATION_PROTOCOL_2026-09-19.md`.
