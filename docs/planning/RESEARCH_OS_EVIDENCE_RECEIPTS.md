# MedScale Research OS Evidence Receipts

**Status:** Planning candidate.

Future subsystems should converge on composable typed receipts rather than prose logs alone.

Candidate receipts:

- `RunReceipt` — model/agent run identity and outcome;
- `ToolReceipt` — tool invocation, capability and bounded result;
- `BrowseReceipt` — web/source retrieval with URL/time/hash/evidence spans;
- `QueryReceipt` — analytical query/plan/input revisions/result identity;
- `RetrievalReceipt` — retrieval plan/index revisions/sources/ranking facts;
- `DeidReceipt` — privacy transformation policy, detectors, mappings and residual state;
- `ComputeReceipt` — worker/runtime/environment/resource/result identity;
- `AudioRouteReceipt` — selected speech route and explicit routing reasons;
- `TranscriptReceipt` — source audio, engine/model, transcript revision and alignment;
- `ApprovalReceipt` — human/service actor, target, scope and decision;
- `ExportReceipt` — exact exported artifacts/revisions and privacy/policy decision.

Receipts are evidence artifacts, not a substitute for canonical data. They should be machine-readable, inspectable in Desktop/CLI, and linkable through the Project Graph.
