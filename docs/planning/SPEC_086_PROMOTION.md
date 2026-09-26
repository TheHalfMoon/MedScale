# Spec 086 Promotion — R Workspace (foundation slice)

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`
**Promotion date:** 2026-09-26
**Canonical base:** BASE_SHA_PENDING (Spec 085 closure PR #151 merge)
**Target branch:** `spec/086-r-workspace`

## Authority

The founder's standing continuation directive requires promoting the next
dependency-ready Research OS unit after each closure without routine
approval. `IMPLEMENTATION_AUTHORITY.md` remains active
(`DEPENDENCY_INSTALL_AUTHORITY = YES_IF_OWNING_SPEC_ADMITS_AND_LOCKS_IT`;
this spec admits nothing).

Dependency proof: `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md` and
`RESEARCH_OS_EXECUTION_ROADMAP.md` number R Workspace **086**, hard
dependency **075 + 079 + 082 + 085**. All four are `CLOSED_CANONICAL`
(see `BUILD_QUEUE.md`; 085 closure recorded in
`evidence/085-compute/CLOSURE.md`). 087 (079 + 084 + 085) and 088
(081 + 084) are also dependency-ready; the repository's strict
numeric-order convention selects 086.

Sandbox truth (live source, `medscale-contracts/src/os_sandbox`): every
per-OS mechanism is `ReadyBaseMeasured`; `platform_qualified=false`;
residuals `macos_app_sandbox_signed_enforcement` and
`multi_os_platform_qualified_composition` remain open. Spec 085 admits
closed MedScale job kinds only and no arbitrary code.

Review policy: `FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## The execution boundary decision

The authority's foundation modes are `Open in RStudio`, `Open in
Positron`, `Open Folder` and `Run R Script` "only through Compute". A
user's R script is arbitrary code: it can read any file the OS user can,
open sockets, load native libraries and start processes. Spec 085's
worker isolation is `ReadyBaseMeasured` defense in depth for MedScale's
own code, not a sandbox for untrusted code, and nothing in the current
evidence can contain arbitrary R. Therefore:

- **`Run R Script` is not admitted in this slice.** A run request is
  validated, bound to its workspace, script digest, `renv.lock` digest and
  `Rscript` identity, recorded, and refused with
  `compute_denied_platform_unqualified`. MedScale starts no R process.
- Admitting managed R execution needs a new canonical decision: a
  platform-qualified sandbox for arbitrary code (network, filesystem,
  process and native-library containment on each OS) or an equivalent
  isolation boundary. It is recorded as an open gate, not as a residual
  of this slice.
- The external IDE modes run the user's own program as the user, outside
  MedScale's authority. MedScale gives that program a staged copy and
  nothing else; it does not and cannot confine it.

The closure gate "runs/publishes output explicitly" is therefore met for
publication only; the run half is `NOT_ADMITTED` and recorded as such.

```text
source snapshot (075) != staged copy (CSV, digest-pinned)
  != script (user's, digested as evidence) != execution (external, not MedScale's)
  != output file (outputs/) != published table (Core-validated, derived, unreviewed)
  != reviewed result (not here) != external effect (none)
```

## Authorized scope

- Contracts (`medscale-contracts/src/r_workspace.rs`): `RStageRequest`,
  `RWorkspaceManifest` (+ `RStagedInput`, `StagedFile`), `RWorkspace`,
  `WorkspaceIntegrity` (the authority's `RWorkspaceState`),
  `RRuntimeIdentity`, `RLockfileRef`, `RLaunchRequest`, `RLaunchReceipt`,
  `RRunRequest`, `RRunReceipt`, `RPublishRequest`, `RPublishReceipt`,
  `RPublishedTable`, `WorkspaceInspection`, `RWorkspaceStatus`,
  `RWorkspaceHistory`; closed vocabularies; deterministic renderers.
- Staging: exact Spec 075 snapshots (digest-verified on load) as canonical
  CSV copies plus schema files, README, `.Rproj` and a MedScale descriptor,
  into a new directory under a host-configured staging directory that must
  lie outside the vault. Data copies are written read-only. No vault path,
  database handle, key, credential or environment is written.
- External launch of RStudio, Positron or a folder opener configured by
  the host (absolute path; no shell, no `PATH` lookup, stdio closed,
  environment reduced to an allowlist; the receipt records variable names
  only).
- Recorded, refused managed-run requests (above).
- Explicit publication of one named regular file from `outputs/`:
  link and bound checks, exact digest (optionally matched against the
  inspected one), typed by the Spec 075 CSV parser with no dropped rows,
  committed with its receipt in one transaction as a derived, unreviewed
  table that inherits the workspace's class and names its input
  snapshots. Every attempt leaves a receipt.
- Inspection: workspace integrity against recorded digests, input
  currency, `renv.lock` evidence, publishable outputs.
- Storage v14 -> v15 (additive), backup/restore with fail-closed parsing,
  consistency checks; Core, facade, `CliSession` and CLI `medscale r ...`.

## Frozen contracts

- Workspace ids are Core-allocated (`r-workspace-N`); receipts
  `r-launch-N`, `r-run-N`, `r-publish-N`; tables `r-table-N`.
- A workspace binds one Project, a label, 1-16 exact snapshots (id,
  content digest, schema fingerprint, rows, class), the layout version,
  staging mode `csv_copy`, output policy `explicit_publish_only`, the most
  restrictive input class (a snapshot without a Spec 079 row counts as
  `local_phi`), and the digest of every generated file. The descriptor is
  the manifest's canonical JSON; its digest is stored. Nothing changes
  after staging.
- Every receipt names its workspace, Project and descriptor digest, and
  is refused by storage unless they match.
- Staging paths and programs come from host configuration only; no
  request carries a path or a program.
- Published tables are `reviewed=false`, carry the workspace class and
  `derived_from` = the workspace's inputs in order.

## Frozen acceptance requirements

1. Staging writes exact, read-only CSV copies whose bytes are a pure
   function of the snapshots; two workspaces from the same snapshots have
   identical generated files; no staged file names the vault.
2. Staging refuses a bad label, 0 or duplicate inputs, another Project's or
   a missing snapshot, and an absent, nonexistent or in-vault staging
   directory, creating nothing.
3. A launch starts only a configured absolute program, with the workspace
   as its only argument and an allowlisted environment; unconfigured,
   missing or relative programs never start; every attempt has a receipt.
4. A run request never executes anything; it is refused with a receipt that
   binds script, lockfile and runtime evidence; bad, missing or linked
   scripts are `script_invalid`.
5. Publication admits a regular UTF-8 CSV with at least one row and no
   ragged rows; it refuses bad names, missing files, directories, links,
   oversized files, invalid CSV, digest mismatches, changed or missing
   workspaces and stale inputs; refused attempts commit no table.
6. Published tables survive restart and backup/restore; tampered rows and
   backups are refused; v14 backups restore with empty R Workspace tables.
7. Nothing crosses Project, realm or authority scope.
8. The CLI reaches R Workspace only through Core.
9. Exact-head and post-main CI pass.

## Explicitly not authorized

- Starting R, `Rscript`, an R package manager or any interpreter; package
  installation or restoration; any network path.
- Arrow/Parquet staging (needs a dependency admission).
- Watching the workspace or importing anything not explicitly published.
- Posit Workbench / Job Launcher (090); remote or cloud execution.
- Any new dependency; claims of `platform_qualified` or of containment of
  the external IDE.
- Real PHI, production credentials, release claims.

Recorded residuals (expected, non-blocking):

- The external program runs as the OS user with the user's full rights:
  it can read the vault directory and use the network. MedScale does not
  hand it vault paths or secrets, but does not confine it.
- The R version is not probed (probing runs R); `Rscript` is identified by
  path and file digest only. `renv.lock` is evidence only.
- The link check before open is exact on Unix (device and inode) but only
  length-based on Windows, where a swap between check and open is not
  fully excluded.
- A crash during staging can leave a `.partial` directory under the
  staging directory; it is never registered.
- Snapshots cannot carry a Spec 079 classification row today, so every
  workspace is `local_phi`.
- No Desktop surface.

## Completion rule

`CLOSED_CANONICAL` only after merge on a green exact head and recorded
post-main verification. Closure of 086 does not authorize 087.
