# MedScale Research OS Source Adoption Matrix V2

**Status:** Planning candidate — not dependency, copy, credential, network, or implementation authority

This document maps candidate sources to narrow MedScale-owned contracts. Source availability or founder permission does not justify wholesale adoption. Any direct dependency/copy/adaptation still requires exact revision/path provenance, license/NOTICE closure, embedded/transitive review, security review, model/data/asset rights, maintenance strategy, and active-spec acceptance evidence.

## Adoption rule

Every external or sibling-project source must receive one posture before implementation:

- `PRIMARY_AUTHORITY` — existing MedScale authority only;
- `REFERENCE` — learn patterns; no source transfer;
- `DEPEND` — use a pinned dependency behind a MedScale contract;
- `ADAPT` — selectively reimplement/adapt patterns;
- `COPY_SELECTIVE` — bounded file/component transfer with provenance;
- `WORKER` — isolate source/runtime behind process/capability contract;
- `EXTERNAL_TOOL` — user/institution tool integrated through files/protocols, not embedded authority;
- `DEFER` — useful but not justified at current frontier.

No donor/source becomes a MedScale authority plane.

## High-value mapping

| Capability | Primary sources / references | Candidate posture | MedScale-owned boundary |
|---|---|---|---|
| Core authority, packs, provenance | current MedScale Core / canonical specs | `PRIMARY_AUTHORITY` | Core commands, storage, signed Packs, evidence/audit |
| Project/artifact substrate | current MedScale Spec 074 contracts | `PRIMARY_AUTHORITY` | Project, Experiment, artifact refs, bounded graph |
| Data Source Fabric | MedScale V2 source contracts | `PRIMARY_AUTHORITY` | `DataSourceManifest`, `DataSnapshot`, import/refresh receipts |
| Local tabular interchange | Apache Arrow / Parquet | `DEPEND` candidate | exact snapshot/interchange contract |
| Native analytics | Apache Arrow / DataFusion | `DEPEND` candidate | governed views, `QueryReceipt` |
| Database read adapters | native Rust DB clients / evidence-qualified drivers | `DEPEND` / `WORKER` by driver risk | 075 adapter SPI; no provider-owned authority |
| Kaggle dataset acquisition | official Kaggle API / KaggleHub semantics | `REFERENCE` + isolated `DEPEND/WORKER` candidate | 075 Kaggle adapter; exact dataset/version/files, opaque credentials |
| Hugging Face datasets | Hugging Face Hub dataset repository/revision semantics | `DEPEND`/`WORKER` candidate | 075 HF dataset adapter; exact revision/files, no trusted remote code |
| Object stores/warehouses | S3-compatible/cloud/warehouse SDKs | `DEFER` to institutional adapters | 075 source contract + 090 institutional adapter |
| R data interchange | Apache Arrow R / Parquet | `EXTERNAL_TOOL` + `DEPEND` candidate where generated/staged | `RWorkspaceManifest` staging |
| R environment reproducibility | `renv` | `EXTERNAL_TOOL` | lockfile digest + `RRunReceipt` |
| R IDE | RStudio Desktop, Positron | `EXTERNAL_TOOL` | launch staged workspace path only |
| Institutional R | Posit Workbench / Job Launcher | `DEFER` / optional adapter | 090 institutional adapter |
| Multi-agent/model fleet | `amElnagdy/delegate-skills`, Munder Difflin, Golam | `REFERENCE / ADAPT` | `AgentLane`, `FleetRun`, capability grant |
| Observable comparison | HarnessMind | `REFERENCE / ADAPT` | observable comparison evidence; unknown remains unknown |
| Human-agent collaboration | `block/buzz` | `COPY_SELECTIVE / ADAPT` after qualification | Room, ProjectEvent, AgentIdentity, approval/activity |
| Team/roles/tasks/approvals | Qdrat, Huly, Plane, Zulip | `REFERENCE / ADAPT` | membership/tasks/threads/activity |
| Audio platform | `debpalash/VoiceStudio`, Himsat, Wispral | `COPY_SELECTIVE / ADAPT / WORKER` after qualification | AudioSession, Audio Pack, VoiceRuntimeRouter |
| Cross-platform capture | Himsat donors: OpenWhispr, Meetily, OpenSuperWhisper | `REFERENCE` / selective adaptation | CaptureSource, CaptureHealth |
| Speech runtimes | sherpa-onnx, whisper.cpp, NeMo-Speech.cpp and qualified models | `DEPEND / WORKER` | AudioRuntime / immutable Audio Pack |
| Browser/tool execution | Ecra, Tarif, Playwright, Browser Use, TinyFish | `DEPEND / ADAPT` | ToolCapability, BrowseReceipt |
| Sandboxed execution | OpenSandbox + current OS sandbox work | `REFERENCE / DEPEND / WORKER` | ComputeJob, filesystem/network capability policy |
| Community plugin UX/policy | Obsidian developer/plugin/community-directory model | `REFERENCE` only | Extension SDK, Community Registry UX; not trust model |
| WASM extension isolation | Wasmtime | `DEPEND` benchmark/admission candidate after Compute | Extension host runtime; no authority transfer |
| Plugin framework/runtime patterns | Extism | `REFERENCE / BENCHMARK` | Extension host/runtime contract; no trusted-process plugin authority |
| Connector capability security | Himsat connector/plugin planning patterns | `REFERENCE / ADAPT` | capability broker, explicit network/filesystem/secret grants |
| Experiment environment manifests | MESC runner/manifest patterns | `REFERENCE / ADAPT` | runtime/environment evidence only; no MESC coupling |
| Advanced BI | Apache Superset, Nao, Metabase | `REFERENCE` / optional adapter | AnalyticsAdapter; never Desktop authority/runtime requirement |
| RAG/retrieval | OpenRAG, Onyx, AnythingLLM | `REFERENCE` / optional worker | RetrievalPlan, RetrievalReceipt |
| Evidence/context graphs | Graphify, code-graph-rag | `REFERENCE / ADAPT` | Project/Evidence Graph projections |
| De-identification | OpenMed, Presidio patterns, MedScale NER Packs | `REFERENCE / DEPEND / ADAPT` | PrivacyGate, DeidReceipt |
| Clinical verification | ProtocolWISE, commandMed | `REFERENCE / ADAPT` | provenance, abstention, proposal-only model semantics |
| Planning/execution discipline | SpecGrain, Diffcipline, Delethos | `PROCESS_REFERENCE` | bounded specs, proof-carrying changes |

## Data Source Fabric source rules

### Kaggle

Use provider APIs/SDK semantics only behind MedScale contracts.

Required adoption posture before code:

1. pin exact client/API revision if a library is embedded;
2. inspect dependency/license tree;
3. isolate Python SDK use if selected rather than adding Python to trusted Desktop;
4. preserve dataset/version/file identity available from provider;
5. no competition submission/upload/write in foundation adapter;
6. token is an opaque credential handle;
7. network access flows through admitted network authority;
8. provider cache is not canonical identity;
9. imported bytes are quarantined/validated before admission.

### Hugging Face datasets

HF dataset repositories are source repositories, not trusted executable packages.

Foundation rules:

- exact repo + revision/commit + selected files where available;
- prefer direct file/repository acquisition in declared formats;
- no remote dataset script execution in trusted path;
- no `trust_remote_code`-style behavior by default;
- gated/private token is an opaque credential handle;
- model Pack admission and dataset import remain separate contracts;
- a dataset card/license field is metadata to preserve, not independent legal clearance.

### Database connectors

Choose drivers by exact engine/platform evidence. Drivers do not define MedScale semantics.

Default foundation engines to qualify:

```text
PostgreSQL
MySQL/MariaDB
SQL Server
external SQLite
```

DuckDB is a candidate local source/analytics bridge only if exact dependency/security/SQLite coexistence constraints are acceptable.

Generic ODBC is deferred until native drivers prove insufficient for institutional adoption.

## R / Posit adoption rules

RStudio/Positron are external IDEs. MedScale should not copy their UI or embed them into Slint.

The integration boundary is:

```text
MedScale DataSnapshot
-> staged Arrow/Parquet/files
-> RWorkspaceManifest
-> external RStudio/Positron OR Compute-mediated Rscript
-> output candidates
-> explicit MedScale publication
-> RRunReceipt
```

`renv.lock`, exact R runtime identity and package restoration state are evidence inputs. They do not make network package repositories permanently reproducible.

Institutional Posit Workbench is optional and belongs behind institutional adapter contracts.

## Community Extensions / Obsidian lesson

The product aspiration is an Obsidian-like developer/community ecosystem: documented SDK, discoverable community directory, straightforward development/submission and broad community ownership.

MedScale MUST NOT copy a desktop plugin trust model that allows arbitrary in-process access to sensitive healthcare/research state.

MedScale adds:

- signed/hashed Extension Packs;
- publisher identity/provenance;
- explicit capability manifest;
- filesystem/network/device/data-class grants;
- sandboxed WASM or isolated worker runtime;
- no ambient vault/keychain access;
- capability diff and re-consent on update;
- SBOM/license/dependency review;
- quarantine/revocation/rollback;
- offline/manual verified install;
- Community Registry metadata separated from client authority.

Wasmtime/Extism are implementation candidates, not architecture authorities. A promoted extension spec must benchmark/admit the runtime against the stable MedScale Extension Host API contract.

## `block/buzz` — selective adoption target

Buzz is valuable because it unifies humans, agents, workflows, media, rooms, search and audit on one collaboration substrate. Study/adapt:

- signed/tamper-evident activity records;
- room/channel and thread semantics;
- agent identity separate from owner identity;
- membership-scoped event delivery;
- workflow traces and approvals;
- ephemeral presence;
- searchable project history;
- media comments anchored to time/frame;
- huddle lifecycle;
- self-hosted relay/service decomposition;
- CLI/agent structured contracts.

Do **not** make Buzz/Nostr the canonical clinical/research database. Canonical artifacts remain Core-owned and collaboration references exact IDs/revisions.

## `debpalash/VoiceStudio` — selective adoption target

Candidate lessons/components:

- engine registry/model orchestration;
- local-first speech service split from native control;
- streaming/batch transcription;
- dictation/output-session safety;
- diarization;
- TTS / voice design / cloning;
- GPU/runtime preflight;
- model integrity/download management;
- diagnostics;
- MCP/API integration;
- optional remote workers;
- synthetic-audio provenance/watermark concepts.

Do not import the entire Electron/Python product architecture. Heavy runtimes may be isolated workers; capture/control remains MedScale-owned.

The public repository license/NOTICE and any separate founder permission must be documented precisely before direct file transfer.

## Himsat connector/security lessons

Useful sibling-project patterns include:

- networking is an explicit capability;
- connector credentials never receive vault master keys;
- external writes require approval/dry-run/effect receipts;
- local plugins/connectors/models remain a security boundary even in a local-first product;
- plugin capability escape and connector payload minimization are first-class qualification targets.

Use these as patterns, not cross-project runtime coupling.

## MESC manifest lessons

Useful sibling patterns:

- deterministic experiment/run manifests;
- record runner/environment as evidence rather than truth;
- content-addressed input identities;
- portability across local/Kaggle/cluster contexts without making the runner define result validity.

Do not couple Research OS to MESC or mutate MESC under this program.

## Sibling-project boundary

Sibling TheHalfMoon repositories can inform architecture, but code transfer is component-by-component. Private repository details must not be disclosed publicly unless explicitly authorized for public disclosure.

## Qualification gate for every adopted source

Before crossing from planning into implementation:

1. pin repository/package/model and exact revision;
2. identify exact files/modules/assets used;
3. record authorization and license/NOTICE obligations;
4. inspect transitive/embedded code and model/data terms;
5. define the MedScale-owned interface;
6. threat-model the boundary;
7. benchmark against minimal native/competing option where meaningful;
8. define update/rollback/exit strategy;
9. preserve provenance in-repo;
10. prove the adopted surface through the active spec's acceptance gates;
11. verify no source introduces an alternate credential, authority, ID, provenance, network or storage plane.