# MedScale Research OS Source Adoption Matrix

**Status:** Planning candidate — not dependency or copy authority

This document maps candidate sources to narrow MedScale-owned contracts. Source availability or permission does not justify wholesale adoption. Any direct copy or adaptation still requires exact revision/path provenance, license/NOTICE closure, embedded third-party review, security review, model/data/asset rights, and a maintenance strategy.

## Adoption rule

Every external or sibling-project source must be assigned one of these postures before implementation:

- `REFERENCE` — learn patterns; no source transfer;
- `DEPEND` — use a stable dependency behind a MedScale contract;
- `ADAPT` — reimplement or selectively adapt patterns;
- `COPY_SELECTIVE` — copy bounded files/components with provenance;
- `WORKER` — isolate the source behind a process/network contract;
- `DEFER` — useful, but not justified at the current frontier.

No source becomes a MedScale authority plane.

## High-value mapping

| Capability | Primary sources | Candidate posture | MedScale-owned boundary |
|---|---|---|---|
| Core authority, packs, provenance | MedScale current Core / Specs 069-073 | PRIMARY_AUTHORITY | Core commands, signed Packs, EvidenceRef, audit |
| Multi-agent/model fleet | `amElnagdy/delegate-skills`, Munder Difflin, Golam | REFERENCE / ADAPT | `AgentLane`, `FleetRun`, `CapabilityGrant` |
| Observable comparison | HarnessMind | REFERENCE / ADAPT | `ComparisonEvidence`, unknown/observed semantics |
| Human-agent collaboration | `block/buzz` | COPY_SELECTIVE / ADAPT after qualification | `Room`, `ProjectEvent`, `AgentIdentity`, `ApprovalEvent` |
| Team/roles/tasks/approvals | Qdrat, Huly, Plane, Zulip | REFERENCE / ADAPT | Project membership, tasks, threads, activity |
| Audio platform | `debpalash/VoiceStudio`, Himsat, Wispral | COPY_SELECTIVE / ADAPT / WORKER after qualification | `AudioSession`, `AudioPack`, `VoiceRuntimeRouter` |
| Cross-platform capture | Himsat donors: OpenWhispr, Meetily, OpenSuperWhisper | REFERENCE / selective adaptation | `CaptureSource`, `CaptureHealth` |
| Speech runtimes | sherpa-onnx, whisper.cpp, NeMo-Speech.cpp and qualified model lanes | DEPEND / WORKER | `AudioRuntime` / immutable Audio Pack |
| Browser/tool execution | Ecra, Tarif, Playwright, Browser Use, TinyFish | DEPEND / ADAPT | `ToolCapability`, `BrowseReceipt` |
| Sandboxed execution | OpenSandbox | REFERENCE / DEPEND / WORKER | `ComputeJob`, network/filesystem capability policy |
| Native analytics | Apache Arrow / DataFusion | DEPEND candidate | `AnalyticsSession`, governed views, `QueryReceipt` |
| Advanced BI | Apache Superset, Nao, Metabase | REFERENCE / optional adapter | `AnalyticsAdapter`; never Desktop authority/runtime requirement |
| RAG/retrieval | OpenRAG, Onyx, AnythingLLM | REFERENCE / optional worker | `RetrievalPlan`, `RetrievalReceipt` |
| Evidence/context graphs | Graphify, code-graph-rag | REFERENCE / ADAPT | Project Graph / Evidence Graph |
| De-identification | OpenMed, Presidio patterns, MedScale NER Packs | REFERENCE / DEPEND / ADAPT | `PrivacyGate`, `DeidReceipt` |
| Clinical verification | ProtocolWISE, commandMed | REFERENCE / ADAPT | explicit provenance, abstention, non-authoritative proposal semantics |
| Planning/execution discipline | SpecGrain, Diffcipline, Delethos | PROCESS_REFERENCE | bounded specs, proof-carrying changes |

## `block/buzz` — selective adoption target

Buzz is valuable because it unifies humans, agents, workflows, media, rooms, search, and audit on one collaboration substrate. MedScale should study/adapt:

- signed or tamper-evident activity records;
- room/channel and thread semantics;
- agent identity separate from owner identity;
- membership-scoped event delivery;
- workflow traces and approval events;
- presence/typing as ephemeral state;
- searchable project history;
- media comments anchored to time/frame;
- huddle lifecycle concepts;
- self-hosted relay/service decomposition;
- CLI/agent-facing structured contracts.

MedScale should **not** make Buzz/Nostr the canonical clinical/research database. FHIR, datasets, consent, model Packs, evidence assertions, and regulated artifacts remain governed by MedScale Core. Collaboration events may reference canonical artifacts by immutable IDs/revisions.

## `debpalash/VoiceStudio` — selective adoption target

VoiceStudio is a broad local speech platform rather than only a TTS application. Candidate lessons/components:

- engine registry and model orchestration;
- local-first speech service split from native control;
- streaming and batch transcription contracts;
- dictation and output-session safety;
- diarization;
- TTS / voice design / cloning;
- GPU/runtime preflight;
- model integrity and download management;
- diagnostics and explicit failures;
- MCP/API integration;
- optional remote workers;
- AI/synthetic-audio watermark/provenance concepts.

MedScale should not import VoiceStudio's entire Electron/Python application architecture. Audio capabilities live behind MedScale-owned contracts and Pack manifests. Heavy Python/model runtimes may run as isolated workers while native capture/control remains Rust-first where justified.

The public VoiceStudio repository is AGPL-3.0. Any broader private permission asserted by the founder must be preserved as documentary provenance with sufficient scope before direct copying into differently licensed MedScale components.

## Buzz licensing note

The inspected `block/buzz` repository declares Apache-2.0. Direct adoption still requires exact revision/file attribution and embedded dependency review.

## Sibling-project boundary

Sibling TheHalfMoon repositories can inform MedScale architecture, but copying code should be justified component-by-component. Private repository names or details must not be disclosed into public planning artifacts unless the founder has explicitly authorized public disclosure of that source.

## Qualification gate for every adopted source

Before a source crosses from this matrix into implementation:

1. pin repository and exact revision;
2. identify exact files/modules/models/assets;
3. record authorization and public license/NOTICE obligations;
4. inspect transitive/embedded code and model/data terms;
5. define the MedScale-owned interface;
6. threat-model the boundary;
7. benchmark against a minimal native implementation or competing source;
8. define update/rollback strategy;
9. preserve provenance in-repo;
10. prove the adopted surface through the active spec's acceptance gates.
