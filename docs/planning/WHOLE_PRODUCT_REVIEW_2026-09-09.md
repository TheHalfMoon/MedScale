# MedScale whole-product architecture review

Date: 2026-09-09. Status: `SOURCE_REVIEW_COMPLETE_WITH_EXPLICIT_COVERAGE_LIMITS`.
Baseline main: `b49592c83d23542363f671afe6a9ae65fc65b276`.
Baseline tree: `8b79737e6f0b320308b95b30e22e5fd94c08b7df`.

This review updates planning, not the historical closure of Specs 000–015. A closed
fixture or READY_BASE scope is not a released product capability. No PHI, live partner,
model, MESC mutation, or production action is authorized by this document.

Read with [delivery decisions](TRUSTED_V1_DELIVERY_PLAN.md),
[trust/privacy model](../security/TRUST_AND_PRIVACY_MODEL.md),
[source review](source-register/2026-09-09-source-review.md), and
[verification/coverage record](../../evidence/whole-product-review-2026-09-09/REVIEW_RECORD.md).

## Product definition

**CURRENT_PRODUCT_DEFINITION:** MedScale is a local Rust health-record workspace whose
trusted core preserves original sources, records explicit attribution and identity decisions,
and derives inspectable timelines, coverage and Briefs. AI may later propose interpretations;
it cannot confer authority. Today the repository is a synthetic-data engineering foundation,
with a CLI and desktop scaffold, rather than a deployable clinical application.

The primary V1 design user is an individual organizing a longitudinal record for a clinical
visit. Initial testers use synthetic cases only. Clinicians are secondary reviewers of
source-linked exports; researchers are secondary evaluators of the reproducible workflow.
Do not attempt three complete persona products in V1. A presentation mode changes language
and information density, never permissions. Clinical actions require separate role, purpose,
scope and authority records, irrespective of the selected mode.

**WHY_SHOULD_MEDSCALE_EXIST?** A record assembled from multiple sources needs visible
provenance, disagreements, incomplete coverage and amendments. A fluent summary is insufficient.
MedScale can make those limits inspectable while keeping useful work local and model-free.

**WHAT_MUST_MEDSCALE_DO_BETTER_THAN_ALTERNATIVES_TO_MATTER?** Reliably reopen the same
source-backed record, explain every displayed item, preserve uncertainty through export,
and make privacy behavior understandable. Those outcomes must be demonstrated with real
restart/recovery and user-task evidence, not inferred from Rust, encryption primitives or tests.

Canonical truth consists of admitted source custody and attributed assertions/decisions,
not an AI answer, FHIR database, search index or projection. An authorized assertion is an
attributed claim, not a guarantee that its clinical content is true. Disagreement remains valid.

## Minimum lovable trusted MedScale

`MINIMUM_LOVABLE_TRUSTED_MEDSCALE` is a qualified desktop + CLI workflow:

1. Start offline, inspect clear capability/privacy status, and load a synthetic example in
   under five minutes after installation. No account, model or network is required.
2. Import bounded supported FHIR R4 files; preview source, subject association, unsupported
   fields and duplicate disposition before authority-changing acceptance.
3. Inspect Timeline, Brief and coverage; distinguish unknown, explicit absence, conflict,
   uncertain time and source correction; drill down to unchanged source bytes.
4. Close the process, reopen and recover the identical admitted record and decision history.
5. Export a scoped, previewed source-linked Brief and bundle; record the disclosure locally.
6. Back up, verify, restore into a new location, and prove the restored record agrees. Explain
   key-loss limits and OS snapshots rather than promising universal secure deletion.

The first daily-use loop is import → inspect changes/conflicts → prepare a visit Brief →
retain the previous version for comparison. Validate usefulness through five representative
synthetic user sessions; record completion, errors and support needed. This is a proposed
evaluation target, not measured adoption or retention. A private-data release requires its
own authorization and qualification. Neither AI nor mobile is a prerequisite to this wedge.

`MEDSCALE_LONG_TERM_NORTH_STAR`: a privately operated, source-linked longitudinal workspace
that helps people and clinicians understand changing records, consult qualified evidence,
and approve bounded interoperable actions. Extend through artifacts and capability-specific
adapters; preserve a small local authority core and export portability.

## Live repository and maturity

The initial GitHub read found main as the sole branch, no open/draft PRs, 33 merged PRs,
no tags or releases, no rulesets, and main unprotected (protection API returned 404).
Main CI run `32940232817` reported success. These are time-bound observations, not permanent
state or proof that all review threads were resolved. Complete historical review/thread
reconstruction remains a coverage limit recorded in the evidence note.

| Capability | Classification / current evidence | Required next outcome |
|---|---|---|
| RECORD | PARTIAL: typed sources, ingest, timeline/Brief fixtures; facade uses in-memory objects | Cross-session persistence of assertions, identities, corrections and audit |
| EVIDENCE | PARTIAL: three-document synthetic lexical corpus and evidence-only evaluation | User-admitted, versioned evidence corpus with freshness/retraction lineage |
| AI | PARTIAL: Pack hashes, format refusal, fixture runtime | Independently verified Pack admission and actual confined runtime |
| DOCUMENTS | PARTIAL: MIME policy and OCR/ASR fixture stubs | Qualified hostile-parser boundary and measured engine, after V1 |
| INTEROPERABILITY | PARTIAL: narrow Patient/Condition/Observation extraction; stub SMART | Published R4 support table and independent validator/export evidence |
| ACTIONS | PARTIAL / RELIABILITY_RISK if activated: in-memory intents and state vocabulary | Durable approval/outbox/reconciliation before any live effect |
| PRIVACY | PARTIAL / PRIVACY_RISK for private-data readiness: plaintext working metadata while unlocked | Complete storage/key/crash privacy proof |
| SECURITY | PARTIAL: narrow safe Rust contracts; no qualified OS isolation or authenticated IPC | Enforced platform boundaries and merge/release integrity |
| MOBILE | DEFERRED_CORRECTLY: policy/FFI stubs, no apps | Companion/capture user evidence before native application work |
| DESKTOP | PARTIAL: non-WebView console scaffold | Qualified shell and integrated founder-supplied v0 design |
| CLI | ADEQUATE for synthetic fixture development | Stable error/JSON contract and real restart workflow |
| DEVELOPER_PLATFORM | ADEQUATE: nine crates, locked toolchain, Windows/Linux CI | Qualification artifacts, dependency evidence and accurate entry points |
| EXTENSIONS | DEFERRED_CORRECTLY | Narrow importer/exporter/Pack contracts only when demanded |
| RELEASE | MISSING: no MedScale tags/releases; license choice remains external | Signed, reproducible, migration-tested installable artifact |

## Top ten strengths

1. Constitution separates proposals, assertions and projections.
2. Source identity is distinct from content digest; transforms retain lineage.
3. Rust ownership and nine dependency-oriented crates avoid a service fleet.
4. H0 provides a useful model-free direction.
5. Explicit unknown/absence/conflict presentation types.
6. Bounded FHIR payload/depth and duplicate-key lexical checks.
7. Original blob digest checks and synthetic durability fixtures.
8. Network, MESC, online Pack and live NPHIES paths currently fail closed.
9. Optional capability/worker architecture avoids a mandatory model vendor.
10. Windows/Linux CI and 98 passing local tests provide a starting regression baseline.

## Top ten weaknesses

1. Closure vocabulary is easy to mistake for product completion.
2. Authority objects and action intents remain in memory in the facade.
3. No production authenticated IPC or caller-bound capability issuance.
4. File-marker/in-process leases do not establish qualified cross-process ownership.
5. Encrypted metadata is materialized as a plaintext working SQLite file.
6. OS key stores and worker isolation are scaffolds, not platform qualification.
7. Temporal fallback labels strings as Instant and sorts lexically; precision/order require work.
8. Desktop/mobile do not yet deliver the daily-use experience.
9. Actual evidence retrieval, Pack signatures and live interoperability exceed fixture scope.
10. GitHub protection and verifiable release engineering are incomplete.

These are architecture/readiness gaps. They are not an exploitation report or a claim that
an exposed production service was compromised. Exact source anchors are in the trust model.

## Object model and longitudinal record review

Keep SourceRecord, DerivedSourceArtifact, Proposal, ClinicalAssertion, EvaluationRecord,
IdentityAssertion, Projection and audit/action distinctions. Do not add a generic graph
database or duplicate all objects as FHIR resources. Keep Realm/scope explicit and validate
all linked objects in the same authority context. IDs must persist uniquely across restart;
the current in-memory sequence is a fixture implementation, not a durable identity allocator.

Add versioned amendment/supersession and explicit retraction decisions to the owning contract.
Source corrections append custody/version relationships; they never overwrite source bytes.
Represent valid/effective time separately from recorded/acquired time; retain timezone and
precision, and never infer exact chronology from partial dates. Unknown, not-observed,
not-applicable, withheld, explicitly absent and conflicting require distinct presentation.
Preserve unresolved identity candidates; explicit merge/unmerge must be replayable and auditable.

Prefer an immutable source-blob store plus one transactional metadata store with rebuildable
indexes. Persist assertions, proposals, identity decisions, evaluations and audit in the same
transactional authority path; do not persist only source metadata and imply full record
durability. Use schema-versioned migrations with backup, interruption and restore tests.
Keep large bytes out of IPC responses; issue bounded read handles after authorization.

## FHIR and interoperability review

Keep FHIR R4 4.0.1 as interchange, with explicit loss/unsupported-field reports. The current
typed extraction subset is not full FHIR validation or SMART conformance. Distinguish lexical,
structural, profile, terminology, reference, provenance and clinical checks. External validator
output remains evidence. [HL7 R4 security](https://hl7.org/fhir/R4/security.html) explicitly
requires a separate security design; FHIR modeling is not authentication or authorization.

V1: local bounded ingest, source-preserving internal mapping, explicit subject confirmation,
local export, provenance and a machine-readable support matrix. A CapabilityStatement may
describe actual supported exchange features; do not imply an online server exists.
V1.5: Composition/document export and separately qualified read-only SMART connection when
product demand and partner authority exist. Questionnaire/SDC, Bulk Data, transaction bundles
and subscriptions remain separately scoped. R5, openEHR, OMOP, CQL, CDS Hooks and DICOMweb
are adapter/export or research candidates, not additional canonical databases or V1 obligations.
OMOP is useful for approved research export; openEHR is a source/versioning reference; CQL/CDS
Hooks need a concrete decision-support use case. Licensed terminology remains outside code.

## Evidence, retrieval, AI and memory review

Keep lexical retrieval first. The current corpus in `authority/retrieval.rs` is three static
synthetic notes; an EvaluationRecord is not proof of clinical evidence quality. Separate DATA,
EVIDENCE, ASSERTION, PROPOSAL, RECOMMENDATION and ACTION in contracts and UI. Add source edition,
retrieved/verified dates, passage coordinates, snapshot digest, supersession/retraction and
conflict relationships. Rank relevance independently from source authority. A recommendation
needs visible supporting and contradicting evidence and an explicit uncertainty disposition.

Measure lexical+filters on admitted local sources before adding embeddings/reranking. Add
hybrid retrieval only if held-out queries improve materially with acceptable latency/privacy
cost. Indexes/vectors are sensitive rebuildable projections; no vector server or GraphRAG in V1.
Canonical health record, user preferences, session context, cached retrieval and AI summaries
have separate retention/authority. AI working memory never writes clinical truth silently.

Keep Pack as the long-term artifact boundary. Evolve compatibly: immutable manifest/version,
weights/tokenizer/processor hashes, runtime ABI/platform/quantization, rights, SBOM, evaluations,
resource limits, signer/root policy, expiry/revocation and anti-rollback metadata. Declaration
of `offline` or `sandboxed` is not enforcement. CPU baseline first; GPU is independently qualified.
Structured output, evidence references, calibrated abstention and missing-source errors are
advisory outputs. Model routing uses task eligibility and quality measurements, not brand.
Bound model context, output, memory, wall time and cancellation. No worker gets DB/key/network
handles; no automatic fallback to a remote model.

## MESC boundary review

Keep artifact-first. The 2026-09-09 read of MESC releases still found v0.1.0 with `assets=[]`;
a v0.2.0 tag alone is not the required artifact. Leave Spec 012 blocked. Its current request
has artifact URI/digest, rights URI, SBOM digest, evaluation digest and Pack-path flag; future
admission must additionally bind released producer identity, source commit/tree, base-model
and tokenizer identity, complete file hashes, training/corpus/evaluation provenance, rights
scope, limitations and runtime requirements. A URL or nonempty digest field is not verified
evidence. See the proposed [artifact acceptance contract](MESC_ARTIFACT_ACCEPTANCE.md).

## Desktop, UI, mobile and user modes

V1 navigation should center on **Record** (Timeline + Brief), **Sources** (import/history),
and **Settings & Privacy**. Evidence appears in context with a dedicated workspace later.
AI and Actions become visible only when an admitted capability exists. Diagnostics belongs
under Settings; avoid twelve equally prominent empty modules.

Keep final visuals owned by v0. Integrate via synthetic fixtures, typed responses, capability
checks and provenance inventory. Qualify keyboard navigation, focus, screen-reader names,
contrast, zoom/reflow, loading/empty/error/conflict/recovery states and source drill-down.
No remote fonts, analytics, generated server routes, credentials or direct database plugin.
Tauri remains conditional: narrowly scoped capabilities help limit frontend access but do not
replace safe Rust or protect against all WebView compromise. [Tauri capabilities](https://tauri.app/security/capabilities/).
Test caches, crash dumps, clipboard, browser storage and screenshots; choose the shell only
after comparing accessibility, memory/startup, offline installation and privacy evidence.

Defer full mobile apps. First qualify a read-only companion or controlled capture workflow
if users need it. No synchronized vault replica/CRDT in V1. Device transfer needs explicit
pairing, encryption, revocation and provenance. Keychain/Keystore, biometric fallback,
background limits, lock-screen notifications, backup exclusions and app-switch previews
require device evidence; FFI policy structs are insufficient. Patient/clinician/researcher
modes never become a shortcut around authorization.

## Network, actions, documents, privacy and security

Keep current live denials. Broker readiness requires destination+method+path+purpose+data-class
and principal binding, canonical URL handling, DNS/IP policy, TLS validation, redirect
revalidation, secret handles, time/size budgets and value-free receipts. Retries are typed and
bounded. Network access is a separate process/capability, not an ambient property of core.
Do not add certificate pinning without a rotation/recovery policy; ordinary certificate
validation is mandatory. Runtime and development acquisition networks have different scopes.

An action requires durable intent → exact approval → dispatch receipt → result/reconciliation.
Approval binds actor, account, destination, payload digest, expiry and operation version.
UNKNOWN freezes automatic retries; FAILED is retryable only if a classified outcome proves
safe retry. Cancellation cannot undo a possibly completed effect. Persist the outbox and audit
atomically before dispatch. No live orders, prescriptions, scheduling, messages or NPHIES in V1.

Keep complex PDF/office/archive/image/ASR processing outside core. Quarantine bytes before
classification; declared MIME, extension and classifier are evidence, never parser authority.
Enforce input, nesting, decompressed size, expansion ratio, pages, output and wall-time limits.
Record transformations, source regions/timestamps, speaker uncertainty and critical-number
alignment. OCR/ASR corrections remain proposals. Magika admission is optional and benchmark-gated.

The [trust/privacy model](../security/TRUST_AND_PRIVACY_MODEL.md) records assets, actual
resources, boundaries, conditional threats, detection, recovery and data lifecycle. It does
not certify security or privacy. Real-data release remains blocked until qualification.

## Quality scorecard

Scores are prioritization aids: 0 absent, 1 scaffold, 2 scoped fixture implementation,
3 qualified user workflow, 4 release evidence, 5 independently sustained deployment evidence.
No area receives a score above its inspected evidence. These are judgment ratings, not tests.

| Area | Score | Evidence / gap |
|---|---:|---|
| Architecture | 2 | Constitution + inward crates; host enforcement absent |
| Product | 1 | Synthetic CLI; no complete restart daily-use evidence |
| Privacy | 1 | Sealed files; working plaintext and platform proof gaps |
| Security | 1 | Fail-closed stubs; no authenticated IPC/OS sandbox |
| Healthcare modeling | 2 | Distinct objects/coverage; time/amendment gaps |
| FHIR | 2 | Bounded lexical/typed subset; no full conformance |
| Evidence | 1 | Three static notes; no production corpus lifecycle |
| AI | 1 | Fixture runtime; no admitted real engine |
| Local-first | 2 | Current model-free/offline fixture path |
| Developer experience | 2 | Cargo/CLI/CI; stale execution entry points |
| Testing | 2 | 98 tests; no complete boundary/platform/release qualification |
| Performance | 0 | No inspected reproducible workload budget results |
| Resilience | 1 | Synthetic backup tests; full object restart/crash gaps |
| UI integration | 1 | Contract + fixture views, final artifact absent |
| Mobile | 1 | Policies/FFI stubs only |
| Release engineering | 1 | Lock/CI; no signed release or enforced branch protection |
| Maintainability | 2 | Small crate set; stale planning and fixture ambiguity |

## Scope and simplification decisions

**KEEP:** Rust-owned semantics, source custody, typed proposal/promotion, local-first baseline,
FHIR interchange, lexical-first retrieval, Pack artifact boundary, current live denials.
**HARDEN:** persistence/keys/leases, temporal semantics, host authorization, artifact trust,
backup/recovery, release evidence, source-linked UI and diagnostics.
**SIMPLIFY:** one record workspace, one metadata store, one source-blob store, no duplicate
client authorities; distinguish spec closure from qualification rather than new readiness slogans.
**REMOVE:** current-status wording that says implementation has not started or Spec 001 is next;
do not delete historical evidence. No code removal justified solely by this review.
**ADD:** bounded restart/privacy/platform qualification and usable import-review-export workflow.
**DEFER:** live clinical actions, mobile replicas/sync, generic plugins, vector/graph servers,
imaging/genomics/browser product, teacher training, custom CUDA and generalized agent workflows.

## Competitor classes and defensible differentiation

EHR systems such as [OpenEMR](https://www.open-emr.org/) center practice workflows; evidence
tools such as [Zotero](https://www.zotero.org/support/) center research collections; OpenMed
provides extraction/model tooling. These are different evaluation objects. MedScale should
not claim to replace an EHR, validate medicine better than literature experts, or surpass all
OpenMed capabilities. Its candidate advantage is a local source-to-record-to-Brief workflow
with visible uncertainty, repeatable recovery and minimal external authority. Validate that
advantage through task completion and independent reproduction before comparative claims.

## Current disposition

`WHOLE_PROJECT_REVIEW_STATUS = PLANNING_REVIEW_WITH_COVERAGE_LIMITS`
`MESC_DEPENDENCY_STATE = BLOCKED_BY_RELEASED_ARTIFACT`
`RELEASE_READY = FALSE`
`NEXT_CANONICAL_UNIT = QUALIFY_TRUSTED_RECORD_FOLLOW_ON_SPEC`

The [delivery plan](TRUSTED_V1_DELIVERY_PLAN.md) is the improved dependency-ordered planning
companion to V2. It does not automatically promote Spec 016+ implementation or clear external
gates. Ordinary preparation can proceed; a material implementation must own a complete
Spec Kit package, exact-head checks and review before touching closed runtime contracts.
