# Research: Spec 021 Minimum Lovable Workflow

## Decisions

### D1 — Compose existing facade (no new host)
- **Decision**: Journey is a documented sequence over CoreFacade / CliSession; no second authority path.
- **Rationale**: Constitution one Rust authority path; Q07 asks for shared facade.

### D2 — Disclosure as append-only audit detail
- **Decision**: `DisclosureRecord` is a typed contract persisted via `AppendDisclosure` as an ActionAuditRecord (`action=disclosure.append`) with structured `detail`.
- **Rationale**: Avoids new object-class migration risk while remaining durable across reopen (Spec 016 snapshot).

### D3 — Preview before accept
- **Decision**: After ingest, create a Proposal + loss-aware export inventory as preview; Promote only on Accept; Reject writes `proposal.reject` audit without promote.
- **Rationale**: Matches MINIMUM_LOVABLE_TRUSTED_MEDSCALE import→preview→accept loop; preserves Proposal != ClinicalAssertion.

### D4 — Two-process reopen in journey test
- **Decision**: Reuse Spec 016 child-process pattern to verify source/assertion identity after close.
- **Rationale**: Q07 explicitly requires restartable workflow; 016 already proved the primitive.

### D5 — Honesty axes
- **Decision**: `WorkflowDoctorStatus { workflow_ready_base: true, release_ready: false }`.
- **Rationale**: Completing a synthetic journey is not product/privacy release readiness.

## Rejected

- New durable Disclosure table / object class (unnecessary for READY_BASE).
- Desktop/v0 visual journey as this unit’s deliverable (UI external/visual gate).
- Claiming RELEASE_READY or PRIVATE_DATA_READY from journey PASS.
