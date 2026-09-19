# Research OS V2 Clinical Intelligence Gap-Closure Review — 2026-09-19

**Status:** `PLANNING_CANDIDATE_ONLY`  
**Purpose:** challenge the Clinical + Research Intelligence expansion for missing product, architecture, privacy, security, clinical, research, data, operational and release concerns before implementation promotion.

## Review conclusion

The previous expansion direction was strong but not implementation-complete by itself. This review identified cross-cutting gaps that could otherwise appear late as expensive architecture changes.

The implementation-ready master plan now assigns every identified architecture/product gap to an owning Spec 075–092 or records it as an explicit external/future gate. The register currently contains 140 challenged gaps/failure risks.

**Important:** `NO_KNOWN_PLANNING_GAP` does not mean the product is proven safe, complete or release-ready. New evidence may reveal new gaps during implementation.

---

# 1. Gap register

| ID | Gap / failure risk | Resolution | Owner | Closure evidence |
|---|---|---|---|---|
| G001 | Patient identity from multiple sources could silently merge | candidate match + evidence + explicit confirmation/authoritative matching; no silent `same_as` | 083 / 090 | ambiguous/mismatch fixtures, merge/unmerge audit |
| G002 | Encounter identity could fragment scribe/data/evidence | one encounter context ID referencing source encounters rather than copying them | 081 / 083 / 090 | cross-source encounter mapping/restart tests |
| G003 | Recording consent/legal workflow could be treated as microphone permission | explicit capture policy/purpose/consent metadata separate from OS permission | 079 / 081 / institution policy | capture-denied/expired/unknown tests |
| G004 | Generic WER hides dangerous medical transcription errors | medical ASR benchmark for drugs/doses/numbers/units/negation/labs | 081 / 092 | category-level accuracy report |
| G005 | Speaker diarization can misattribute patient/clinician statements | reviewable speaker roles + unknown/ambiguous states; persistent identity opt-in | 081 | diarization/role fixtures + correction audit |
| G006 | Second-pass ASR can silently rewrite source meaning | immutable transcript revisions with source/time alignment | 081 | revision/replay/provenance tests |
| G007 | Note generation can introduce unsupported facts | unsupported/contradiction/negation/numeric checks + source-linked spans | 081 / 077 | hallucination/source-support test corpus |
| G008 | Note text may lose source provenance after clinician edits | claim/span lineage plus `CLINICIAN_ADDED` / amended state | 081 / 083 | edit/review lineage tests |
| G009 | Clinician style learning could leak PHI or silently change templates | local policy-controlled learned style, versioned template, explicit enablement | 088 / 079 | no-egress + template version tests |
| G010 | Coding suggestions may rely on restricted code systems | terminology/code Packs with explicit rights/version | 088 / 089 | license/NOTICE/version gate |
| G011 | Proposed orders/tasks could become real actions accidentally | ActionProposal -> explicit approval -> EffectIntent -> durable outbox | 088 / 090 | timeout/duplicate/unknown effect tests |
| G012 | Nursing workflow could be overlooked as scribe expands | dedicated structured nursing draft/flowsheet/handoff slice | 088 | nursing synthetic workflows + source links |
| G013 | Patient instructions could become unreviewed personalized medical advice | clinician-reviewable artifact; evidence/context distinction; approval required | 088 | review/export state tests |
| G014 | Pre-visit summary could hide stale source data | source timestamps/freshness/conflict state in summary | 088 / 083 | stale/conflicting longitudinal fixtures |
| G015 | Evidence citations may exist but not support a claim | separate citation-existence and claim-support verification | 077 / 083 / 092 | citation support benchmark |
| G016 | Retractions/corrections can make literature stale | publication/retraction/correction metadata and Pack refresh | 080 / 089 | retracted/corrected source fixtures |
| G017 | Evidence strength could become an opaque score | multidimensional appraisal: design/bias/consistency/directness/precision/applicability/recency | 083 / 089 | transparent evidence profile fixture |
| G018 | Guideline advice can vary by jurisdiction/version/population | explicit guideline version, jurisdiction and applicability metadata | 089 | conflicting guideline fixtures |
| G019 | Publisher/full-text rights could be violated by local corpus building | rights-aware acquisition; metadata/citation separate from full text | 080 / 089 | license/access class tests + source ledger |
| G020 | CME/MOC cannot be implemented by software alone | external accreditation/partner gate; never claimed from activity logging | external / future | signed accreditation evidence if pursued |
| G021 | Clinical Graph could become a second clinical record | rebuildable projection only; edges carry authority/source/run/review | 083 | rebuild equality + source deletion/tombstone tests |
| G022 | Graph inference can be mistaken for source truth | EXTRACTED/MAPPED/DERIVED/INFERRED/CONFIRMED classes | 083 | UI/API/state tests |
| G023 | Graph loses temporal truth | valid/effective/source/ingestion time semantics | 083 | historical “knowledge at time T” fixtures |
| G024 | Graph server could become mandatory infrastructure | local projection first; external graph engine only after measured need | 083 | offline local graph qualification |
| G025 | AFFiNE-like blocks could duplicate patient facts | blocks reference canonical object/artifact IDs | 083 | cross-view ID consistency tests |
| G026 | CRDT merge can corrupt clinical facts | CRDT/revision merge only for user-authored collaboration content; clinical facts use authority-aware revisions | 076 / 083 | conflicting edit fixtures |
| G027 | Data Workbench could become an alternate patient DB | analytical/research datasets only; clinical sources remain references/snapshots | 075 / 083 | source-vs-derived authority tests |
| G028 | Formula results can silently change | versioned transformation -> new immutable snapshot for durable use | 075 | formula/transformation replay tests |
| G029 | Live database analysis is not reproducible | exact snapshot/query/materialization receipt before reproducible analysis | 075 / 082 | DB mutation-after-run test |
| G030 | Row/cell lineage can become too expensive | column/row lineage baseline; compressed/on-demand cell lineage where needed | 075 / 082 | scale/perf + lineage correctness tests |
| G031 | Cohort filters may silently treat unknown as false | explicit unknown/missing inclusion semantics and stage counts | 082 | cohort missingness fixtures |
| G032 | AI-generated SQL/formulas can mutate data | proposal only + read-only parser/planner guard | 075 / 082 | DDL/DML/multi-statement denial tests |
| G033 | Statistical functions may be wrong or assumptions hidden | independent numeric fixtures; assumptions `PASS/FAIL/NOT_RUN` | 082 / 092 | oracle comparisons |
| G034 | Charts can drift when source refreshes | chart binds exact AnalysisRun/DataSnapshot | 082 | refresh-after-render test |
| G035 | Python/R can gain unrestricted local authority | only through Compute/staged workspace; explicit publication | 085 / 086 | FS/network/secret escape tests |
| G036 | R package restore is not permanently reproducible | exact lock/runtime + explicit unavailable/offline state | 086 | offline restore and missing-package evidence |
| G037 | Hostile PDFs/docs can execute or exhaust resources | Signthos-style active-content deny, limits, isolated OCR/parser where needed | 075 / 085 / document path | malformed/resource bomb/active-content tests |
| G038 | Imaging is missing from a “full medical OS” | DICOM/DICOMweb adapter and qualified VLM/image path remain explicit 090/Pack scope | 090 / 089 | DICOM fixture + rights/security qualification |
| G039 | Terminology mappings can be licensed/version-sensitive | terminology Packs, version/checksum/rights and mapping loss reports | 089 / 090 | licensed fixture/version mismatch tests |
| G040 | Model updates can silently change behavior | immutable ModelPack revision + admission benchmark + rollback/revoke | 078 / 089 | pack update/regression tests |
| G041 | A model license may differ from repository code license | model-level rights record mandatory | 078 | model admission receipt |
| G042 | Local hardware can be insufficient | resource profiles, route decision, transparent Low Power/Balanced/Accuracy modes | 078 / 081 / 085 | hardware matrix + OOM/degradation tests |
| G043 | Remote model fallback can violate privacy | no automatic fallback; explicit Privacy Gate route only | 079 / 078 | packet/no-egress tests |
| G044 | Browser pages can prompt-inject MedAgent | web content is data; capability/system instructions outside page context | 080 | adversarial prompt-injection suite |
| G045 | Browser can access private network/metadata endpoints | SSRF/private-network deny policy | 080 | loopback/RFC1918/link-local tests |
| G046 | Downloaded web files may become trusted automatically | quarantine -> normal import/document validation | 080 / 075 | malicious download fixtures |
| G047 | Collaboration could grant authority via membership | membership/agent identity separate from capability/clinical authority | 076 / 084 | privilege-escalation tests |
| G048 | Offline collaboration can overwrite work | revision/conflict-copy/manual resolution rules | 076 / 084 | concurrent offline edit tests |
| G049 | Multi-user Hub could leak tenants/projects | scope before search/cache/pubsub/object lookup | 084 | cross-tenant adversarial tests |
| G050 | Hub loss could destroy local ownership | local Core remains authoritative; Hub optional | 084 | Hub-offline/lost-server tests |
| G051 | Revoked access may persist in caches/devices | revocation + cache invalidation + sync denial | 084 / 079 | revoke-during-session tests |
| G052 | External write timeout could be treated as failure/success | durable effect state with UNKNOWN + reconciliation | 090 | lost-response/duplicate-submit fixtures |
| G053 | FHIR profile/version mismatch can silently lose meaning | exact profile/version mapping + loss report | 090 / existing FHIR work | round-trip/loss fixtures |
| G054 | HL7 v2/CDA/DICOM adapters can expand without profiles | each institutional adapter is separately version/profile-qualified | 090 | adapter conformance packets |
| G055 | Patient data could enter logs/crash dumps/telemetry | content-minimized local observability; redaction; no patient payload by default | 079 / 092 | log/crash artifact scanning |
| G056 | Deletion may be undone by caches/backups | deletion/tombstone contract covering indexes/cache/Hub/workers/backups | 079 / 084 / 092 | delete/restart/restore tests |
| G057 | Legal retention can conflict with user deletion | explicit policy override state; never silently claim deletion | 079 / 090 | retention-policy fixtures |
| G058 | Backups may contain untracked sensitive material | backup manifest/classification/encryption + restore qualification | existing vault / 084 / 092 | backup content and restore tests |
| G059 | Mobile scope could explode and delay desktop | companion-first scope: capture/review/evidence/tasks before full analytics | 081 / 088 / future mobile integration | mobile-specific promotion package |
| G060 | Arabic/RTL can be bolted on too late | RTL/layout/localization plus Arabic clinical ASR/evidence datasets from early specs | 081 / 083 / 088 / 092 | RTL UI + Arabic benchmark |
| G061 | Accessibility can be lost in dense graph/data UI | keyboard/focus/semantic alternatives; graph never sole representation | 083 / V0 / 092 | accessibility test campaign |
| G062 | Extensions can become in-process malware | capability manifest + isolated/WASM worker + no ambient vault/network/secrets | 087 | malicious extension campaign |
| G063 | Extension updates can expand permissions silently | capability diff + re-consent + rollback/revoke | 087 | upgrade permission tests |
| G064 | Research Packs could execute arbitrary code | declarative by default; executable work through existing workers | 089 | malicious Pack tests |
| G065 | Federation could leak raw site data | data-stays-at-site default; typed aggregates; policy intersection | 091 | privacy/network partition tests |
| G066 | Federated stats can be invalid despite privacy | each method requires statistical validity evidence | 091 | oracle/simulation campaign |
| G067 | Supply chain/model artifacts can be tampered | checksums/signatures/SBOM/NOTICE/Pack admission | existing release work + 078/089/092 | tamper/rollback tests |
| G068 | App/Pack upgrade can make old projects unreadable | versioned migrations + backup + rollback/recovery | every persisted spec / 092 | N-1 upgrade/rollback tests |
| G069 | Product diagnostics could expose content | local metadata-only diagnostics by default; explicit content bundle export | 079 / 092 | diagnostic bundle inspection |
| G070 | Support bundle may need user redaction | preview/field-level exclusion before export | 092 | synthetic sensitive bundle tests |
| G071 | Institutional admins need governance without seeing all content | policy/audit/admin metadata separation from raw clinical content | 084 / 090 | role/scope tests |
| G072 | Revenue-cycle features could overclaim billing compliance | proposals/discrepancy review only until licensed rules + validation | 088 / 090 | coding-rule qualification |
| G073 | Front-desk/communication automation could expand clinical authority | keep administrative agents separate; messages/actions explicit effect capabilities | 087 / 090 future | scoped communication tests |
| G074 | Patient messaging may expose PHI in channels | channel-specific consent/data minimization/encryption/adapters | 090 future | channel policy tests |
| G075 | Export/import portability can be forgotten | open artifacts/CSV/Parquet/FHIR/Markdown/PDF exports with manifests | 075 / 083 / 090 / 092 | export->reimport fixtures |
| G076 | Vendor lock-in through model/runtime | stable MedScale contracts + replaceable workers + multiple qualified candidates | 078 / 085 | model/runtime substitution tests |
| G077 | Vendor lock-in through graph/workspace donor | MedScale-owned graph/block contracts, donors optional | 083 | donor-free local baseline |
| G078 | Search caches can leak across users/projects | permission scope in cache/index key | 083 / 084 | cross-scope cache tests |
| G079 | Evidence source changes may invalidate saved answers | saved answer binds snapshot/version; refresh creates new evaluation | 080 / 083 / 089 | source-update comparison fixture |
| G080 | Clinical advice and research hypothesis can be visually confused | explicit artifact/claim kinds and authority labels | 077 / 083 / V0 | UI semantics review |
| G081 | Care Signals could become opaque surveillance | every signal declares rule/model/source/applicability and review state | 083 / 088 / 090 | signal provenance fixtures |
| G082 | Dataset source terms can prohibit reuse | dataset rights metadata preserved; no automatic legal clearance claim | 075 / 089 | source terms/admission state |
| G083 | Real PHI readiness can be conflated with encrypted storage | separate release/privacy qualification; current real PHI remains unauthorized | 079 / 092 + existing gates | canonical gate reconciliation |
| G084 | Medical-device/regulatory boundary can drift | claims registry + intended-use review; software never self-certifies | 092 / external legal | approved intended-use/claims packet |
| G085 | Compliance labels can be used without audit | no HIPAA/GDPR/PDPL/SOC claim from architecture alone | 092 / external | independent/legal/compliance evidence |
| G086 | Competitor “replacement” claim can outrun evidence | parity/surpass matrix tied to measured use cases and dated competitor capabilities | 092 | matched benchmark/usability evidence |
| G087 | No real clinician/researcher UX validation | formal usability/workflow studies after technical foundation | 092 | study protocol/results |
| G088 | Safety benchmark can overfit known fixtures | held-out/versioned benchmark Packs and external review | 089 / 092 | frozen hidden evaluation set under governance |
| G089 | Model-generated patient summary could omit critical info | omission tracked as missing coverage, not “normal” summary compression | 077 / 083 / 088 | coverage/critical-fact fixtures |
| G090 | Clinical data provenance could be lost after FHIR export/import | source + mapping + version + loss receipt | 090 | round-trip provenance tests |
| G091 | Large datasets/audio can exhaust local disk | preflight estimates, quotas, cleanup/retention and cancellation | 075 / 081 / 085 | disk-full/quota tests |
| G092 | Long sessions can leak memory/resources | bounded buffers, streaming persistence, long-run soak tests | 081 / 084 / 085 / 092 | multi-hour soak campaign |
| G093 | Model/browser/extension cancellation may leave effects running | cancellation propagates; late effects quarantined/reconciled | 077 / 080 / 085 / 087 / 090 | cancel-race tests |
| G094 | Timezone/date precision can distort longitudinal record | existing MedicalTime semantics propagated into graph/data/evidence | all clinical specs | timezone/imprecise-date fixtures |
| G095 | Synthetic fixtures may not reflect clinical reality | staged evaluation: synthetic -> licensed deidentified -> approved prospective studies only when authorized | 081 / 089 / 092 | dataset governance packet |
| G096 | Inference confidence can be mistaken for evidence quality | confidence, authority, evidence quality and review state are separate fields | 077 / 083 | schema/UI tests |
| G097 | AI personalization could reinforce prior documentation errors | personalization limited to format/style unless explicitly reviewed; source facts not learned into template | 088 | erroneous-edit/template tests |
| G098 | “Consensus” across models can create false confidence | model agreement is an observable fact only; no correctness promotion | 078 | adversarial correlated-error cases |
| G099 | Patient/research role mixing can widen access | explicit role/project/patient-context grants and minimal context manifests | 077 / 079 / 084 / 090 | role-boundary tests |
| G100 | Project closure/archival can leave live credentials/jobs | revoke sessions/jobs/connector access; archive artifacts with explicit retention | 074 integration / 075 / 085 / 090 | archive/reopen tests |
| G101 | Prior authorization could become an opaque payer automation that submits unsupported medical necessity | draft/evidence packet remains reviewable; rule/payer/version and source evidence explicit; submission only through approved EffectIntent | 088 / 090 | unsupported-necessity, expired-rule, denial and review fixtures |
| G102 | Prior-auth retry after uncertain response could duplicate a payer request | durable idempotency + UNKNOWN state + status/reconciliation before retry | 090 | lost-response/duplicate/reconcile tests |
| G103 | HCC/risk-gap logic could incentivize unsupported diagnosis documentation | candidate signal only; payer/EHR/registry source shown; MEAT/documentation evidence required; clinician review | 083 / 088 / 090 | unsupported-HCC/adversarial incentive fixtures |
| G104 | E&M calculation could be mistaken for billing compliance | licensed/versioned rules, rationale and explicit proposal state; coder/clinician review | 088 / 089 / 090 | boundary/rule-version fixtures |
| G105 | Pre-bill CDI/DRG review could treat coding output as clinical truth | compare coded claim to documentation evidence without rewriting clinical source facts | 088 / 090 | coded-vs-source mismatch and correction tests |
| G106 | Nursing flowsheet generation could silently create structured chart facts | conversational rows remain drafts linked to transcript/source until nurse verification | 088 / 090 | draft/review/writeback state tests |
| G107 | Cross-shift nursing context could leak information beyond role/need | role/project/encounter-scoped ContextManifest and freshness boundaries | 079 / 084 / 088 | role-scope and stale-handoff tests |
| G108 | Pre-round summaries could omit high-risk changes | critical-fact coverage and change detection reported; omissions remain measurable | 083 / 088 / 092 | longitudinal change/critical-omission benchmark |
| G109 | Trial matching could claim eligibility from incomplete patient data | criterion-by-criterion map with UNKNOWN/UNAVAILABLE; candidate match only | 080 / 083 / 089 | false-eligibility and missing-criterion fixtures |
| G110 | Trial registry status/location can go stale | exact registry snapshot/retrieval time and refresh semantics | 080 / 089 | closed/recruitment-change fixtures |
| G111 | Deep evidence consult can hide its search path and cherry-pick sources | expose plan, queries, source set, excluded/weak evidence, contradictions and immutable snapshot | 077 / 080 / 083 | reproducible deep-consult benchmark |
| G112 | Secure patient communication can expand MedScale into an uncontrolled messaging/telephony system | channel adapters are optional institutional capabilities; local core stays independent; identity/consent/retention/effect receipts required | 079 / 090 | wrong-recipient, revoked-consent, delivery-UNKNOWN tests |
| G113 | Discharge/order-set suggestions may blur evidence synthesis and action authority | proposals explicitly separate evidence, patient applicability and action; approval required | 077 / 088 / 090 | unsafe-proposal/approval/effect tests |
| G114 | Evidence inserted into encounter notes could make general literature look like patient-specific fact | patient source facts and external evidence use distinct claim/evidence classes and visual semantics | 077 / 081 / 083 | patient-vs-literature provenance/UI tests |
| G115 | Payer/CDI features could distort the product toward reimbursement rather than care/research | product governance keeps clinical truth, evidence quality and patient benefit independent of revenue optimization; metrics do not reward unsupported coding | 088 / 090 / 092 | adversarial governance/use-case review |
| G116 | Research protocol can be rewritten after results and hide post-hoc decisions | versioned protocol plus explicit amendment/pre-specified/post-hoc state | 083 | protocol amendment/replay fixtures |
| G117 | Citation imports can duplicate/misidentify papers | DOI/PMID/metadata reconciliation with ambiguous duplicate state | 080 / 083 | duplicate/metadata-conflict fixtures |
| G118 | Literature search cannot be reproduced | persist exact source/query/filter/date/result snapshot | 080 / 083 | search receipt replay/refresh delta |
| G119 | AI screening could silently exclude relevant studies | AI only ranks/proposes; human screening policy and exclusion reason remain explicit | 076 / 083 | high-recall/false-exclusion adversarial benchmark |
| G120 | Reviewers can influence each other in supposedly independent screening | optional blinded review state + immutable independent decisions before reveal | 076 | dual-review/blinding tests |
| G121 | PRISMA-style counts can drift from real review state | generate counts from event/artifact history, not editable numbers | 076 / 083 | screening-event count consistency |
| G122 | AI data extraction may fabricate study values | each extracted field links source span and review state; unresolved remains unresolved | 077 / 083 | extraction/source-support benchmark |
| G123 | Risk-of-bias framework/version can be applied incorrectly | framework Pack declares version/domain; incompatible study types fail explicit applicability checks | 083 / 089 | framework/study mismatch fixtures |
| G124 | Meta-analysis can hide assumptions or use wrong effect model | method/assumptions explicit, independent numerical oracle, sensitivity results | 082 / 086 | statistics oracle and edge-case tests |
| G125 | Living review refresh can silently change a published conclusion | new review version/delta only; prior corpus/synthesis immutable | 080 / 083 / 089 | source-update/review-version tests |
| G126 | Manuscript text can detach from figures/tables/citations | manuscript references exact artifact revisions and flags stale dependencies | 083 | artifact-update/stale-reference tests |
| G127 | Clinical calculator can use missing value as zero/default | required/optional inputs explicit; missing/ambiguous fails or requests review | 077 / 089 | missing/unit/boundary tests |
| G128 | Guideline pathway may be outdated or wrong jurisdiction | version/jurisdiction/source and superseded status visible | 083 / 089 | old/new/jurisdiction fixtures |
| G129 | Drug interaction/dosing coverage can be incomplete due to source rights | coverage and source/version explicit; no “complete” claim from partial Pack | 089 / 090 | coverage/rights/version evidence |
| G130 | Model-prefilled clinical-tool input could be wrong | input source/value/unit/time shown; user confirmation where required | 077 / 083 / 089 | wrong-prefill/ambiguous-source tests |
| G131 | Dataset card could be treated as legal/privacy clearance | rights/privacy fields are evidence/state, not automatic approval | 075 / 079 | denied-use and missing-rights fixtures |
| G132 | Annotation schema changes can invalidate existing labels silently | schema versions, migration mapping and incompatible-state handling | 075 / 076 | schema-evolution tests |
| G133 | Model-generated labels can be confused with human ground truth | annotator identity/type and review/adjudication state mandatory | 076 / 078 | provenance/UI/API tests |
| G134 | Active learning can contaminate held-out evaluation data | split/role policy prevents training/selection access to protected evaluation set | 075 / 078 / 085 | contamination/permission tests |
| G135 | Medical ML split can leak same patient/site/episode across train/test | group-aware and temporal split constraints + leakage detector | 075 / 082 | patient/site/episode leakage fixtures |
| G136 | Training may violate model/dataset license or intended-use terms | training job admission requires model/data rights and purpose metadata | 079 / 085 / 089 | rights-denial/admission tests |
| G137 | Training on sensitive data can bypass Privacy Gate through Compute | staged classified inputs + Compute capability policy + no ambient vault/network | 079 / 085 | sensitive-input egress/FS escape tests |
| G138 | Training output could be promoted directly to clinical use | output is Candidate ModelPack only; independent admission/evaluation required | 078 / 085 / 089 | candidate-vs-admitted state tests |
| G139 | Evaluation can overfit or leak the benchmark into tuning | frozen held-out Pack, access audit, contamination checks, versioned evaluation | 078 / 089 / 092 | hidden/held-out governance evidence |
| G140 | Dataset/model research can drift into an unbounded MLOps platform | keep training/annotation as project research capabilities behind existing Project/Data/Compute/Pack contracts; no alternate registry/orchestrator authority | 075 / 078 / 085 / 089 | architecture/dependency review |

---

# 2. Explicit non-software/external gates

These cannot be solved by repository implementation alone.

## E001 — Real PHI authorization
Requires canonical project decision after privacy/security/release evidence. Current authority remains NO.

## E002 — Clinical terminology/content licensing
CPT, SNOMED CT and other restricted assets require jurisdiction/use-specific rights.

## E003 — Publisher/full-text rights
Open-access sources can be locally admitted under their terms. Subscription/restricted content requires organization/user rights and must not be redistributed automatically.

## E004 — CME/MOC/CE accreditation
Requires appropriate accreditation/partnership, not only software activity tracking.

## E005 — EHR vendor credentials/sandboxes
Production integrations require institution/vendor access and credentials.

## E006 — Regulatory/intended-use review
Product labeling/market-specific medical-device or clinical decision-support obligations require legal/regulatory review.

## E007 — Compliance attestations
SOC 2, certification, external audits and similar claims require external evidence.

## E008 — Prospective clinical validation
Requires approved study governance, participants/data authorization and protocol.

External gates must block only the affected claim/path, not unrelated repository work.

---

# 3. No-gap promotion checklist

Before promoting any 075–092 unit, answer:

- What exact user outcome is this spec delivering?
- What is the smallest end-to-end vertical slice?
- Which current MedScale IDs/types remain authority?
- What new persisted/wire types are introduced?
- What migration/recovery is needed?
- What privacy classes can enter?
- What can leave the machine?
- What secrets are required and where do they live?
- What external effects are possible?
- What happens on timeout/crash/restart/cancel?
- What happens with stale/conflicting/unknown input?
- What donor/dependency is used and at which exact revision/path/license?
- What performance envelope is expected?
- What security abuse cases must fail?
- What accessibility/localization state is affected?
- Which exact tests/evidence prove closure?
- How can the feature be rolled back/removed?
- Does Personal/offline mode still work?

If any applicable answer is missing, the spec must remain in refinement rather than implementation.

---

# 4. Residual planning posture

After this review:

```text
KNOWN_UNASSIGNED_ARCHITECTURE_GAPS = 0
KNOWN_UNASSIGNED_PRODUCT_GAPS = 0
EXTERNAL_GATES = EXPLICIT
IMPLEMENTATION_EVIDENCE = NOT_YET_PROVEN_FOR_075_PLUS
RELEASE_READY = FALSE
PRIVATE_DATA_READY = FALSE unless live canonical governance later changes it
```

These are planning statements only. Implementation may discover additional gaps, which must be added to this register rather than worked around silently.
