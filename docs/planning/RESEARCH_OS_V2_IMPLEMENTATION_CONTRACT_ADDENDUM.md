# Research OS V2 Implementation Contract Addendum

**Status:** Planning contract addendum — not implementation authority  
**Applies to:** Candidate Specs 075-092 after `RESEARCH_OS_PROGRAM_AMENDMENT_001_DATA_EXTENSIONS.md`  
**Does not change:** already-promoted Spec 074 scope

This addendum extends `RESEARCH_OS_MASTER_IMPLEMENTATION_CONTRACT.md`. Where candidate 075+ numbering or dependency statements conflict, this V2 addendum and the amendment/roadmap V2 are the current planning truth. Live canonical repository authority still wins at promotion time.

## 1. Cross-plane invariant: one governed data ingress

Every local/remote dataset, database result, Kaggle/Hugging Face acquisition, institutional object, R input, Analytics input, Knowledge source, Compute input and extension-provided source must enter through a MedScale-owned data/source contract.

No subsystem may invent a parallel connector identity, credential store, source revision model, snapshot model or import receipt.

Minimum shared semantics:

```text
DataSourceId
DataSourceManifest
CredentialRef
SourceCapability
SourceSchema
SchemaFingerprint
SourceRevision
ImportPlan
DataSnapshot
ImportReceipt
RefreshReceipt
SourceHealth
```

## 2. Data source authority boundaries

### 2.1 Source definition is not source truth

A `DataSourceManifest` describes how MedScale may locate/query/acquire source material. It does not make remote content canonical clinical truth.

### 2.2 Credentials

Secrets MUST NOT be serialized into:

- Project artifacts;
- `DataSourceManifest`;
- `ImportReceipt`;
- logs;
- prompts;
- agent context;
- extension manifests;
- R workspace manifests;
- Analytics receipts.

Only opaque `CredentialRef` handles cross trusted boundaries.

### 2.3 Read-only foundation

Foundation source capabilities are:

```text
discover
inspect_metadata
preview
read
snapshot/import
refresh_snapshot
health_check
```

External write/delete/upload/submit capabilities are separate later grants and are never inferred from read access.

### 2.4 Snapshot rule

A completed reproducible analytical/retrieval/research run MUST bind either to:

1. an immutable MedScale `DataSnapshot`; or
2. an explicitly declared `MutableSourceBinding` with partial-reproducibility state and captured remote revision/query facts.

UI MUST distinguish live preview from frozen snapshot.

### 2.5 Import transforms

Normalization transforms never overwrite raw acquired bytes. A normalized table/file is a derived artifact with:

- parent snapshot/source reference;
- transform identity/version;
- input/output digest;
- schema before/after;
- classification propagation;
- loss/normalization notes;
- receipt.

## 3. Adapter contract

Every source adapter implements a versioned interface semantically equivalent to:

```text
capabilities()
describe_source()
health_check()
discover()
preview(request)
plan_import(request)
acquire(plan)
resume(acquisition_state)
materialize_snapshot(acquired_candidate)
```

An adapter does not write the canonical vault directly. Adapter outputs are candidates admitted by Core/storage authority after validation.

## 4. Database adapter contract

Foundation DB adapters MUST:

- separate secret handles from non-secret connection metadata;
- default to read-only sessions/transactions where driver/source supports it;
- bound connection/query/row/byte/time limits;
- reject DDL/DML and unsafe multi-statement execution for foundation query paths;
- expose schema/table metadata through typed structures, not concatenated SQL strings;
- use parameter binding for user values;
- record source engine/driver/adapter versions;
- capture transaction/isolation/snapshot facts when available;
- redact server errors before persistence/UI if they may contain secrets;
- cancel/close connections deterministically;
- never reuse a connection across authority scopes unless the pool key includes the full security context.

## 5. Local file contract

Local import MUST record:

- original selected path only under local policy; persisted artifacts prefer opaque/source-relative identity where privacy requires;
- file size and modification metadata;
- content digest after stable read;
- race/change detection between preview and import;
- format parser identity/version;
- schema fingerprint;
- hostile-input limits;
- archive traversal/symlink defenses where archive/folder import is admitted.

A file changed between preview and materialization is `ChangedSincePreview`, never silently accepted as the previewed object.

## 6. Kaggle/Hugging Face acquisition contract

Remote dataset adapters MUST:

- route all networking through MedScale network authority or isolated adapter network capability;
- pin exact dataset identity and available revision/version information;
- enumerate selected files before acquisition where possible;
- capture provider metadata/license/terms identifiers when exposed;
- estimate size before download when available;
- support interruption/cancellation;
- verify downloaded sizes/digests when provider metadata enables it, otherwise compute local digests;
- quarantine before parser admission;
- prohibit provider-supplied arbitrary code in the trusted path;
- never auto-enable remote-code trust options;
- never expose provider credentials to model/agent context.

## 7. R Workspace contract

R is an external/bounded execution environment, never a second authority plane.

### 7.1 Workspace manifest

`RWorkspaceManifest` contains only admitted references and reproducibility metadata:

```text
workspace_id
project_id
input_snapshot_refs
staging_plan
r_version_requirement
lockfile_ref/digest
entrypoints
output_contract
classification
created_by
created_at
```

### 7.2 Staging

- input data are copied/materialized/exported from exact MedScale snapshots;
- staged inputs are read-only where platform permits;
- no vault DB, vault key, credential database or ambient Project directory is mounted;
- paths crossing into the workspace are explicit manifest entries;
- workspace regeneration is deterministic from the manifest within external dependency availability constraints.

### 7.3 External IDE launch

`Open in RStudio` / `Open in Positron` launches an external application with the workspace path only. Launch does not confer MedScale API authority.

### 7.4 Compute-mediated R execution

`Rscript` or other automated R execution uses normal Compute contracts:

```text
ComputeJobManifest
exact staged inputs
network policy
filesystem policy
resource limits
secret policy
runtime identity
output contract
```

### 7.5 Output publication

External R output is untrusted candidate material until explicit `Publish to MedScale`:

```text
candidate output
-> file/schema validation
-> digest
-> classification
-> output contract match
-> Core admission
-> Project artifact reference
-> RRunReceipt
```

No directory watcher silently imports files.

## 8. Community Extension contract

### 8.1 Artifact identity

Extension artifacts are distinct from model/runtime Packs and Research Packs.

```text
ExtensionPack
ExtensionManifest
ExtensionPublisher
ExtensionInstallRecord
ExtensionGrant
ExtensionRuntimeReceipt
```

### 8.2 No ambient authority

Default extension capabilities are empty.

Extensions MUST NOT gain ambient access to:

- vault DB or master key;
- OS keychain;
- arbitrary filesystem;
- network;
- clipboard;
- microphone/camera;
- source credentials;
- Project contents;
- PHI/PII;
- Core mutation APIs;
- other extensions' state.

### 8.3 Capability grants

Every capability is typed and scope-bound. Examples:

```text
project.read_metadata(project_id)
artifact.read(artifact_id, revision)
data_source.provider(provider_kind)
network.connect(destination_policy)
compute.submit(profile)
ui.contribute(surface)
audio.process(session_scope)
```

A grant MUST include scope, expiry/revocation semantics, data-class ceiling and audit identity.

### 8.4 Runtime placement

Executable extension code runs by default in:

1. an evidence-qualified WASM sandbox with narrow host API; or
2. an isolated worker process through the capability broker.

Declarative contributions require no executable runtime.

Native dynamic libraries inside the trusted Desktop/Core process are denied unless a future dedicated spec independently proves a narrow exception.

### 8.5 Host API

The Extension Host API MUST be:

- versioned;
- typed;
- capability-mediated;
- bounded in input/output size;
- cancellation/time-out aware;
- deterministic about denial;
- free of raw DB handles, master keys and unrestricted filesystem paths.

### 8.6 UI contributions

Extensions may contribute only declared surface types. They do not inject arbitrary web/native UI code into the trusted shell by default.

Candidate safe contribution classes:

```text
command/action
settings schema
read-only panel data model
data-source provider descriptor
analysis action descriptor
visualization specification
MedAgent tool descriptor
Research Pack augmentation descriptor
```

### 8.7 Install/update

Install pipeline:

```text
artifact acquired
-> package digest
-> signature/provenance
-> manifest/API compatibility
-> license/SBOM/dependency policy
-> malware/static policy
-> capability diff
-> user/admin approval
-> sandbox activation
```

Any new/expanded capability on update requires visible re-consent.

### 8.8 Registry

Community Registry metadata MUST include immutable release identity, publisher identity, version, digest, signature/provenance state, compatibility, requested capabilities, license and revocation state.

Registry listing does not itself prove safety or clinical correctness.

Offline/manual verified installation remains supported.

## 9. Extension/source interaction

A community data-source connector does not create its own source model. It implements the 075 adapter contract and returns normal source candidates/receipts.

An analysis extension does not bypass Analytics/Compute authority.

A browser extension does not bypass Governed Browse/network/privacy authority.

An audio extension does not bypass AudioFlow consent/capture policy.

## 10. Research Packs versus Extensions

Research Pack:
- domain semantics;
- schemas;
- workflows;
- views;
- evidence rules;
- usually declarative.

Extension Pack:
- executable or integration contribution;
- capability-scoped;
- sandboxed/isolated when executable.

A Research Pack may declare optional Extension dependencies, but installing a Research Pack does not silently install/authorize executable Extensions.

## 11. Failure taxonomy additions

Data Source Fabric adds at least:

```text
AuthenticationRequired
CredentialExpired
ProviderUnavailable
RateLimited
SourceNotFound
SourceRevisionUnavailable
ChangedSincePreview
UnsupportedSource
UnsupportedFormat
SchemaChanged
PartialAcquisition
AcquisitionCorrupt
SnapshotConflict
InsufficientDisk
PolicyDenied
NetworkDenied
```

R Workspace adds:

```text
RNotInstalled
IDEUnavailable
RuntimeMismatch
LockRestoreFailed
PackageUnavailable
ComputeDenied
ScriptFailed
OutputContractMismatch
OutputRejected
```

Extensions add:

```text
SignatureInvalid
PublisherUntrusted
ApiIncompatible
CapabilityDenied
CapabilityChanged
SandboxUnavailable
ExtensionCrashed
ExtensionTimedOut
ExtensionQuarantined
ExtensionRevoked
HostApiViolation
```

Do not collapse these into generic `Error` where user action or audit semantics differ.

## 12. Migration / rollback additions

- adding Data Source Fabric does not transform existing artifacts into remote-source objects automatically;
- imported snapshots remain valid if an adapter is later disabled;
- deleting a source connection does not delete already-materialized snapshots without explicit artifact deletion authority;
- R workspace metadata may be deleted independently of canonical published outputs;
- uninstalling an Extension disables executable behavior while preserving extension-created canonical artifacts according to their owning schemas;
- extension rollback restores the previous admitted package/grants, not arbitrary mutable extension state without schema migration proof;
- registry outage cannot break already-installed offline-capable extensions.

## 13. Logging and privacy

Logs MUST redact or omit:

- passwords/tokens/OAuth secrets;
- full sensitive connection strings;
- raw PHI/PII samples;
- R environment variables containing secrets;
- extension secret values;
- dataset sample values unless a dedicated evidence fixture intentionally permits them.

## 14. Implementation order after 074

No implementation agent may infer 075+ authority from this addendum.

When separately promoted, the intended order is:

```text
075 Data Source Fabric
076 Collaboration
077 MedAgent
078 Fleet
079 Privacy
080 Browse
081 AudioFlow Foundation
082 Analytics
083 Knowledge
084 Hub
085 Compute
086 R Workspace
087 Community Extensions
088 AudioFlow Advanced
089 Research Packs
090 Institutional Adapters
091 Federation
092 Whole-Platform Qualification
```

Every unit still requires its own live-truth promotion packet.