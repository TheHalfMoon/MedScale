# Spec 081 Promotion — AudioFlow Foundation

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`
**Promotion date:** 2026-09-23
**Canonical base:** `c682178ed4b8a4bbee67fe64d89cc94450d60b13`
**Target branch:** `spec/081-audioflow-foundation`

## Authority

The founder's standing continuation directive requires promoting the next
dependency-ready Research OS unit after each closure without routine
approval. `IMPLEMENTATION_AUTHORITY.md` remains active.

Live verification at promotion time (2026-09-23, `gh pr view` / `gh run view`):

- Spec 080 is `CLOSED_CANONICAL`: final head `fe0a72e` passed exact-head run
  `35867289972` (6/6); PR #139 merged as `a8e32be`; post-merge main run
  `35873449163` passed 6/6. Closure PR #140 (exact-head run `35879136073`, 6/6 on `21db0d1`) merged as `c682178`.
- Specs 077 and 079 are `CLOSED_CANONICAL` (see `BUILD_QUEUE.md`).

Dependency proof: `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md` numbers
AudioFlow Foundation **081** with hard dependency **077 + 079**; both are
closed. `LOCAL_MEDICAL_SCRIBE_PLAN.md` §21 assigns capture, ASR,
diarization and transcript authority to 081. Decision register entries
Q18 (native capture behind a MedScale contract), Q21 (audio runtime identity
extends Pack provenance, no separate model store) and Q22 (visible,
explicitly user-started capture; no stealth capture) apply.

Review policy: `FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## Scope decision recorded here (smallest honest foundation)

The canonical closure gate asks for live and imported local audio paths
with reproducible transcript/evidence lineage and explicit
device/runtime/failure states. This promotion builds all of the lineage,
state and authority machinery, and freezes two honest limits instead of
admitting new native dependencies inside a foundation spec:

1. **Native device capture** needs an OS audio backend crate. Q18 makes
   that conditional on exact platform API and dependency qualification
   (supply-chain audit, CI system packages, `unsafe_code = "deny"`
   interaction). No backend is admitted here. The capture lifecycle
   (explicit start, pause, resume, stop, cancel, health, crash recovery)
   is built against a `CaptureBackend` seam. It is proven with a scripted
   backend that feeds synthetic PCM frames. The `native_device` backend
   reports `unavailable` with a fixed reason. This is the same pattern as
   the Spec 080 scripted transport.
2. **Speech recognition models** need an admitted Audio Pack (Q21, Spec
   078 Model Fleet). No ASR model is admitted here. The `local_asr_pack`
   route reports `unavailable`. A deterministic `fixture_asr` engine
   (synthetic, digest-keyed, labelled as a fixture in every receipt) proves
   the transcript lineage end to end. It is never presented as speech
   recognition. `cloud_asr` exists in the vocabulary only to be denied:
   there is no cloud route and no hidden fallback.

What is real in this spec: WAV/PCM import with digest and format checks;
storage of source audio inside the vault metadata store (encrypted at rest
when the vault is an encrypted SQLCipher vault; synthetic vaults hold
synthetic data only);
deterministic energy-based voice-activity segmentation and capture-health
measurement (silence, clipping); timestamped segments; immutable source
audio; transcript revisions with parent links and human corrections;
explicit `COMMAND` / `CONTEXT` / `DICTATION` modes, of which `COMMAND` never
executes anything; audio evidence references that bind a source digest,
time range, transcript revision and segment; and receipts.

## Authorized scope

- Contracts (`medscale-contracts/src/audio.rs`): `AudioSource`,
  `AudioSourceKind`, `PcmFormat`, `AudioSession`, `CaptureState`,
  `CaptureHealth`, `CaptureBackendKind`, `AudioRouteRequest`, `AudioRoute`,
  `AudioRouteStatus`, `AudioRouteDecision`, `TranscriptRevision`,
  `TranscriptOrigin`, `TranscriptSegment`, `SegmentStatus`, `SpeakerLabel`,
  `DiarizationState`, `AudioEvidenceRef`, `VoiceInputMode`,
  `TranscriptReceipt`, `VadParameters`, `AsrEngineIdentity`.
  The canonical candidate names `AudioRouteReceipt` and
  `DiarizationRevision` are folded in: one `TranscriptReceipt` records the
  route decision of every request (allowed or denied), and diarization is a
  state (`not_requested` / `unavailable`) until an engine exists.
- Import: RIFF/WAVE, PCM 16-bit, mono or stereo, 8-48 kHz, at most 32 MiB
  and 30 minutes. Everything else is refused with an explicit reason.
- Capture chunks: at most 1 MiB each, whole PCM frames, same total bounds.
- Capture sessions: explicit start by an authenticated holder, visible
  state, pause/resume/stop/cancel, health; stop produces an `AudioSource`;
  cancel discards the frames; a session left `recording` across a restart
  reopens as `interrupted` and keeps no partial audio.
- Segmentation: deterministic energy VAD with frozen parameters recorded in
  every receipt.
- Transcription: route decision persisted; segments with timestamps;
  speakers `unknown` (diarization route `unavailable`; labels never imply
  identity).
- Corrections: a new revision with origin `human_correction`, parent link
  and reason; earlier revisions are never modified.
- Privacy: audio never leaves the device; no network route exists for
  audio. Real PHI stays unauthorized; all fixtures are synthetic tones or
  silence generated in code.
- Storage schema v9 -> v10 (additive), backup/restore, consistency checks.
- CLI `medscale audio ...` (human and JSON) and a Desktop Audio route over
  Core, with capture state always visible.

## Explicitly not authorized

- Native microphone/system-audio backends or any new third-party
  dependency.
- Real ASR/diarization/VAD models, model downloads, or Audio Pack admission.
- Any cloud or remote speech route; any hidden fallback.
- Wake word, ambient or background capture, TTS, voice cloning.
- Clinical extraction, note generation, coding, EHR writes (Specs 083,
  088, 090).
- Executing `COMMAND`-mode transcripts; voice control of MedAgent.
- Real PHI or real patient audio.

## Frozen acceptance requirements

1. A valid synthetic WAV imports as an immutable `AudioSource` with digest,
   format and duration. Malformed, unsupported, oversized or too-long input
   is refused with a reason and stores nothing.
2. Capture starts only on an explicit request, and its state is always
   readable. Invalid transitions are refused. Stop yields a source whose
   bytes equal the frames captured. Cancel yields none. The
   `native_device` backend is `unavailable`.
3. A session left `recording` at restart reopens `interrupted`, with no
   partial source.
4. Segmentation is deterministic, and repeated runs give identical
   segments. Silence and clipping appear in capture health.
5. Transcription persists a route decision. `local_asr_pack`, `cloud_asr`
   and diarization are refused or unavailable with reasons. `fixture_asr`
   produces a revision whose receipt names the engine as a fixture.
6. A correction creates a new revision that links its parent; the parent
   is byte-identical afterwards.
7. Every segment resolves to an `AudioEvidenceRef` whose source digest and
   time range match stored audio.
8. `COMMAND` mode records text only; no code path executes it.
9. Sources, sessions, revisions and receipts survive reopen and
   backup/restore. Tampered audio bytes, rows or cross-row links are
   refused.
10. CLI and Desktop reach AudioFlow only through Core. There is no network
    code and no new dependency.
11. Exact-head and post-main CI pass.

Recorded residuals: no native capture backend, no real ASR or diarization
engine, and medical ASR quality (medications, doses, numbers, negation,
Arabic/English) is `UNMEASURED` until an engine is admitted and evaluated
under `LOCAL_MEDICAL_SCRIBE_EVALUATION_PROTOCOL_2026-09-19.md`.

## Completion rule

`CLOSED_CANONICAL` only after merge on a green exact head and recorded
post-main verification. Closure of 081 does not authorize 082.
