# MedScale Community Extensions Product Plan

**Status:** Planning candidate — no implementation authority  
**Candidate owner:** V2 candidate Spec 087  
**Depends on:** Privacy Gate + MedScale Hub + MedScale Compute; Data Source Fabric integration for connector extensions  
**Reference UX:** Obsidian-like discoverability/community ownership, but with stronger healthcare/research safety boundaries

## 1. Product goal

Create a community ecosystem where researchers, labs, institutions and developers can extend MedScale without waiting for the MedScale core team, while preserving one authority model, local-first operation, explicit permissions and reproducible provenance.

The product goal is:

> **Obsidian-level extensibility with MedScale-level authority, privacy and isolation.**

## 2. Product surfaces

### MedScale Extensions

Desktop surface:

```text
Extensions
├── Installed
├── Community
├── Updates
├── Permissions
├── Developer
└── Quarantined
```

### MedScale Hub Community Registry

Hub surface:

```text
Community Registry
├── Featured
├── Data Sources
├── Analytics
├── MedAgent Tools
├── Visualizations
├── Research
├── Audio
├── Import / Export
└── Institutional
```

Personal/offline mode remains able to install an explicitly sideloaded signed Extension Pack without requiring a MedScale-hosted cloud service.

## 3. Artifact taxonomy

Do not collapse these concepts:

```text
Model Pack       = admitted model/runtime artifact
Research Pack    = declarative domain semantics/workflow/schema contribution
Extension Pack   = executable or declarative community extension
Data Snapshot    = immutable acquired/materialized data
```

An Extension may integrate with a Research Pack, but it is not automatically trusted because the Research Pack is trusted.

## 4. Extension contribution classes

Candidate classes:

### 4.1 Data Source Connector

Implements bounded `discover`, `preview`, `acquire`, `health` semantics through the Data Source Fabric contract.

Examples:

- REDCap;
- custom LIMS;
- proprietary database;
- public scientific repository;
- institutional object store.

### 4.2 Importer / Exporter

Converts a bounded external format into candidate MedScale artifacts or exports approved project artifacts.

### 4.3 MedAgent Tool

Exposes a typed tool call to MedAgent through explicit capability grants and structured input/output schemas.

### 4.4 Analytics Action

Adds a bounded analysis operation or transformation. Heavy/arbitrary computation runs through Compute.

### 4.5 Visualization

Adds a safe visualization specification/renderer integration without unrestricted Desktop memory/DB access.

### 4.6 AudioFlow Processor

Adds enhancement, segmentation or other bounded audio processing through the Audio/Compute contract.

### 4.7 Research Pack Augmentation

Adds tools/views/importers for an existing domain pack while respecting that Pack's owned schema and authority.

### 4.8 Institutional Adapter

Connects organization infrastructure under explicit credentials/network/effect authority.

## 5. Extension Pack manifest

Candidate `ExtensionManifest` fields:

```text
extension_id
publisher_id
name
description
version
api_version
minimum_medscale_version
maximum_medscale_version?
package_digest
signatures[]
source_repository?
source_revision?
license
notice_files[]
sbom_digest?
entrypoint_kind
entrypoints[]
capabilities[]
network_policy
filesystem_policy
data_class_policy
credential_types[]
project_permissions[]
tool_contributions[]
data_source_contributions[]
ui_contributions[]
settings_schema
state_schema_version
migration_contract?
update_policy
rollback_policy
homepage?
support_url?
```

Unknown manifest fields in security-sensitive sections should fail closed according to versioning policy.

## 6. Runtime classes

### 6.1 Declarative extension

No arbitrary code. Contributes metadata/configuration such as:

- menu/command declarations;
- templates;
- schemas;
- static visualization specs;
- source definitions backed by already-admitted generic adapters.

Use this class whenever code is unnecessary.

### 6.2 WASM extension

Candidate safe-default executable class if an exact Wasmtime/alternative runtime qualifies.

Properties:

- no ambient filesystem;
- no ambient network;
- explicit host functions;
- memory/time/fuel limits;
- capability-bound handles;
- deterministic serialization contracts where required;
- no arbitrary native dynamic library loading.

WASI capabilities are not granted wholesale merely because the runtime supports them.

### 6.3 Isolated worker extension

For capabilities that require native libraries, Python/R runtimes, database drivers or larger dependencies.

Runs under MedScale Compute/worker isolation with:

- staged exact inputs;
- explicit filesystem lease;
- explicit network lease;
- explicit credentials lease;
- resource limits;
- output validation;
- receipt/evidence.

### 6.4 Trusted native extension

Not a community default.

A native library linked/loaded into the trusted MedScale process requires a separate first-party/core admission decision and cannot be installed from the community registry as ordinary plugin behavior.

## 7. Capability model

Extensions start with **zero ambient capabilities**.

Candidate capability families:

```text
project.read_metadata
project.read_artifact:<kind>
project.propose_artifact
project.publish_candidate
source.discover
source.preview
source.acquire
analytics.read_snapshot
analytics.propose_transform
medagent.tool
network.request:<policy-id>
credential.use:<credential-type>
filesystem.workspace_read
filesystem.workspace_write
clipboard.read
clipboard.write
microphone.read
audio.process
notification.show
```

Sensitive capabilities can require per-project or per-use approval even after installation.

No extension can translate `project.read_artifact` into direct vault DB access.

## 8. Permission UX

Before install, show a human-readable capability summary:

```text
This extension wants to:
✓ Read tabular snapshots in projects where you enable it
✓ Connect to api.example.org
✓ Store extension-local settings
✗ It will NOT receive direct vault access
✗ It will NOT receive microphone access
✗ It will NOT receive unrestricted internet access
```

Capability additions in an update require explicit re-consent.

Capability removals do not require re-consent but must not silently delete user data.

## 9. Data-class policy

An extension manifest declares the maximum data classes it is designed to handle, but the declaration does not itself grant access.

Runtime evaluation combines:

```text
extension admission
+ user/project grant
+ artifact classification
+ Privacy Gate decision
+ requested operation
+ destination/network route
```

Default public community extensions should not receive `LOCAL_PHI` simply because they are installed.

Sensitive-data capability may require higher review/admission tiers.

## 10. Credential model

Extensions never receive secret-store enumeration or raw global credentials.

If an extension needs credentials:

```text
extension requests typed credential use
-> user selects/configures CredentialRef
-> broker issues narrow operation/session lease
-> secret injected only to isolated route that requires it
-> extension does not persist plaintext secret
-> receipt records credential reference/type, never value
```

## 11. Network model

No network by default.

Manifest-declared network destinations are an upper bound, not automatic permission.

Runtime route also requires active capability/policy.

Network rules should support:

- exact domains/hosts;
- HTTPS requirement;
- redirect policy;
- request methods;
- request/response size limits;
- rate limits;
- credential attachment policy;
- timeout/cancellation;
- audit receipt.

Dynamic arbitrary host access is high risk and not a default community capability.

## 12. Filesystem model

Community extensions do not receive the MedScale vault path.

Candidate filesystem surfaces:

- extension-local state directory;
- ephemeral scratch directory;
- explicitly staged workspace files;
- user-selected import/export destination through a brokered file picker.

Symlink/path traversal must not escape the granted root.

## 13. UI extension model

MedScale Desktop remains native Slint.

Do not introduce an embedded browser runtime simply because community plugins often use web UI.

Foundation UI contribution options should be bounded/declarative, for example:

```text
commands
menus
toolbar actions
settings forms
artifact actions
simple forms
status cards
visualization specification host
```

A rich extension UI that needs its own web/native application may open an isolated external/worker surface but does not become arbitrary trusted UI code inside Desktop.

The Extension SDK should prioritize stable data/action APIs over unrestricted DOM/widget injection.

## 14. Extension SDK

The SDK needs:

- versioned manifest schema;
- generated typed API bindings;
- local development host;
- capability simulator;
- fixture Project/data generator;
- log viewer with secret redaction;
- contract tests;
- packaging/signing command;
- permission diff tool;
- compatibility checker;
- example extensions;
- publication validation.

Candidate CLI:

```text
medscale extension init
medscale extension dev
medscale extension validate
medscale extension test
medscale extension pack
medscale extension sign
medscale extension inspect
medscale extension publish
```

Publication may remain separate from the local CLI in the first release if Hub API is not yet qualified.

## 15. Community Registry architecture

The Registry is metadata/distribution, not execution authority.

Candidate records:

```text
ExtensionListing
PublisherIdentity
ExtensionRelease
ReviewRecord
SecurityScanRecord
CompatibilityRecord
RevocationRecord
```

Registry install flow:

```text
fetch listing metadata
-> select exact release
-> verify publisher/signature/digest
-> retrieve package
-> verify package again locally
-> inspect manifest/capability diff
-> scan/policy gate
-> user consents
-> install disabled
-> initialize sandbox/state
-> activate under granted capabilities
```

Never execute code before package verification and admission.

## 16. Publisher model

A publisher may be:

- individual developer;
- lab;
- institution;
- company;
- MedScale first-party.

Publisher identity and code signing are separate concepts; account compromise/revocation requires release revocation support.

A verified publisher badge must not imply medical correctness of the extension.

## 17. Review tiers

Candidate tiers:

```text
Sideloaded
CommunityListed
Reviewed
SensitiveDataReviewed
FirstParty
Revoked
```

Tier names are product candidates and can change, but the distinction is necessary.

Review considers:

- manifest validity;
- source/release provenance;
- license/NOTICE;
- SBOM/dependencies;
- code scanning;
- malware indicators;
- obfuscation;
- network behavior;
- capability minimization;
- secret handling;
- update mechanism;
- data retention;
- telemetry;
- security response contact.

Automated scan is evidence, not proof of safety.

## 18. Developer policies

Community-listed extensions must follow policies at least as strict as:

- no hidden/obfuscated behavior without justified reviewed binary artifacts;
- no unapproved telemetry;
- no dynamic ads in MedScale surfaces;
- no self-install/self-update outside MedScale package manager;
- no hidden dependency download during activation;
- no bypass of capability broker;
- no credential persistence in extension state;
- no security control disabling;
- no misleading medical/regulatory claims;
- no silent upload of Project data;
- no PHI handling claim without declared/reviewed capability;
- no malicious or deceptive code.

This draws inspiration from community-directory policies such as Obsidian's while adding MedScale-specific data/security constraints.

## 19. Updates

Every release is immutable and content-addressed.

Update flow:

```text
new release discovered
-> signature/provenance check
-> compatibility check
-> permission diff
-> migration preview
-> user/admin policy
-> install side-by-side/staged
-> migrate extension state
-> health check
-> activate
-> rollback old release if failure
```

No silent capability escalation.

Automatic updates, if later supported, require organization/user policy and still preserve permission-diff rules.

## 20. Extension state

Extension-owned state is isolated from Core canonical objects.

State schema version and migration are declared.

Uninstall options:

```text
Disable only
Uninstall code, keep state
Uninstall code and extension state
```

Uninstall never deletes referenced canonical Project artifacts unless a separate user-authorized Core action does so.

## 21. Revocation and quarantine

MedScale must support:

- local manual disable;
- registry security revocation;
- publisher key revocation;
- package digest block;
- version-specific quarantine;
- network capability emergency disable;
- organization denylist.

Air-gapped installations cannot depend on a live registry for safety, so revocation data must support signed offline updates where the product update model allows.

A revocation does not silently erase evidence required to understand prior runs.

## 22. Receipts

Every extension execution producing Project-relevant output emits an `ExtensionRunReceipt` or subsystem-specific receipt referencing:

```text
extension_id
release version
package digest
publisher/signature identity
capabilities actually used
input artifact revisions/digests
data classifications
network routes used
credential reference types used
started/finished time
exit/result state
output candidate digests
host API version
sandbox/runtime identity
```

## 23. Extension / MedAgent interaction

An installed MedAgent tool extension is not automatically available to every agent.

Three distinct gates:

```text
extension installed
extension enabled for Project
specific MedAgent lane granted tool capability
```

Tool input is structured. The agent does not receive an arbitrary shell or extension internal API.

## 24. Extension / Data Source interaction

Connector extensions implement the Data Source Fabric adapter contract.

The community connector returns discovery/acquisition results to MedScale for validation and admission. It does not create canonical DataSnapshots by writing directly to storage.

## 25. Extension / Compute interaction

Isolated worker extensions use Compute primitives rather than a parallel process manager.

Compute owns:

- process/container/WASM lifecycle as admitted;
- filesystem leases;
- network leases;
- secrets lease;
- resource limits;
- cancellation;
- output quarantine;
- receipts.

Extension Platform owns package identity, permissions and host API contracts.

## 26. Extension / Hub interaction

Hub may sync:

- installed extension identity/version metadata;
- organization allow/deny policies;
- Project enablement policy;
- registry metadata cache.

Do not blindly sync executable extension binaries into every device without compatibility and local admission checks.

## 27. Offline operation

A previously installed Extension Pack can operate offline if its capabilities do not require network.

Registry browsing/update checking naturally requires network but is not required for core MedScale operation.

Organizations may run a private mirrored Community Registry.

## 28. Supply-chain qualification

Required before publication/install:

- exact package digest;
- source/revision metadata where available;
- license/NOTICE;
- dependency/SBOM evidence;
- signature verification;
- malware/static scan;
- capability manifest;
- host API compatibility;
- reproducible-build evidence where a publisher provides it, without falsely requiring it for all ecosystems initially.

Native runtime dependencies require the existing native/FFI admission contract.

## 29. Security/adversarial test families

At minimum:

- plugin tries direct vault path;
- plugin enumerates secret store;
- undeclared network host;
- DNS/redirect escape;
- capability escalation after update;
- symlink filesystem escape;
- WASM host-function abuse;
- worker IPC malformed payload;
- memory/CPU exhaustion;
- fork bomb/process spawn;
- log secret injection;
- output candidate race/change;
- malicious state migration;
- downgrade attack;
- signature substitution;
- package digest mismatch;
- revoked release launch;
- cross-project data access;
- PHI read without sensitive capability;
- tool invocation by unauthorized agent lane.

## 30. Acceptance contract

Candidate Community Extensions closure requires evidence equivalent to:

```text
EXTENSION_MANIFEST_VERSIONED=PASS
SIGNED_PACKAGE_VERIFY=PASS
DECLARATIVE_EXTENSION=PASS
SANDBOXED_EXECUTABLE_EXTENSION=PASS
ZERO_AMBIENT_VAULT_ACCESS=PASS
ZERO_AMBIENT_NETWORK=PASS
CAPABILITY_GRANT_REVOKE=PASS
CAPABILITY_UPDATE_DIFF=PASS
EXTENSION_STATE_MIGRATION_ROLLBACK=PASS
COMMUNITY_REGISTRY_INSTALL_FLOW=PASS
REVOCATION_QUARANTINE=PASS
CONNECTOR_EXTENSION_CONTRACT=PASS
MEDAGENT_TOOL_EXTENSION_CONTRACT=PASS
SUPPLY_CHAIN_SCAN_EVIDENCE=PASS
CROSS_PROJECT_ISOLATION=PASS
REAL_PHI_USED=false
```

## 31. Non-goals

Foundation does not:

- guarantee all arbitrary existing VS Code/Obsidian/Jupyter plugins run in MedScale;
- embed Electron/Chromium solely for plugin UIs;
- allow arbitrary native DLL/dylib loading from the marketplace;
- allow plugins to modify MedScale Core code at runtime;
- make marketplace access mandatory;
- promise extensions are clinically validated;
- let a high star/download count override security policy;
- let extensions bypass Privacy Gate or Compute isolation;
- make community code part of the trusted computing base by default.
