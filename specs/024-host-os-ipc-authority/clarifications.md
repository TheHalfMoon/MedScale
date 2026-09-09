# Clarifications — Spec 024

| ID | Question | Resolution |
|----|----------|------------|
| C1 | Strict default vs opt-in? | Fail-closed Strict default on `CoreFacade::new()` and always on IPC host. |
| C2 | Keep lease-only path? | Only via `CoreFacade::new_legacy_lease_only_engineering()` for in-process suite continuity; not the qualified IPC path. |
| C3 | Which caps need session? | Mutating caps only; bootstrap `Acquire/Release/Open/RevokeSession` exempt; read-only presentation/list/ping exempt. |
| C4 | Transport crate? | `interprocess` 2.x local sockets (pipe/UDS); no localhost TCP claim. |
| C5 | MULTI_CLIENT_RELEASE_READY? | Always false in this unit. |
| C6 | Deferred advanced? | Renumber deferred bucket to **025+** (plugins/GraphRAG/etc.). |
