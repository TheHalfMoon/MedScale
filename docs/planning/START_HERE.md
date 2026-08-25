# MedScale — Start Here

## 1. Current gate

```text
PLAN = CANONICAL_V2
REPOSITORY_PLANNING_FINALIZATION = COMPLETE
PRODUCT_IMPLEMENTATION = NOT_STARTED
NEXT_EXECUTABLE_GATE = EXPLICIT_FOUNDER_IMPLEMENTATION_AUTHORIZATION
```

Do not initialize Cargo, install dependencies, execute models, access PHI, or start product code merely because the plan is present.

## 2. Build order

Follow this dependency chain exactly unless a later canonical spec explicitly changes it:

```text
000 Constitution + Source Authority
  -> 001 Rust Repository + Spec Kit Bootstrap
  -> 002 Trusted Object / Source / Authority + Process/Text Foundation
  -> 003 H0-A Trusted Ingest + Durability
  -> 004 H0-B Trusted Presentation + Coverage
  -> 005 Local Private Vault + Encryption + Recovery
```

Then controlled parallel tracks:

```text
004 -> 006 CLI + Desktop product discovery/foundation preparation
004 -> 007 OpenMed parity/absorption research (research only may start early)

005 + 006 + 007 -> 008 Local AI Capability Fabric + offline pack admission + worker confinement
005 + 006       -> 009 Mobile iOS + Android
008             -> 010 Documents + OCR + Voice
004 + 008       -> 011 Evidence / Retrieval / Medical Intelligence
008 + released MESC artifact -> 012 MESC Artifact Integration
005 + 006       -> 013 FHIR / SMART / Network Broker
013 + product evidence -> 014 Controlled Actions / NPHIES
008 + 013       -> 015 Online Pack Ecosystem + Hugging Face distribution
016+            -> deferred advanced work only after evidence
```

The authoritative dependency details are in `SPECKIT_MASTER_ROADMAP_V2.md`.

## 3. First product wedge

The first coherent product outcome is the **Trusted Local Longitudinal Record**:

- exact local source custody;
- deterministic FHIR R4 synthetic ingest first;
- explicit identity and time semantics;
- deterministic timeline;
- narrow LLM-free Brief;
- coverage/conflict/unknown-vs-absence accounting;
- source/provenance drill-down;
- one Rust-owned authority path shared by CLI and Desktop.

AI is added later as bounded capability. AI output remains `DerivedSourceArtifact`, `Proposal`, or `EvaluationRecord` until authorized promotion.

## 4. Before implementing any spec

A builder must:

1. Verify live `main`, branches, PRs, exact heads, changed files, CI/reviews, and current task status.
2. Read `AGENTS.md`.
3. Read this file and the owning roadmap row.
4. Read the relevant canonical architecture/source/donor/parity material.
5. Create the complete Spec Kit planning package for that spec.
6. Run `/speckit.analyze` and close contradictions.
7. Obtain explicit implementation authorization.
8. Only then run `/speckit.implement`.
9. Produce exact evidence and converge before closing the unit.

## 5. Evidence rule

No `PASS`, `PARITY`, `SURPASS`, `PRIVATE`, `OFFLINE`, `CONFORMANT`, or `CLOSED_CANONICAL` claim is valid without evidence bound to exact repository revision, toolchain/dependencies, platform, fixture/corpus hash, commands/configuration, outputs, and limitations.

OpenMed comparisons additionally bind the exact OpenMed revision, model/artifact revision, hardware/OS, quality, latency, memory, network/privacy behavior, provenance, safety, and limitations.

## 6. Explicitly not next

Do not begin with models, agents, NPHIES, imaging, genomics, browser/WebGPU, cloud services, vector databases, or a large plugin ecosystem. The trusted foundation and first local product wedge come first.
