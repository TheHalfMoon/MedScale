# Spec 082 Promotion — Analytics Gate

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`
**Promotion date:** 2026-09-23
**Canonical base:** `44f71e606ac74b632d417f2aa86202e79f71f9ea`
**Target branch:** `spec/082-analytics-gate`

## Authority

The founder's standing continuation directive requires promoting the next
dependency-ready Research OS unit after each closure without routine
approval. `IMPLEMENTATION_AUTHORITY.md` remains active.

Live verification at promotion time (2026-09-23, `gh pr view` / `gh run view`):

- Spec 081 is `CLOSED_CANONICAL`: final head `be0ad73` passed exact-head run
  `35899434716` (6/6); PR #141 merged as `876b9fe`; post-merge main run
  `35906389709` passed 6/6. Closure PR #143 (exact-head run
  `35915171239`, 6/6 on `1f5aabd`) merged as `44f71e6`.
- Specs 074, 075 and 079 are `CLOSED_CANONICAL` (see `BUILD_QUEUE.md`).

Dependency proof: `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md` and
`RESEARCH_OS_EXECUTION_ROADMAP.md` number the Analytics Gate **082**, with
hard dependency **074 + 075 + 079**, all closed. Decision register entries
V2-Q04 (immutable `DataSnapshot` as canonical input), V2-Q44 (live results
must be snapshotted or marked), Q24 (engine choice), Q25 (R/Python only
through the Compute sandbox) and Q27 (AI proposes, the parser enforces)
apply.

Review policy: `FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## Engine decision (founder, 2026-09-23)

Q24 names DataFusion as the first native engine candidate, and the roadmap
allows a bounded alternative behind the same contract. Asked directly, the
founder chose the **already-admitted SQLite** (the workspace's SQLCipher
build of `rusqlite`) as the foundation engine:
- the engine sits behind an engine-neutral contract (`EngineIdentity` on
  every receipt);
- no new dependency is admitted;
- DataFusion qualification is deferred, and can slot in behind the same
  contract when a reproducible gap is shown.

## Authorized scope

- Contracts (`medscale-contracts/src/analytics.rs`): `QueryRequest`,
  `ViewBinding`, `PinnedInput`, `QueryReceipt`, `QueryOutcome`,
  `QueryDenyReason`, `QueryOrigin`, `InputReproducibility`,
  `EngineIdentity`, `ResultTableDoc`, `DerivedTable`, `CohortDefinition`,
  `CohortCriterion`, `CohortOp`, `StatisticKind`, `StatisticValue`,
  `StatisticResult`, `QueryView`, `ReplayReport`, `ReplayVerdict`.
- Engine (`medscale-storage::analytics_engine`): a private in-memory SQLite
  database per query, holding only the bound, digest-verified snapshot rows.
  - Text screen: the query must start with `SELECT`/`WITH`, and no
    `ATTACH`, `DETACH`, `PRAGMA` (including `pragma_*` functions), `VACUUM`,
    extension loading, `sqlcipher_export` or file functions may appear.
  - SQLite must accept exactly one statement and report it read-only.
  - The connection is in `query_only` mode.
  - Bounds: 8,000 SQL characters, 8 bindings, 10,000 result rows,
    128 columns, text cell size, and a 10 s interrupt.
- Governed views: a binding pins a snapshot of the same Project by id,
  content digest, schema fingerprint and row count. Partial snapshots make
  the receipt `partial_inputs`.
- Receipts for every request that reaches a Project, whether completed,
  truncated, denied, timed out or failed. A completed query stores an
  immutable derived table with digest and provenance.
- Replay: a receipt re-runs against its pinned inputs and reports
  `reproduced`, `diverged`, `input_unavailable` or `not_replayable`. A later
  snapshot refresh never changes a completed run.
- Cohort builder: typed criteria (AND) compiled to SQL with quoted
  identifiers and bound parameters.
- Descriptive statistics (count, missing, mean, sample SD, min, median,
  max) over a stored result, with hand-computed correctness fixtures.
  States are `insufficient` and `not_numeric`, never a silent zero.
- Storage schema v10 -> v11 (additive), backup/restore, consistency checks.
- CLI `medscale analytics ...` (human and JSON) and a Desktop Analytics
  route over Core.

## Explicitly not authorized

- DataFusion, Arrow/Parquet or any new dependency.
- Writes, DDL, `ATTACH`, pragmas, extensions or file access from queries.
- R, Python, shell or arbitrary code (Specs 085/086).
- Natural-language-to-SQL generation (needs an admitted model route; a
  later spec may add proposals that still pass this gate).
- Figures and charts; inferential statistics (tests, models, p-values).
- Live database queries as canonical input (V2-Q44).
- Real PHI; remote egress of results.

## Frozen acceptance requirements

1. A read-only query over bound snapshots completes, pins each input's
   digest, and stores a derived table whose digest matches the receipt.
2. Writes, DDL, attachment, pragmas, multiple statements, unknown tables,
   syntax errors, bad aliases, missing or other-Project snapshots are
   denied with a reason and a receipt, and no snapshot changes.
3. More rows than the limit gives `truncated`, never a full-result claim.
   Runaway queries end at the time limit.
4. Replay reproduces a completed receipt. A later snapshot of the same
   source does not change it.
5. Cohort values are bound parameters; an injection-shaped value is data.
6. Statistics match hand-computed fixtures and report `insufficient` or
   `not_numeric` honestly.
7. Receipts, derived tables and cohorts survive reopen and backup/restore.
   Tampered rows or bytes are refused.
8. CLI and Desktop reach analytics only through Core; there is no new
   dependency and no network code.
9. Exact-head and post-main CI pass.

Recorded residuals: no DataFusion qualification; no NL-to-SQL; no charts or
inferential statistics; performance at scale is unmeasured beyond the
snapshot row bound.

## Completion rule

`CLOSED_CANONICAL` only after merge on a green exact head and recorded
post-main verification. Closure of 082 does not authorize 083.
