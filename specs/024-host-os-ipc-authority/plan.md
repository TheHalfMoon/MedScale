# Plan: Spec 024 Host OS IPC Authority

1. Spec Kit package (specify/clarify/plan/research/ADR/tasks/checklist/analyze/converge).
2. Session enforcement: `SessionEnforcement::Strict` default on `CoreFacade`; mutating ops require `session_id`. Keep `new_legacy_lease_only_engineering()` for existing in-process suite escape (doctor-visible name).
3. IPC: `crates/medscale-core/src/ipc/` — length-prefixed LE u32 + JSON envelopes over `interprocess` local sockets; `HostIpcServer` holds `WriterLock` + Strict facade; `HostIpcClient` connect/dispatch.
4. Wire `CliSession` to open a client session after lease and attach `session_id` on requests.
5. Doctor: `HostAuthorityDoctorStatus::ready_base()` sets `os_ipc_qualified=true`; keep `multi_client_release_ready=false`.
6. CLI: `medscale host-ipc serve|call` for operator exercise of the transport.
7. Tests: strict deny without session; IPC round-trip open/mutate/revoke; two-client deny; second host WriterHeld.
8. Evidence `evidence/024-host-os-ipc-authority/{SUMMARY,LIMITATIONS}.md`.
9. BUILD_QUEUE / SPECKIT_MASTER_ROADMAP_V2 / START_HERE: 024 CLOSED; deferred **025+**.
10. Gates: fmt, clippy `-D warnings`, focused tests then `cargo test --workspace --locked` (`CARGO_TARGET_DIR=D:\medscale-target`).

## Architecture

```text
HostIpcClient  --(named pipe / UDS)-->  HostIpcServer
                                         |-- WriterLock (016)
                                         |-- CoreFacade (Strict sessions)
                                         `-- SessionRegistry (018)
```

Framing: `[u32 LE length][utf-8 JSON AuthorityRequest|AuthorityResponse]`.
