# Research OS V2 Repository Implementation Map Addendum

**Status:** Planning map — not implementation authority  
**Purpose:** Bind the Data Source Fabric, R Workspace and Community Extensions to the current MedScale repository architecture without pre-authorizing code.

This extends `RESEARCH_OS_REPOSITORY_IMPLEMENTATION_MAP.md`. At promotion time, live code inspection may refine exact module names, but ownership direction is not negotiable without an explicit architecture decision.

## 1. Existing crate ownership remains primary

Default placement remains:

```text
medscale-contracts  -> cross-plane types, manifests, receipts, typed errors
medscale-core       -> authority, commands, validation, admission, orchestration
medscale-storage    -> canonical encrypted persistence/migrations
medscale-network    -> all product network authority/brokered remote access
medscale-pack       -> existing model/runtime Pack semantics; not community extensions by default
medscale-cli        -> CLI surface through Core only
medscale-desktop    -> native Slint UI/adapters through Core only
```

Do not create a new crate merely because a capability has a product name. A new crate is justified only when dependency direction, optional heavy dependencies, isolation, build times or platform/runtime boundaries require it.

## 2. Candidate 075 — Data Source Fabric

### Contracts

Default location:

```text
crates/medscale-contracts/src/data_sources/
```

Candidate modules:

```text
mod.rs
manifest.rs
capability.rs
schema.rs
revision.rs
snapshot.rs
receipt.rs
health.rs
error.rs
```

Reuse existing:
- opaque IDs and scope headers;
- digests;
- authority scope;
- audit/evidence primitives;
- existing data classification once canonicalized by owning privacy contracts.

### Core

Default location:

```text
crates/medscale-core/src/data_sources/
```

Candidate ownership:

```text
create_source
update_source_metadata
remove_source_connection
health_check_source
discover_source
preview_source
plan_import
admit_snapshot
refresh_snapshot
attach_snapshot_to_project
```

Core owns validation/admission and Project attachment. Provider adapters do not write canonical storage directly.

### Storage

Default additions live in the existing canonical encrypted store/migration system.

Semantic tables/families may include:

```text
data_sources
data_source_credentials_refs   # opaque refs/metadata only; secrets remain outside DB if existing key-store contract requires
data_snapshots
source_snapshot_files
source_import_receipts
source_refresh_receipts
```

Exact schema names follow live conventions.

### Network

Remote providers must use `medscale-network` authority.

Candidate adapter routing:

```text
network/data_sources/kaggle
network/data_sources/hugging_face
network/data_sources/postgres_remote_metadata  # only if network crate is current transport owner
...
```

Do not embed independent HTTP clients in Desktop, CLI, Analytics or provider-specific modules if that bypasses broker policy.

### Adapter placement

Pure Rust, memory-safe, bounded format/driver paths may remain in a narrow adapter library after audit.

Heavy Python SDKs, provider runtimes or unsafe/native drivers default to isolated worker/adapter placement.

Potential new crate only if justified:

```text
medscale-data-adapters
```

This crate, if created, contains provider implementations only. It does not own authority/storage.

### CLI

Candidate family:

```text
medscale source list
medscale source add
medscale source show
medscale source health
medscale source discover
medscale source preview
medscale source import
medscale source refresh
medscale snapshot list
medscale snapshot show
```

### Desktop

First-class `Data Sources` workspace in native Slint.

Candidate panels:

```text
Connected
Local
Databases
Kaggle
Hugging Face
Imports
Snapshots
Credentials/Access
Health
```

Credential entry calls the admitted secret-store path. UI must never persist secrets in local app state files/logs.

## 3. Candidate 082 — Analytics integration

Analytics does not own database/provider connections.

It consumes:

```text
ProjectArtifactRef -> DataSnapshot -> governed analytical view
```

Any live-source exploration must use 075 contracts and mark mutable/non-reproducible state explicitly.

## 4. Candidate 083 — Knowledge integration

Knowledge/indexing consumes exact imported/snapshotted artifacts. It does not fetch Kaggle/HF/database content through a parallel downloader.

## 5. Candidate 085 — Compute shared worker substrate

The Compute worker substrate is the preferred execution home for:

- R scripts;
- heavy source adapters;
- extension isolated workers;
- provider Python SDKs when direct safe/native adapters are not justified.

Workers receive exact staged capabilities/inputs, never ambient vault mounts.

## 6. Candidate 086 — R Workspace

### Contracts

Default:

```text
crates/medscale-contracts/src/r_workspace/
```

Candidate modules:

```text
manifest.rs
runtime.rs
staging.rs
receipt.rs
output.rs
error.rs
```

### Core

Default:

```text
crates/medscale-core/src/r_workspace/
```

Candidate commands:

```text
create_workspace
regenerate_workspace
inspect_workspace
launch_external_ide
submit_r_job
publish_r_output
archive_workspace_metadata
```

### Staging

Staging may justify a narrow utility module/crate if platform filesystem semantics are substantial. It must not become an authority owner.

Workspace files live outside the canonical vault DB in an explicit user-controlled/cache/workspace root, with metadata/digests in canonical storage.

### Compute

Automated `Rscript` uses the 085 Compute job path. R-specific code prepares runtime/job manifests; it does not spawn unrestricted processes directly from Desktop.

### Desktop

`R Workspace` appears from Project/Data/Analytics contexts.

Candidate actions:

```text
Create R Workspace
Open in RStudio
Open in Positron
Open Folder
Run Script
Inspect Environment
Publish Outputs
```

Unavailable IDE/runtime is a normal explicit state.

## 7. Candidate 087 — Community Extensions

### Contracts

Default:

```text
crates/medscale-contracts/src/extensions/
```

Candidate modules:

```text
manifest.rs
publisher.rs
package.rs
capability.rs
grant.rs
host_api.rs
install.rs
runtime.rs
registry.rs
receipt.rs
error.rs
```

Do not reuse model Pack types if that obscures different trust/capability semantics.

### Core

Default:

```text
crates/medscale-core/src/extensions/
```

Core owns:

```text
inspect_extension_pack
verify_extension_pack
install_extension
grant_capabilities
revoke_capability
activate_extension
disable_extension
update_extension
rollback_extension
uninstall_extension
invoke_extension_contribution
admit_extension_output
```

### Storage

Semantic families:

```text
extension_publishers
extension_packages
extension_installs
extension_grants
extension_state_metadata
extension_runtime_receipts
extension_registry_cache
```

Extension arbitrary internal state should live in a bounded extension-specific state area with quota/schema/version, not arbitrary canonical table creation.

### Runtime

Executable extension runtime must be physically separable from Core/Desktop.

Possible future crate/processes after evidence:

```text
medscale-extension-host
medscale-extension-worker
```

WASM/Wasmtime, Extism or another runtime is evidence-selected. The contract is stable even if the runtime changes.

The host owns no master key and receives only brokered capabilities.

### Host API

Host API calls route back to Core/network/Compute through typed IPC/capability tokens. Never pass raw Rust object pointers, DB handles or secret-store handles to untrusted extension code.

### UI

Trusted Slint Desktop renders MedScale-owned components from declarative contribution models. Extensions do not inject arbitrary JS/WebView/native DLL UI into the shell by default.

### Community Registry / Hub

Hub owns registry service/data only after 084 exists.

Candidate Hub service modules:

```text
registry metadata
publisher identity/signatures
release index
revocation list
review tier/policy metadata
artifact storage/reference
search/discovery
```

Registry has no authority to auto-install or auto-grant capabilities on clients.

### SDK

SDK lives under a clearly versioned developer surface, for example:

```text
sdk/extensions/
examples/extensions/
docs/extensions/
```

Do not put SDK examples inside production trusted runtime code.

## 8. Source connector extensions

A community connector extension implements the 075 adapter SPI through the extension host.

Flow:

```text
Extension connector
-> Extension Host API
-> Data Source adapter broker
-> network/credential capability check
-> candidate source metadata/files
-> Core validation
-> normal DataSnapshot/ImportReceipt
```

There is no `PluginDataSnapshot` or plugin-owned credential database.

## 9. Research Pack interaction

Candidate 089 Research Packs remain declarative domain semantics in their owning Pack path.

If a Research Pack needs executable community behavior:

```text
ResearchPackManifest
-> optional required/recommended Extension IDs/API ranges
-> separate Extension install + capability consent
```

Never bundle executable extension authority implicitly inside a Research Pack install.

## 10. Dependency-direction rules

Required direction:

```text
Desktop/CLI
   -> Core
      -> contracts
      -> storage
      -> network broker
      -> Compute/Extension host brokers
```

Forbidden:

```text
Desktop -> database driver direct
Desktop -> Kaggle/HF HTTP direct
Analytics -> source password
R process -> vault DB
Extension -> vault DB
Extension -> OS keychain
Extension -> unrestricted network
Provider adapter -> canonical storage direct
```

## 11. Migration rule

Each promoted unit adds schema through the current MedScale migration mechanism and proves:

- pre-unit vault opens/migrates;
- new objects persist/reopen;
- failure leaves recoverable state;
- backup/restore preserves new metadata;
- disabling optional runtime/provider does not make existing canonical snapshots/artifacts unreadable.

## 12. Test placement

Prefer focused tests next to owning crates plus integration/evidence harnesses under existing repository conventions.

Required test families include:

```text
contracts serialization/invariants
Core authority denial
storage migration/recovery
provider adapter conformance
secret redaction
network denial
Desktop/CLI parity
Compute isolation
Extension host capability escape
R workspace staging/output admission
```

Do not create an entirely separate test framework for Research OS if current Rust/test/evidence infrastructure can carry the proof.