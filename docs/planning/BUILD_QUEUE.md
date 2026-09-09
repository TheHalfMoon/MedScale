# MedScale Autonomous Build Queue

**Queue owner:** repository canonical plan  
**Execution agent:** Cursor  
**Rule:** update this file whenever a unit enters or leaves a canonical state.

**Autonomous stop status (main `18d6eca`, 2026-08-26):** `IMPLEMENTATION_COMPLETE_PENDING_EXTERNAL_GATES`

Ungated Spec Kit units 000–011 and 013–015 are `CLOSED_CANONICAL`. Spec 012 remains blocked solely by `MESC_RELEASED_ARTIFACT` (empty MESC release assets; TRAINING_CODE_READY ≠ RELEASE_READY). Spec 016+ stays `DEFERRED_BY_CANONICAL_DESIGN`. No further eligible autonomous implementation remains until an external gate clears or a new unit is canonically promoted.

## 2026-09-09 planning refinement

The stop status above is historical scoped V2 closure, not product readiness.
Review baseline: main `b49592c83d23542363f671afe6a9ae65fc65b276`.
See [whole-product review](WHOLE_PRODUCT_REVIEW_2026-09-09.md) and
[delivery plan](TRUSTED_V1_DELIVERY_PLAN.md). Next eligible planning task is T01 in the
[bounded follow-on package](../../specs/review-trusted-v1-2026-09-09/README.md):
prepare and analyze the owning durable-record specification before runtime changes.
This does not promote all advanced work or reopen previous scoped closures.
No founder permission is needed for ordinary specification preparation. RELEASE_READY = FALSE.

## Historical scoped queue (closures preserved)

| Order | Spec | State | Next action |
|---:|---|---|---|
| 000 | Constitution + Source Authority | `CLOSED_CANONICAL` | Planning V2/source authority already canonicalized; Spec 001 materializes it into the Spec Kit repository structure without reopening founder decisions. |
| 001 | Rust Repository + Spec Kit Bootstrap | `CLOSED_CANONICAL` | Spec Kit + Rust workspace + CI/supply-chain bootstrap merged; continue Spec 002. |
| 002 | Trusted Object / Source / Authority + Process/Text Foundation | `CLOSED_CANONICAL` | Object/authority foundation merged; continue Spec 003. |
| 003 | H0-A Trusted Ingest + Durability | `CLOSED_CANONICAL` | Synthetic FHIR R4 ingest + durability merged; continue Spec 004. |
| 004 | H0-B Trusted Presentation + Coverage | `CLOSED_CANONICAL` | Deterministic timeline/Brief/coverage merged; continue Spec 005. |
| 005 | Local Private Vault + Encryption + Recovery | `CLOSED_CANONICAL` | Encrypted vault + recovery merged; continue Spec 006. |
| 006 | CLI + Desktop Foundation | `CLOSED_CANONICAL` | CLI wedge + non-WebView desktop scaffold merged; continue eligible units. |
| 007 | OpenMed Absorption / Parity Research | `CLOSED_CANONICAL` | OpenMed v2.2 pin + parity matrix/dispositions merged; Spec 008 unblocked for fabric. |
| 008 | Local AI Capability Fabric | `CLOSED_CANONICAL` | Offline Pack v0 + worker ambient-deny merged; OS sandbox PLATFORM_QUALIFIED remains OPEN gate. |
| 009 | Mobile iOS + Android | `CLOSED_CANONICAL` | READY_BASE: doctor/mobile axes, keystore sync forbid, FFI stubs; no apps. |
| 010 | Documents + OCR + Voice | `CLOSED_CANONICAL` | MIME quarantine + OCR/ASR stubs merged; real engines deferred. |
| 011 | Evidence / Retrieval / Medical Intelligence | `CLOSED_CANONICAL` | Lexical retrieval to evidence-only EvaluationRecords; relevance is not authority. |
| 012 | MESC Artifact Integration | `BLOCKED_BY_RELEASED_MESC_ARTIFACT` | Fail-closed admit + doctor axis shipped; ARTIFACT_IMPORT still blocked (empty MESC release assets). |
| 013 | FHIR / SMART / Network Broker | `CLOSED_CANONICAL` | Fail-closed broker + stub SMART/FHIR adapters merged; continue Spec 014 when workflow evidence ready. |
| 014 | Controlled Actions / NPHIES | `CLOSED_CANONICAL` | READY_BASE: outbox + payload-bound intents; NPHIES remains external gate. |
| 015 | HF + Online Pack Ecosystem | `CLOSED_CANONICAL` | READY_BASE deny path via Network Broker; HF online remains external gate. |
| 016+ | Advanced work | `DEFERRED_BY_CANONICAL_DESIGN` | Promote only with new evidence/spec authority. |

## Automatic progression

For the first `READY` unit: create/complete its Spec Kit package, analyze it, implement tasks in dependency order, qualify exact head, converge, merge if all required gates pass, mark `CLOSED_CANONICAL`, recompute this queue, and immediately start the next eligible unit.

Do not stop merely because a PR merged, one milestone passed, or an external optional gate exists.
