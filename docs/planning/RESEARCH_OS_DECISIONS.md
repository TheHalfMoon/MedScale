# MedScale Research OS Planning Decisions

**Status:** Candidate decisions for review. These are not ADRs and do not override active canonical specifications.

## D1 — MedScale remains authority-centric

**Decision:** Keep one MedScale authority plane. New intelligence/collaboration subsystems may propose or reference artifacts but cannot silently create clinical/research authority.

**Why:** The product becomes less trustworthy, not more, if models, collaboration events, BI tools, and external agents each invent their own source of truth.

## D2 — Projects become the organizing primitive

**Decision:** Future Research OS work should organize around Projects and their artifact graph rather than around a flat collection of unrelated feature routes.

**Why:** Researchers work in projects/experiments, and Projects provide the common context required by MedAgent, Analytics, AudioFlow, collaboration, evidence, and compute.

## D3 — Everything-in-one-place does not mean everything-in-one-process

**Decision:** Preserve one UX and one authority model while allowing bounded workers and optional self-hosted services.

**Why:** Speech, large-model inference, BI, search, and HPC workloads have incompatible runtime/resource profiles. A monolithic Desktop would harm reliability, packaging, and privacy.

## D4 — Buzz is a collaboration donor, not the regulated datastore

**Decision:** Selectively adapt Buzz room/event/agent/workflow/media/audit patterns behind MedScale-owned contracts. Do not replace MedScale Core/FHIR/artifact authority with Nostr events.

## D5 — AudioFlow is an intelligence plane

**Decision:** Treat voice/audio as a first-class project modality covering capture, transcription, diarization, meetings, dictation, agent control, TTS, and evidence mapping.

**Why:** A microphone button does not scale to interviews, meetings, medical terminology, long-form recordings, multilingual work, and research evidence.

## D6 — Multi-engine routing beats one universal speech model

**Decision:** AudioFlow owns a Voice Runtime Router with task/language/device/privacy/resource-aware routing and explicit fallback decisions.

## D7 — Voice commands carry explicit semantics

**Decision:** Distinguish `COMMAND`, `CONTEXT`, and `DICTATION`, with interruption/cancellation/steering as first-class lifecycle operations.

**Why:** Spoken context must not accidentally become authority to execute.

## D8 — Native analytics first

**Decision:** Qualify Arrow/DataFusion or an equivalent embeddable Rust-compatible analytics path for the default product. Keep Superset/Metabase/Nao-class BI as optional adapters.

**Why:** The default Desktop should stay local, native, scriptable, reproducible, and lightweight enough for individual researchers.

## D9 — RAG is governed retrieval, not vector search alone

**Decision:** Combine structured Project Graph context, lexical search, vector retrieval where justified, explicit evidence spans, and permission filtering.

## D10 — Agents are first-class participants, not first-class authorities

**Decision:** Agents receive identities, memberships, tasks, and capability sets. Sensitive reads/writes, exports, approvals, and clinical authority remain separately granted.

## D11 — Privacy Gate is cross-cutting

**Decision:** PII/PHI/de-identification policy applies to browser, model, Hub, connector, export, compute, and analytics-adapter boundaries rather than living as one isolated feature.

## D12 — No mandatory MedScale cloud

**Decision:** Personal/offline operation remains valid. Team operation uses an optional user-controlled MedScale Hub deployable on workstation, LAN, on-prem, or private infrastructure.

## D13 — Scale through workers/contracts

**Decision:** MedScale Compute runs bounded jobs using explicit manifests and least-privilege artifact access. Lab GPU/HPC support should not widen the Desktop or vault trust boundary.

## D14 — Domain scale through Research Packs

**Decision:** Clinical Research, AI Research, Imaging, Omics, Wet Lab, and Systematic Review capabilities should extend a stable Core rather than hard-code every research domain into the kernel.

## D15 — Source permission is not adoption proof

**Decision:** Even where the founder has permission to use source code, direct transfer requires exact path/revision provenance, license/NOTICE and embedded-code review, model/data/asset terms, security qualification, and proof that selective adoption beats a smaller native/dependency solution.

## D16 — Unknown stays unknown

**Decision:** Comparison and telemetry must distinguish not observed, not measured, unavailable, failed, and passed. No fake parity between models/agents/runtimes.

## D17 — Consensus is not truth

**Decision:** Multi-model agreement is shown separately from evidence agreement, deterministic validation, human approval, and clinical/research authority.

## D18 — Planning must remain subordinate to canonical governance

**Decision:** The Research OS documents live under `docs/planning/` and use candidate language until a future canonical promotion explicitly authorizes a bounded specification. Live `specs/CURRENT.md` remains authoritative.
