# MedScale Research OS Terminology

**Status:** Planning candidate.

- **MedScale Core** — trusted authority plane.
- **Project** — principal research/work context and ownership boundary.
- **Experiment** — structured scientific unit within a Project.
- **Artifact** — versioned/provenance-bearing project object.
- **Project Graph** — relationships among canonical artifacts and derived references.
- **MedAgent** — governed AI/agent workbench operating under capabilities.
- **Agent Lane** — model/runtime/role/tools/policy configuration used in a Run.
- **FleetRun** — coordinated multi-lane run with independent results.
- **Privacy Gate** — cross-boundary data classification/transformation/enforcement layer.
- **Analytics Gate** — governed analytical workspace/execution layer.
- **AudioFlow** — audio/voice intelligence subsystem; the UI route is `Audio`.
- **Audio Pack** — governed executable speech/audio model/runtime package.
- **MedScale Hub** — optional user-controlled collaboration/sync service.
- **MedScale Compute** — bounded job execution plane for local/lab/institutional workers.
- **Research Pack** — versioned domain extension adding scientific schemas/workflows without becoming a new authority plane.
- **Room** — collaboration stream bound to Project work or an artifact.
- **EvidenceRef** — provenance-bearing link to exact source evidence.
- **Receipt** — structured evidence of an action/run/query/retrieval/privacy/browse decision.

Avoid introducing donor-specific nouns into the product vocabulary unless a future ADR deliberately selects them.
