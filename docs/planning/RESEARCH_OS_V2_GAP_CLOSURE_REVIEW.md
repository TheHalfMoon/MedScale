# MedScale Research OS V2 Gap Closure Review

**Status:** Planning gap audit — not implementation authority  
**Branch:** `plan/research-os-data-extensions-amendment`  
**Base at amendment creation:** `a80c33307afc4577790282652e5b20911beb4bbe`  
**Scope:** Program Amendment 001 — Data Source Fabric, R Workspace, Community Extensions, and resulting 075+ renumber/dependency changes

## 1. Purpose

The first Research OS planning packet was review-ready for its then-current scope. Founder direction subsequently added three material product requirements:

1. first-class Data Sources / database and dataset acquisition;
2. first-class R/RStudio/Posit workflow;
3. Obsidian-like community extensibility through MedScale Hub, with stronger healthcare/research security boundaries.

These additions change dependencies and therefore require a new gap audit rather than being appended as isolated features.

This review asks whether any material capability still lacks an owner, contract, authority boundary, failure model, repository placement, verification campaign, or dependency gate.

## 2. V2 planning authority chain

For candidate work after 074, read in this order:

```text
RESEARCH_OS_PROGRAM_AMENDMENT_001_DATA_EXTENSIONS.md
RESEARCH_OS_EXECUTION_ROADMAP.md
RESEARCH_OS_V2_DECISION_REGISTER.md
RESEARCH_OS_V2_IMPLEMENTATION_CONTRACT_ADDENDUM.md
RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md
RESEARCH_OS_V2_REPOSITORY_MAP_ADDENDUM.md
RESEARCH_OS_V2_VERIFICATION_ADDENDUM.md
DATA_SOURCE_FABRIC_PLAN.md
R_WORKSPACE_PRODUCT_PLAN.md
COMMUNITY_EXTENSIONS_PRODUCT_PLAN.md
SOURCE_ADOPTION_MATRIX.md
```

Original V1 planning documents remain useful for unchanged semantics, but old candidate numbering at 075+ is historical planning context when it conflicts with V2.

Live canonical repository authority always wins at promotion time.

## 3. Numbering audit

V2 candidate sequence:

```text
074 Project + Artifact Graph Foundation       [already promoted separately]
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

### Finding

The original merged planning packet still contains historical V1 references such as `075 Collaboration`, `081 Analytics`, and candidate range `074-089` in some documents. They are not silently rewritten because they document the original accepted packet.

### Closure

- V2 Program Amendment explicitly supersedes 075+ candidate numbering for future promotion.
- V2 Plan Index labels original files as V1 where relevant.
- V2 per-spec contracts exist for 075-092.
- V2 repository and verification addenda use the new numbering.
- Any future 075+ promotion MUST bind the semantic candidate name from V2 and reverify that the number remains unused on live main.
- Already-promoted 074 is not renumbered or expanded.

**Status:** CLOSED_FOR_PLANNING.

## 4. Data Source Fabric cross-plane audit

### 4.1 Project integration

Question: can a Project reference source connections/snapshots without turning Project into source authority?

Closure:
- source definitions and snapshots receive normal MedScale identities;
- Project references them as artifacts/references through existing Project/Artifact semantics;
- Project does not store provider credentials or duplicate snapshot payloads.

**Status:** CLOSED_FOR_PLANNING.

### 4.2 Analytics integration

Question: does Analytics create its own database/import layer?

Closure:
- no;
- Analytics consumes exact `DataSnapshot`/governed view revisions from 075;
- live preview is distinct from canonical analysis input;
- mutable sources retain explicit partial-reproducibility state.

**Status:** CLOSED_FOR_PLANNING.

### 4.3 Knowledge/RAG integration

Question: can Knowledge index remote/provider data without source provenance?

Closure:
- no;
- retrieval/index manifests reference admitted snapshots/artifacts/revisions;
- raw provider content is acquired and validated through 075 before indexing;
- stale/deleted snapshot handling remains explicit.

**Status:** CLOSED_FOR_PLANNING.

### 4.4 MedAgent integration

Question: can MedAgent directly open database/provider connections?

Closure:
- no ambient connector access;
- MedAgent requests typed source/query/import tools;
- Core/capability policy checks access;
- credentials stay behind opaque handles;
- source results return bounded typed outputs/receipts.

**Status:** CLOSED_FOR_PLANNING.

### 4.5 Compute integration

Question: can remote workers mount source databases or the vault directly?

Closure:
- default Compute receives exact staged snapshot digests, not vault/source credentials;
- a future source-aware worker needs an explicit short-lived scoped credential/network capability owned by the source/compute contract;
- no ambient vault mount.

**Status:** CLOSED_FOR_PLANNING.

### 4.6 R Workspace integration

Question: can R read directly from MedScale vault or live DB credentials?

Closure:
- foundation R Workspace stages exact snapshots;
- no vault/master key/credential store mount;
- external IDE receives a workspace path only;
- automated R execution uses Compute;
- output returns as untrusted candidate until explicit publication.

**Status:** CLOSED_FOR_PLANNING.

### 4.7 Extension connector integration

Question: can plugins invent arbitrary connector semantics?

Closure:
- no;
- community connector extensions implement the same 075 source adapter contract;
- normal source health/import/snapshot/receipt policy remains mandatory;
- extension sandbox/capability policy adds another boundary but not another source model.

**Status:** CLOSED_FOR_PLANNING.

## 5. Credential model audit

Potential gap: database, Kaggle, Hugging Face, R package mirrors, institutional systems and extensions might each create separate secret handling.

Closure:
- one admitted secret-store boundary;
- manifests/receipts/workspaces/extensions carry only opaque use-scoped references;
- raw secrets never enter Project artifacts, prompts, plugin manifests, canonical receipts or ordinary logs;
- origin/destination/action scope is part of credential-use authority;
- revocation/expiry is explicit.

**Status:** CLOSED_FOR_PLANNING.

## 6. Reproducibility audit

Potential gap: remote/live sources can mutate after analysis.

Closure:
- canonical research operations prefer immutable `DataSnapshot`;
- exact provider revision/version/ETag/commit/table/query/isolation facts captured where available;
- local content digests always computed after stable acquisition;
- live/mutable source use is visibly marked partially reproducible;
- refresh creates a new snapshot identity;
- normalization creates derived artifacts with transform lineage;
- R/Analytics/Knowledge/Compute receipts bind exact snapshot digests.

**Status:** CLOSED_FOR_PLANNING.

## 7. Source-side effects audit

Potential gap: DB write, Kaggle submission, HF upload or provider mutation could be accidentally inferred from connector access.

Closure:
- 075 is read/discover/preview/import/refresh only;
- write/upload/delete/submit/DDL/DML are separate future effect capabilities;
- external effect state must preserve `Unknown` when remote confirmation is uncertain;
- no blind retry.

**Status:** CLOSED_FOR_PLANNING.

## 8. Hostile data/import audit

Potential gap: files/datasets can contain archives, malformed parsers, symlinks, bombs, executable scripts or provider code.

Closure:
- acquisition output is quarantined/untrusted candidate data;
- size/type/parser/depth/time limits required;
- archive traversal/symlink/bomb defenses required before archive/folder admission;
- provider-supplied arbitrary code is not executed for acquisition;
- HF remote-code trust is denied in trusted path;
- complex parsers may be isolated according to MedScale engine placement policy.

**Status:** CLOSED_FOR_PLANNING.

## 9. Source license/terms audit

Potential gap: public datasets are technically downloadable but may have usage restrictions.

Closure:
- capture provider-exposed license/terms identifiers and source metadata;
- user/admin acknowledgement may be required by owning adapter policy;
- MedScale does not infer usage rights merely from download success;
- exact source revision and acquisition receipt remain recorded.

**Status:** CLOSED_FOR_PLANNING.

## 10. R Workspace audit

### 10.1 IDE coupling

RStudio/Positron are external IDEs. No WebView dependency or embedded unrestricted interpreter is required.

### 10.2 Environment

`renv.lock`, R version, script digest, staged input digests, command and output digests form the reproducibility core. `sessionInfo()` is supplemental only.

### 10.3 Package restore/network

Package restore network is an explicit workspace/Compute network capability and can target approved mirrors; strict offline mode remains valid.

### 10.4 Output admission

No watched-folder silent import. User or bounded workflow explicitly publishes selected outputs through validation/classification/Core admission.

### 10.5 Secrets

R environment secrets are use-scoped and excluded/redacted from canonical receipts/logs.

**Status:** CLOSED_FOR_PLANNING.

## 11. Community Extensions audit

### 11.1 Sandbox

Default executable extension placement is evidence-qualified WASM or isolated worker. Native in-process dynamic plugins are denied by default.

### 11.2 Authority

Default capability set is empty. Extension install does not inherit user/Project permissions.

### 11.3 Host API

Typed/versioned/capability-mediated; no raw DB handle, vault key, keychain enumeration or arbitrary filesystem/network access.

### 11.4 UI

Default extension UI is declarative/typed contributions; no arbitrary WebView/native code injection.

### 11.5 Registry

Hub Community Registry is optional/self-hostable. Offline/manual verified installation remains supported.

### 11.6 Supply chain

Extension Pack includes immutable digest, publisher/provenance/signature, compatibility, license/SBOM/dependency policy, requested capability set and update/rollback metadata.

### 11.7 Updates

Capabilities cannot silently expand. Capability diff requires explicit re-consent/admin policy.

### 11.8 Revocation

Revocation disables execution but does not silently delete user-owned canonical research artifacts.

### 11.9 Telemetry

No network/telemetry by default. Destination-scoped network capability required.

### 11.10 Research Pack distinction

Research Packs remain domain/declarative semantics. Extension Packs remain executable/integration contributions. One does not silently authorize/install the other.

**Status:** CLOSED_FOR_PLANNING.

## 12. Hub integration audit

Potential gap: Community Registry might make Hub mandatory or create MedScale SaaS dependency.

Closure:
- Personal mode works without Hub;
- registry is optional and self-hostable;
- offline verified install remains supported;
- already-installed offline-capable extensions do not require registry reachability for ordinary use;
- Hub remains collaboration/registry distribution mechanism, not Core authority.

**Status:** CLOSED_FOR_PLANNING.

## 13. Privacy integration audit

Potential gap: source previews, database errors, R logs, extension logs or registry metadata may leak sensitive data.

Closure:
- data classification propagates through snapshot/import/derived outputs;
- Privacy Gate applies before external egress and extension data access;
- logs redact secrets and avoid raw sensitive samples by default;
- extension grants include data-class ceiling;
- R staged workspace classification is explicit;
- remote providers do not receive `LOCAL_PHI` or `TEAM_PROTECTED` content merely because a connector exists.

**Status:** CLOSED_FOR_PLANNING.

## 14. Failure/recovery audit

Data Sources now have typed failures for authentication, expiry, provider outage, rate limit, missing revision, schema drift, changed local file, partial acquisition, corruption, disk exhaustion, policy/network denial.

R has typed failures for missing IDE/runtime, lock restore, unavailable package, denied Compute, script failure and output mismatch.

Extensions have typed failures for signature/publisher/API/capability/sandbox/crash/timeout/quarantine/revocation/Host API violation.

Rollback semantics preserve canonical snapshots/artifacts independently from connector/extension availability.

**Status:** CLOSED_FOR_PLANNING.

## 15. Scale audit

V2 verification addendum requires measurement across:

- large local imports;
- wide DB schemas and bounded previews;
- interrupted/resumed provider downloads;
- snapshot storage growth/dedup behavior;
- Analytics over admitted snapshots;
- R workspace staging/package restore/output publication;
- extension startup/memory/host-call throughput;
- extension crash storms/timeouts;
- registry catalog scale and offline behavior.

No universal performance claims are made before measurement.

**Status:** CLOSED_FOR_PLANNING.

## 16. Accessibility/product UX audit

Data Sources, R Workspace and Extensions require native Desktop surfaces with loading/empty/error/denied/offline/conflict/recovery states and keyboard/focus/accessibility parity with MedScale design authority.

Extensions do not bypass accessibility by injecting arbitrary UI code.

**Status:** CLOSED_FOR_PLANNING.

## 17. Donor/source audit

New source families have explicit postures in `SOURCE_ADOPTION_MATRIX.md`:

- official Kaggle semantics / `kagglehub` — reference or isolated adapter;
- Hugging Face datasets — governed exact-revision acquisition;
- Arrow/Parquet / Arrow R — interchange candidate;
- `renv` — R environment reproducibility;
- RStudio/Positron/Posit Workbench — external IDE/institutional integration;
- Obsidian — community developer experience reference, not trust model;
- Wasmtime — extension sandbox qualification candidate;
- Extism — reference/benchmark candidate;
- Himsat — connector/capability security reference;
- MESC — experiment environment/manifest pattern reference only, with no runtime coupling.

No donor becomes a MedScale authority plane.

**Status:** CLOSED_FOR_PLANNING.

## 18. Spec 074 non-interference audit

Potential gap: new requirements could pressure current 074 implementation to add source/plugin/R foundations early.

Closure:
- Program Amendment explicitly leaves 074 unchanged;
- PR #122 remains the dedicated 074 implementation lane;
- 074 must not add DB connectors, Kaggle/HF acquisition, R execution, extension sandbox/registry, or Hub plugin APIs;
- 075+ remains unpromoted unless a newer explicit founder/canonical promotion exists.

**Status:** CLOSED_FOR_PLANNING.

## 19. Remaining evidence-selected decisions

The following are intentionally not frozen to a technology without qualification:

- exact DB driver crates;
- whether DuckDB adds sufficient value for 075;
- native provider API vs isolated official Python client for Kaggle/HF paths;
- exact R discovery/launch implementation by OS;
- exact WASM engine admission (Wasmtime candidate);
- whether Extism is useful over a MedScale-owned Host API/worker protocol;
- extension package archive format;
- registry backing store/search implementation;
- institutional mirrors/SSO/object-store providers.

These are **not planning gaps** because each has:

1. an owning spec;
2. a fixed MedScale contract/boundary;
3. a fail-safe default;
4. required qualification evidence;
5. no authority for implementer preference to override the contract.

## 20. Review result

```text
RESEARCH_OS_V2_AMENDMENT_REVIEW_READY = TRUE
V2_CANDIDATE_RANGE = 075-092
SPEC_074_SCOPE_CHANGED = FALSE
DATA_SOURCE_FABRIC_FIRST_CLASS = TRUE
DATABASE_SOURCE_MODEL_UNIFIED = TRUE
LOCAL_KAGGLE_HF_IMPORT_UNIFIED = TRUE
R_WORKSPACE_FIRST_CLASS = TRUE
COMMUNITY_EXTENSIONS_FIRST_CLASS = TRUE
EXTENSION_DEFAULT_AMBIENT_AUTHORITY = ZERO
COMMUNITY_REGISTRY_MANDATORY_CLOUD = FALSE
MATERIAL_KNOWN_V2_PLANNING_GAPS = 0
V2_IMPLEMENTATION_AUTHORIZED_BY_PACKET = FALSE
SPEC_075_IMPLEMENTATION_AUTHORIZED = FALSE
REAL_PHI_AUTHORIZED_BY_PACKET = FALSE
```

`MATERIAL_KNOWN_V2_PLANNING_GAPS = 0` means no presently known material requirement in this amendment lacks a planned owner/boundary/verification path. It does **not** mean the technologies are qualified or implemented.
