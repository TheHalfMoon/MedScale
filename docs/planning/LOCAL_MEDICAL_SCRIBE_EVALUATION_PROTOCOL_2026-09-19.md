# Local Medical Scribe Evaluation Protocol — 2026-09-19

**Status:** `PLANNING_CANDIDATE_ONLY`  
**Purpose:** define the minimum evidence required to evaluate a MedScale local medical scribe. This protocol prevents generic WER, UI demos or anecdotal clinician preference from being treated as proof of clinical documentation quality.

## 1. Evaluation principle

The scribe is a pipeline, not one model.

Evaluate separately:

```text
capture
 -> audio conditioning
 -> ASR
 -> diarization
 -> clinical extraction
 -> context selection
 -> note generation
 -> source linkage
 -> review
 -> effect/export
```

A strong overall impression cannot hide a dangerous failure in one stage.

## 2. Dataset tiers

### Tier A — Deterministic synthetic
Use fully synthetic scripted encounters for CI/regression.

Must cover:
- medications/doses/routes/frequencies;
- allergy/negation;
- numbers/units;
- lab values;
- dates/ages;
- family/social history;
- overlapping speakers;
- correction statements;
- contradictory prior context;
- specialty terminology.

### Tier B — Licensed / publicly usable non-PHI audio
Use rights-cleared medical speech or institution-approved de-identified material for more realistic acoustic variation.

### Tier C — Prospective approved study
Only after governance/ethics/privacy authority exists.

No real-PHI study data enters ordinary development or public benchmark artifacts without explicit authorization.

## 3. Required languages

Initial benchmark matrix:

- English;
- Arabic;
- Arabic-English code switching.

Additional languages are admitted as Packs only when representative data and evaluation exist.

## 4. Acoustic conditions

Each benchmark should stratify:

- quiet room;
- typical clinic room;
- background noise;
- distant microphone;
- phone/telehealth codec;
- overlapping speech;
- interruptions;
- masks or muffled speech where useful;
- fast speech;
- long pauses;
- multi-hour session soak separately.

## 5. ASR metrics

### General
- WER;
- CER where appropriate;
- real-time factor;
- first partial latency;
- stable segment latency.

### Medical-critical
- medication-name recall/precision;
- dose exactness;
- route/frequency exactness;
- numeric exactness;
- unit exactness;
- lab-value exactness;
- date/time exactness;
- negation preservation;
- allergy statement exactness;
- laterality where present;
- abbreviation handling;
- specialty-term recall.

A substitution that changes `25 mg` to `250 mg` must be counted as a critical numeric error even if aggregate WER remains low.

## 6. Diarization metrics

Measure:
- DER;
- speaker count accuracy;
- role assignment accuracy;
- ambiguous/unknown handling;
- overlapping-speech behavior;
- correction success.

A speaker-role error involving a clinically meaningful statement receives a separate severity label.

## 7. Transcript revision metrics

Compare live and refined transcripts:

- text improvement;
- regression rate;
- timestamp drift;
- segment merge/split correctness;
- source-audio alignment;
- correction lineage completeness.

Refinement may not silently destroy the live transcript revision.

## 8. Clinical extraction metrics

By entity/relation family:
- precision;
- recall;
- F1;
- exact-span where appropriate;
- normalized concept/code accuracy only when licensed/qualified.

Required families include:
- symptom;
- problem/condition;
- medication;
- dose/route/frequency;
- allergy;
- procedure;
- lab/result;
- vital sign;
- family/social history;
- follow-up/action candidate.

## 9. Context-selection metrics

Evaluate whether the context assembler retrieves:
- relevant recent encounters;
- active medications/allergies;
- important abnormal results;
- applicable prior specialist context;
- selected guideline/template material.

Measure:
- critical context recall;
- irrelevant context rate;
- stale-context rate;
- wrong-patient/wrong-encounter rate;
- context size/resource cost.

Wrong-patient context is a critical failure.

## 10. Note factuality taxonomy

Every generated note statement should be classifiable as:

```text
SUPPORTED_EXACTLY
SUPPORTED_PARAPHRASE
CLINICIAN_ADDED
REASONABLE_STRUCTURE_ONLY
UNSUPPORTED_ADDITION
CONTRADICTED
NEGATION_ERROR
NUMERIC_ERROR
TEMPORAL_ERROR
WRONG_SPEAKER
WRONG_PATIENT_CONTEXT
OVERSTATED_CERTAINTY
OMISSION
```

Measure:
- unsupported fact rate;
- contradiction rate;
- critical numeric/medication error rate;
- critical omission rate;
- source-support coverage;
- severity-weighted error rate.

Do not use a single “note quality” score alone.

## 11. Linked Evidence metrics

For every material note span:
- is a source link present when expected?
- does the linked span/audio/resource actually support the note text?
- is the locator exact?
- is the source revision correct?
- does the link survive restart and note edit?
- are conflicting sources also visible?

Measure evidence precision and evidence coverage separately.

## 12. Specialty evaluation

At minimum qualify the specialties/settings actually advertised.

Candidate founding set:
- primary care;
- internal medicine;
- emergency/urgent care;
- cardiology;
- psychiatry/behavioral health;
- pediatrics.

Add surgical, oncology, obstetrics, nursing and other specialties only with adequate fixtures/review.

## 13. Clinician review workload

Measure:
- time to first usable draft;
- time to final reviewed note;
- number of edits;
- edit distance;
- critical corrections;
- sections most frequently rewritten;
- evidence-inspection interactions;
- rejected suggestions;
- user-reported cognitive burden using a predefined instrument where appropriate.

A faster draft that requires unsafe correction is not a success.

## 14. Action/coding proposal metrics

For suggestions such as:
- ICD candidates;
- order/task proposals;
- care gaps;
- patient instructions;

measure:
- support by source/context;
- incorrect proposal rate;
- abstention when insufficient;
- clinician acceptance/rejection;
- attempted unauthorized-effect rate (must be zero by architecture);
- effect reconciliation correctness after explicit approval.

## 15. Privacy metrics

During local mode:
- packet capture / network observation;
- DNS attempts;
- telemetry payload inspection;
- logs;
- crash reports;
- temporary files;
- swap/snapshot residual according to current platform qualification;
- deleted-audio lifecycle;
- de-identification boundary when used.

No “private” claim follows from local model execution alone.

## 16. Resource/performance matrix

For each supported hardware class record:
- CPU/GPU/accelerator;
- RAM/VRAM;
- model sizes;
- startup/load time;
- streaming CPU/GPU utilization;
- peak memory;
- disk growth per hour;
- battery impact where relevant;
- thermal throttling;
- long-session stability.

Report p50/p95 for user-visible latency where sample size supports it.

## 17. Offline campaign

With network disabled:
1. open encounter;
2. start capture;
3. transcribe;
4. diarize;
5. draft note;
6. inspect Linked Evidence;
7. edit/review;
8. save;
9. restart;
10. reopen;
11. export local artifact;
12. queue an external effect without claiming delivery.

The campaign fails if any required local stage secretly depends on network.

## 18. Adversarial/safety cases

Include:
- patient asks AI-directed question during encounter;
- clinician dictates an instruction that resembles a tool command;
- transcript contains prompt-injection-like text;
- prior note contains malicious instruction text;
- patient says “ignore that, I meant 25 not 250”;
- two speakers contradict each other;
- source result is amended;
- audio device disconnects;
- disk becomes full;
- model crashes/OOM;
- app restarts during capture;
- evidence source unavailable;
- EHR write times out after send.

## 19. Human evaluation

When authorized, use blinded review by appropriately qualified clinicians with:
- standardized rubric;
- adjudication process;
- specialty matching;
- error severity;
- inter-rater agreement;
- exact system/model revision;
- disclosed source context.

Do not publish “clinician preferred” or “clinically accurate” without the study design and data.

## 20. Release thresholds

Each promoted scribe spec must define numeric or categorical acceptance thresholds before the final evaluation run. Thresholds must not be invented after results are seen.

Where no defensible threshold exists, report the metric and retain the capability as experimental/not-qualified.

## 21. Evidence output

A qualification packet should bind:
- code revision;
- model/runtime Packs;
- benchmark Pack revision;
- hardware;
- OS;
- commands/config;
- raw aggregate results;
- error taxonomy summary;
- failed cases;
- limitations;
- reviewer protocol;
- decision.

No screenshot is sufficient evidence of scribe quality.
