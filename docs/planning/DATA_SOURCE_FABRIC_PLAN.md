# MedScale Data Source Fabric Plan

**Status:** Planning candidate — no implementation authority  
**Candidate owner:** V2 candidate Spec 075  
**Depends on:** Spec 074 Project + Artifact Graph Foundation  
**Hard integration predecessors for later sensitive/external workflows:** Privacy Gate, Compute, Institutional Adapters as applicable

## 1. Product goal

Make MedScale the place where a researcher can discover, connect, inspect, import, snapshot, refresh and attach data without leaving the Project workspace, while preserving reproducibility, privacy and explicit authority.

The Data Source Fabric is not an analytics engine and not a second storage authority. It is the governed acquisition/connection plane between external/local sources and MedScale Project artifacts.

## 2. User promise

A user should be able to answer all of these from one surface:

- Where did this dataset come from?
- Which exact version/revision/file/table was used?
- Was this a live preview or immutable snapshot?
- Which credentials were required, without exposing them?
- What schema did MedScale observe?
- What transformations occurred during import?
- What license/terms metadata was present?
- Can this analysis be reproduced later?
- Is the source stale/unavailable/changed?
- Which Projects and runs depend on this snapshot?

## 3. Product surfaces

### Data Sources navigation

```text
Data Sources
├── Overview
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

### Project integration

Inside a Project:

```text
Project
├── Data
│   ├── Sources
│   ├── Snapshots
│   ├── Tables
│   └── Import history
└── Activity
    └── Source/import/refresh events
```

A Project stores references to source/snapshot artifacts rather than provider-specific hidden state.

## 4. Core contract families

The exact Rust paths are selected during promotion after live inspection, but the semantic contract is fixed here.

### 4.1 Source identity

```text
DataSourceId
DataSourceManifest
DataSourceKind
SourceLocation
SourceCapability
SourcePolicy
SourceHealth
SourceRevision
CredentialRef
```

`DataSourceManifest` candidate fields:

```text
header/id
schema_version
project_binding?           # source can exist in library before Project attachment
kind
adapter_id
adapter_version
location                   # non-secret canonical location/handle
credential_ref?            # opaque reference only
capabilities               # discover, preview, snapshot, refresh, live_query, write (later)
network_route               # local / governed network / worker
classification_hint
license_metadata?
provider_metadata
created_by
created_at
last_health_state
```

### 4.2 Discovery/schema

```text
SourceNamespace
SourceObject
SourceField
SourceSchema
SchemaFingerprint
DiscoveryReceipt
PreviewRequest
PreviewResult
```

Schema discovery must be bounded and cancelable. A preview is non-canonical and must be visibly labeled as preview/live data.

### 4.3 Acquisition/materialization

```text
AcquisitionMode
ImportPlan
ImportTransform
DataSnapshot
SnapshotManifest
ImportReceipt
RefreshPlan
RefreshReceipt
```

Candidate `AcquisitionMode`:

```text
Snapshot
ReadOnlyLivePreview
MountedReadOnly
ExternalReference
```

`LiveSync` and external writes are not foundation defaults.

### 4.4 DataSnapshot

A `DataSnapshot` is the reproducible Project-facing artifact.

Required metadata where observable:

```text
source_id
provider_revision/version/etag/commit
selected source objects/files/tables
query or selector digest
retrieved_at
adapter_id/version
schema_fingerprint
raw digests
normalized digests
row_count/file_count/byte_count
materialization format
transform chain
classification
license/terms metadata
reproducibility_state
```

Candidate reproducibility states:

```text
Exact
Strong
Partial
Unknown
```

Do not label a mutable database result `Exact` unless the adapter can prove a stable snapshot/version and inputs are pinned.

## 5. Credential architecture

Credentials must follow a reference model:

```text
DataSourceManifest
    -> CredentialRef
        -> admitted secret-store boundary
```

Never store secret values in:

- Project objects;
- source manifests;
- evidence files;
- logs;
- model context;
- extension manifests;
- import receipts;
- analytics query receipts.

The source adapter receives only the credential capability required for the active operation and must not be able to enumerate unrelated credentials.

Credential states should include:

```text
Present
Missing
Expired
Denied
NeedsUserAction
Unavailable
```

## 6. Foundation source adapters

### 6.1 Local files/folders

Priority formats:

- CSV;
- TSV;
- Parquet;
- Arrow IPC / Feather if qualified;
- JSON;
- JSONL.

Optional later formats require their own hostile-input qualification.

Rules:

- inspect before full ingest where possible;
- file identity includes digest, size and path-at-import as metadata, but path is not identity;
- imported source bytes or canonical normalized representation are content-addressed/evidence-linked as current MedScale architecture permits;
- source file modification after import does not silently mutate the existing `DataSnapshot`;
- refresh creates a new snapshot/revision.

### 6.2 Local database/file sources

Candidate:

- SQLite source file;
- DuckDB file if qualification demonstrates value.

These are source databases, not the MedScale canonical vault.

Never open a source SQLite/DuckDB file using code paths that could confuse it with canonical MedScale storage authority.

### 6.3 PostgreSQL

Foundation posture:

- read-only credentials/role strongly preferred;
- TLS posture explicit;
- schema/table discovery;
- bounded preview;
- snapshot/materialization;
- query timeout;
- statement cancellation;
- no DDL/DML in foundation;
- connection string secrets never persisted in plaintext.

### 6.4 MySQL/MariaDB

Same foundation posture as PostgreSQL with provider-specific TLS/schema behavior captured in adapter evidence.

### 6.5 SQL Server

Same read-only posture. Authentication variants and driver/native dependency choices are evidence-selected during promotion.

### 6.6 Generic ODBC

Defer until native adapter coverage is measured. ODBC broadens driver/native complexity and must not be the automatic foundation simply because it supports many databases.

## 7. Kaggle adapter

### 7.1 User experience

Users can:

- search or paste a Kaggle dataset handle;
- inspect metadata, files, size, version and license information exposed by the provider;
- select a specific dataset version where available;
- select exact files/directories;
- estimate disk usage;
- authenticate through a `CredentialRef` when required;
- download into the acquisition quarantine/staging path;
- validate/digest;
- materialize an immutable MedScale snapshot;
- attach the snapshot to a Project.

### 7.2 Provider constraints

Kaggle datasets, competition data and notebook outputs have different terms/consent behavior. The adapter must not pretend one policy applies to all.

Competition data may require rule acceptance in the user's Kaggle account. If provider access fails because terms/rules are not accepted, MedScale reports `NeedsUserAction`; it must not automate acceptance.

### 7.3 Runtime posture

The official `kagglehub`/Kaggle client ecosystem may be used behind an isolated adapter if that is the most robust route. Python is not loaded into the trusted Desktop process merely for Kaggle.

The adapter should pin its own exact client/runtime version in the receipt.

### 7.4 Writes

Dataset upload, version creation, competition submission and notebook execution are out of foundation scope and require explicit later effect authority.

## 8. Hugging Face dataset adapter

### 8.1 User experience

Users can:

- search or paste `namespace/dataset`;
- inspect the dataset card/repository metadata;
- see gating/private-access requirements;
- select exact revision/commit/tag and files;
- inspect size/format/license metadata where available;
- authenticate with a `CredentialRef` for gated/private data;
- download through the governed network path;
- digest and validate files;
- materialize/import as a `DataSnapshot`;
- retain exact Hub repository revision in provenance.

### 8.2 Safety rule

The default MedScale dataset path downloads repository files/data. It does not execute remote dataset scripts, arbitrary Python or provider code in the trusted process.

If a future format genuinely requires executable loading logic, that logic must be treated as untrusted worker code under Compute/Extension policy.

### 8.3 Large datasets

Support staged/partial file selection first. Lazy mounting/streaming may be evaluated later but must preserve explicit network activity, cache policy and revision binding.

### 8.4 Model Hub distinction

Dataset sources and model Packs are separate concepts even though both may originate on Hugging Face Hub. Dataset import must not silently admit model executable/runtime authority.

## 9. Source refresh semantics

`Refresh` never mutates an existing immutable snapshot.

Flow:

```text
existing source
-> inspect current provider revision
-> compare to last acquisition
-> build RefreshPlan
-> user/policy authorize network/size
-> acquire new bytes/result
-> validate
-> create new DataSnapshot
-> RefreshReceipt
-> optionally mark Project's preferred snapshot pointer
```

Project runs already bound to an older snapshot remain bound to it.

## 10. Mutable databases and reproducibility

Database connectors are inherently tricky because source rows can change without an immutable provider revision.

Preferred qualification order:

1. database-native snapshot/transaction semantics if they can be pinned and later identified;
2. materialize exact query result into immutable Parquet/Arrow snapshot;
3. record source timestamp/transaction/isolation metadata;
4. classify reproducibility honestly.

Canonical downstream Analytics should operate on a MedScale `DataSnapshot` by default. Direct live database analytics is an explicit mode with weaker reproducibility labeling.

## 11. Import transformation model

The acquisition layer may perform bounded, explicit transforms such as:

- archive extraction;
- encoding normalization;
- CSV delimiter/schema parsing;
- type coercion under a declared schema;
- conversion to Parquet/Arrow;
- column rename/mapping;
- row filtering when explicitly selected;
- partitioning.

Every transform is recorded in `ImportReceipt`.

Do not hide data cleaning inside the adapter. Rich cleaning belongs to Analytics/workflows and produces derived artifacts.

## 12. Classification/privacy interaction

Before the later Privacy Gate exists, source imports use existing MedScale data classification and local authority. Promotion must define the exact bridge.

Once Privacy Gate is canonical:

- imported source classification propagates into snapshots;
- network export from a source is separately evaluated from network import;
- external providers never receive raw Project context merely because the user connected a dataset account;
- a source credential does not grant a MedAgent/Extension permission to use that source.

## 13. Network model

All remote source traffic goes through MedScale's admitted network authority/broker or an isolated worker whose network lease is granted by that authority.

Adapter network policy includes:

```text
allowed provider/domain set
redirect policy
TLS requirements
request size/time limits
download size quota
concurrency
proxy policy
credential scope
retry policy
```

Retries must be idempotent for read operations and bounded.

## 14. Quarantine and hostile input

Remote/local imported bytes are untrusted.

Pipeline:

```text
acquire
-> quarantine/staging
-> size/type validation
-> archive bomb/path traversal checks where applicable
-> format parser isolation according to engine-placement policy
-> schema inspection
-> digest
-> optional malware/policy scanning where applicable
-> materialize admitted snapshot
```

Do not parse complex hostile formats in the trusted process solely for convenience.

## 15. UI states

Every adapter must represent:

```text
Disconnected
Connecting
Ready
NeedsCredentials
NeedsUserAction
Denied
RateLimited
Offline
Unavailable
Stale
Changed
Importing
Cancelled
Failed
Corrupt
Unsupported
```

No infinite spinner and no generic `Something went wrong` as the only evidence.

## 16. CLI candidate shape

Exact syntax follows live CLI conventions, but semantic commands should cover:

```text
medscale source add
medscale source list
medscale source show
medscale source test
medscale source discover
medscale source preview
medscale source import
medscale source refresh
medscale source snapshots
medscale source detach
```

Provider-specific configuration may be expressed through subcommands or typed config files, not untyped key/value soup if avoidable.

## 17. Data-source extension contract

The Foundation ships a small trusted/qualified adapter set. Community/institutional connectors later implement the same source contract through the Extension/Adapter boundary.

A connector extension never receives direct Core storage handles. It implements capabilities such as:

```text
discover()
preview()
acquire()
health()
```

through bounded IPC/host calls and returns candidate manifests/bytes/results for Core admission.

## 18. Performance and scale dimensions

Qualification must measure:

- million-row local CSV/Parquet ingest;
- very wide tables;
- many small files;
- multi-GB provider download with cancellation/resume where supported;
- schema discovery on databases with many tables;
- snapshot materialization throughput;
- disk-space preflight;
- low-space failure;
- interrupted download/import recovery;
- connection pool/resource cleanup;
- large schema rendering in Desktop.

Do not claim warehouse-scale support from a single benchmark.

## 19. Security test families

Required adversarial cases include:

- credential leakage into logs/receipts;
- malicious connection strings;
- redirect credential exfiltration;
- provider path traversal;
- archive bombs;
- malformed CSV/Parquet/JSON;
- source DB timeout/cancellation;
- schema name injection into generated queries;
- symlink/path confusion for local files;
- TOCTOU change between file inspection and import;
- provider version changes mid-download;
- stale cached credential;
- wrong account/private dataset denial;
- extension connector attempting ambient filesystem/network access;
- source refresh attempting to overwrite old snapshot.

## 20. Acceptance contract

Candidate Spec 075 cannot close until at least:

```text
LOCAL_FILE_IMPORT=PASS
LOCAL_SNAPSHOT_REOPEN=PASS
PROJECT_ATTACHMENT=PASS
SCHEMA_FINGERPRINT=PASS
IMMUTABLE_REFRESH=PASS
CREDENTIAL_PLAINTEXT_LEAK=DENIED
REMOTE_NETWORK_BROKER_ENFORCEMENT=PASS
ONE_RELATIONAL_DATABASE_ADAPTER=PASS
KAGGLE_EXACT_VERSION_IMPORT=PASS_OR_EVIDENCE_BOUND_EXTERNAL_BLOCKER
HF_EXACT_REVISION_IMPORT=PASS_OR_EVIDENCE_BOUND_EXTERNAL_BLOCKER
INTERRUPTED_IMPORT_RECOVERY=PASS
HOSTILE_IMPORT_FIXTURES=PASS
REAL_PHI_USED=false
```

Provider-specific external outages may block a live external qualification, but they do not justify fabricating success. Offline contract tests/recorded fixtures remain required.

## 21. Explicit non-goals

Foundation does not include:

- database writes/DDL;
- generic ETL orchestration platform;
- continuous CDC;
- automatic cloud sync;
- data warehouse administration;
- arbitrary notebook execution;
- provider credential sharing with agents/plugins;
- hidden remote-code execution;
- automatic competition submissions;
- making Python a required MedScale Desktop runtime;
- replacing existing FHIR authority with generic Data Sources.
