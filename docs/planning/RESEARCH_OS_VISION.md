# MedScale Research OS Vision

**Status:** Planning candidate — not implementation authority

## Product thesis

MedScale should evolve from a local-first clinical intelligence workspace into a local-first Research and Clinical Intelligence OS for laboratories, research groups, health-data teams, clinicians, and scientific organizations.

The product should make projects, data, models, agents, analytics, evidence, audio, and collaboration available in one coherent workspace without collapsing authority, privacy, or provenance boundaries.

## Non-negotiable promise

**One workspace. Many engines. One authority. User-owned data. Local by default. Evidence everywhere.**

MedScale Core remains the authority plane. Models, agents, browser tools, retrieval systems, analytics engines, and speech systems produce proposals, derived artifacts, and evidence-bearing outputs; they do not silently become canonical clinical or research truth.

## Target users

MedScale should serve:

- clinical researchers;
- wet-lab and translational research groups;
- epidemiology and public-health teams;
- health-data-science groups;
- medical AI/ML teams;
- imaging and radiology research groups;
- genomics/omics teams;
- systematic-review teams;
- hospital quality-improvement and operations teams;
- university and institutional research centers.

## Product shape

The durable architecture is a small governed kernel with extensible workspaces, packs, workers, and optional self-hosted services:

```text
                         MedScale
                            |
        +-------------------+-------------------+
        |                                       |
  MedScale Desktop                        MedScale CLI
        |                                       |
        +-------------------+-------------------+
                            |
                    MedScale Core
             authority | identity | policy
             projects | artifacts | evidence
             privacy | audit | provenance
                            |
        +---------------+---+----+----------------+
        |               |        |                |
     MedAgent        Analytics  AudioFlow      Knowledge
        |               |        |                |
   model fleet       DataFusion  STT/TTS         RAG
   tools/browser     cohorts     meetings        graph
   compare           notebooks   dictation       search
   workflows         reports     voice control   evidence
        |               |        |                |
        +---------------+---+----+----------------+
                            |
                     Project Graph
                            |
               Team | Tasks | Notes | Rooms
                            |
                     MedScale Hub
                optional | self-hosted
                            |
          +-----------------+------------------+
          |                                    |
   MedScale Compute                      MedScale Packs
 CPU/GPU/lab/HPC workers             models/tools/domain packs
```

## Core object model

The platform should converge around a small set of durable concepts:

- `Project`
- `Experiment`
- `Artifact`
- `Dataset`
- `Document`
- `AudioSession`
- `Run`
- `EvidenceRef`
- `Finding`
- `Task`
- `Room`
- `Workflow`
- `ModelPack`
- `Member`
- `CapabilityGrant`

Every durable artifact should carry identity, revision, digest, provenance, data classification, project ownership, permissions, creator, and evidence links.

## Project Graph

A Project is not a folder. It is a provenance graph.

```text
Protocol
   |
   +-- defines --> Cohort
   |                 |
   |                 +-- generates --> Dataset v3
   |                                      |
   |                                      +-- analyzed_by --> Run #82
   |                                      |                    |
   |                                      |                    +-- produces --> Figure 4
   |                                      |
   |                                      +-- discussed_in --> Audio Session
   |
   +-- supported_by --> Paper
                           |
                           +-- evidence_span --> page 14
```

This graph becomes the substrate for retrieval, analysis, collaboration, audit, and MedAgent context.

## Research Packs

The Core must stay domain-general enough to scale. Domain semantics should be admitted through governed packs rather than hard-coded into one giant application.

Candidate packs:

- Clinical Research Pack — FHIR, cohorts, encounters, protocols, consent;
- AI Research Pack — datasets, models, experiments, evaluations, benchmarks;
- Imaging Pack — DICOM, series, annotations, measurements;
- Omics Pack — FASTQ/BAM/VCF, pipelines, variants;
- Wet Lab Pack — samples, batches, assays, reagents, instrument runs;
- Systematic Review Pack — papers, screening, extraction, risk-of-bias, PRISMA.

## Deployment ladder

MedScale should scale without changing the product model:

1. **Personal** — Desktop, encrypted local vault, local models, local audio, no server.
2. **Lab** — Desktop clients + one self-hosted Hub + shared storage + optional GPU workstation.
3. **Research Center** — multiple teams, SSO, policy engine, object storage, worker pool, audit, backup.
4. **Institution** — multiple Hubs, department isolation, institutional identity/policy, HPC and cluster adapters.
5. **Federation** — controlled cross-institution project bundles and federated analysis without mandatory central MedScale cloud.

## Authority rule

MedScale may make agents and workflows first-class participants, but never first-class authorities by default.

Agents may receive identity, room membership, tasks, tools, and scoped access. Clinical authority, export authority, consent authority, project ownership, and irreversible mutations remain explicit capabilities with policy and approval boundaries.

## Success condition

A researcher should be able to open MedScale and conduct the full lifecycle of a project — ingest, discuss, record, search, analyze, compare models, run agents, manage evidence, collaborate, and publish — without surrendering control of sensitive data or losing the ability to explain how an output was produced.
