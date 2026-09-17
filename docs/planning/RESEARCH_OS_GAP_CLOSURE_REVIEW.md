# MedScale Research OS Gap Closure Review

**Status:** Planning review evidence — not implementation authority.

## Review basis

```text
CANONICAL_MAIN=c795796132ce633b23b693d55bccc0521d0d26aa
REVIEWED_PLANNING_HEAD=14e5e8c1716b2f50f3a80921dbbc87effb9fa124
BRANCH=plan/medscale-research-os
PR=121
AHEAD_OF_MAIN=57
BEHIND_MAIN=0
```

The commit that adds this review note contains no architecture/implementation change; it records the review performed on the planning packet immediately before the note.

## Scope verification

Git comparison confirmed the planning branch changes only `docs/planning/` artifacts relative to the verified main base. No Rust code, canonical spec package, `AGENTS.md`, Product/Design authority, CI workflow, release evidence, or production configuration is changed by this planning packet.

## Material gap found and closed

### Governed Browse was initially under-specified

The original brainstorming and architecture required MedAgent web browsing, and the Master Implementation Contract contained browser rules, but the early execution roadmap did not give browsing an independently owned implementation/verification unit.

That was a material gap because browsing has distinct:

- egress/privacy policy;
- credential handling;
- redirect/SSRF/private-network risk;
- prompt-injection/hostile-content risk;
- browser-process isolation;
- download quarantine;
- evidence snapshot/span provenance;
- cancel/timeout/crash lifecycle;
- future side-effect/Unknown-state semantics.

The gap is now closed by candidate **Spec 079 — Governed Browse**.

## Final candidate numbering

```text
074 Project + Artifact Graph Foundation
075 Collaboration Substrate
076 MedAgent Workbench
077 Model Fleet + Compare
078 Privacy Gate
079 Governed Browse
080 AudioFlow Foundation
081 Analytics Gate
082 Knowledge + Research Canvas
083 MedScale Hub
084 AudioFlow Advanced
085 MedScale Compute
086 Research Packs
087 Institutional Adapters
088 Federation
089 Whole-Platform Qualification
```

The roadmap, Spec Implementation Contracts, Repository Implementation Map, Verification Matrix, Decision Resolution Register, Planning Index, Manifest, Packet Version, Acceptance Framework, Open Questions inventory, and Planning Status were reconciled to this numbering.

## Cross-plane dependency review

### Project/authority substrate
- 074 owns Project/Artifact Graph organization while preserving existing MedScale object IDs/provenance.
- no later plane is allowed to introduce a second authority/ID/audit system.

### Collaboration
- 075 owns local collaboration semantics.
- 083 adds optional self-hosted synchronization after local semantics and Privacy Gate exist.
- collaboration events reference canonical artifacts; they do not become the regulated/scientific record.

### MedAgent/Fleet
- 076 proves one local, zero-network, project-grounded agent path first.
- 077 adds independently scoped lanes; lane permissions are never unioned.
- consensus remains distinct from evidence/authority.

### Privacy/Browse
- 078 owns cross-boundary classification/transformation/egress decisions.
- 079 requires 076 + 078 + current Network Broker authority.
- public Browse denies raw protected/local project context, raw credentials, private-network targets and foundation side-effecting actions by default.

### Audio
- 080 depends on MedAgent + Privacy for voice-control/data handling.
- source audio remains evidence; transcripts are non-destructive revisions.
- 084 requires foundation AudioFlow plus collaboration/Hub for shared huddles; remote-worker features wait for Compute where needed.

### Analytics/Knowledge
- 081 owns governed read-only native analytics and reproducible QueryReceipts.
- 082 owns permission-aware retrieval/index/Canvas projections.
- Browse/audio/analytics source integration in Knowledge is gated on the corresponding source plane being closed.

### Hub/Compute
- 083 is optional/user-controlled and cannot become alternate Core authority.
- 085 owns general least-privilege worker/job semantics; no ambient vault mount.
- deterministic browser workers in 079 remain bounded and may later converge on 085 infrastructure without making 085 a hidden prerequisite for basic Browse shaping.

### Research Packs/Institution/Federation
- 086 extends domains declaratively without bloating Core or allowing arbitrary trusted-process UI code.
- 087 owns institution-specific SSO/storage/LIMS/EHR/HPC/BI/search/action adapters.
- 088 remains later research; data stays at site by default.
- 089 qualifies explicit release profiles rather than manufacturing one global PASS.

## Implementation-ambiguity review

The packet now provides:

- Master Implementation Contract;
- per-Spec implementation contracts;
- Repository Implementation Map tied to current crates;
- safe Decision Resolution Register defaults;
- hardened Future Spec Template;
- Implementer Instructions;
- L0-L9 Verification Matrix;
- universal + subsystem acceptance gates;
- migration/recovery rules;
- donor/source adoption rules;
- explicit stop conditions and evidence truth rules.

Open engine/vendor choices are evidence-selected rather than delegated to implementer preference.

## Stale-reference review

The PR planning diff was checked for obsolete associations introduced before Governed Browse insertion, including:

```text
079 = AudioFlow
082 = Hub
083 = AudioFlow Advanced
084 = Compute
085 = Research Packs
087 = Federation
candidate Specs 074-088
```

No material stale association remained after reconciliation. The roadmap expression `074-088 -> 089 Whole-Platform Qualification` is intentional: 089 qualifies the preceding candidate units claimed by the target release.

## Remaining uncertainty classification

No known **architecture-default gap** remains in this planning packet.

Remaining uncertainties are deliberately evidence-selected/external and have fail-safe defaults, including:

- exact deterministic/agentic browser implementation;
- speech/VAD/diarization engines;
- DataFusion gaps/secondary engine need;
- vector/index implementation;
- worker isolation technology by platform/workload;
- institutional provider/adapters;
- future federation methods;
- real PHI authority and external release gates.

These do not authorize implementation choice by preference.

## Planning verdict

```text
RESEARCH_OS_PLANNING_PACKET_REVIEW_READY=TRUE
MATERIAL_KNOWN_PLANNING_GAPS=0
RESEARCH_OS_IMPLEMENTATION_AUTHORIZED=FALSE
RESEARCH_OS_IMPLEMENTED=FALSE
RESEARCH_OS_RELEASE_READY=FALSE
REAL_PHI_AUTHORIZED_BY_THIS_PACKET=FALSE
```

Before any candidate unit is implemented, canonical governance must reverify live main/frontier, promote exactly one bounded unit, bind exact current paths/types/tests, and preserve the normal Spec Kit/evidence process.
