# Research OS V2 Requirements Traceability Matrix — 2026-09-19

**Status:** `PLANNING_CANDIDATE_ONLY`  
**Purpose:** prove that the feature floor and MedScale differentiators are assigned to owning specs and future evidence rather than left as unowned vision.

## Traceability rules

- `P` = primary owning spec.
- `I` = required integration spec.
- `E` = external/non-software gate.
- A requirement is not delivered because it appears in this table.
- Closure requires the evidence named in the owning promoted spec.

| ID | Requirement | Source / rationale | Owner | Required proof |
|---|---|---|---|---|
| CRQ-001 | Explicit ambient encounter capture | Abridge/Suki/Dragon/Freed category floor | 081-P, 079-I | visible capture + no hidden egress + restart |
| CRQ-002 | Local streaming transcription | local-first differentiator | 081-P, 078-I | latency/WER + network-disabled run |
| CRQ-003 | High-accuracy second-pass transcription | scribe quality | 081-P | revision lineage + medical token benchmark |
| CRQ-004 | Speaker diarization | ambient workflow | 081-P | DER/role accuracy + correction |
| CRQ-005 | Arabic clinical speech | product differentiation | 081-P, 088-I | Arabic medical benchmark |
| CRQ-006 | Arabic-English code switching | regional/multilingual use | 081-P, 088-I | code-switch benchmark |
| CRQ-007 | Specialty note templates | Abridge/Suki/Freed floor | 088-P | declarative templates + specialty fixtures |
| CRQ-008 | Custom note template builder | Freed/Suki floor | 088-P | versioned template edit/replay |
| CRQ-009 | Clinician formatting/style personalization | Abridge/Freed | 088-P, 079-I | local policy + no-PHI-egress test |
| CRQ-010 | Pre-visit summary | Suki/Freed | 088-P, 083-I | source/freshness-linked summary |
| CRQ-011 | Prior encounter/context reasoning | Abridge | 077-P, 083-I | bounded ContextManifest + source links |
| CRQ-012 | Patient/chart Q&A | Suki/Freed | 077-P, 083-I | patient-scoped local answer + evidence |
| CRQ-013 | Note span -> transcript/audio evidence | Abridge Linked Evidence + MedScale extension | 081-P, 083-I | span click resolves exact source |
| CRQ-014 | Note span -> FHIR/document/result evidence | MedScale Linked Evidence Everywhere | 083-P, 090-I | exact resource/page/span resolution |
| CRQ-015 | Unsupported-fact detection | scribe safety | 081-P, 077-I | hallucination fixture corpus |
| CRQ-016 | Negation/numeric/dose conflict checks | clinical speech/note safety | 081-P | targeted error taxonomy |
| CRQ-017 | Problem/entity extraction | Abridge/OpenMed | 077-P, 078-I | local NER + proposal state |
| CRQ-018 | Problem grouping/organization | Abridge | 088-P, 083-I | source-linked grouping, no diagnosis authority |
| CRQ-019 | Order/task proposals | Abridge/Suki | 088-P, 090-I | approval + EffectIntent |
| CRQ-020 | Coding proposals | Abridge/Suki/Freed | 088-P, 089-I | licensed code Pack + validation |
| CRQ-021 | Revenue-cycle documentation review | Abridge | 088-P, 090-I | discrepancy fixtures, no compliance overclaim |
| CRQ-022 | Nursing documentation drafts | Abridge nursing | 088-P | source-linked flowsheet/assessment fixtures |
| CRQ-023 | Patient instructions/after-visit material | Suki/Freed/OpenEvidence TakeHome signal | 088-P | clinician review + readability/localization |
| CRQ-024 | Voice commands around documentation | Dragon/Suki | 081-P, 077-I | explicit command mode + capability check |
| CRQ-025 | Dictation mode | Dragon | 081-P | local direct dictation + edit workflow |
| CRQ-026 | EHR/FHIR note writeback | Abridge/Suki/Dragon category floor | 090-P | idempotent effect + unknown reconciliation |
| CRQ-027 | EHR task/order writeback | Abridge/Suki | 090-P, 088-I | durable outbox + duplicate/lost-response tests |
| CRQ-028 | Clinical signal/care-gap proposals | Abridge Care Signals | 083-P, 088-I | rule/model/source + review state |
| CRQ-029 | Enterprise/admin workflow analytics | Abridge enterprise | 084/090-P | minimized metadata + scope controls |
| CRQ-030 | Clinical evidence question answering | OpenEvidence | 077-P, 080-I, 083-I | evidence-grounded answer benchmark |
| CRQ-031 | Explicit per-claim citations | OpenEvidence | 077-P, 083-I | citation/source spans |
| CRQ-032 | Citation existence validation | MedScale differentiator | 080-P, 083-I | DOI/PMID/source resolution |
| CRQ-033 | Claim-support validation | MedScale differentiator | 083-P | claim/citation support benchmark |
| CRQ-034 | Evidence quality dimensions | OpenEvidence EvidenceGrade + stronger transparency | 083-P, 089-I | design/bias/directness/etc visible |
| CRQ-035 | Contradictory evidence detection | MedScale differentiator | 083-P | conflict fixtures |
| CRQ-036 | Population/applicability assessment | evidence quality | 083-P | PICO/applicability fixtures |
| CRQ-037 | Guideline version/jurisdiction awareness | evidence safety | 089-P | conflicting-version/jurisdiction tests |
| CRQ-038 | Retraction/correction awareness | evidence safety | 080-P, 089-I | retracted/corrected source fixture |
| CRQ-039 | Voice medical Q&A | OpenEvidence Voice + local differentiator | 077-P, 081-I | local speech question -> evidence answer |
| CRQ-040 | Saved evidence collections | evidence/research workflow | 083-P | project artifact persistence/reopen |
| CRQ-041 | Patient-facing evidence handout | OpenEvidence TakeHome | 088-P | reviewed handout + source state |
| CRQ-042 | CME/MOC/CE tracking | OpenEvidence | E | external accreditation required |
| CRQ-043 | Local clinical NER | OpenMed | 078-P | admitted model + benchmark |
| CRQ-044 | Local PHI/PII detection | OpenMed | 079-P, 078-I | category metrics + residual uncertainty |
| CRQ-045 | Local de-identification | OpenMed | 079-P | immutable transformed artifact + DeidReceipt |
| CRQ-046 | Local multilingual medical models | OpenMed | 078-P | model/language Pack admission |
| CRQ-047 | Apple MLX acceleration option | OpenMed/Apple ecosystem | 078-P | runtime/hardware qualification |
| CRQ-048 | Local medical VLM/image model option | OpenMed | 078-P, 085-I, 090-I | isolated runtime + image/DICOM source evidence |
| CRQ-049 | Model catalog | OpenMed + MedScale platform | 078-P | model rights/runtime/resource metadata |
| CRQ-050 | Model compare | MedScale differentiation | 078-P | identical-context multi-lane evidence |
| CRQ-051 | Model/runtime replacement without authority change | MedScale architecture | 078-P | substitution tests |
| CRQ-052 | Offline model execution | local-first | 078-P | network-disabled run |
| CRQ-053 | Local file dataset import | research/data need | 075-P | CSV/JSON/Parquet/XLSX qualified import |
| CRQ-054 | Database read connectors | research/lab need | 075-P | read-only DB snapshot |
| CRQ-055 | Hugging Face dataset import | research/lab need | 075-P | exact revision/files receipt |
| CRQ-056 | Kaggle dataset import | research/lab need | 075-P | exact version/files receipt |
| CRQ-057 | Airtable-class grid | founder direction | 075-P | dense typed grid over snapshot/table |
| CRQ-058 | Forms | Airtable/Grist/Baserow floor | 075-P | form -> typed row with validation |
| CRQ-059 | Gallery | Airtable-class floor | 075-P | saved view semantics |
| CRQ-060 | Kanban | Airtable-class floor | 075-P | saved/grouped view semantics |
| CRQ-061 | Calendar/time view | Airtable-class floor | 075-P | date field view + timezone correctness |
| CRQ-062 | Linked records/relations | relational workbench | 075-P | stable row/object references |
| CRQ-063 | Formula/computed fields | spreadsheet-database floor | 075-P | deterministic formula/version behavior |
| CRQ-064 | Data dictionary/schema | research governance | 075-P | schema/version/provenance |
| CRQ-065 | Row/column lineage | reproducible research | 075-P | source-to-output lineage |
| CRQ-066 | Cohort builder | clinical research | 082-P | explicit criteria + unknown counts |
| CRQ-067 | SQL analysis | analytics | 082-P | read-only exact-snapshot QueryReceipt |
| CRQ-068 | Statistical analysis | analytics | 082-P | oracle fixtures + assumption state |
| CRQ-069 | Charts/figures | analytics | 082-P | run/source-linked figures |
| CRQ-070 | AI-assisted query/method proposals | modern analytics UX | 082-P, 077-I | inspect/approve/validate before run |
| CRQ-071 | Python analysis | researcher need | 085-P | staged sandboxed job + receipt |
| CRQ-072 | R/RStudio/Posit workflow | founder Research OS direction | 086-P | staged workspace + RRunReceipt |
| CRQ-073 | Reproducible analysis | core differentiator | 082/085/086-P | exact snapshot/runtime/method replay |
| CRQ-074 | Paper/library workflow | OpenEvidence + research OS | 080/083-P | source identity + saved project artifact |
| CRQ-075 | Research Canvas | AFFiNE-inspired | 083-P | document/canvas/table/graph blocks over shared IDs |
| CRQ-076 | Clinical/Research Graph | Graphify-inspired | 083-P | explained typed edges + rebuild |
| CRQ-077 | Graph path/query/explain | Graphify-inspired | 083-P | deterministic query fixtures |
| CRQ-078 | Temporal clinical graph | healthcare requirement | 083-P | historical-state queries |
| CRQ-079 | Evidence ledger | MedScale differentiator | 083-P | support/contradiction/limitations |
| CRQ-080 | Manuscript/report workspace | researcher workflow | 083-P | citations/data/figures linked by artifact IDs |
| CRQ-081 | Comments/review/tasks | team workflow | 076-P | artifact-revision anchored collaboration |
| CRQ-082 | Offline collaboration semantics | local-first | 076-P | restart/conflict tests |
| CRQ-083 | Self-hosted team Hub | labs/institutions | 084-P | two-client sync + revocation/isolation |
| CRQ-084 | Governed medical web browse | research/evidence | 080-P | brokered fetch + BrowseReceipt |
| CRQ-085 | Prompt-injection resistant browser | security | 080-P | adversarial page campaign |
| CRQ-086 | Local PDF/document intelligence | research/clinical | 075/085-P | hostile-doc isolation + exact revisions |
| CRQ-087 | OCR | research/clinical | 085-P | isolated OCR worker + source region mapping |
| CRQ-088 | DICOM/DICOMweb support | full medical platform | 090-P | exact adapter/profile/security evidence |
| CRQ-089 | Privacy Gate | local/private differentiator | 079-P | egress-denial and deid receipts |
| CRQ-090 | Capture/use/share purpose policy | clinical privacy | 079-P, 081-I | purpose/consent state tests |
| CRQ-091 | Retention/deletion policy | privacy | 079-P | cache/index/backup lifecycle proof |
| CRQ-092 | Capability-scoped compute/tools | security | 085-P | FS/network/secret escape tests |
| CRQ-093 | Extension ecosystem | MedScale differentiation | 087-P | signed/capability-scoped isolated extension |
| CRQ-094 | Research/Evidence Packs | reusable local knowledge | 089-P | signed/versioned rights-aware Pack |
| CRQ-095 | Institutional FHIR/SMART adapters | clinical integration | 090-P | profile/version qualification |
| CRQ-096 | HL7 v2/CDA institutional adapters | institutional reality | 090-P | explicit profile mapping/loss evidence |
| CRQ-097 | Durable external effects | safe action | 090-P | idempotency + unknown reconciliation |
| CRQ-098 | Federation | multi-site later | 091-P | data-stays-site + statistical validity |
| CRQ-099 | Arabic/RTL product UX | localization | 083/088/092-P | RTL + long-string + language tests |
| CRQ-100 | Whole-platform release qualification | completion truth | 092-P | integrated authority/privacy/security/perf/recovery evidence |
| CRQ-101 | Pre-round / inpatient context summary | Abridge 2026 platform direction + inpatient workflow | 088-P, 083-I | source-linked summary, freshness/conflict and omission benchmark |
| CRQ-102 | Clinical-trial candidate matching | Abridge 2026 platform direction + research integration | 083/089-P, 080-I | criterion-by-criterion mapping, unknown handling, registry freshness |
| CRQ-103 | Prior-authorization draft/evidence assembly | Abridge/Availity + current clinical-assistant category | 088-P, 090-I | source-grounded request packet, policy/rules version, clinician review |
| CRQ-104 | Prior-authorization submit/status/reconcile | payer-provider workflow | 090-P | explicit EffectIntent, idempotency, denial/appeal/status and UNKNOWN reconciliation |
| CRQ-105 | Inpatient CDI before discharge | Abridge revenue-cycle floor | 088-P, 090-I | documentation-gap/discrepancy fixtures with source evidence and review |
| CRQ-106 | Pre-bill diagnosis/DRG discrepancy review | Abridge Sep 2026 capability | 088-P, 090-I | final-coded-vs-documentation comparison, evidence behind discrepancy |
| CRQ-107 | E&M level proposal and rationale | current scribe/coding category | 088-P, 089-I | licensed/versioned rules, rationale, clinician/coder review, no billing-authority claim |
| CRQ-108 | HCC / risk-gap candidate surfacing | Abridge Care Signals / RCM | 083-P, 088-I, 090-I | payer/EHR/registry source provenance, rationale, review state |
| CRQ-109 | MEAT criteria documentation support | Abridge Care Signals workflow | 088-P, 089-I | explicit criteria version/rationale and source-linked evidence |
| CRQ-110 | Conversational nursing flowsheet rows | Abridge nursing | 088-P | spoken-source-to-draft-row linkage, nurse verification before chart effect |
| CRQ-111 | Cross-shift/unit nursing context | Abridge nursing/care coordination | 088-P, 083-I | source/freshness/role-scoped context and handoff evaluation |
| CRQ-112 | Deep evidence consultation | OpenEvidence DeepConsult-style research workflow | 077-P, 080-I, 083-I | inspectable research plan, source set, subquestion synthesis, gaps and snapshot |
| CRQ-113 | Secure clinician-patient communication | OpenEvidence Dialer/current communication category | 090-P, 079-I | channel-specific identity/privacy, consent, delivery/UNKNOWN receipts; local core unaffected |
| CRQ-114 | Discharge planning / order-set suggestion workflow | OpenEvidence/clinical-assistant category | 088-P, 090-I | source/evidence-linked proposals, explicit review and controlled effects |
| CRQ-115 | Evidence-aware encounter documentation | OpenEvidence Visits + Abridge CDS convergence | 077/081-P, 080/083-I | note assessment/plan can show evidence without conflating patient fact and general literature |

## Coverage assertion

```text
TRACEABLE_REQUIREMENTS = 115
UNASSIGNED_REQUIREMENTS = 0
IMPLEMENTED_BY_THIS_DOCUMENT = 0
IMPLEMENTATION_AUTHORITY_GRANTED = 0
```

This matrix should be updated whenever a new competitor capability or founder requirement enters scope. A feature without an owning spec and proof target is a planning gap.
