# Research — Spec 024 Host OS IPC

## Spec 018 residual

`evidence/018-host-client-authority/LIMITATIONS.md`: `os_ipc_qualified=false`; sessions in-process; mutating calls still allow legacy lease-only without `session_id`. TRUSTED_V1_DELIVERY_PLAN Q04 P0 residual after 018 READY_BASE.

## Transport choice

- Prefer OS-local IPC over TCP localhost (no remote auth surface).
- `interprocess` 2.4.x: stable mapping Windows→named pipe, Unix→UDS; zero framing of its own (we add length-prefix).
- License: 0BSD OR Apache-2.0 — admissible DEPENDENCY (pin exact version in Cargo.lock).

## Writer lock reuse

Spec 016 `WriterLock` (`BEGIN EXCLUSIVE` on `writer.lock.sqlite3`) already proves OS-process exclusivity. IPC host binds that lock to the serve lifetime.

## Session model

Reuse Spec 018 `SessionRegistry` / `OpenSession` / `RevokeSession`. No new remote identity.
