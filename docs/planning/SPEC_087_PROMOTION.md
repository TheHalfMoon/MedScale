# Spec 087 Promotion — Community Extensions (declarative foundation)

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`
**Promotion date:** PROMOTION_DATE_PENDING
**Canonical base:** BASE_SHA_PENDING (Spec 086 closure merge)
**Target branch:** `spec/087-community-extensions`

## Authority

The founder's standing continuation directive requires promoting the next
dependency-ready Research OS unit after each closure without routine
approval. `IMPLEMENTATION_AUTHORITY.md` remains active; this spec admits
no dependency.

Dependency proof: `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md` and
`RESEARCH_OS_EXECUTION_ROADMAP.md` number Community Extensions **087**,
hard dependency **079 + 084 + 085** (075 for data-source extensions; 080
for extensions requesting browser capabilities). All are
`CLOSED_CANONICAL`. 088 (081 + 084) is also dependency-ready; the strict
numeric-order convention selects 087.

Sandbox truth: `platform_qualified=false`; Spec 085 admits closed MedScale
job kinds only; Spec 086 refuses managed R. No WASM runtime is admitted.

## The execution boundary decision

The authority requires executable extension code to run "in a qualified
WASM sandbox or isolated worker" and forbids native library injection.
Neither a WASM runtime (a new dependency needing admission) nor a
platform-qualified isolated worker for untrusted code exists. Therefore
this slice admits **declarative extensions only**: a signed manifest whose
commands name operations of a closed, versioned, read-only Host API that
Core itself executes. No extension code is loaded or run. Executable
extensions (WASM or worker) remain a recorded gate requiring their own
dependency and security admission.

```text
pack bytes != verified release != install (per Project)
  != grant (per capability, data-class ceiling)
  != invocation (Core runs the mapped operation)
  != result (read-only; no external effect)
```

## Authorized scope

- Contracts (`medscale-contracts/src/extensions.rs`): `ExtensionManifest`
  (identity, version, publisher id and key, Host API range, license,
  description, entrypoint `declarative`, capabilities, commands),
  `ExtensionPack`, `ExtensionPublisher`, `ExtensionRelease`,
  `ExtensionInstallRecord`, `ExtensionGrant`, `ExtensionLifecycleReceipt`,
  `ExtensionRuntimeReceipt`, `HostOperation` (Host API v1:
  `project_summary`, `snapshot_schema`, `snapshot_rows`), closed
  capability, state, refusal and denial vocabularies.
- Verification: pack size bound, canonical manifest bytes, closed
  vocabularies (unknown capability or entrypoint fails closed), publisher
  explicitly trusted by the user (no built-in trust root), key match,
  strict ed25519 signature over a domain-separated digest, Host API
  compatibility, release not revoked.
- Lifecycle per Project: install (no grants), upgrade (strictly higher
  version, same publisher; capability expansion forces `pending_consent`),
  rollback to the previous release, enable (also re-consent) / disable,
  uninstall (revokes grants), grant / revoke a declared capability with a
  data-class ceiling; publisher and release revocation quarantine every
  affected install atomically. Every attempt leaves a receipt.
- Invocation: enabled install, trusted publisher, unrevoked release,
  declared command, active grant, target in the Project and within the
  ceiling (unclassified data is `local_phi`); bounded results; a runtime
  receipt for every call.
- SDK helper to build and sign a pack; storage v16 (additive),
  backup/restore, consistency; Core, facade, `CliSession`, CLI
  `medscale extension ...`.

## Explicitly not authorized

- Executing extension code of any kind (WASM, JS, Python, native
  libraries, shell, scripts); write or network Host API operations.
- A Hub-hosted registry or marketplace, automatic updates, self-update.
- Any new dependency; any built-in or remote trust root.
- Real PHI, release claims.

Recorded residuals (expected): no Hub registry distribution or review
tiers; licenses are declared, not verified; no SBOM scanning of
declarative packs (they contain no code); project metadata is treated as
`local_phi`; no Desktop surface.

## Completion rule

`CLOSED_CANONICAL` only after merge on a green exact head and recorded
post-main verification.
