# Local Medical Scribe Plan

**Date:** 2026-09-19  
**Status:** `PLANNING_CANDIDATE_ONLY`  
**Target:** an Abridge/Suki/Dragon/Freed-class documentation workflow that remains useful with no MedScale cloud and keeps patient audio/data local by default.

## 1. Product promise

A clinician should be able to:

1. open an encounter;
2. explicitly start local capture;
3. conduct the visit normally;
4. see a live local transcript;
5. receive a structured note draft;
6. click any material note statement to inspect transcript/audio and other source context;
7. edit/review the draft;
8. inspect suggested problems, codes, orders, tasks and patient instructions as proposals;
9. approve only the items they want;
10. export/write back through an authorized EHR/FHIR path.

The system must remain useful with network egress disabled.

## 2. Trust model

```text
Audio bytes               = source evidence
Transcript                 = derived evidence
Clinical extraction        = derived proposal
Generated note             = draft proposal
Suggested code/order/task  = action proposal
Clinician-approved output  = reviewed artifact
External EHR write         = controlled effect
```

No layer silently promotes itself to a higher authority class.

## 3. Capture subsystem

Candidate capture inputs:

- microphone;
- selected system audio where the OS/platform legally and technically permits;
- telehealth audio through explicit supported integration;
- imported audio/video;
- dictated post-encounter addendum.

Required controls:

- visible capture state at all times;
- explicit start/stop/pause;
- input-device selection;
- input quality meter;
- silence/VAD state;
- local disk-space/resource warning;
- explicit consent/workflow reminder configurable by institution/jurisdiction;
- no stealth/background capture design that bypasses OS privacy indicators.

Raw audio enters an encrypted rolling buffer rather than an unrestricted general file path.

## 4. Speech pipeline

Use a two-pass architecture.

### Pass A — live

Goals:

- low latency;
- stable partial transcripts;
- robust pause/turn handling;
- bounded CPU/GPU use;
- enough accuracy for live navigation.

Candidate engines to qualify include:

- sherpa-onnx streaming ASR;
- whisper.cpp streaming/segmented modes;
- Moonshine-class streaming models where license/quality permit;
- platform-native engines only as explicit adapters, never as hidden cloud dependencies.

### Pass B — refinement

After the encounter or at segment boundaries:

- run a higher-accuracy local decode;
- reconcile live and refined text;
- retain correction history;
- preserve timestamp alignment;
- never overwrite source evidence silently.

Himsat and Wispral provide sibling-project research patterns for local capture/STT; direct code reuse requires exact donor qualification.

## 5. Medical speech quality

Qualification datasets must include synthetic/licensed non-PHI examples covering:

- medication names;
- doses and units;
- allergies;
- negation;
- family history;
- numbers and dates;
- lab values;
- abbreviations;
- specialty terminology;
- clinician/patient interruption;
- overlapping speech;
- noisy rooms;
- telephone/telehealth audio;
- Arabic;
- English;
- Arabic/English code switching;
- accented English;
- names that resemble medical terms.

Measure at minimum:

- WER/CER;
- medical concept recall;
- medication/dose exactness;
- numeric/unit error rate;
- speaker attribution accuracy;
- timestamp alignment;
- real-time factor;
- first-token and stable-segment latency;
- CPU/GPU/RAM/VRAM;
- battery impact where applicable.

General WER alone is not sufficient qualification.

## 6. Diarization

Local diarization should classify speakers as reviewable roles such as:

```text
Clinician
Patient
Caregiver
Interpreter
Other
Unknown
```

Optional clinician voice enrollment may improve role assignment, but biometric identity must remain separately consented and protected.

Unknown or ambiguous speaker identity remains visible. Do not silently attribute a statement to the patient.

## 7. Transcript authority

Each transcript segment should bind:

- segment ID;
- source audio object/revision;
- start/end timestamp;
- speaker label and confidence/state;
- first-pass text;
- refined text;
- correction history;
- ASR model/runtime/version;
- language;
- quality flags;
- extraction references.

The audio span must remain independently replayable while retained.

## 8. Clinical extraction

Local extraction may propose:

- symptoms;
- conditions;
- medications;
- doses/routes/frequencies;
- allergies;
- procedures;
- vital signs;
- laboratory values;
- imaging references;
- family/social history;
- review-of-systems concepts;
- physical-exam statements;
- assessment/plan concepts;
- follow-up intervals;
- action candidates.

OpenMed is a key candidate donor/model ecosystem for local NER and privacy tooling.

Extraction never becomes a canonical diagnosis or medication fact merely because a model recognized it.

## 9. Context assembler

Before note generation, assemble a bounded encounter context from admitted local sources:

- current encounter transcript;
- active problem list;
- medications;
- allergies;
- recent results;
- previous encounter summaries;
- relevant specialist notes;
- clinician-selected templates;
- clinician style preferences;
- institution-local documentation guidance;
- locally admitted guideline/evidence snippets where justified.

Each context element retains source identity and timestamp.

Avoid sending an entire longitudinal record to a model when a bounded relevant subset is sufficient.

## 10. Note generation

Candidate note formats:

- SOAP;
- H&P;
- progress note;
- consult;
- emergency/urgent care;
- procedure note;
- discharge/visit summary;
- specialty templates;
- nursing draft documentation;
- custom clinician/institution template.

Generation should be schema-aware when the target workflow has a structured template.

The clinician owns editing and sign-off.

## 11. Linked Evidence

Every material draft statement should have one of these states:

- `SOURCE_LINKED`;
- `DERIVED_FROM_CONTEXT`;
- `INFERRED_REVIEW_REQUIRED`;
- `UNSUPPORTED`;
- `CONFLICTING_SOURCE`;
- `CLINICIAN_ADDED`.

A source-linked statement may resolve to:

- transcript segment;
- audio time span;
- FHIR resource/field;
- document page/region;
- prior note span;
- result/observation;
- guideline/paper citation.

This should be available at sentence/span granularity, not only whole-note provenance.

## 12. Hallucination/conflict controls

Before showing a draft as ready for review:

- detect newly introduced facts not supported by admitted context;
- detect negation reversals;
- detect medication/dose mismatches;
- detect demographic mismatch;
- detect source conflicts;
- detect temporal contradictions;
- detect uncertain speaker attribution;
- flag unsupported specificity;
- separate omitted content from contradicted content.

These checks reduce risk; they do not prove clinical correctness.

## 13. Active assistant

Beyond passive scribing, MedScale may support local voice/text commands such as:

- show last potassium;
- summarize recent cardiology encounters;
- open the evidence for this diagnosis statement;
- draft a follow-up task;
- propose an order;
- generate patient instructions;
- compare this medication plan with the local guideline pack.

Any operation with external or clinical consequences remains a proposal until approved.

## 14. Coding and revenue support

Candidate proposal surfaces:

- ICD-10 diagnosis candidates;
- CPT/E&M support where licensed rules/data permit;
- HCC candidates;
- documentation gaps;
- diagnosis-to-documentation support;
- pre-bill discrepancy review;
- missing specificity prompts.

Do not claim coding compliance or reimbursement optimization without qualified rules/content and independent validation.

Restricted terminology/code assets require explicit licensing and pack governance.

## 15. Nursing workflow

A future AudioFlow Advanced slice may draft:

- flowsheet candidates;
- structured assessments;
- handoff summaries;
- care-plan updates;
- patient education documentation;
- task/event capture.

Every structured field should link back to conversation/source context.

## 16. Patient instructions

Generate clinician-reviewable after-visit material with:

- plain-language summary;
- medication/instruction list;
- warning signs;
- follow-up;
- reading-level control;
- supported languages;
- source/clinician-review state.

Do not silently transform general evidence into patient-specific medical advice.

## 17. EHR/FHIR integration

Preferred order:

1. MedScale-reviewed artifact;
2. FHIR/SMART or institution adapter where available;
3. explicit copy/export fallback;
4. controlled write with durable outbox and reconciliation.

A lost connection must never make the UI claim a write succeeded.

## 18. Privacy architecture

Default posture:

- raw audio local;
- transcript local;
- note local;
- model inference local;
- no patient content in telemetry;
- no patient content in crash reports;
- no remote model fallback;
- no automatic cloud sync;
- no hidden upload for transcription;
- keys protected through the qualified MedScale vault/key custody architecture;
- raw-audio retention configurable by user/institution;
- deletion produces auditable lifecycle state without retaining the deleted content.

Optional remote providers may only exist behind a future explicit Privacy Gate policy with clear user/institution authorization.

## 19. Resource modes

Offer transparent local profiles rather than hidden quality changes:

```text
Low Power
Balanced
High Accuracy
Custom / Institution Pack
```

Each profile reports the actual ASR/diarization/generation models and expected resource needs.

## 20. Offline behavior

With network disabled, the user should still be able to:

- capture;
- transcribe;
- draft;
- edit;
- inspect linked evidence;
- search local patient context;
- use admitted local models;
- save;
- export to local artifacts;
- queue a later external write without claiming delivery.

Evidence search against remote literature or EHR adapters may become unavailable, but local packs and previously admitted sources remain usable.

## 21. Candidate ownership in Research OS V2

- 079 Privacy Gate — export/remote-model/retention policy;
- 081 AudioFlow Foundation — capture, ASR, diarization, transcript authority;
- 077 MedAgent — active assistant and context-aware reasoning;
- 078 Model Fleet — ASR/LLM/NER runtime and model admission;
- 083 Knowledge Canvas — encounter artifacts and linked evidence views;
- 088 AudioFlow Advanced — specialty templates, multilingual/code-switching, nursing flows;
- 090 Institutional Adapters — EHR/SMART/FHIR/vendor integration;
- 092 Whole-Platform Qualification — final cross-surface scribe qualification.

This document does not promote those candidates.
