# CLI_QUALIFICATION — Spec 075

## Groups (all through Core; JSON stable; exits 0/1/2 per existing convention)

```text
medscale datasource create|list|show|update|archive
medscale snapshot preview|import|list|show|rows|refresh [--allow-schema-change]
medscale dataview create|list|show|update
medscale datatransform execute --input ... --op ...
medscale datarelease create|list|show
```

## Parity and error codes

- Human + `--json` output for every command; typed codes: denied,
  not_found, conflict, stale_reference, invalid, corrupt, unsupported_schema,
  unavailable, cancelled, internal.
- CLI has no storage/driver/network edge (`cli_does_not_depend_on_storage_or_rusqlite`
  suite still passes; direction gate still passes).

## Vertical-slice log

`logs/cli-vertical-slice.log` records a local CSV end-to-end run
(source create -> preview -> import -> rows -> view -> transform ->
release -> refresh-unchanged). Local `cargo run` is unavailable on this
workstation (toolchain destruction); the log is produced from CI-attested
test output plus command transcripts at close. No log is fabricated: if a
live CLI run cannot be captured, this file says so explicitly.

```text
RESULT = PENDING (exact-head CI on PR #129 branch spec/075-data-source-fabric)
```
