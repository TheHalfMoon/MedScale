# Security — Spec 082 Analytics Gate

| # | Threat | Control | Proof |
|---|---|---|---|
| Q1 | Data modification through SQL | text screen (`SELECT`/`WITH` first), SQLite read-only check, `query_only`, private in-memory database | engine `writes_ddl_and_escapes_are_refused`; Core `writes_escapes_and_foreign_inputs_are_refused_with_receipts` (snapshot unchanged) |
| Q2 | File or database escape | `ATTACH`/`DETACH`/`PRAGMA`/`pragma_*`/`VACUUM`/extension/file functions refused; the engine opens no file | engine test; Core test |
| Q3 | Statement stacking | SQLite must accept exactly one statement | engine and Core tests |
| Q4 | SQL injection through cohorts | quoted identifiers, bound parameters | Core unit `cohorts_compile_to_bound_parameters`; Core `cohorts_bind_values_and_reproduce`; engine `parameters_are_bound_not_spliced` |
| Q5 | Cross-Project or cross-scope reads | bindings resolve only snapshots of the request's Project in scope | Core deny test (other Project) |
| Q6 | Resource exhaustion | SQL length, bindings, rows, columns, text size, 10 s interrupt | engine `runaway_queries_time_out`, `results_are_bounded_and_honest` |
| Q7 | Partial answers presented as complete | `truncated` outcome and flag; `partial_inputs` reproducibility | Core truncation test; contract receipt test |
| Q8 | Silent result drift | receipts pin snapshot digests; replay compares result digests | Core replay and refresh tests |
| Q9 | Tampered results or backups | digest checks on read and restore; consistency checks | storage `restore_rejects_hand_edited_082_snapshots` (7 cases) |
| Q10 | Misleading statistics | `insufficient` and `not_numeric` states; hand-computed fixtures | Core unit and integration statistics tests |

Results stay local. Egress remains governed by the Spec 079 Privacy Gate,
and no analytics egress path exists. Real PHI remains unauthorized, and all
fixtures are synthetic.
