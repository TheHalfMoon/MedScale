# MedScale Research OS Threat and Scale Model

**Status:** Planning candidate — not implementation authority

## Purpose

The Research OS vision expands MedScale's attack surface. This document records the minimum architectural threats and scale constraints that future specifications must close explicitly.

## Trust zones

```text
Zone A — Local trusted Core/Vault
Zone B — Local bounded workers
Zone C — User-controlled MedScale Hub / institutional infrastructure
Zone D — Approved institutional adapters
Zone E — External/public network and providers
```

Data movement from a lower-numbered/high-trust zone to a wider zone requires explicit capability, classification, policy, and receipt.

## Principal identities

- local human user;
- project/team member;
- organization administrator;
- MedAgent instance;
- external/delegate agent;
- Hub service;
- compute worker;
- connector/adapter;
- workflow identity;
- model/runtime identity.

No identity class inherits another's authority implicitly.

## Key threats

### T1 — Agent confused-deputy access
An agent is asked to perform a benign task but gains access to unrelated PHI, secrets, or project artifacts.

Required response: capability-scoped context and tools; explicit artifact selectors; no ambient vault handles.

### T2 — Prompt/tool exfiltration
Malicious document/web content instructs an agent to export sensitive data.

Required response: content is untrusted evidence, not policy; tool capability checks occur outside the model; Privacy Gate applies at egress.

### T3 — Cross-project/team leakage
Search, RAG, room subscriptions, caches, or workers return artifacts from another Project/tenant.

Required response: project/tenant context must be part of authorization, indexes, caches, event delivery, and worker manifests.

### T4 — Collaboration-event authority confusion
A message or agent event claims that an artifact was approved/changed when Core did not admit that state.

Required response: events reference Core facts; authority-changing state requires typed Core command/receipt.

### T5 — Audio capture abuse
Microphone/system audio activates without clear user intent or continues beyond expected scope.

Required response: visible capture state, explicit start/stop, OS privacy compliance, source health, interruption, recovery, and tests for crash/suspend/resume.

### T6 — Speaker-biometric overreach
Diarization labels silently become persistent biometric identities.

Required response: segmentation and persistent identity are separate capabilities; embeddings are opt-in, encrypted, deletable, and access-controlled.

### T7 — Remote worker escape/exfiltration
Compute/audio/model workers access host filesystem, unrestricted network, secrets, or unrelated data.

Required response: sandbox/isolation, immutable job manifest, bounded mounts, egress policy, resource/time limits, scrubbed logs, cancellation.

### T8 — Silent runtime fallback
A local/GPU/private route silently changes to CPU, cloud, or another provider.

Required response: route decision is explicit, inspectable, and policy checked; unavailable routes fail with reasons.

### T9 — Model/source supply chain
Floating model revisions, remote code, compromised downloads, or untracked donor changes alter behavior.

Required response: immutable Pack manifests, exact digests, rights/NOTICE closure, no floating remote executable code, rollback/quarantine.

### T10 — BI/RAG service overreach
Optional Superset/OpenRAG/search adapters gain direct unrestricted access to the vault.

Required response: adapters receive materialized governed views or scoped artifact interfaces only.

### T11 — Hub compromise
A self-hosted Hub is compromised and attempts to read or mutate local sensitive artifacts.

Required response: Hub authority is narrower than local Core; encryption and object-transfer policies should minimize server plaintext exposure where practical; local Core validates incoming state.

### T12 — Federation provenance loss
Cross-institution aggregation loses data-use restrictions or source lineage.

Required response: federation is deferred until signed bundles/policies/provenance/revocation semantics are proven.

## Scale envelopes to design for

Future benchmarks should define explicit supported envelopes rather than vague "enterprise scale" claims.

Candidate dimensions:

- artifacts per Project;
- Projects per local user;
- concurrent team members;
- rooms/messages/events;
- evidence spans;
- dataset bytes/rows/columns;
- audio session duration and concurrent streams;
- transcript tokens;
- model Packs and loaded models;
- concurrent MedAgent lanes;
- concurrent compute jobs;
- worker fleet size;
- Hub reconnect backlog;
- search/vector/graph index sizes;
- audit/event retention.

## Suggested qualification tiers

### Personal tier
- one user;
- offline-first;
- local CPU/GPU;
- no Hub dependency;
- modest project/data/audio scale.

### Lab tier
- small team;
- one Hub;
- shared object storage optional;
- one or more GPU workers;
- real-time rooms/huddles;
- project-level authorization.

### Institutional tier
- many teams/projects;
- SSO/organization policy;
- horizontally scalable Hub/search/worker services where justified;
- department/tenant isolation;
- HPC/cluster adapters;
- backup/DR requirements.

No higher tier may change the semantics of provenance, capability, approval, or artifact identity.

## Failure-first scenarios

Every major subsystem spec should include at least:

- network loss mid-sync;
- Hub restart during event/activity write;
- worker crash mid-run;
- GPU OOM;
- model corruption;
- disk full;
- long audio source disconnect;
- microphone device change;
- duplicate/replayed event;
- conflicting offline edits;
- revoked member/agent during active run;
- Privacy Gate denial after an agent has prepared an external call;
- cancelled FleetRun with partial lane results;
- search/vector index stale or unavailable;
- restore from backup with pending local changes.

## Product truth rule

Feature presence never establishes readiness at lab or institutional scale. Every scale tier requires measured limits, recovery evidence, and explicit unsupported conditions.
