# MedScale CLI Product Guide

`medscale` is a first-class local client of the same Rust authority path used by Desktop. It has no direct database, key, or network privilege.

## Start here

```text
medscale status
medscale status --json
medscale capabilities
medscale capabilities --json
medscale doctor --json
```

## Patient reads

```text
medscale patient show --vault-id demo --subject synthetic-subject --json
medscale patient timeline --vault-id demo --subject synthetic-subject --json
medscale patient brief --vault-id demo --subject synthetic-subject --json
medscale patient coverage --vault-id demo --subject synthetic-subject --json
```

Legacy top-level `timeline`, `brief`, and `coverage` paths remain compatible.

## Authority and audit reads

```text
medscale actions outbox --vault-id demo --json
medscale audit disclosures --vault-id demo --json
medscale fhir support --vault-id demo --json
```

Outbox and disclosure commands are read-only. `UNKNOWN` action state is not a retry authorization. FHIR support is a narrow support matrix, not a full-conformance claim.

## Vault, Packs, journey, and Host IPC

Use `medscale vault --help`, `medscale packs --help`, `medscale journey --help`, and `medscale host-ipc --help` for mutation and persistent-host workflows. Passphrases are read from a named environment variable, never argv.

The current CLI may own a transient in-process Core Host when no persistent host is used. A transient command does not imply a long-running shared-vault session. Use Host IPC when operating an explicitly running Core Host.

## Privacy / release boundary

Current product scope is synthetic/permitted development data only. `REAL_PHI` is not authorized. JSON/text output must not contain secrets. `RELEASE_READY`, `PRIVATE_DATA_READY`, `MULTI_CLIENT_RELEASE_READY`, full FHIR conformance, and WCAG conformance remain false unless separately evidenced.
