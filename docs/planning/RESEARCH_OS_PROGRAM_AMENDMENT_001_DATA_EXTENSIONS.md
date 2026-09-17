# Research OS Program Amendment 001 — Data Sources, R Workspace, and Community Extensions

**Status:** `PLANNING_AMENDMENT / NOT_IMPLEMENTATION_AUTHORITY`  
**Date:** 2026-09-17  
**Applies to:** Research OS candidate work after promoted Spec 074  
**Base main at amendment creation:** `a80c33307afc4577790282652e5b20911beb4bbe`

## 1. Founder direction captured

This amendment records three major product requirements that must be part of the Research OS program before later candidate specs are promoted:

1. **Data sources are a first-class product plane.** MedScale must connect to and import from local files/folders, databases, Kaggle, Hugging Face datasets, and later approved institutional/object-store sources through one governed source model.
2. **R is a first-class research environment.** MedScale must integrate with R/RStudio/Posit without embedding an unrestricted scripting runtime in the trusted Desktop process.
3. **MedScale Hub must support a community extension ecosystem.** Developers must be able to build and publish extensions, while MedScale keeps capability, privacy, signing, sandbox and authority boundaries stronger than a normal desktop plugin system.

These are not optional polish. They materially affect the dependency graph for Analytics, Knowledge/RAG, MedAgent, Compute, Research Packs and institutional deployment.

## 2. Why this is an amendment rather than a Spec 074 expansion

Spec 074 is already explicitly promoted as **Project + Artifact Graph Foundation** and must remain bounded. It should not absorb databases, remote dataset providers, R runtimes, plugin execution or Hub marketplace behavior.

Therefore:

- Spec 074 remains unchanged in product scope;
- old candidate numbering at `075+` is superseded by the V2 roadmap in this amendment packet;
- no old `075+` candidate has implementation authority merely because it existed in the first Research OS planning packet;
- any later promotion must use the V2 semantic name/dependency graph and reverify unused numbering at promotion time.

## 3. New program architecture

Research OS now has these additional planes:

### 3.1 Data Source Fabric

A MedScale-owned source plane that normalizes acquisition and connection semantics across:

- local files and directories;
- local tabular/database files;
- relational databases;
- Kaggle datasets and competition inputs where terms permit;
- Hugging Face Hub datasets;
- later object stores, warehouses, LIMS/ELN and institutional systems.

The Fabric owns **source identity, credentials references, discovery, preview, import/materialization, immutable snapshot lineage, schema fingerprints, refresh intent, receipts, and failure state**. It does not own analytics semantics or clinical authority.

### 3.2 R Workspace

A reproducible research bridge that stages exact project snapshots into a bounded workspace and supports:

- external RStudio Desktop / Positron launch;
- later Posit Workbench integration;
- `Rscript` execution only through the normal bounded Compute authority;
- Arrow/Parquet interchange by default where appropriate;
- `renv.lock` / R version / script digest capture;
- explicit output publication back into MedScale rather than ambient vault writes.

### 3.3 Community Extensions

A signed, capability-scoped extension ecosystem with a Hub Community Registry. Extensions never become a second authority plane.

Candidate extension contribution classes include:

- data-source connectors;
- importers/exporters;
- MedAgent tools;
- analysis actions;
- visualization contributions;
- Research Pack augmentations;
- AudioFlow processors;
- institutional adapters.

Executable extensions run in an admitted sandbox/worker runtime, not in the trusted Desktop/Core process by default.

## 4. Corrected candidate roadmap V2

The prior candidate numbers `075-089` are planning aliases only and are superseded by this V2 sequence for future promotion:

```text
074 Project + Artifact Graph Foundation              [already promoted separately]
075 Data Source Fabric
076 Collaboration Substrate
077 MedAgent Workbench
078 Model Fleet + Compare
079 Privacy Gate
080 Governed Browse
081 AudioFlow Foundation
082 Analytics Gate
083 Knowledge + Research Canvas
084 MedScale Hub
085 MedScale Compute
086 R Workspace
087 Community Extensions
088 AudioFlow Advanced
089 Research Packs
090 Institutional Adapters
091 Federation
092 Whole-Platform Qualification
```

This numbering is still candidate-only for `075+` and must be reconciled with live canonical main before every promotion.

## 5. Dependency graph V2

```text
074 Project + Artifact Graph
 |
 +--> 075 Data Source Fabric
 |
 +--> 076 Collaboration ---------------------------> 084 Hub
 |
 +--> 077 MedAgent --> 078 Fleet
 |       |
 |       +--> 079 Privacy
 |               |
 |               +--> 080 Governed Browse
 |               +--> 081 AudioFlow Foundation
 |               +--> 082 Analytics Gate <---- 075
 |               +--> 083 Knowledge/Canvas <--- 075
 |               +--> 084 Hub <------------- 076
 |               +--> 085 Compute
 |
075 + 079 + 082 + 085 ----------------------------> 086 R Workspace
079 + 084 + 085 ----------------------------------> 087 Community Extensions
081 + 084 ----------------------------------------> 088 AudioFlow Advanced
074 + 075 + 077 + 082 + 083 ----------------------> 089 Research Packs
075 + 079 + 084 + 085 + 089 ----------------------> 090 Institutional Adapters
090 ------------------------------------------------> 091 Federation
074-091 --------------------------------------------> 092 Whole-Platform Qualification
```

Some subsystems may be shaped in parallel after their hard predecessors close, but integration acceptance remains dependency-owned.

## 6. Data Source Fabric non-negotiable semantics

### 6.1 One source model

Do not create one architecture for database connections, another for Kaggle, another for Hugging Face and another for local files. All adapters implement a MedScale-owned source contract.

Minimum semantic families:

```text
DataSourceId
DataSourceKind
DataSourceManifest
CredentialRef
SourceLocation
SourceCapability
SourceSchema
SchemaFingerprint
SourceRevision
AcquisitionMode
ImportPlan
DataSnapshot
ImportReceipt
RefreshReceipt
SourceHealth
```

### 6.2 Credentials are references

`DataSourceManifest` never stores plaintext database passwords, API tokens, OAuth refresh tokens, Kaggle tokens or Hugging Face tokens.

Secrets remain in the admitted secret-store boundary. The manifest stores only opaque credential references and non-secret connection metadata.

### 6.3 Read-only first

Foundation adapters are read/discover/preview/import first.

Database DDL/DML, upload, push, delete, notebook submission, Kaggle competition submission, Hub dataset publication, or other external writes require a later explicit side-effect capability.

### 6.4 Snapshot before canonical analysis

A mutable remote query result is not automatically reproducible research evidence.

MedScale may support preview/live read-only exploration, but a canonical analytical run should bind to an immutable `DataSnapshot` or explicitly record that the source could not provide a stable revision/snapshot and therefore reproducibility is partial.

A snapshot records, where available:

- exact source identity;
- source revision/version/ETag/commit/dataset version;
- selected files/tables/query;
- retrieval timestamp;
- schema fingerprint;
- raw/normalized digests;
- row/file counts;
- adapter version;
- policy/data classification;
- license/terms metadata where relevant.

### 6.5 No arbitrary remote code on import

Dataset acquisition must not execute provider-supplied scripts or arbitrary code merely to obtain data.

Hugging Face dataset import defaults to raw repository/files/declared formats with exact revisions; no `trust_remote_code`-style behavior in the trusted path.

Kaggle/HF Python libraries, if used, run behind an isolated adapter/worker contract rather than becoming a trusted Desktop dependency.

## 7. Data-source priority order

### Foundation adapters

1. Local file/folder:
   - CSV/TSV;
   - Parquet;
   - Arrow IPC/Feather where supported;
   - JSON/JSONL;
   - bounded archive import after hostile-input qualification.
2. Local database/file engines:
   - SQLite as a source distinct from MedScale's own canonical vault;
   - DuckDB file/source if qualification justifies it.
3. Relational database read connectors:
   - PostgreSQL;
   - MySQL/MariaDB;
   - SQL Server;
   - generic ODBC later if native connectors do not cover institutional needs.
4. Kaggle dataset acquisition.
5. Hugging Face dataset acquisition.

### Later adapters

- S3-compatible object storage;
- Azure Blob / ADLS;
- Google Cloud Storage;
- Snowflake/Databricks/BigQuery and other warehouses;
- LIMS/ELN;
- EHR/FHIR server sources where not already owned by existing MedScale clinical integration;
- institutional registries.

Later adapters do not become required dependencies for Personal mode.

## 8. Data Sources product UX

The Desktop should gain a first-class **Data Sources** workspace rather than hiding source connections inside Analytics.

Candidate interaction model:

```text
Data Sources
├── Connected
├── Local
├── Databases
├── Kaggle
├── Hugging Face
├── Imports
├── Snapshots
├── Credentials
└── Health
```

`New Data Source` should support provider-specific setup while preserving one underlying contract.

Database flow:

```text
Choose adapter
-> connection metadata
-> CredentialRef
-> Test connection
-> Browse schemas/tables
-> Preview bounded sample
-> Choose Import Snapshot or Read-only Live Preview
-> classify/policy
-> materialize DataSnapshot
-> attach to Project
```

Kaggle/Hugging Face flow:

```text
Search or paste exact dataset identifier
-> inspect card/metadata/license/version/files
-> choose exact revision/version/files
-> acquisition policy + disk estimate
-> download through admitted adapter/network path
-> validate/digest
-> normalize only through explicit import transform
-> DataSnapshot + ImportReceipt
-> attach to Project
```

## 9. R Workspace architecture

### 9.1 Trusted-boundary rule

Do not embed unrestricted R in `medscale-desktop` or `medscale-core`.

R code executes either:

- in an external user-controlled IDE/session over a staged workspace; or
- through `MedScale Compute` as a bounded job.

### 9.2 Workspace staging

A Project may create an `RWorkspaceManifest` containing exact references to staged inputs and environment state.

Candidate workspace layout:

```text
<workspace>/
├── medscale-workspace.json
├── data/
│   ├── <snapshot>.parquet
│   └── ...
├── scripts/
├── outputs/
├── renv.lock
├── .Rprofile
├── renv/
├── <project>.Rproj
└── README.md
```

Data staging should be read-only where the platform permits. The external IDE receives the workspace, not the MedScale vault or master key.

### 9.3 Reproducibility receipt

Published R outputs require an `RRunReceipt` containing at minimum:

- exact input snapshot digests;
- script/notebook digest;
- R version;
- platform/runtime identity;
- `renv.lock` digest;
- package restoration state;
- relevant environment configuration;
- command/entry point;
- exit state;
- stdout/stderr digest or retained logs under policy;
- output file digests;
- publication actor/time;
- classification propagation.

`sessionInfo()` may be captured as supplemental evidence but is not a substitute for a lockfile and exact input provenance.

### 9.4 IDE integration

Foundation UX may offer:

- **Open in RStudio**;
- **Open in Positron**;
- **Open folder**;
- **Run R Script** (only after Compute authority is qualified).

Posit Workbench/Job Launcher is an institutional adapter, not a Personal-mode requirement.

## 10. Community Extensions architecture

### 10.1 Product names

Use these semantic names unless later brand review changes them:

- **MedScale Extensions** — extension platform;
- **Community Registry** — Hub-hosted discover/install/publish surface;
- **Extension SDK** — developer contract;
- **Extension Pack** — signed distributable artifact, distinct from Research Packs and model Packs.

### 10.2 Extension manifest

Every executable extension declares before install:

```text
extension_id
publisher_id
name
version
api_version
minimum_medscale_version
package_digest
signature/provenance
license
entrypoint_kind
capabilities
network_policy
data_classes
filesystem_policy
tool contributions
UI contributions
source/provider contributions
configuration schema
migration hooks
update/rollback metadata
```

### 10.3 Capability-first security

An extension has no ambient access to:

- vault database;
- vault master key;
- OS keychain;
- arbitrary filesystem;
- network;
- clipboard;
- microphone/camera;
- Project data;
- PHI/PII;
- Core mutation APIs.

Each capability is explicitly granted and mediated.

### 10.4 Runtime posture

Default executable-extension placement is one of:

- WASM sandbox with a narrow MedScale host API if Wasmtime/another engine qualifies;
- isolated worker process with explicit IPC/capability broker;
- declarative no-code contribution where execution is unnecessary.

Arbitrary native libraries loaded into the trusted Desktop process are denied by default.

### 10.5 Community Registry

The Hub may host a registry containing signed metadata and release artifacts or references to immutable releases.

Registry workflow:

```text
Developer SDK
-> build Extension Pack
-> manifest/schema validation
-> dependency/license/SBOM scan
-> malware/static policy scan
-> capability diff
-> signature/provenance verification
-> review tier
-> publish registry entry
-> user install preview
-> explicit capability grant
-> sandboxed install/activate
```

Extensions may not self-update or silently add capabilities. Capability changes require a visible re-consent/update decision.

### 10.6 Obsidian lesson, MedScale boundary

The desired product quality is Obsidian-like community extensibility and discoverability, not Obsidian-like trust assumptions copied wholesale.

Useful lessons:

- documented SDK;
- simple submission flow;
- community directory;
- clear developer policies;
- GitHub-hosted release compatibility;
- strong community ownership.

MedScale must add stronger requirements for healthcare/research data: signing, sandboxing, capability manifests, no client telemetry by default, no hidden network, no self-managed dependency installers inside the trusted app, and explicit sensitive-data permissions.

## 11. Source/reference decisions added by this amendment

Candidate sources/reference families:

| Capability | Source/reference | Posture |
|---|---|---|
| Kaggle acquisition | official `Kaggle/kagglehub` + Kaggle API semantics | isolated adapter / reference; no trusted Python dependency |
| Hugging Face datasets | HF Hub dataset download/revision semantics | direct governed adapter or isolated client; exact revision/file manifests |
| Arrow/Parquet interchange | Apache Arrow Rust + Arrow R | dependency candidate / canonical interchange candidate |
| R environment reproducibility | `renv` | external workspace dependency / receipt input |
| RStudio/Positron | Posit IDEs | external IDE integration |
| Institutional R | Posit Workbench + Job Launcher | optional institutional adapter |
| Community plugin UX/policy | Obsidian developer docs/community directory | reference only |
| WASM extension isolation | Wasmtime | benchmark/admission candidate after Compute |
| Worker plugin framework | Extism or MedScale-owned worker IPC | benchmark/reference; no authority transfer |
| connector security | Himsat capability/connector planning patterns | sibling reference/adapt |
| experiment environment manifests | MESC runner/manifest patterns | sibling reference/adapt, no MESC coupling |

## 12. Program invariants after amendment

The following remain non-negotiable:

1. One Core authority.
2. Local-first Personal mode remains valid without Hub.
3. Remote source access is explicit network capability, never hidden fallback.
4. Credentials never enter model prompts or plugin manifests as plaintext.
5. Imports are hostile/untrusted until validated.
6. External datasets are not clinical truth.
7. Extensions do not inherit user permissions automatically.
8. R does not receive ambient vault access.
9. Research Packs remain domain semantics; Extensions are executable/integration capability. They are different artifact classes.
10. Data snapshots and published outputs carry exact provenance and classification.
11. No provider adapter may silently change source revision on a reproducible run.
12. No community marketplace may weaken air-gapped/offline operation.

## 13. Immediate governance effect

This amendment changes planning only.

It does **not**:

- change Spec 074 implementation scope;
- authorize Spec 075 implementation;
- authorize any plugin runtime;
- authorize database network egress;
- authorize Kaggle/HF credentials;
- authorize R execution;
- authorize Hub marketplace operation;
- authorize real PHI.

After this amendment is reviewed/merged, Spec 074 remains the only promoted Research OS implementation unit unless a newer explicit founder promotion exists.
