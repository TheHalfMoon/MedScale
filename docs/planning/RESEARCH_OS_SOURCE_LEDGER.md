# MedScale Research OS Source Ledger

**Status:** Planning candidate. This is a curated Research OS ledger, not a claim that every source is adopted or licensed identically.

## Founder-owned / sibling sources to reverify before reuse

| Source | Candidate value |
|---|---|
| `TheHalfMoon/MedScale` | primary product/core authority, Packs, model runtime, evidence, privacy and UX foundations |
| `TheHalfMoon/Qdrat` | organization, roles, workforce/team lifecycle, tasks/helpdesk, approvals, reporting/audit patterns |
| `TheHalfMoon/Himsat` | local capture, transcription/diarization research, evidence-linked audio, long-session and voice source qualification |
| `TheHalfMoon/Wispral` | command-vs-context voice semantics, interruption, agent control and permission UX |
| `TheHalfMoon/Golam` | local-first agent/tool/memory/policy architecture reference |
| `TheHalfMoon/Ecra` | browser/search/agent capability routing and execution receipts |
| `TheHalfMoon/Tarif` | default-deny agent action authority, secret isolation and receipts |
| `TheHalfMoon/Sentrdel` | security/policy/control-plane patterns |
| `TheHalfMoon/HarnessMind` | observable agent comparison, evidence semantics, unknown-is-unknown discipline |
| `TheHalfMoon/ProtocolWISE` | clinical knowledge verification, provenance, abstention and semantic-validity discipline |
| `TheHalfMoon/commandMed` | medical AI safety/evidence/tool separation reference |
| `TheHalfMoon/SpecGrain` | bounded decomposition/planning method |
| `TheHalfMoon/Diffcipline` | Think -> Challenge -> Minimize -> Change -> Prove execution method |
| `TheHalfMoon/Delethos` | bounded delegation/proof-carrying patch patterns |

Private sibling repositories, if any are consulted through authorized connected access, must not be named or disclosed in the public MedScale repository without explicit authorization for public disclosure.

## Explicitly added sources for this plan

| Source | Candidate value | Observed public license / note |
|---|---|---|
| `block/buzz` | self-hosted human+agent collaboration, event model, rooms/threads, workflows, media annotations, huddles, search, audit | Apache-2.0 observed; exact revision/path review still required |
| `debpalash/VoiceStudio` | local speech platform, STT/TTS, voice cloning/design, diarization, dictation, streaming/batch APIs, model/runtime orchestration, MCP | AGPL-3.0 observed; founder states separate permission exists; preserve documentary scope before direct copy |
| `amElnagdy/delegate-skills` | fleet/lane delegation contracts across agent CLIs | reference/adapter candidate; exact current license/revision must be reverified at adoption |

## Analytics / data candidates

- Apache Arrow / DataFusion — native analytical execution and columnar interchange;
- Apache Superset — optional advanced self-hosted BI adapter/reference;
- Nao — data/analytics interaction reference;
- Metabase — BI interaction/reference candidate;
- PostHog / Umami — event/analytics and privacy-aware telemetry patterns where appropriate.

## Retrieval / knowledge candidates

- `langflow-ai/openrag` — ingestion/retrieval/agentic RAG reference or optional worker;
- Onyx — enterprise retrieval/search patterns;
- Graphify — graph/context patterns;
- code-graph-rag — graph retrieval patterns;
- AnythingLLM — local knowledge/LLM UX reference;
- OpenSearch / Meilisearch — search/index candidates subject to scale need.

## Collaboration / knowledge candidates

- Huly Platform — integrated project/communication/workspace patterns;
- Plane / OpenProject — project/task patterns;
- Zulip — topic/thread communication model;
- Mattermost — self-hosted team communication reference;
- AFFiNE / AppFlowy / Outline — local/collaborative knowledge and document/canvas patterns.

## Identity / authorization candidates

- OpenFGA — relationship-based authorization reference/engine candidate;
- OPA / Cerbos — policy evaluation candidates;
- Keycloak / ZITADEL — institutional identity/SSO candidates.

## Browser / worker / automation candidates

- Microsoft Playwright — deterministic browser automation primitive;
- Browser Use — model-assisted browser patterns;
- TinyFish — browser-agent/web automation reference;
- OpenSandbox — sandbox lifecycle, filesystem/network policies and worker isolation;
- Temporal / pg-backed durable jobs — long-running workflow/job candidates only when justified by measured complexity.

## Audio/speech candidates carried from Himsat research

Candidate engines and primitives must be reverified at shaping time, but the current research universe includes:

- sherpa-onnx;
- whisper.cpp;
- NVIDIA NeMo-Speech.cpp;
- OpenWhispr;
- OpenSuperWhisper;
- Meetily;
- Silero VAD / TEN VAD;
- WebRTC AudioProcessing / Sonora;
- DeepFilterNet / RNNoise;
- pyannote-audio;
- joint long-form transcription+diarization challengers;
- multilingual/Arabic challengers;
- medical-specialized ASR challengers.

No engine is selected merely by inclusion in this ledger.

## Healthcare / scientific ecosystem candidates

Depending on Research Pack scope:

- Medplum / HAPI FHIR — healthcare/FHIR reference/integration;
- Synthea — synthetic health fixtures;
- OHIF / Orthanc / dcm4che — imaging/DICOM;
- relevant LIMS/ELN and omics tooling to be researched during Pack shaping;
- institutional HPC/Slurm/Kubernetes adapters to be researched during Compute/Institutional shaping.

## Source adoption discipline

For every source selected for implementation, create a source-adoption record containing:

```text
source repository
exact revision/tag
public license
separate permission basis if relied upon
selected files/modules/models/assets
embedded/transitive code review
notices/attribution
MedScale-owned contract
security/privacy review
benchmark or comparative justification
modifications
update/rollback strategy
acceptance evidence
```

The ledger is intentionally broader than the implementation set. The product should minimize adopted code while maximizing proven capability.
