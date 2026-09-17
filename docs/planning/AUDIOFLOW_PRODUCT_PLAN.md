# MedScale AudioFlow Product Plan

**Status:** Planning candidate — not implementation authority

## Product definition

**Audio** is the user-facing workspace. **AudioFlow** is the MedScale subsystem that turns user-authorized audio into provenance-preserving, privacy-governed project artifacts and voice interaction.

AudioFlow covers:

- microphone and permitted system-audio capture;
- imported audio/video;
- live and offline transcription;
- diarization and alignment;
- dictation;
- MedAgent voice control;
- meeting/huddle intelligence;
- TTS/listen-back;
- later voice design/cloning and dubbing under explicit permission;
- audio evidence, search, and project memory.

## Why AudioFlow is a plane, not a feature

Short dictation, a two-hour interview, a clinical encounter, a conference lecture, and a six-hour multi-speaker research session have different latency, quality, diarization, memory, and privacy requirements. One model and one pipeline must not be assumed to fit all tasks.

## Core contracts

Candidate durable types:

```text
AudioSession
AudioSource
CaptureRoute
CaptureHealth
AudioSegment
TranscriptRevision
SpeakerSegment
SpeakerIdentityBinding
AudioEvidenceRef
AudioPackManifest
VoiceRouteDecision
AudioJob
AudioAnnotation
```

`AudioEvidenceRef` should bind project artifact identity plus source-time range and transcript revision, allowing evidence to point to exact audio even when transcription is later improved.

## Voice Runtime Router

Route decisions should be explainable from explicit metadata:

```text
requested_task
language_or_language_set
streaming_required
diarization_required
timestamp_requirement
medical_or_domain_profile
device_class
available_ram
accelerator_kind
battery_or_thermal_policy
offline_required
network_lock_state
model_digest
engine_digest
rights_state
runtime_health
```

The router never silently changes locality or provider. If a requested route is unavailable, return an explicit reason and allowed alternatives.

### Candidate lanes

1. **Live general** — dictation, live notes, meetings; optimize first-token latency and stability.
2. **Multilingual / Arabic / code-switch** — first-class Arabic and Arabic-English evaluation.
3. **Medical-specialized** — medication names, doses, units, numbers, negation, anatomy and procedures receive separate scoring.
4. **Long-form multi-speaker** — diarization, overlap handling, timestamp drift and speaker stability.
5. **Low-resource/mobile** — predictable memory/battery/thermal behavior.
6. **Offline quality/import** — slower heavy quality pass with resumability and high alignment quality.

## Non-destructive transcript model

Audio source is immutable evidence. Transcript improvements produce revisions:

```text
AudioSource
  -> TranscriptRevision v1 (live)
  -> TranscriptRevision v2 (offline quality)
  -> TranscriptRevision v3 (user-corrected)
```

Every correction preserves the previous revision and reason/source. Terminology hints or model-based cleanup must not silently rewrite medication, numeric, or clinical content.

## Voice interaction semantics

AudioFlow should distinguish at least:

- `COMMAND` — user requests an action;
- `CONTEXT` — spoken information should enter context/notes but does not authorize execution;
- `DICTATION` — text insertion into an explicit target.

This follows the strongest principle from Wispral: voice is an agent control surface, not just speech-to-text. Interruption, cancellation, steering, and visible permission state are first-class.

## MedAgent integration

Examples:

- "Compare the two admitted models on this deidentified dataset." -> proposed `FleetRun` under current project/capabilities.
- "Stop. Only use the deidentified cohort." -> immediate cancellation/route correction.
- "This is only a note: batch 12 used the older reagent." -> context artifact, not execution authority.
- "Read the evidence summary aloud." -> local TTS route if admitted.

## Huddles

A Project may host an Audio Huddle containing humans and capability-scoped agents.

Huddle outputs can include:

- live transcript;
- speaker timeline;
- citations/evidence surfaced by agents;
- decisions;
- open questions;
- tasks;
- dataset/document/model mentions;
- timestamped annotations;
- final summary with exact links back to audio evidence.

Agents in a huddle are participants, not authorities. Their ability to read PHI, browse, create tasks, run analytics, or export is controlled independently.

## VoiceStudio adoption boundary

Candidate selective reuse from `debpalash/VoiceStudio`:

- engine registry/model orchestration;
- speech-platform protocol concepts;
- native control versus audio data-plane separation;
- streaming and batch transcription;
- diarization;
- dictation/output-session safety;
- GPU/runtime preflight;
- diagnostics/error journal/no-silent-fallback lessons;
- TTS/voice design/cloning workflows;
- MCP/agent-facing interfaces;
- optional remote-worker patterns;
- synthetic-audio watermark/provenance concepts.

Do not import the whole Electron application. The public repository's AGPL-3.0 status and any founder-held broader permission must be recorded precisely before source transfer.

## Himsat adoption boundary

Himsat research is a high-value design input for:

- capture lifecycle and health;
- long-session stability;
- multi-engine route selection;
- Arabic/code-switch and medical-specialized evaluation;
- evidence-linked transcript revisions;
- speaker privacy;
- worker isolation for heavyweight/custom-code models;
- explicit no-silent-fallback behavior.

Himsat planning documents are not implementation proof; any copied code still requires exact live qualification.

## Buzz adoption boundary

Buzz contributes huddle/event concepts:

- room membership;
- human + agent presence;
- huddle lifecycle events;
- media annotations;
- searchable collaboration history;
- independent agent identities and audit.

MedScale retains its own clinical/research authority and audio privacy model.

## Privacy and consent

Required principles:

1. Capture is visible and explicitly user-started/authorized.
2. Platform privacy indicators and protected-content controls are never bypassed.
3. Raw audio retention policy is explicit per session/project.
4. `SPEAKER_01` style diarization does not imply persistent person identity.
5. Persistent speaker embeddings/identity are opt-in, encrypted, export-controlled, and deletable.
6. Browser/cloud speech routes are opt-in and pass through Privacy Gate.
7. Wake word is deferred, local-first if ever admitted, and independently qualified for false activation, privacy, battery, and disable/reset behavior.
8. Voice cloning requires explicit permission/provenance; exported synthetic audio should carry provenance/watermarking where technically appropriate.

## Qualification dimensions

AudioFlow requires more than generic WER:

- WER and timestamp quality;
- medication/drug/name/dose/unit/number/negation error rates;
- Arabic and Arabic-English code switching;
- diarization DER/JER plus long-session speaker stability;
- overlap/noise/far-field/Bluetooth behavior;
- first partial/final latency;
- interruption/cancellation latency;
- CPU/RAM/GPU footprint;
- battery/thermal impact where relevant;
- six-hour stability and memory growth;
- crash recovery/resume;
- model integrity and rights closure;
- privacy/log scrubbing;
- accessibility and visible capture state.

## Initial UI

The Audio workspace should prioritize work, not studio chrome:

```text
Audio
  Live
  Sessions
  Imports
  Transcripts
  Huddles
  Voices        (later)
  Models
```

A session view centers the waveform/timeline, transcript, speakers, evidence annotations, notes/tasks, and MedAgent context. Audio engine details remain inspectable but secondary unless the user enters Models/diagnostics.
