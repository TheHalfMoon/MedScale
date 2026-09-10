# MESC artifact acceptance contract

Status: Spec **012** remains `BLOCKED_BY_RELEASED_MESC_ARTIFACT`. Spec **036** delivers MedScale
synthetic verifier **READY_BASE**. MESC is an independent upstream. MedScale does not edit its
repository, import training code, share authority, or treat a tag as a distributable model.
Observed historically: v0.1.0 release has no assets; a v0.2.0 tag does not satisfy the
released-artifact gate.

## Verifier input (Spec 036 READY_BASE)

MedScale verifies a local release directory against `manifest.json` (`MescReleaseManifestV0`,
`schema_version=1`, deny unknown fields) containing: producer/release/tag/source commit/tree;
model/tokenizer/base-model/corpus identities; training and evaluation receipt digests; SBOM
path+digest; rights license + notice path; provenance note; limitations; runtime requirements;
monotonic epoch; and artifact enumeration (kind, relative path, byte length, sha256).

`Capability::MescArtifactVerify` returns `MescVerifyReport`. Successful synthetic verify yields
state `Verified` with **`product_admit_authorized=false`**. Digests establish byte identity only —
not publisher trust, rights to redistribute, quality, or clinical safety. Model bytes are never
executed to decide trustworthiness.

Reject reasons are stable (`MescVerifyReason`): missing/malformed/unsupported schema, duplicate
paths, missing fields/files, size/digest mismatch, missing rights/SBOM/evaluation/training
receipts, anti-rollback, replay. In-process `MescEpochStore` records producer→epoch after a
successful verify.

## Admission state machine

`DISCOVERED -> QUARANTINED -> VERIFIED -> QUALIFIED -> INSTALLED_DISABLED -> ENABLED`.
Spec 036 covers synthetic discovery→verify only. Product admit remains
`ExternalGateRequired(MESC_RELEASED_ARTIFACT)`. Any missing/malformed prerequisite → `REJECTED`.

Remaining before gate clearance: real immutable upstream assets; publisher trust/signature
policy; brokered retrieval; rights/SBOM/evaluation for that release; qualified sandbox/runtime;
owned smoke fixtures; atomic install + explicit enable. No shell installers, ambient network,
patient-data upload, or database access. Model output remains a proposal.

## Acceptance evidence and recovery

Synthetic fixtures live under `evidence/012-mesc-artifact-integration/fixtures/` (good +
adversarial). Assert no product admit authorization on success or rejection. Gate clearance still
requires an actual immutable released asset and independently recorded verifier/rights/platform
evidence. Engineering preparation for verify is READY_BASE; artifact admission is not.

External residual after Spec 036:

```text
BLOCKED_BY_UPSTREAM_MESC_RELEASE_ASSETS
```
