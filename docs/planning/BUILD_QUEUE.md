# MedScale Autonomous Build Queue

**Queue owner:** repository canonical plan  
**Execution agent:** Cursor  
**Rule:** update this file whenever a unit enters or leaves a canonical state.

**Autonomous stop status (historical V2 scoped closure):** superseded for Trusted V1 follow-on work.
**Live follow-on status (2026-09-09):** Spec **016**–**021** `CLOSED_CANONICAL`. Spec **020** FHIR interchange (Q08) and Spec **021** minimum lovable workflow (Q07) are READY_BASE with `RELEASE_READY = FALSE`. Spec 012 remains MESC-blocked. Next eligible work is release-qualification prep / remaining Q05 evidence hardening (immutable pins, evidence artifacts) — not a RELEASE_READY claim. Advanced work **022+** remains deferred. `RELEASE_READY = FALSE`.

## 2026-09-09 planning refinement

See Trusted V1 delivery plan. Specs 018–021 READY_BASE closed. Next: Q05 release-evidence hardening / release-qualification prep (honestly partial; no RELEASE_READY). Spec 012 MESC-blocked. 022+ deferred.
Main after Spec 020: `987d596`. Spec 021 on branch `spec/021-minimum-lovable-workflow`.

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
| 016 | Durable Trusted Record (Q02) | `CLOSED_CANONICAL` | Full authority object graph persists across process restart; see evidence/016. |
| 017 | Vault Privacy Qualification (Q03) | `CLOSED_CANONICAL` | Work/WAL wipe + doctor honesty; PRIVATE_DATA_READY remains FALSE. |
| 018 | Host / Client Authority (Q04) | `CLOSED_CANONICAL` | READY_BASE: in-process SessionRegistry; MULTI_CLIENT_RELEASE_READY=false. |
| 019 | Record Semantics (Q06) | `CLOSED_CANONICAL` | READY_BASE: precision-aware MedicalTime, AmendAssertion, missingness, identity unresolved; RELEASE_READY=false. |
| 020 | FHIR Interchange Qualification (Q08) | `CLOSED_CANONICAL` | READY_BASE: honest support matrix + loss-aware export; no full conformance; RELEASE_READY=false. |
| 021 | Minimum Lovable Workflow (Q07) | `CLOSED_CANONICAL` | READY_BASE: synthetic restartable import-review-export-backup journey + disclosure; RELEASE_READY=false. |
| 022+ | Advanced deferred work | `DEFERRED_BY_CANONICAL_DESIGN` | Plugins/GraphRAG/replicas/imaging/CUDA/etc. |

## Automatic progression

For the first `READY` unit: create/complete its Spec Kit package, analyze it, implement tasks in dependency order, qualify exact head, converge, merge if all required gates pass, mark `CLOSED_CANONICAL`, recompute this queue, and immediately start the next eligible unit.

Do not stop merely because a PR merged, one milestone passed, or an external optional gate exists.

**Next eligible (honest):** Q05 release-evidence / release-qualification prep remains open (immutable action pins, locked builds, evidence artifacts; repo settings may be external). Spec 012 remains MESC-blocked. Do **not** claim `RELEASE_READY`.
