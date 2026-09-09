# Spec 018 LIMITATIONS

- `MULTI_CLIENT_RELEASE_READY = FALSE`
- `os_ipc_qualified = FALSE` — sessions are in-process only; OS IPC transport not qualified
- READY_BASE is **opt-in**: existing mutating authority calls without `session_id` are not forced to present a session (legacy lease-only path remains valid)
- Lock-file / Spec 016 writer lease is not session proof; sessions are a separate capability grant layer
- Workers still receive no ambient DB/keys/filesystem/network; no worker runtime shipped here
- Remote auth, hostile UI sandbox, and Q06 multi-client semantics are out of scope
