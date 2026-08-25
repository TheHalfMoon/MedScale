# Research: Spec 007 OpenMed Capability Absorption / Parity Research

**Date**: 2026-08-25  
**Spec**: `007-openmed-capability-absorption-parity`  
**Mode**: RESEARCH_ELIGIBLE — docs/matrices/evidence; no product runtime

## Decisions

### D1 — Frozen OpenMed baseline is the only parity floor

- **Decision**: All Spec 007 parity and absorption work pins:
  - **URL**: `https://github.com/maziyarpanahi/openmed`
  - **Tag**: `v2.2.0`
  - **Commit**: `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837`
  - **Tree**: `1c949e35b2b8f2ea69da4284b370074fc4bf84ab`
- **Alternatives**: Track `main`; re-pin every release; dual-baseline.
- **Rationale**: SOURCE_ACQUISITION / AGENTS.md / OPENMED_PARITY_SURPASS_MATRIX_V2 forbid moving-branch comparison. Informational tip deltas may be logged but never replace the floor.

### D2 — Spec 007 is research/admission contracts only

- **Decision**: Spec 007 delivers matrices, corpus designs, dispositions, licensing track start, Saudi/Arabic program design, provenance templates. **No** Rust NER/PII/de-ID fabric, **no** ONNX/llama/Candle admission, **no** OpenMed Python DEPENDENCY in trusted core.
- **Alternatives**: Combine 007+008 in one unit; ship a thin Python sidecar “for parity.”
- **Rationale**: SPECKIT_MASTER_ROADMAP — “no runtime required by this spec itself”; BUILD_QUEUE `RESEARCH_ELIGIBLE`; 008 needs qualified 007.

### D3 — Phase-scoped parity, not wholesale feature clone

- **Decision**: Adopt OPENMED_PARITY_SURPASS_MATRIX_V2 classifications unchanged:
  - `REQUIRED_PARITY`
  - `REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE`
  - `USEFUL_PARITY`
  - `REQUIRED_ABSORB_PATTERN`
  - `MEDSCALE_SUPERSEDES_CONCEPT`
  - `DEFER`
- Deferred / useful rows do not block Spec 007 research close. Surpass claims require BenchmarkManifest exact-head evidence.
- **Rationale**: MASTER_BUILD_PLAN §8; matrix §1 gate semantics.

### D4 — Absorption vocabulary and OpenMed forbidden interpretations

- **Decision**: Use SOURCE_ACQUISITION operation vocabulary exclusively. For OpenMed:
  - Prefer `COPY_BOUNDED` tests/fixtures/deterministic rules + `PORT_TO_RUST` contracts in owning specs.
  - Prefer `REFERENCE_ONLY` for service/MCP/GraphQL/security plane ideas.
  - `ARTIFACT_IMPORT` for models/packs later (008/015)—not 007.
  - **DO_NOT_COPY**: wholesale repo fork; Python clinical domain as trusted core; restricted terminology tables; treating model output as ClinicalAssertion.
- **Rationale**: SOURCE §2 forbidden interpretations; OSS matrix OpenMed row.

### D5 — Parity corpora are synthetic-first and claim-bound

- **Decision**: Corpus descriptors define fixture families, trap taxonomies, metrics (F1/P/R, PHI recall, span fidelity, latency/RSS placeholders), and rights. Spec 007 may ship small synthetic seeds; large measured runs are Spec 008 evidence. Raw model catalogue cardinality is **never** a parity metric.
- **Rationale**: Matrix §3; H0/REAL_PHI gates; decision defaults (evidence over brand).

### D6 — Terminology rights track starts here; tables never from OpenMed

- **Decision**: Start track for SNOMED CT, LOINC, UCUM, ICD, ATC, UMLS/Athena (minimum). Rights classes: `open`, `caller_supplied`, `gated_license`, `unknown_pending_counsel`. Pack metadata must include version, checksum, rights URI, NOTICE, expiry/revocation hooks. OpenMed terminology snapshots → contract/test REFERENCE only—**never** copy tables. Counsel/license acceptance → EXTERNAL_GATES; does not block 007 research close.
- **Rationale**: SOURCE §5; OSS terminology rows; IMPLEMENTATION_DECISION_DEFAULTS escalate-to-gate.

### D7 — Saudi/Arabic program: design now, measure in 008

- **Decision**: Spec 007 designs trap classes (MSA/dialect, code-switch EN↔AR, national ID / Iqama-style patterns as **synthetic** traps, medication/dose numbers, named entities, RTL/Unicode offset traps) and metrics. Classification remains `REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE`. No REAL_PHI corpus. Surpass forbidden until pinned OpenMed vs MedScale exact-head run.
- **Rationale**: Matrix Saudi/Arabic row; HARD-negative PR ideas (#2243/#2682) as REFERENCE for synthetic generation patterns.

### D8 — Critical v2.2 capability families to represent in 007 handoff

| Family | 007 role | Later owner |
|---|---|---|
| Clinical NER / PII / de-ID | corpus + disposition + metrics | 008 |
| Multilingual + Saudi/Arabic | program design + corpus slots | 008 |
| Terminology grounding/calibration | contracts + rights track | 011 (+ 008 packs) |
| Unicode/offset | adversarial test design handoff | 002/010 |
| Document MIME/quarantine breadth | corpus notes | 010 |
| FHIR/SDC/privacy patterns | REFERENCE + test ideas | 011/013 |
| Staged promotion latest/last_green/canary | absorb pattern notes | 008/015 |
| Mobile privacy patterns | REFERENCE | 009 |
| Browser/WebGPU, watchOS/visionOS, REST plane | DEFER explicit | 016+ |

### D9 — BenchmarkManifest is mandatory for claims

- **Decision**: Any future `PARITY` / `SURPASS` / `PRIVATE` claim binds: OpenMed commit, MedScale commit, artifact hashes, corpus hash, rights, hardware/OS/runtime, quality metrics, latency/cold-start/throughput, RAM/VRAM, network/privacy observation, failure behavior, limitations, harness revision.
- **Rationale**: OPENMED_PARITY §3; DEFINITION_OF_DONE culture.

### D10 — Provenance template before first COPY_BOUNDED line

- **Decision**: Spec 007 ships `third_party/provenance/_TEMPLATE-openmed-component.md` (or package-local template). First real provenance file is created in the **owning implement spec** immediately before first donor-derived line—not during pure research if no bytes are copied.
- **Rationale**: SOURCE acquisition mandatory record; 30-source-acquisition rule.

### D11 — Placement for future engines (handoff, not implement)

- **Decision**: Reaffirm OSS placement: complex hostile parsers/models → `P1_ISOLATED_REQUIRED` on desktop; mobile `P2` only after evidence; Rust ≠ automatic P0.
- **Rationale**: OSS_CODE_ABSORPTION_MATRIX_V2 §1; Spec 008/010 consume.

## Alternatives rejected

| Option | Why rejected |
|---|---|
| Wholesale OpenMed fork as MedScale | Forbidden by constitution / SOURCE |
| OpenMed Python as MedScale DEPENDENCY | Trusted core must be Rust-owned |
| Re-pin baseline to tip for “fairness” | Moving floor destroys evidence |
| Bundle SNOMED/LOINC from OpenMed | Rights; SOURCE forbids |
| Implement NER runtime in 007 | Roadmap: 008 owns fabric |
| Require counsel closure for 007 research close | External gate; continue independent research |

## Open research follow-ups (non-blocking)

1. Exact changed-file inventories for cited OpenMed PRs when first COPY_BOUNDED executes (freeze then).
2. Measured Saudi/Arabic corpus volume targets under 008 evidence budget.
3. Counsel determination for each `gated_license` terminology system (EXTERNAL_GATES).
