# Clarifications: Spec 054

- Format: CycloneDX 1.5 JSON (already used by scaffold scripts; industry
  standard; deterministic subset). SPDX document format NOT introduced; public
  project license decision stays external.
- `SBOM_DOCUMENT_FORMAT = CycloneDX-1.5` vs `PUBLIC_PROJECT_LICENSE = UNDECIDED
  (external legal decision)`. Generator records per-component license ids from
  cargo metadata only; unknown → `NOASSERTION`.
- Reproducibility: `REPRODUCIBLE_BUILD = UNPROVEN` recorded in doc properties.
- Native deps that cannot be enumerated on host: explicitly classified
  (`host-tool`, `os-provided`, `not-applicable`) rather than omitted silently.
- The committed `SBOM.cdx.json` is a READY_BASE qualification artifact bound to
  the merge HEAD; any source/tree/lock change invalidates it by design
  (verifier rejects stale digests).
