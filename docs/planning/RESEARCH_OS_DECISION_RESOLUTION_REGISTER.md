# MedScale Research OS Decision Resolution Register

**Status:** Planning defaults. These remove implementation ambiguity without pretending that unmeasured engine/vendor choices are already qualified.

Rule: an implementation spec uses the default below unless that spec produces the listed evidence and records a superseding decision. An open research choice is therefore never permission for an implementer to choose by preference.

| ID | Topic | Default decision | Owner | Evidence required to change |
|---|---|---|---|---|
| Q01 | Generic vs specialized artifacts | Keep authority-bearing/FHIR/Pack/policy types specialized; expose generic `ArtifactDescriptor` references only for organization/search. | 074 | Concrete duplication/incompatibility proof showing current typed model cannot support project references. |
| Q02 | Migration to Projects | Add Project as an optional organizing layer over existing objects first; no destructive migration or duplicate canonical copies. | 074 | Migration tests proving an alternative preserves IDs, provenance and CLI/Desktop semantics. |
| Q03 | Canonical graph edges | Only typed relationships explicitly created by Core are canonical; search/vector/knowledge graph edges are projections. | 074/082 | A relation needs authority semantics and has deterministic validation/storage rules. |
| Q04 | Signed collaboration events | Initial release uses authenticated actor identity plus tamper-evident audit/checkpoints; per-event end-user cryptographic signatures are not mandatory. | 075/083 | Threat model/use case requiring offline portable signatures and a proven key UX/recovery model. |
| Q05 | Buzz reuse | Selective copy/adapt of isolated room/event/agent/workflow/media patterns only; no wholesale relay/UI adoption. | 075 | Component-level comparison proving direct reuse is smaller/safer and provenance/legal/security gates close. |
| Q06 | Collaboration E2EE | Local vault data stays encrypted; Hub collaboration uses transport encryption and project authorization initially. E2EE for message bodies is deferred because server-side search/workflows would be incompatible without a separate design. | 083 | Explicit deployment requirement plus searchable/workflow-compatible key distribution, recovery and revocation design. |
| Q07 | Offline conflicts | Messages append; tasks use optimistic concurrency; notes/canvases preserve conflict copies and require user resolution; canonical metadata never LWW. No CRDT initially. | 075/083 | Multi-user measurements show unacceptable conflict burden and a qualified CRDT preserves authorization/audit/deletion semantics. |
| Q08 | ReBAC engine | Implement MedScale-owned relationship semantics first behind a policy interface; do not require OpenFGA/OPA service for Personal/Lab. | 075/083 | Policy complexity/performance/administration evidence shows external engine reduces risk without creating mandatory network dependency. |
| Q09 | SSO binding | Local MedScale identity remains stable; institutional identity is an external binding/credential to that identity, not the canonical object ID. Offline cached authorization is bounded and expires. | 087 | Provider-specific protocol requirements that cannot preserve stable local identity. |
| Q10 | Revocation in-flight | Revoke blocks all new actions immediately, sends cancel where possible, and quarantines late results until re-authorization. | 076/083/085 | No weaker alternative permitted. |
| Q11 | Fleet lane semantics | Support both same-task comparison and role-specialized lanes; lane manifest explicitly declares task transform/role. | 077 | N/A; product requirement. |
| Q12 | Compare metrics | Productize only observable/deterministic facts: refs, schema validity, tool calls, timing/resources, supported contradiction/claim extraction with provenance. No intelligence/quality score without benchmark. | 077 | Metric has a reproducible benchmark and explicit uncertainty/ground truth. |
| Q13 | Context cache | Cache only immutable prepared/model/index state by digest; sensitive user context retention is opt-in/bounded, never an opaque cross-run memory cache. | 076 | Measured performance requirement plus encrypted/expiry/revocation design. |
| Q14 | External delegates | Disabled for `LOCAL_PHI`/`TEAM_PROTECTED`; initially allowed only for PUBLIC and explicitly approved EXTERNAL_DEIDENTIFIED tasks. | 077 | Provider-specific data agreement/policy and canonical authorization expand the allowed class. |
| Q15 | Jurisdiction semantics | Core owns invariant data classes/capabilities; jurisdictional/legal profiles are deployment policy Packs/configuration, not hard-coded assumptions in core types. | 078/087 | A legal requirement must be an invariant across all supported deployments. |
| Q16 | Reversible pseudonymization | Mapping key stays under separate local/institutional key authority; re-identification is a separate audited capability. | 078 | No weaker alternative permitted. |
| Q17 | Privacy benchmark data | Use synthetic/permitted multilingual fixtures only until real-PHI benchmark authorization exists; publish class-wise recall/precision/error analysis, not blanket safety claim. | 078 | Explicit real data authorization with retention/governance record. |
| Q18 | Native audio capture | Implement minimum OS-native capture abstraction behind MedScale contract; donor app shells do not become runtime architecture. | 080 | Exact platform API/dependency qualification. |
| Q19 | Speech engines | No engine preselected as universal winner. Benchmark admitted candidates per lane; safe fallback is Unavailable. | 080 | Reproducible target-device/task benchmark. |
| Q20 | Python speech worker | Permitted only for lanes whose measured quality benefit justifies it; must be isolated, pinned and optional. Minimal default route should prefer native/portable runtime where qualified. | 080 | Benchmark + packaging/security evidence. |
| Q21 | Audio Pack format | Extend existing MedScale Pack provenance/admission concepts with audio task/runtime metadata; do not create independent model store. | 080 | Existing Pack schema demonstrably cannot represent required audio runtime identity. |
| Q22 | Recording consent UX | Product always shows capture/transcription state and requires explicit user initiation. Deployment profiles may add consent prompts/attestations. No stealth capture. | 080/087 | Platform/legal requirement may add stricter behavior only. |
| Q23 | TTS/cloning scope | TTS/listen-back is advanced optional feature; voice cloning/design are later opt-in Audio Packs with subject permission/provenance. Not foundation blockers. | 084 | Accessibility/product evidence may promote TTS earlier, not cloning. |
| Q24 | DataFusion sufficiency | DataFusion is first native engine candidate; validate required SQL/performance. Add secondary engine only behind same contract for proven gaps. | 081 | Reproducible unsupported feature/performance gap. |
| Q25 | Notebook model | Typed/reproducible cells; SQL/native operations may execute trusted path; Python/R/arbitrary code only through Compute sandbox. | 081/085 | No unrestricted in-process notebook permitted. |
| Q26 | Native vs Python/R statistics | Common deterministic operations may be native after validation; specialized methods use sandboxed workers. | 081/085 | Accuracy/library coverage evidence. |
| Q27 | NL-to-SQL | AI only proposes; parser/planner enforces read-only governed views; user can inspect exact plan/SQL. | 081 | No weaker alternative permitted. |
| Q28 | Personal/Lab index | Start with embedded/local lexical search and minimal vector index if benchmark justifies it; no OpenSearch requirement. | 082 | Dataset/latency/concurrency measurements exceed embedded envelope. |
| Q29 | Vector database | None mandatory. Admit embedded vector implementation only when retrieval benchmark shows value. | 082 | Retrieval benchmark and storage/permission design. |
| Q30 | Embedding/index staleness | `IndexManifest` binds exact source revision, parser/chunker and embedding Pack; changed source marks entries stale/tombstoned until rebuilt. | 082 | No weaker alternative permitted. |
| Q31 | Retrieval permissions | Filter before disclosure and before cache reuse; cache scope includes authorization/project context. | 082 | No weaker alternative permitted. |
| Q32 | OpenSandbox | Do not make mandatory initially. Prove minimal local worker first; qualify OpenSandbox/container isolation for higher-risk/remote workloads. | 085 | Cross-platform security/ops comparison. |
| Q33 | HPC transfer | Stage only exact artifact digests via short-lived job/input leases; no vault mount. Return candidate outputs by digest for Core admission. | 085/087 | Scheduler constraints may alter transport, not least-privilege semantics. |
| Q34 | Execution isolation type | Choose by workload risk: trusted admitted native operation -> in-process only if contract allows; arbitrary/user/model code -> worker; remote/custom model code -> isolated worker; containers/WASM/OS sandbox qualified per target. | 085 | Threat/performance evidence. |
| Q35 | First Packs | Clinical Research first, AI Research second, Systematic Review third; Imaging/Omics/Wet Lab after domain source qualification. | 086 | Documented adoption priority/use-case evidence. |
| Q36 | Pack UI extension | Declarative view/navigation/form descriptors first; no arbitrary untrusted native UI code in trusted Desktop. | 086 | Proven extension need plus sandboxed UI/plugin security model. |
| Q37 | Pack migrations/uninstall | Versioned migrations; uninstall disables Pack and preserves unknown/domain data until explicit export/delete. No silent destructive uninstall. | 086 | No weaker alternative permitted. |
| Q38 | Federation release need | Not required for first Personal/Lab/Institution release; remains later research. | 088 | Concrete signed multi-institution program requirement. |
| Q39 | Federated computation | Default data-stays-at-site. Only explicitly defined aggregate/model tasks with statistical/privacy validation are candidates. | 088 | Use-case-specific protocol and validity proof. |
| Q40 | Cross-site policy | Exchange a minimal versioned vocabulary: identity, purpose, data class, allowed computation, retention/expiry, export constraints, provenance. Unknown policy -> deny. | 088 | Interop research may expand vocabulary, never default-allow. |
| Q41 | Open vs paid packaging | Core local/offline authority, personal projects, local models, evidence and export ownership must not depend on paid cloud. Institutional deployment/support/adapters may be packaged separately. | product governance | Founder/business decision may refine commercial packaging but cannot silently weaken local-first product promise. |
| Q42 | Daily lab adoption MVP | Project + artifacts + tasks/notes + MedAgent + Privacy + Governed Browse + Analytics + AudioFlow foundation + Knowledge is the minimum integrated lab thesis; Hub enables the shared-lab tier. Advanced voice/Compute/Packs follow as scale layers. | 074-083 | User research/adoption evidence may change ordering but must preserve dependency constraints. |
| Q43 | Browse routing | Deterministic local/project retrieval first, then brokered HTTP/search, then deterministic browser automation, then agentic browser only when required and explicitly admitted. | 079 | Task corpus proves a different order is safer/more reliable without widening authority or sensitive egress. |
| Q44 | Browser credentials | No raw credentials in model context. Use scoped credential handles bound to approved origin/action/session; login/MFA uses explicit human takeover where needed. | 079 | No weaker alternative permitted. |
| Q45 | Browser SSRF/private network | Public Browse denies loopback, link-local, cloud metadata and private-network targets by default. Institutional private targets require a separately admitted adapter/policy. Redirects are re-evaluated. | 079/087 | Explicit institutional destination contract with network and data-class policy. |
| Q46 | Browser side effects | Spec 079 is research/read/download-first. Purchasing, form submission, clinical-system writes and other side effects remain denied until a separate bounded effect contract is promoted. | 079/087 | Use-case-specific idempotency/confirmation/approval/Unknown-state design and adversarial qualification. |
| Q47 | Browser isolation | Rendering/browser execution does not run as Core authority. Use a bounded worker/adapter with filesystem/network/credential scope; exact implementation is evidence-selected. | 079/085 | Platform comparison can select implementation, not weaken isolation. |
| Q48 | Web evidence retention | Preserve URL/time/route/span and content digest/snapshot where obtainable. Changed pages create new evidence identity/state; do not silently replace historical evidence. | 079/082 | Site/runtime limitations may reduce snapshot fidelity but must be explicit in receipt. |

## Blocking versus benchmark choices

The register deliberately separates two types of uncertainty.

### Architecture defaults are closed

The following are not open to implementer preference:

- one Core authority plane;
- no ambient vault access;
- network through admitted broker;
- no consensus-equals-truth;
- no unrestricted code execution in trusted Desktop;
- no silent fallback;
- exact provenance/revisions;
- local-first Personal path;
- append/revision/conflict semantics above;
- public Browse does not receive raw sensitive project context;
- raw browser credentials never enter model prompts;
- deterministic browsing is preferred before agentic browsing;
- side-effecting browser actions are not foundation behavior;
- data-stays-at-site default for federation;
- source audio and source data are not overwritten by derived transformations.

### Engine/vendor choices remain evidence-selected

These are intentionally selected during their owner spec through benchmark/qualification:

- deterministic browser automation implementation and optional agentic browser engine;
- live/offline/medical speech engine;
- VAD/diarizer/audio conditioning components;
- exact embedded vector/search implementation;
- DataFusion gaps/secondary analytical engine;
- worker isolation implementation per platform;
- optional external policy engine;
- institutional connector/provider implementations.

An evidence-selected choice MUST have a benchmark/qualification packet with exact revision, license/permission, platform, workload, acceptance threshold and fallback behavior before adoption.
