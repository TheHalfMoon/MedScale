# Spec 016 evidence summary

**Status**: Implementation + qualification on branch `spec/016-durable-trusted-record`  
**Limitations (binding)**:
- Does **not** establish `PRIVATE_DATA_READY`
- Open working SQLite metadata may be plaintext while unlocked
- Spec 005 sealed-at-close remains closed-vault encryption posture
- Q03 must qualify WAL/temp/crash/open-vault confidentiality
- Q04 must qualify authenticated multi-client authority beyond OS writer lock
- `SUPERIORITY_OVER_OPENMED = UNMEASURED`
- `RELEASE_READY = FALSE`

## Delivered

- Schema v2: `authority_objects`, `store_state`, journaled migration
- `WriterLock` via exclusive SQLite lock DB (not lock-file presence)
- `CoreFacade` load-on-open + persist-after-dispatch when vault open
- Backup manifest schema_version 2 includes objects + next_seq
- Tests: two-process restart, writer contention, corrupt digest refuse, same-process reopen

## Commands (local)

```text
cargo test --workspace
cargo test -p medscale-core --test durable_restart_016
```

Record exact commit/tree after merge on main.
