# Feature Specification: Authenticated Host / Client Authority (Q04)

**Feature Branch**: `spec/018-host-client-authority`  
**Created**: 2026-09-09  
**Status**: READY for specify/plan completion; runtime NOT_QUALIFIED  
**Depends on**: Spec 016 writer lock, Spec 017 privacy honesty, Specs 002/006 process topology  

## Intent

Qualify authenticated peer/session, scoped capabilities, lease expiry/revocation, and OS-exclusive writer ownership before untrusted UI/workers. Do not confuse logical policy structs with enforced runtime isolation.

## In scope (planned)

- Authenticated session identity for local IPC clients
- Capability grants bound to caller + vault + scope
- Lease/session expiry and revocation
- Enforce no ambient DB/keys/filesystem/network to workers
- Keep one Core Host writer (builds on Spec 016 lock)

## Out of scope

Remote multi-tenant auth; REAL_PHI; MESC; full desktop UI; Q06 semantics.

## Next

Complete clarify/plan/ADR/tasks before implementation. Q02 lock is not Q04 completion.
