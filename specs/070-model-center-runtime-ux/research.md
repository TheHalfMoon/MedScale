# Research — Spec 070

## Existing authority seam

`Capability::PacksList` is already read-only under Strict session enforcement and returns `Vec<PackManifestV0>`. `CliSession::packs_list` and `CliSession::packs_install_local` keep Desktop on the Core authority path. No Desktop dependency on `medscale-pack` is required.

## Persistence truth

`specs/016-durable-trusted-record/contracts/field-mapping.md` explicitly records leases, allowlist and packs as `not persisted in 016` with `process lifetime`. Spec 070 therefore exposes session-local inventory and must not label it restart-persistent installation.

## Provenance truth

`PackManifestV0` directly exposes content digest, rights URI, SBOM ref, runtime requirements, benchmark links, promotion state and trust root. Exact Hugging Face repository/revision provenance lives in the signed model metadata artifact introduced by Spec 069, so Model Center may show exact external revision only for the pinned qualification reference where repository evidence proves it. Generic admitted Pack rows must not invent a source repository or revision.

## Product boundary

Local Pack admission is an operator action, not model authority. Qualification references are evidence, not installed state. Runtime results remain evidence/proposal-only and real PHI remains denied.
