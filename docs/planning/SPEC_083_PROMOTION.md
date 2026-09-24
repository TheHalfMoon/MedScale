# Spec 083 Promotion — Knowledge + Research Canvas

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`
**Promotion date:** 2026-09-24
**Canonical base:** `__BASE__`
**Target branch:** `spec/083-knowledge-canvas`

## Authority

The founder's standing continuation directive requires promoting the next
dependency-ready Research OS unit after each closure without routine
approval. `IMPLEMENTATION_AUTHORITY.md` remains active.

Live verification at promotion time (`gh pr view` / `gh run view`):

- Spec 082 is `CLOSED_CANONICAL`: final head `7cc58ec` passed exact-head run
  `35928650470` (6/6); PR #142 merged as `3ea6610`; post-merge main run
  `35969739867` passed 6/6. __CLOSURE__
- Specs 074, 075, 077 and 079 are `CLOSED_CANONICAL` (see `BUILD_QUEUE.md`).

Dependency proof: `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md` and
`RESEARCH_OS_EXECUTION_ROADMAP.md` number Knowledge + Research Canvas
**083**, with hard dependency **074 + 075 + 077 + 079**, all closed. The
roadmap's integration gates (web evidence after 080, audio timestamps after
081, analytics derivatives after 082) are all closed. Decision register
entries Q28 (embedded lexical search first), Q29 (no vector database until a
benchmark shows value), Q30 (index manifests bind exact source revisions;
changed sources are marked stale or tombstoned) and Q31 (filter before
disclosure and before cache reuse) apply.

Review policy: `FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## Authorized scope

- Contracts (`medscale-contracts/src/knowledge.rs`): `IndexManifest`,
  `IndexSourceRef`, `IndexChunk` (the `ChunkRef` of the contract list),
  `EvidenceSpanRef`, `SpanLocator`, `IndexStatus`, `Freshness`,
  `RetrievalRequest`, `RetrievalPlan`, `RetrievalReceipt`,
  `RetrievalResult`, `CanvasRevision` (the `Canvas`), `CanvasNode`,
  `CanvasEdge`, `CanvasOp`, `CanvasView`.
- A per-Project lexical index (`lexical-v1`, deterministic integer
  scoring) over the latest revision of each Spec 075 snapshot lineage,
  Spec 081 transcript lineage, every Spec 080 web evidence excerpt and
  every Spec 082 derived table of the Project. Text cells, segments and
  excerpts are split into windows of at most 1,000 characters.
- Live freshness (`current`, `superseded`, `tombstoned`) on every request;
  default retrieval serves current spans only; stale spans are listed only
  on request and tombstoned spans never carry text; no retrieval cache.
- A `RetrievalReceipt` for every request that reaches a Project, including
  refusals, with `insufficient_evidence` as a first-class outcome.
- Research Canvas: immutable contiguous revisions of notes, evidence
  references and typed links (`supports`, `contradicts`, `relates`,
  `questions`), optimistic concurrency, and a live view with unsupported
  notes, contradictions, superseded and missing evidence.
- Storage schema v11 -> v12 (additive), backup/restore, consistency checks.
- CLI `medscale knowledge ...` (human and JSON) and a Desktop Knowledge
  route over Core.

## Explicitly not authorized

- Vector or embedding retrieval, and any new dependency (Q28/Q29).
- Model-generated answers, summaries or rankings.
- A literature library with citation metadata, PDF or OCR import (no
  admitted parser); captured web evidence is the library input in this
  foundation.
- Project Graph expansion in retrieval plans (a later spec may add it behind
  the same plan contract).
- Graphical canvas layout, sharing or collaboration on canvases.
- Real PHI; remote egress of results.

## Frozen acceptance requirements

1. An index version pins every source revision it read (kind, object,
   content digest, lineage) and a digest of its chunk list; rebuilding
   unchanged sources gives an identical chunk digest.
2. Search returns spans from snapshot cells, transcript segments (with time
   spans), web evidence excerpts and derived-table cells, each resolving to
   its exact revision and character range, with text re-read from the live
   source.
3. A superseded source is reported as such and not served by default; an
   archived source is tombstoned and its text is never shown; the index
   status counts superseded, tombstoned and unindexed sources.
4. No match is `insufficient_evidence`; refused requests (no index, empty,
   too long, too many terms, bad hit limit) leave receipts.
5. A Canvas stores references, not copies; evidence must be a current span
   of the Canvas's Project; stale writers are refused; the live view reports
   unsupported notes, contradictions, superseded and missing evidence.
6. No cross-Project or cross-scope search, citation or read.
7. Index versions, receipts and canvases survive reopen and backup/restore;
   tampered rows are refused.
8. CLI and Desktop reach knowledge only through Core; no new dependency and
   no network code.
9. Exact-head and post-main CI pass.

Recorded residuals: lexical only (no semantic matching, stemming or
synonyms); no literature library; no graph expansion; retrieval performance
at scale is unmeasured beyond the chunk bound.

## Completion rule

`CLOSED_CANONICAL` only after merge on a green exact head and recorded
post-main verification. Closure of 083 does not authorize 084.
