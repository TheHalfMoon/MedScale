# MedScale Research OS Final Planning Assertions V2

These assertions summarize the amended planning packet without promoting implementation.

1. MedScale scales to laboratories and institutions only if Project/Artifact/Capability semantics are stabilized before adding feature islands.
2. **Data Source Fabric is a first-class platform plane.** Local files, databases, Kaggle, Hugging Face datasets and later institutional sources use one governed source/credential/snapshot/receipt model.
3. Reproducible research binds to immutable `DataSnapshot` inputs whenever possible; mutable live-source use must be labeled as partially reproducible rather than silently treated as frozen evidence.
4. Data Sources are a product surface, not an Analytics implementation detail. Analytics, Knowledge, R, Compute and Extensions consume the same admitted source/snapshot contracts.
5. MedAgent is a governed intelligence workbench; multi-model Fleet comparison reports observable evidence and uncertainty rather than winner scores.
6. Privacy Gate sits across model, data-source, Browse, export, Hub, Compute, R, connector and extension boundaries rather than existing as one de-identification screen.
7. Governed Browse is independently owned because web research introduces egress, credential, SSRF/redirect, hostile-content, download-quarantine and provenance risks.
8. AudioFlow is first-class because capture, STT, diarization, voice control, meetings, TTS and audio evidence share governance/runtime concerns.
9. DataFusion/Arrow remains a strong qualification target for native analytics; BI/RAG servers remain optional adapters unless evidence proves otherwise.
10. **R Workspace is first-class research integration** but unrestricted R does not enter the trusted Desktop/Core. Exact snapshots are staged to external RStudio/Positron or Compute-mediated R jobs, and outputs return only through explicit validation/publication with `RRunReceipt`.
11. **Community Extensions are a first-class ecosystem** but not an ambient-trust plugin model. Extension Packs are signed/provenance-bound, capability-scoped, sandboxed/isolated, versioned, revocable and independently consented on capability changes.
12. The Community Registry improves discoverability/developer adoption but never auto-installs, auto-grants or becomes client authority. Offline/manual verified Extension Pack installation remains valid.
13. Research Packs and Extension Packs are different: Research Packs describe domain semantics; executable/integration behavior uses Extension/Compute authority.
14. A community data-source connector implements the same Data Source Fabric contract and cannot invent its own source authority, credential model or snapshot semantics.
15. Buzz, VoiceStudio, Himsat, Obsidian, Kaggle/HF client ecosystems, Posit/R tooling, Wasmtime/Extism and other donors/references are consumed selectively behind MedScale-owned contracts and exact provenance/rights/security review.
16. Hub and Compute scale collaboration/execution. Neither widens Core authority by default.
17. Personal/offline operation remains a valid product topology at every generation; Hub/community/cloud/provider access is never mandatory for core local workflows.
18. Already-promoted Spec 074 remains Project + Artifact Graph only. The amendment does not expand PR #122.
19. The next future candidate after canonical 074 closure is Data Source Fabric only if a separate live promotion authorizes it.
20. The V2 amendment has product plans, dependency roadmap, 50 explicit decision defaults, shared implementation contracts, per-spec contracts through 092, repository mapping, source-adoption rules, subsystem verification campaigns and a completed cross-plane/stale-numbering gap audit.
21. Historical V1 candidate numbers remain useful only as background where compatible; Program Amendment 001 and the V2 authority chain control future 075+ planning.
22. Technology choices intentionally left evidence-selected have explicit owners, qualification requirements and fail-safe defaults; they are not delegated to implementer preference.

```text
RESEARCH_OS_ORIGINAL_V1_PACKET_MERGED = TRUE
RESEARCH_OS_V2_AMENDMENT_REVIEW_READY = TRUE
RESEARCH_OS_CANDIDATE_RANGE = 074-092
PROJECT_ARTIFACT_FOUNDATION_OWNING_CANDIDATE = 074
DATA_SOURCE_FABRIC_OWNING_CANDIDATE = 075
COLLABORATION_OWNING_CANDIDATE = 076
MEDAGENT_OWNING_CANDIDATE = 077
MODEL_FLEET_OWNING_CANDIDATE = 078
PRIVACY_GATE_OWNING_CANDIDATE = 079
GOVERNED_BROWSE_OWNING_CANDIDATE = 080
AUDIOFLOW_FOUNDATION_OWNING_CANDIDATE = 081
ANALYTICS_GATE_OWNING_CANDIDATE = 082
KNOWLEDGE_CANVAS_OWNING_CANDIDATE = 083
MEDSCALE_HUB_OWNING_CANDIDATE = 084
MEDSCALE_COMPUTE_OWNING_CANDIDATE = 085
R_WORKSPACE_OWNING_CANDIDATE = 086
COMMUNITY_EXTENSIONS_OWNING_CANDIDATE = 087
AUDIOFLOW_ADVANCED_OWNING_CANDIDATE = 088
RESEARCH_PACKS_OWNING_CANDIDATE = 089
INSTITUTIONAL_ADAPTERS_OWNING_CANDIDATE = 090
FEDERATION_OWNING_CANDIDATE = 091
WHOLE_PLATFORM_QUALIFICATION_OWNING_CANDIDATE = 092
MATERIAL_KNOWN_V2_PLANNING_GAPS = 0
SPEC_074_IMPLEMENTATION_AUTHORIZED = TRUE
SPEC_074_SCOPE_CHANGED_BY_V2 = FALSE
SPEC_075_PLUS_IMPLEMENTATION_AUTHORIZED = FALSE
RESEARCH_OS_V2_IMPLEMENTED = FALSE
RESEARCH_OS_RELEASE_READY = FALSE
REAL_PHI_AUTHORIZED_BY_PACKET = FALSE
MESC_MUTATION_AUTHORIZED_BY_PACKET = FALSE
```

`MATERIAL_KNOWN_V2_PLANNING_GAPS = 0` is a planning statement only: no presently known founder requirement in Amendment 001 lacks a planned owner, boundary, failure model, repository placement or verification path. It does not mean source adapters, R runtimes, extension sandboxes, registry services or the broader Research OS are implemented or qualified.

The amended packet may inform future canonical promotion, but it does not itself promote any candidate specification or authorize database/provider network credentials, R execution, extension runtimes, community marketplace operation, real PHI, or candidate Spec 075+ implementation.