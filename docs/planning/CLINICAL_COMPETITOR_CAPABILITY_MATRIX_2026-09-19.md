# Clinical Competitor Capability Matrix — 2026-09-19

**Status:** `PLANNING_CANDIDATE_ONLY`  
**Purpose:** define the current external capability floor MedScale should study and eventually match or exceed through local-first, privacy-first architecture.  
**Warning:** public vendor capabilities change quickly. Reverify before implementation or parity claims.

## Capability matrix

| Capability | Abridge | OpenEvidence | OpenMed | Suki / Dragon / Freed signal | MedScale target |
|---|---|---|---|---|---|
| Ambient clinical capture | Core | Emerging visit/documentation surface | No | Core scribe category | Local-first capture with visible consent/state |
| Live transcription | Core workflow | Voice Q&A, not primary scribe historically | Local model ecosystem can support pieces | Core scribe category | Local streaming ASR + refined second pass |
| Speaker diarization | Ambient workflow requirement | Not central | Model ecosystem dependent | Common scribe need | Local diarization with reviewable roles |
| Structured note draft | Core | Visit/documentation surface evolving | Models/components, not complete workflow | Core | Specialty-aware local drafts |
| Multi-specialty | Yes | Evidence across specialties | Broad medical model catalog | Yes | Pack/template-based specialty support |
| Multilingual | Yes | Broad evidence UI/content | 21+ language claims in OpenMed | Suki/others support multilingual features | Arabic + English + code-switching as first qualification target |
| Transcript-linked evidence | Linked Evidence | Citations to external literature | Provenance varies by model | Not consistently category-wide | Linked Evidence Everywhere: audio + transcript + FHIR + docs + literature |
| Prior patient context | Contextual Reasoning Engine | Clinical Q&A context can be supplied | N/A | Suki pre-visit summaries/chart Q&A | Local bounded longitudinal context |
| Clinician preference/style | Yes | User history/personalization possible | N/A | Freed learns formatting; Suki templates | Local template/style profile, never authority |
| Problem extraction/grouping | Yes | Clinical answers/differential support | NER model floor | Common clinical assistant feature | Local extraction + explicit proposal state |
| Order/action proposals | Yes, clinician review | Not primary | No | Suki order staging | Controlled proposals + approval + durable outbox |
| Coding suggestions | Yes/revenue-cycle alignment | Not primary | Model components possible | Suki/Freed coding | Licensed/rules-aware coding proposals with evidence |
| Revenue-cycle review | First-class | No | No | Some scribe products add coding | Documentation-to-code discrepancy review |
| Inpatient CDI before discharge | First-class | No | No | Varies | Conversation-grounded CDI candidates before signature/discharge, review required |
| Pre-bill diagnosis/DRG review | Announced/current revenue-cycle capability | No | No | Varies | Evidence-backed diagnosis/DRG discrepancy review before submission |
| E&M level support | Revenue-cycle workflow | OpenEvidence/other assistants have coding workflows | No | Suki/Freed category signal | Rules/licensing-aware E&M proposal with rationale; no billing authority |
| HCC / risk-gap surfacing | Care Signals / revenue cycle | Not primary | No | Some assistants | Source-linked risk-gap/HCC candidates with visible provenance and review |
| MEAT criteria support | Care Signals documentation workflow | Not primary | No | Varies | Explicit criteria/rationale support; never automatic coding authority |
| Prior authorization | Abridge/Availity real-time direction | Drafting/workflow capabilities reported in current OpenEvidence ecosystem | No | Suki/category workflows | Draft, evidence assembly, submit/reconcile through explicit institutional adapter |
| Pre-round / inpatient summary | Public 2026 platform direction | Visits/rounding support | No | Suki pre-visit category | Local patient-context pre-round summary with freshness/conflict state |
| Clinical trial matching | Public 2026 platform direction | Deep research can inform | No | Varies | Local/controlled trial candidate matching with criterion-by-criterion evidence |
| Secure patient communication | Not primary core | Dialer/calls/messaging/voicemail/fax product surface reported | No | Front-desk/communication category | Optional institution adapter; no default cloud dependency; channel-specific privacy |
| Deep evidence synthesis | CDS | DeepConsult-style complex synthesis | No | CDS assistants vary | Transparent long-running Evidence Consult with plan, source set, gaps and immutable evidence snapshot |
| Nursing documentation | First-class | No | No | Limited category coverage | AudioFlow Advanced nursing flowsheet/assessment drafts |
| Care gaps/signals | Care Signals | Evidence Q&A can inform | No | Some assistants | Evidence-linked, explicitly non-authoritative signals |
| Clinical evidence Q&A | CDS capability | Core | Model floor, not full product | Freed adds CDS | Local evidence copilot with rights-aware retrieval |
| Explicit citations | Linked source data / CDS sources | Core | Depends on app | Varies | Per-claim citations + source support checks |
| Evidence quality grading | Not central product signature | EvidenceGrade | No | Varies | Multi-dimensional evidence appraisal, not false-precision single score only |
| Guideline integration | Health-system guidelines | Major content/guideline partners | Possible local packs | Varies | Versioned local guideline/research packs |
| Citation existence validation | Source-linked internal data | Strong public reference record; still independently validate | N/A | Varies | DOI/PMID/title/author/version verification |
| Claim-support verification | Linked Evidence for source inputs | Must still be independently checked | N/A | Varies | Separate claim-support verifier |
| Contradiction tracking | Contextual reasoning likely internal | Not a primary public UX | No | Varies | First-class contradiction ledger |
| Voice medical Q&A | No primary public differentiator | Voice Mode | Model/runtime pieces | Suki voice-first; Dragon voice | Fully local voice Q&A over same evidence engine |
| Patient education | Could draft outputs | TakeHome | No | Suki multilingual patient instructions | Clinician-reviewed local patient handouts |
| CME/MOC/CE | No | Education platform | No | No | External-partnership candidate only |
| Local/offline model execution | No, cloud enterprise architecture | No | Core strength | Mostly hosted | Core MedScale differentiator |
| Local PHI de-identification | No product premise | Not product premise | Core strength | Hosted compliance models | Privacy Gate + qualified local de-ID packs |
| Model catalog/fleet | Proprietary | Proprietary | Broad open model ecosystem | Proprietary | Open local model fleet + compare + exact provenance |
| Vision/multimodal medical models | Product-specific | Multimedia content | Medical VLM model families | Varies | Admitted local VLM/document/image tools |
| Datasets/workbench | No | No | Model datasets, not user workbench | No | First-class local Data Workbench |
| Reproducible analytics | Admin analytics, not research plane | No | Benchmarks | No | Analytics Gate with snapshot/run receipts |
| Research canvas | No | Saved evidence/search, not full canvas | No | No | Docs + canvas + tables + graph + citations |
| Clinical/patient graph | Internal context, not exposed as open graph | No | No | No | Provenance-rich derived Clinical Graph |
| Team/project workspace | Enterprise deployment, not research OS | Account-oriented | No | Enterprise admin | Projects, team review, artifacts, shared research |
| Governed browser/research | No | Medical search product | No | No | Source-capture browser with network/provenance receipts |
| PDF/document intelligence | Context inputs vary | Literature content | Components | Varies | Local document/OCR pipeline with Signthos-style isolation |
| Self-host / air-gap | Not public core model | Not public core model | Strong local story | Generally hosted | Explicit target |
| Open source / inspectable runtime | No | No | Yes | No | MedScale-owned open contracts + auditable local components |
| Data ownership | Enterprise governed cloud | Hosted service | Local-first | Hosted | User/institution-owned local vault and exports |
| Per-action capability policy | Enterprise governance | Account/security | Not main focus | Vendor controls | Deterministic Privacy/Capability Gate |
| Offline degradation | Hosted dependency | Hosted dependency | Strong local model operation | Hosted dependency | Core workflows remain useful offline |

## Abridge observations

Public Abridge product material describes one platform with clinician, nursing and revenue-cycle experiences plus CDS and Care Signals. Its core workflow includes EHR-integrated clinical notes, multiple care settings, multilingual support, prior-encounter and guideline context, clinician preferences, problem prediction/grouping, actionable outputs such as orders for clinician review, and Linked Evidence across source inputs.

Current 2026 public material also expands the floor to pre-visit and pre-round summaries, clinical trial matching, inpatient CDI, real-time revenue-cycle intelligence, HCC/risk-gap workflows, evidence-backed pre-bill diagnosis/DRG discrepancy review, conversational nursing flowsheet drafts, and a payer-provider direction for real-time prior authorization.

References:
- https://www.abridge.com/
- https://www.abridge.com/platform/clinicians
- https://www.abridge.com/platform/nursing
- https://www.abridge.com/platform/revenue-cycle
- https://www.abridge.com/keynote
- https://www.abridge.com/press-release/pre-bill-review-for-cdi-and-coding-teams
- https://www.abridge.com/press-release/abridge-availity-collaboration-announcement

## OpenEvidence observations

Peer-reviewed literature describes OpenEvidence as retrieval-augmented clinical question answering grounded in curated biomedical sources with explicit citations. Current product surfaces also include EvidenceGrade, voice interaction, licensed publisher/guideline content, education/CME-MOC features, patient TakeHome materials, and Visits-style clinical documentation/patient-document context. Secondary current product reviews also describe DeepConsult-style extended synthesis, privacy-oriented clinician-patient communications, and administrative/coding workflow expansion; these are research signals that must be reverified against primary product truth before implementation parity claims.

References:
- https://www.nature.com/articles/s41746-026-03077-4
- https://www.nature.com/articles/s44401-026-00142-8
- https://takehome.openevidence.com/
- https://apps.apple.com/us/app/openevidence/id6612007783
- https://www.newswise.com/articles/openevidence-launches-evidencegrade-empowering-physicians-to-see-the-strength-of-cited-evidence-beneath-each-ai-answer
- https://www.newswise.com/articles/openevidence-wide-releases-ai-integrated-doctor-dialer-for-privacy-centric-doctor-patient-telemedicine-calls-messaging-and-voicemail-in-one-unified-clinical-platform-with-live-clinical-decision-ai-deeply-integrated

The evidence base is still evolving; product usefulness does not remove the need for prospective validation and clinician judgment.

## OpenMed observations

OpenMed's public repository positions it as local-first healthcare AI with on-device clinical NER and PII/PHI de-identification, broad medical model availability, multilingual support, Apple MLX/Python paths and no-cloud configurations.

Reference:
- https://github.com/maziyarpanahi/openmed

MedScale should absorb qualified models/contracts selectively. OpenMed never becomes MedScale clinical authority.

## Other ambient-scribe observations

A 2026 comparison of Suki, Dragon Copilot, Abridge and Freed highlights additional category features MedScale should account for:

- Suki: pre-visit summaries/chart Q&A, ambient notes, voice commands, order staging, coding and multilingual patient instructions;
- Dragon Copilot: dictation plus ambient documentation in a unified clinical voice workflow;
- Freed: specialty templates, formatting personalization, browser-extension EHR push, pre-visit context, coding, clinician assistant, front-desk/reception workflow and evidence-oriented CDS.

Reference:
- https://intuitionlabs.ai/articles/suki-vs-nuance-dax-vs-abridge-vs-freed

Treat vendor metrics and third-party estimates as research inputs, not proof of MedScale requirements or competitor superiority.

## Surpass criteria

MedScale should use explicit evidence rather than marketing language before claiming parity/surpass.

Candidate proof axes:

| Axis | Required MedScale proof |
|---|---|
| Privacy | packet capture / network tests establish no patient content egress in local mode |
| Offline | end-to-end scribe/evidence/data workflows run with network disabled |
| Traceability | every material generated claim resolves to exact admitted source spans/objects |
| Medical ASR | specialty + numeric + medication + Arabic/English benchmark suite |
| Note quality | blinded clinician evaluation with source-grounded error taxonomy |
| Evidence | citation existence + claim support + evidence-quality + contradiction checks |
| Data | snapshot-bound transformations and reproducible lineage |
| Analytics | deterministic/replayable run receipts |
| Model choice | multiple local model/runtime candidates under identical authority contracts |
| Resource efficiency | measured RAM/VRAM/CPU/GPU/latency on declared hardware |
| Safety | unsupported/conflict/unknown states remain visible and cannot silently authorize actions |
| Interoperability | FHIR/SMART behavior qualified against exact profile/version/adapters |
| User experience | task-time/usability studies for capture, review, evidence inspection and analysis |

No `PARITY`, `SURPASS`, `PRIVATE`, `OFFLINE`, or clinical-quality claim is valid before the corresponding exact evidence exists.
