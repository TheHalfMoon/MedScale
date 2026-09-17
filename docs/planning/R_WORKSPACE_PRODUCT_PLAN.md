# MedScale R Workspace Product Plan

**Status:** Planning candidate — no implementation authority  
**Candidate owner:** V2 candidate Spec 086  
**Depends on:** Data Source Fabric + Privacy Gate + Analytics Gate + MedScale Compute  
**Optional institutional integration:** Posit Workbench / Job Launcher through Institutional Adapters

## 1. Product goal

Make R a first-class reproducible analysis environment for MedScale researchers without placing unrestricted R execution, package installation, IDE rendering, or ambient filesystem access inside MedScale's trusted Desktop/Core process.

The user experience should feel integrated:

- select exact Project data;
- create/open an R workspace;
- launch RStudio or Positron;
- work normally in R;
- run reproducible scripts;
- publish selected results back into MedScale;
- retain exact provenance and environment evidence.

## 2. Product naming

User-facing surface: **R Workspace**.

Do not call the subsystem `RStudio` because MedScale must support multiple R environments and because RStudio/Positron/Workbench are adapters, not the authority model.

## 3. Trust boundary

### Forbidden default architecture

Do not:

- embed an unrestricted R interpreter in `medscale-desktop`;
- give R a canonical vault DB connection;
- give R the vault master key;
- expose all Project files by mounting the whole vault;
- load arbitrary R native libraries into the trusted MedScale process;
- treat an R script as a Core command authority;
- silently import every file created by an R session.

### Allowed architecture

```text
MedScale Project
   |
   +-> exact DataSnapshot / Artifact refs
          |
          v
   R Workspace Stager
          |
          +-> bounded workspace directory
          |      ├ data/      read-only where possible
          |      ├ scripts/
          |      ├ outputs/
          |      ├ renv.lock
          |      └ *.Rproj
          |
          +-> external RStudio / Positron
          |
          +-> Compute job for managed Rscript execution

outputs/
   -> Publish Preview
   -> validation/classification
   -> explicit Core admission
   -> Project artifacts + RRunReceipt
```

## 4. Workspace manifest

Candidate `RWorkspaceManifest` semantic fields:

```text
workspace_id
project_id
schema_version
created_by
created_at
input_artifacts[]
input_snapshots[]
input_staging_mode
r_version_requirement
renv_lock_digest?
ide_preference?
compute_profile?
network_policy
classification
output_policy
workspace_revision
```

The manifest is MedScale-owned. `.Rproj` and R IDE metadata are derived integration artifacts.

## 5. Workspace layout

Candidate staged directory:

```text
medscale-r-<workspace-id>/
├── medscale-workspace.json
├── README.md
├── data/
│   ├── snapshot-<id>.parquet
│   ├── snapshot-<id>.arrow
│   └── metadata/
├── scripts/
├── notebooks/
├── outputs/
├── logs/
├── renv.lock
├── .Rprofile
├── renv/
│   └── activate.R
└── <project>.Rproj
```

The exact generated files depend on whether `renv` is enabled and which IDE is available.

## 6. Data interchange

Preferred tabular interchange order:

1. Parquet for durable columnar snapshots;
2. Arrow IPC where zero/low-copy interoperability or schema fidelity materially helps;
3. CSV only when required by a tool/user workflow;
4. provider-native formats only when a qualified adapter exists.

The R environment should use the Apache Arrow R package where appropriate rather than converting everything into CSV/data.frame eagerly.

MedScale does not require the user to know Arrow to use the feature.

## 7. RStudio Desktop integration

### Detection

MedScale may detect configured/local RStudio Desktop installations using platform-specific, non-invasive discovery.

Do not scan arbitrary filesystem locations without reason. Users can manually configure an executable path.

### Launch

`Open in RStudio` launches the generated `.Rproj`/workspace through the OS process boundary.

The launch receipt records:

```text
ide_kind=RStudioDesktop
ide_version_if_observable
workspace_id
workspace_revision
input snapshot ids/digests
launch time
```

Launching an IDE is not proof that an analysis ran successfully.

## 8. Positron integration

Treat Positron as another external IDE adapter.

`Open in Positron` opens the bounded workspace directory/project and does not grant ambient MedScale authority.

The same staged inputs and explicit output publication semantics apply.

## 9. R version management

The foundation should detect available R installations and record exact version/runtime identity.

MedScale should not silently install or upgrade system R.

Later optional environment management may support approved portable/containerized R runtimes through Compute, but the user must be able to see which R runtime is used.

Candidate states:

```text
Ready
NotInstalled
UnsupportedVersion
Misconfigured
Unavailable
NeedsUserAction
```

## 10. renv reproducibility

`renv` is the preferred R package-environment reproducibility mechanism.

### Foundation semantics

A managed R workspace should support:

- initializing or adopting an `renv.lock`;
- recording the exact `renv.lock` digest;
- checking whether the environment matches the lockfile;
- restoring packages only in an explicit R/Compute environment;
- preserving failed/partial restore state honestly;
- snapshotting environment changes only through an explicit action.

Do not auto-edit `renv.lock` after every package installation without user intent.

### Offline/private operation

An existing restored environment can operate offline. A fresh `renv::restore()` may require repository/network access; MedScale must represent that requirement explicitly and route it through normal network/Compute policy rather than hidden package downloads.

### Lockfile is evidence, not complete environment proof

`renv.lock` records package/version/source state, but system libraries, compiler/runtime details and external dependencies may also matter. The run receipt records additional runtime metadata.

## 11. Managed R execution

`Run R Script` belongs to MedScale Compute rather than the Desktop process.

Candidate flow:

```text
script + workspace revision + exact input snapshots
-> ComputeJobManifest
-> R runtime qualification
-> filesystem lease limited to workspace
-> network denied by default
-> CPU/RAM/time limits
-> execute Rscript
-> capture logs/exit state
-> inspect outputs
-> RRunReceipt
-> explicit publish
```

If an R script needs network access, the job requires an explicit network capability and target policy. Package installation should be a separate environment-preparation action, not a side effect hidden inside an analysis run.

## 12. RRunReceipt

Required fields should include:

```text
run_id
workspace_id
workspace_revision
project_id
actor
entrypoint script/notebook digest
command/arguments
input artifact ids
input snapshot digests
R version
platform/runtime identity
renv_lock_digest
renv_status
package library fingerprint where practical
network policy
filesystem policy
resource limits
started_at
finished_at
exit state
stdout/stderr/log digests
output candidates[]
output digests
classification result
publication state
```

Supplemental:

- `sessionInfo()`;
- locale/timezone;
- environment variables after redaction;
- compiler/system library evidence where relevant.

Never record secrets in the receipt.

## 13. Output publication

R output is not automatically canonical just because it exists in `outputs/`.

Candidate publish flow:

```text
outputs/
-> enumerate candidates
-> show type/size/digest
-> validate format
-> classify
-> user selects outputs
-> choose artifact semantics
-> Core admission
-> Project attachment
-> publication receipt
```

Supported early output candidates:

- Parquet/Arrow/CSV derived datasets;
- PNG/SVG/PDF figures after format qualification;
- JSON/structured result files;
- Markdown/Quarto reports where qualified;
- model/evaluation result manifests if they satisfy owning contracts.

The original files remain evidence-linked where retention policy allows.

## 14. Notebook/report posture

R Markdown and Quarto are valuable but are not trusted-output shortcuts.

Rendering may execute arbitrary R code and must run under the same Compute/runtime restrictions as scripts.

A rendered report records:

- source document digest;
- rendering command/runtime;
- inputs;
- environment receipt;
- output digest;
- execution/log state.

## 15. R package for MedScale

A later optional `medscaleR` package may improve ergonomics by providing typed helpers such as:

```text
ms_workspace()
ms_inputs()
ms_read_snapshot()
ms_output_path()
ms_write_manifest()
ms_publish_candidate()
```

The package must not contain vault credentials or bypass Core. It operates only on the staged workspace contract.

The R package is an adapter/SDK convenience layer, not a server authority.

## 16. Database workflow interaction

R users often connect directly to databases. MedScale should not pretend this will never occur, but the integrated workflow should encourage reproducibility:

Preferred flow:

```text
Data Source Fabric
-> database read/query
-> immutable DataSnapshot
-> R Workspace
```

If a user deliberately configures direct DB access from R, that credential/network path is outside the default managed workflow unless MedScale later implements an explicit credential lease for Compute jobs.

A direct external-IDE database connection cannot be represented as fully MedScale-governed merely because the IDE was launched from MedScale.

## 17. Institutional Posit Workbench

Later Institutional Adapter support may:

- create/open a MedScale-derived workspace in Posit Workbench;
- stage approved snapshots to accessible storage;
- launch RStudio Pro/Positron sessions;
- submit Workbench/Launcher jobs to Slurm/Kubernetes;
- use managed credentials only under explicit institutional policy;
- ingest output receipts back into MedScale.

This is optional and must not make Posit Workbench a Personal/Lab requirement.

## 18. Privacy/classification

The R Workspace inherits the most restrictive classification of staged inputs unless an explicit Privacy transformation creates a different artifact.

External IDE mode displays the classification and warns when the workspace contains sensitive data.

Network is not assumed merely because RStudio is open.

MedScale cannot claim to control user actions performed manually in an external IDE outside the staged workspace boundary. The UI and evidence model must distinguish:

```text
MedScale-managed execution
External IDE launched by MedScale
User-managed external activity
```

## 19. Failure states

Required explicit states:

```text
RNotInstalled
RVersionMismatch
IDENotFound
WorkspaceStageFailed
InputUnavailable
InsufficientDisk
RenvLockMissing
RenvRestoreRequired
RenvRestoreFailed
ComputeDenied
ComputeUnavailable
RunFailed
RunCancelled
RunTimedOut
OutputInvalid
OutputClassificationDenied
PublishConflict
```

No generic success if the R process exited nonzero.

## 20. Cross-platform qualification

At minimum qualify local integration on supported MedScale desktop platforms:

- Windows;
- macOS;
- Linux.

Do not infer platform support from one OS.

Platform-specific process launch, paths, R installation discovery, file permissions and line endings need evidence.

## 21. Security/adversarial tests

Required families:

- R script attempts to traverse outside workspace;
- symlink escape;
- output path traversal;
- subprocess spawn policy;
- network denial;
- environment-secret leakage;
- malicious `.Rprofile`;
- malicious existing workspace metadata;
- package install attempt during network-denied run;
- huge output/disk exhaustion;
- native package crash;
- cancellation/timeout cleanup;
- output file changed during admission;
- classification downgrade attempt;
- stale input snapshot binding;
- IDE launch path injection.

## 22. Performance dimensions

Measure:

- staging time for representative datasets;
- disk amplification;
- Parquet/Arrow read throughput from R;
- workspace reopen time;
- managed R job startup latency;
- output publication time;
- environment restore time separately from analysis time.

## 23. Acceptance contract

Candidate R Workspace closure requires evidence equivalent to:

```text
R_WORKSPACE_STAGE=PASS
EXACT_INPUT_BINDING=PASS
RSTUDIO_EXTERNAL_LAUNCH=PASS_ON_SUPPORTED_PLATFORM_OR_EXPLICIT_NOT_AVAILABLE
POSITRON_ADAPTER=QUALIFIED_OR_DEFERRED_WITH_REASON
RENV_LOCK_CAPTURE=PASS
MANAGED_RSCRIPT_COMPUTE=PASS
NETWORK_DEFAULT_DENY=PASS
WORKSPACE_ESCAPE_DENIED=PASS
EXPLICIT_OUTPUT_PUBLICATION=PASS
R_RUN_RECEIPT=PASS
CLASSIFICATION_PROPAGATION=PASS
WINDOWS_MACOS_LINUX_MATRIX=EVIDENCE_BOUND
REAL_PHI_USED=false
```

## 24. Non-goals

Foundation does not:

- rebuild RStudio;
- embed the RStudio UI inside Slint;
- bundle Posit Workbench;
- install CRAN packages silently;
- make arbitrary R code trusted;
- let R mutate MedScale canonical storage directly;
- give external IDEs hidden Hub/agent credentials;
- promise byte-identical statistical results across unqualified BLAS/system-library environments;
- replace Analytics Gate with R.
