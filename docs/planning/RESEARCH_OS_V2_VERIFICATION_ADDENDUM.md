# Research OS V2 Verification Addendum

**Status:** Planning verification addendum — not implementation authority  
**Applies to:** Data Source Fabric, R Workspace, Community Extensions, and the renumbered V2 program through candidate Spec 092.

This addendum extends `RESEARCH_OS_VERIFICATION_MATRIX.md` and `RESEARCH_OS_ACCEPTANCE_FRAMEWORK.md`.

## 1. Universal rule

No feature is accepted because its happy path works.

Every promoted unit must prove:

```text
contract correctness
storage/migration correctness
Core authority
surface parity
failure semantics
security/adversarial behavior
resource/scale behavior
exact-head CI
post-merge main verification
```

`SKIPPED`, `UNKNOWN`, `UNAVAILABLE`, `PENDING`, or an untested provider/platform are not PASS.

## 2. Data Source Fabric verification campaign

### 2.1 Local files

Required fixtures:
- valid CSV/TSV/JSONL/Parquet/Arrow fixture set;
- malformed headers/rows/schema drift;
- oversized fields;
- truncated/corrupt files;
- file changes between preview and import;
- symlink/path traversal fixtures where folders/archives are admitted;
- insufficient disk;
- interrupted copy;
- duplicate import;
- Unicode/RTL filenames and values.

Required assertions:

```text
preview_is_bounded=true
changed_since_preview_detected=true
raw_bytes_not_overwritten=true
snapshot_digest_stable=true
schema_fingerprint_stable=true
corrupt_input_rejected=true
classification_propagated=true
```

### 2.2 Database sources

At minimum qualify the actual foundation drivers selected by the promoted spec.

Required adversarial/invariant tests:
- wrong/expired credential;
- secret not present in logs/receipts;
- network denied;
- unavailable host;
- TLS/config mismatch where supported;
- permission-limited user;
- large table preview bounds;
- cancellation;
- connection timeout;
- SQL injection values are parameterized;
- DDL/DML rejected in read-only foundation path;
- multi-statement bypass rejected;
- schema changes after preview;
- transaction/snapshot facts captured where available;
- connection pooling cannot cross authority/security context;
- source deletion does not delete materialized snapshots.

### 2.3 Kaggle acquisition

Use only permitted/synthetic/public test datasets suitable for CI/qualification.

Prove:
- unauthenticated/public path if supported by chosen adapter;
- credential-handle path if required;
- token does not appear in logs;
- exact dataset/version identity captured where provider exposes it;
- file list/size captured where available;
- cancellation/partial acquisition state;
- corrupted local cache rejected;
- retry does not duplicate canonical snapshots;
- rate-limit/provider-unavailable states remain explicit;
- provider code is not executed merely to acquire data;
- license/metadata capture behavior is deterministic and honest.

### 2.4 Hugging Face dataset acquisition

Prove:
- dataset repo identity and exact revision/commit where available;
- selected files recorded;
- gated/private access failure remains explicit;
- token never enters model prompts/logs/receipts;
- remote-code execution is not enabled on trusted import path;
- unsupported script-only dataset path fails closed or is isolated behind an explicitly qualified worker;
- file digests and snapshot receipt survive restart;
- cache invalidation does not silently change existing snapshots.

### 2.5 Snapshot reproducibility

Given one admitted source fixture, repeated snapshot materialization with unchanged source revision MUST either:

- produce identical canonical content digest/schema fingerprint; or
- document a source/format property that prevents byte identity while proving normalized semantic identity under an explicit contract.

A changed remote revision must produce a new `DataSnapshot` identity/revision, never mutate the old snapshot.

## 3. Data Source scale campaign

Measure at least:

```text
10 GB-class local-file planning path without loading whole file in memory
10M-row-class tabular scan/preview fixture or equivalent generated fixture
1000-table schema discovery fixture where practical
100 concurrent/queued source-health records without UI lockup
large remote acquisition cancellation/resume behavior
```

Actual fixture sizes may be reduced in routine CI, but the promoted spec must define a separately executed scale campaign and record exact hardware/data sizes.

## 4. R Workspace verification campaign

### 4.1 Workspace determinism

From the same `RWorkspaceManifest` and same available runtime/package sources:

- staged filenames/paths are deterministic;
- input digests match the manifest;
- `.Rproj`/README/config generation is stable;
- no unlisted vault path is exposed;
- deleting/regenerating the workspace does not mutate MedScale artifacts.

### 4.2 IDE isolation

Prove:

```text
external_ide_receives_workspace_path_only=true
vault_db_handle_exposed=false
vault_master_key_exposed=false
source_credentials_exposed=false
ambient_project_directory_exposed=false
```

Test RStudio/Positron launch as optional platform capabilities, not as mandatory CI dependencies where unavailable.

### 4.3 Compute-mediated R

Using a small deterministic R fixture:
- exact input snapshot staged;
- exact R version recorded;
- `renv.lock` digest recorded;
- successful run output digest recorded;
- syntax/runtime failure remains `ScriptFailed`;
- timeout/cancel works;
- network denied profile prevents package/network access;
- package restore failure remains explicit;
- stdout/stderr secret-redaction fixture;
- output outside allowed directory is rejected/quarantined;
- output contract mismatch is rejected;
- late result after revocation is quarantined.

### 4.4 Publication

Prove output publication is explicit:

```text
no_directory_auto_import=true
publish_requires_user_or_authorized_action=true
output_schema_validated=true
output_digest_recorded=true
classification_propagated=true
RRunReceipt_links_inputs_outputs=true
```

## 5. Community Extensions verification campaign

### 5.1 Package admission

Fixtures include:
- valid signed extension;
- invalid signature;
- altered package after signature;
- unsupported API version;
- unsupported MedScale version;
- missing license/provenance metadata;
- dependency policy violation;
- malformed manifest;
- duplicate extension/version digest mismatch;
- revoked package.

### 5.2 Capability escape

A malicious test extension attempts:

```text
read arbitrary filesystem
write arbitrary filesystem
open arbitrary network socket
read environment secrets
read OS keychain
open vault DB
read vault master key
read unrelated Project artifact
read PHI above granted ceiling
use clipboard
use microphone/camera
invoke Core mutation not granted
call another extension state
escalate capabilities after install
```

Every non-granted action must fail deterministically and produce the required audit/receipt without leaking the protected value.

### 5.3 Sandbox failure

Test:
- infinite loop/CPU exhaustion;
- memory exhaustion;
- process crash/trap;
- host API oversized payload;
- malformed IPC;
- timeout;
- cancellation;
- stale/revoked grant;
- runtime unavailable;
- worker restart.

The trusted Desktop/Core must remain healthy.

### 5.4 Update and rollback

Prove:
- same-capability patch update can preserve grants only under explicit policy;
- new capability forces visible consent;
- removed capability is revoked immediately;
- rollback to prior admitted version works;
- extension-created canonical artifacts remain readable after extension disable/uninstall where their owning schema promises it;
- registry outage does not disable already-installed offline-capable extension;
- revoked extension cannot silently reactivate.

### 5.5 Registry attacks

Test metadata/package mismatch, stale cache, revoked publisher, duplicate publisher/name confusion, dependency confusion, malicious search metadata, and package URL redirect policy.

Registry descriptions/readmes are untrusted display content and cannot execute code or change capability decisions.

## 6. Extension SDK compatibility matrix

Every released Extension API version must record:

```text
host_api_version
minimum_supported_extension_api
maximum_supported_extension_api
migration/deprecation window
known incompatible changes
```

At least one previous supported API fixture must remain in CI during the declared compatibility window.

## 7. Cross-plane integration tests

Required once owning specs exist:

### Data Source -> Analytics

```text
remote/local source
-> immutable DataSnapshot
-> Analytics QueryReceipt
-> source refresh
-> prior query remains pinned to old snapshot
```

### Data Source -> Knowledge

```text
snapshot/document import
-> index
-> source changes
-> old index marked stale/rebuild required
-> no silent evidence rebinding
```

### Data Source -> R

```text
DataSnapshot
-> RWorkspaceManifest
-> staged Parquet/Arrow
-> R run
-> output publication
-> Project artifact
```

### Extension -> Data Source

A test connector extension returns a source candidate through the normal 075 contract. It cannot bypass credential/network/import policy.

### Extension -> Analytics/Compute

An analysis extension receives only explicit inputs/capabilities and returns candidate outputs through normal admission.

### Extension -> Browse

Network/browser capability uses Governed Browse/network authority; direct socket access remains denied unless a separately typed network grant explicitly permits a narrow destination.

## 8. Whole-platform V2 additions

Candidate Spec 092 must include integrated campaigns for:

- source credential rotation while Projects remain open;
- remote source outage during Analytics/R/Knowledge workflows;
- snapshot refresh while multiple users/agents reference old revision;
- extension update during active Project session;
- extension revocation while worker job is running;
- Hub registry unavailable/offline;
- R package repository unavailable;
- Personal mode with all Hub/community features absent;
- air-gapped mode with local file/database sources and manually installed verified Extension Packs;
- backup/restore preserving source/snapshot/R/extension provenance;
- upgrade from pre-075 vault and from earlier extension API versions.

## 9. Evidence naming

Promoted specs should create evidence roots semantically equivalent to:

```text
evidence/075-data-source-fabric/
evidence/086-r-workspace/
evidence/087-community-extensions/
evidence/092-whole-platform-qualification/
```

Exact numbers must follow live promotion truth.

Every terminal claim binds:
- exact SHA;
- exact command/tool version;
- fixture identity/digest;
- platform;
- result;
- retained log/artifact location where policy permits.