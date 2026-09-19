# Founder GitHub Donor Synthesis — 2026-09-19

**Status:** `PLANNING_CANDIDATE_ONLY`  
**Scope:** review of repositories accessible through the founder-connected GitHub account for ideas that may strengthen MedScale.  
**Privacy rule:** private-repository implementation details are not reproduced here. A private source may be used only after exact authorization, provenance and disclosure review.

## 1. Review scope

The connected account exposed 33 repositories at review time, including public and private projects. The purpose of this pass was not to merge sibling projects into MedScale. It was to identify reusable product and architecture patterns while preserving independent project authority.

No sibling repository becomes a dependency, authority plane or code donor merely because it is controlled by the same founder.

## 2. Highest-value public sibling sources

| Repository | Relevant value for MedScale | Candidate posture |
|---|---|---|
| TheHalfMoon/Himsat | local conversation/audio capture, two-pass transcription direction, diarization, evidence-linked memory, document intelligence, encrypted local vault posture | `REFERENCE / ADAPT / COPY_SELECTIVE after qualification` |
| TheHalfMoon/Wispral | local streaming STT comparison, whisper.cpp/sherpa-onnx/Moonshine research, voice interruption/session semantics | `REFERENCE / ADAPT` |
| TheHalfMoon/commandMed | medical model safety, multilingual evaluation, evidence/tool separation, abstention and resource-aware medical intelligence | `REFERENCE / ADAPT` |
| TheHalfMoon/Ecra | governed browser/search, source-aware knowledge, human/agent action, capability-scoped execution | `REFERENCE / ADAPT` |
| TheHalfMoon/Signthos | local PDF inspection/editing, OCR/conversion worker isolation, no-silent-upload, hostile-document handling | `REFERENCE / ADAPT / bounded component transfer after qualification` |
| TheHalfMoon/Zyara | healthcare graph, FHIR boundaries, patient/provider identity, consent, voice/AI safety, analytics patterns | `REFERENCE / ADAPT` |
| TheHalfMoon/commandF | FHIR package/conformance and breaking-change intelligence | `REFERENCE / optional external qualification tool` |
| TheHalfMoon/Sentrdel | evidence classification, provenance, contradiction, graph and local security control-plane patterns | `REFERENCE / ADAPT` |
| TheHalfMoon/Golam | local-first agent/model/memory/tool ownership patterns | `REFERENCE` |
| TheHalfMoon/kernux | capability kernel, browser/tool/runtime orchestration and evidence model | `REFERENCE / ADAPT after owning spec` |
| TheHalfMoon/Tarif | deterministic authorization for agent/tool actions, secret isolation, explainable authority | `REFERENCE / ADAPT` |
| TheHalfMoon/Flake | local project continuity, evidence-linked decisions and replaceable-agent context | `REFERENCE` |
| TheHalfMoon/SpecGrain | bounded specifications and explicit proof requirements | `PROCESS_REFERENCE` |
| TheHalfMoon/Diffcipline | think/challenge/minimize/change/prove discipline | `PROCESS_REFERENCE` |
| TheHalfMoon/Kodac | proof-oriented agent execution and Done Gate concepts | `PROCESS_REFERENCE` |
| TheHalfMoon/Winds | independent exact-snapshot verification/evidence | `PROCESS_REFERENCE` |
| TheHalfMoon/Delethos | proof-carrying delegation and independent review | `PROCESS_REFERENCE` |
| TheHalfMoon/Ascout | bounded changed-code verification and evidence receipts | `PROCESS_REFERENCE` |
| TheHalfMoon/MESC | FHIR-native verification and reproducible medical research patterns only | `REFERENCE`; separate project; no coupling |

## 3. Scribe synthesis from Himsat + Wispral

MedScale should reuse lessons, not create runtime coupling.

From Himsat planning:

- explicit user-authorized microphone/system-audio capture;
- local/background capture within platform rules;
- two-pass transcription;
- local speaker diarization;
- multimodal session context;
- evidence-first summaries;
- local cross-session memory/search;
- encrypted local vault;
- local agent/automation capability boundaries.

From Wispral research:

- measure local streaming ASR rather than selecting by popularity;
- compare whisper.cpp, sherpa-onnx and other qualified engines under one harness;
- preserve interruption and session semantics;
- measure CPU-only viability and resource needs;
- treat speech recognition, entity resolution and agent behavior as separate qualification axes.

MedScale-specific addition:

- clinical numeric/unit/medication accuracy;
- specialty terminology;
- Arabic/English code switching;
- transcript-to-audio evidence;
- note drafting and clinician approval;
- EHR/FHIR output under controlled effect semantics.

## 4. Evidence Copilot synthesis from commandMed + MedScale foundations

commandMed research reinforces:

- medical usefulness must be measured;
- model/retrieval/tool/runtime/provider boundaries need independent security;
- evidence and tool output cannot collapse into model authority;
- multilingual and resource-constrained evaluation matters;
- abstention/escalation is a first-class result.

MedScale should combine those ideas with its own evidence/retrieval and artifact authority rather than import commandMed as a runtime dependency.

## 5. Governed Browse synthesis from Ecra + existing Orca candidate planning

Ecra contributes product thinking for:

- one interface across web/local sources/tools;
- provenance-aware search and memory;
- typed human/agent actions;
- receipts and capability-scoped execution.

The separate MedScale Orca draft plan contributes candidate pane/session/browser interaction mechanics.

Combined target:

```text
Browse request
 -> explicit network capability
 -> browser/source fetch
 -> capture receipt
 -> immutable source artifact
 -> project attachment
 -> citation/evidence extraction
```

No browsing engine receives ambient access to patient vault data.

## 6. Documents synthesis from Signthos

Useful Signthos principles:

- malformed/hostile document threat model;
- active content default deny;
- no automatic network from document parsing;
- resource limits and cancellation;
- explicit encrypted-document secret handling;
- content-changing operations create new revisions;
- OCR/conversion behind isolated providers;
- local/offline operation visible to the user.

MedScale should adapt these principles to clinical/research PDF, image and office-document ingestion.

## 7. Healthcare graph synthesis from Zyara

Useful public Zyara planning patterns include:

- FHIR-aligned patient timelines;
- explicit per-field/source authority;
- health-data consent and auditability;
- patient/provider/practitioner-role graph distinctions;
- analytics separated from raw clinical access;
- voice/transcript data minimized by default;
- AI/tool action confirmation.

MedScale should reuse these concepts where consistent with existing Core contracts, not adopt Zyara's marketplace/booking product semantics.

## 8. Authority synthesis from Tarif / Kernux / Sentrdel

Together these sibling projects reinforce one architecture:

```text
Model proposes
 -> policy evaluates
 -> capability granted
 -> isolated tool/runtime executes
 -> receipt records effect
 -> MedScale Core reconciles outcome
```

This is particularly important for:

- EHR writes;
- orders;
- coding/export;
- browser actions;
- remote source acquisition;
- dataset refresh;
- external messaging;
- agent tools.

## 9. Research and verification synthesis

SpecGrain, Diffcipline, Winds, Kodac, Delethos and Ascout should remain process/verification references.

MedScale should continue to require:

- bounded candidate specs;
- explicit dependencies;
- exact revision binding;
- independent checks;
- evidence receipts;
- no self-reported completion authority;
- no release/privacy claims inferred from code existence.

## 10. Private-repository boundary

Private repositories available through the connected account were reviewed for relevance, but this public MedScale planning document intentionally does not enumerate unpublished implementation details.

Before a private sibling contributes code or architecture:

1. record explicit founder authorization for use and disclosure;
2. pin exact repository/revision/path;
3. record license/ownership status;
4. decide whether provenance can be made public;
5. define the MedScale-owned interface;
6. threat-model the transfer;
7. copy/adapt only the bounded component justified by the owning spec.

## 11. External donor additions

The following external sources should be added to future exact donor qualification:

| Source | Candidate value | Initial posture |
|---|---|---|
| Graphify-Labs/graphify | graph provenance, explained edges, path/query/explain, heterogeneous artifact graph | `REFERENCE / ADAPT` |
| toeverything/AFFiNE | docs/canvas/table workspace patterns, local/self-host UX, collaborative blocks | `REFERENCE / ADAPT`; exact licensing review before transfer |
| gristlabs/grist-core | relational spreadsheet/data workbench patterns, Apache-2.0 community edition | `REFERENCE / ADAPT / COPY_SELECTIVE` |
| baserow/baserow OSE | no-code database/view/API patterns, MIT OSE | `REFERENCE / ADAPT / COPY_SELECTIVE` |
| nocodb/nocodb | Airtable-alternative feature research | `REFERENCE_ONLY` under current Sustainable Use License |
| teableio/teable | spreadsheet-database UX research | `REFERENCE_ONLY` for core app; exact MIT-package review only |

## 12. Anti-pattern: sibling monolith

Do not solve the founder's broad goal by embedding entire sibling/external products.

Reject:

```text
MedScale
 + full AFFiNE
 + full Graphify
 + full Grist
 + full Himsat
 + full Ecra
 + full OpenMed
```

That would create duplicated identities, storage systems, permissions, runtimes and upgrade surfaces.

Prefer:

```text
MedScale-owned stable contracts
 + selectively qualified algorithms/components
 + isolated workers where required
 + external tools behind explicit adapters
 + one source/provenance/authority model
```

The objective is feature breadth with architectural coherence.
