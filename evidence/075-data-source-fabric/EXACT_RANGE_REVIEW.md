# EXACT_RANGE_REVIEW — Spec 075 (draft; finalized at exact head)

## Canonical base and branch

```text
BASE = ae0441918296c2d1510a71061249c7e55e65d760 (PR #128 merge)
BRANCH = spec/075-data-source-fabric
```

## Authorized scope

`docs/planning/SPEC_075_PROMOTION.md` (only Spec 075: Data Source Fabric +
Data Workbench Foundation; 076+ explicitly not authorized).

## Changed files (at PR #129 head; verify with git diff at close)

Product code:

```text
crates/medscale-contracts/src/data_sources.rs (new)
crates/medscale-contracts/src/envelopes/mod.rs (14 capabilities, 18 requests, 14 responses, Cancelled)
crates/medscale-contracts/src/lib.rs (module registration)
crates/medscale-contracts/src/network/mod.rs (DatasetMirrorRead purpose only)
crates/medscale-contracts/tests/data_sources_075.rs (new)
crates/medscale-storage/src/data_sources.rs (new: v4 rows + external SQLite adapter)
crates/medscale-storage/src/sqlite_meta.rs (v4 migration + snapshot_bytes v4)
crates/medscale-storage/src/backup.rs (restore_v4, manifest v4)
crates/medscale-storage/src/lib.rs (module + type exports)
crates/medscale-storage/tests/data_sources_075.rs (new)
crates/medscale-core/src/authority/data_sources.rs (new: authority paths)
crates/medscale-core/src/authority/data_acquire.rs (new: parsers/materialize/remote)
crates/medscale-core/src/authority/facade.rs (ds() helper + 18 arms + capability pairs)
crates/medscale-core/src/authority/mod.rs (module registration)
crates/medscale-core/src/cli_session.rs (16 wrappers)
crates/medscale-core/src/authority/data_acquire.rs unit tests (inline)
crates/medscale-core/tests/data_sources_075.rs (new)
crates/medscale-core/Cargo.toml (rusqlite dev-dependency for fixture creation only)
crates/medscale-cli/src/data_source.rs (new: 5 command groups)
crates/medscale-cli/src/main.rs (wiring)
crates/medscale-cli/src/project.rs (Cancelled arm only)
crates/medscale-desktop/src/data_workbench.rs (new: view-models + tests)
crates/medscale-desktop/src/main.rs (Data route wiring + actions)
crates/medscale-desktop/src/project_workspace.rs (Cancelled arm only)
crates/medscale-desktop/ui/app.slint (Data structs/properties/nav/route)
```

Planning/spec/evidence (authorized promotion + package + evidence):

```text
docs/planning/SPEC_075_PROMOTION.md (new)
docs/planning/BUILD_QUEUE.md (075 row + footer)
specs/075-data-source-fabric/ (7 files)
evidence/075-data-source-fabric/ (this packet)
```

## Unexpected files

```text
NONE EXPECTED. Verify at close: git diff --name-only BASE...HEAD must equal
the union of the two lists above. Any other file is a blocking defect.
```

## New dependencies

```text
PRODUCT DEPENDENCIES ADDED: NONE.
- No new [dependencies] in any crate (verified via Cargo.toml diff + cargo-deny).
- medscale-core [dev-dependencies] gains workspace-pinned rusqlite (same
  qualified 0.37.0 already in Cargo.lock) for external-SQLite fixture
  creation in tests only. No product edge: Core reaches SQLite exclusively
  through medscale-storage. Dependency-direction gate stays green.
```

## Secret scan

```text
- credential_ref is OpaqueId-only; no password/token literal exists in new code.
- CLI JSON outputs serialize Core results; scanned for secret markers at close.
- Fixture data is synthetic (town names/doses); no PHI, no licensed content.
```
