# Clarifications: Spec 021

| ID | Topic | Resolution |
|---|---|---|
| C1 | CLI shape | Single `medscale journey run` that executes the documented synthetic sequence; individual ops remain available. |
| C2 | Accept default | Happy-path journey Accepts the previewed Patient proposal; Reject path covered by unit/facade test. |
| C3 | Disclosure storage | Append-only via durable audit trail with typed DisclosureRecord detail. |
| C4 | Reopen proof | Integration test close → new process → reopen → same source digest / assertion presence. |
| C5 | Release claim | Explicitly forbidden; doctor `release_ready=false`. |
| C6 | PHI | Synthetic fixtures only; REAL_PHI unauthorized. |
