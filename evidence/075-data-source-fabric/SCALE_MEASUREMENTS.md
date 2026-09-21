# SCALE_MEASUREMENTS — Spec 075

## Fixtures

- `csv-scale`: 20,000 deterministic rows (`town-{i}`, dose `i % 1000`) in
  `crates/medscale-core/tests/data_sources_075.rs: large_table_import_pages_correctly`.
- Storage/contract suites use small synthetic fixtures; row/page bounds are
  constants in `medscale-contracts/src/data_sources.rs`.

## Method

Timings below are local-only observations from CI logs. They are capacity
evidence, not budget claims: `budgets_claimed_met=false` is unchanged.

```text
SCALE_RUN = PENDING (CI run 35416747817 on head a11b08e)
COMMAND = cargo test --workspace --locked (rust ubuntu/macos/windows jobs)
```

## Results

```text
PENDING. Record at close: per-job large-table test durations, snapshot byte
sizes, part counts, page counts, runner hardware (GitHub-hosted labels).
```

## Limits stated honestly

- Snapshot bytes cap `SNAPSHOT_BYTES_MAX` (256 MiB), rows cap
  `SNAPSHOT_ROWS_MAX` (1M), page cap 1,000 rows, parts of 500 rows.
- Paging reads one snapshot blob per query (bounded by the byte cap);
  per-part windowing without full-blob reads is future work, not claimed.
