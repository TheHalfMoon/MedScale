# Plan: Spec 021 Minimum Lovable Trusted Workflow

1. Contracts: `DisclosureRecord`, import preview, `WorkflowDoctorStatus`, journey report/step types, CLI JSON error envelope.
2. Facade: `RejectProposal`, `AppendDisclosure`, `ListDisclosures`; preview composed from existing ingest/proposal/export.
3. Core: `workflow::run_minimum_lovable_journey` composing existing CoreFacade ops (no new architecture).
4. CLI: `medscale journey run` with documented synthetic end-to-end path; stable exit codes + `--json` errors.
5. Tests: CoreFacade journey integration including close/reopen two-process verification.
6. Doctor axis + notes honesty (`workflow_ready_base` / `release_ready=false`).
7. Spec package + evidence + BUILD_QUEUE + SPECKIT_MASTER_ROADMAP_V2.
8. Gates: `cargo fmt`; clippy `-D warnings`; `cargo test --workspace` with `CARGO_TARGET_DIR=D:\medscale-target`.

## Architecture

- Prefer composition of Spec 003–020 facade capabilities over new stores or IPC.
- Disclosure append uses durable authority audit trail (append-only ActionAuditRecord with typed detail).
- Preview never promotes; Accept uses PromoteProposal; Reject records non-authority rejection audit.
- Close/reopen reuses Spec 016 vault durability; journey test includes two-process reopen.
- RELEASE_READY remains false everywhere.
