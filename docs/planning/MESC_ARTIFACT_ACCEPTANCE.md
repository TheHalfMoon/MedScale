# MESC artifact acceptance contract

Status: DESIGN_ONLY; Spec 012 remains BLOCKED_BY_RELEASED_MESC_ARTIFACT.
MESC is an independent upstream. MedScale does not edit its repository, import training code,
share authority or treat a tag as a distributable model. Observed 2026-09-09: v0.1.0 release
has no assets; a v0.2.0 tag does not satisfy the released-artifact gate.

## Future verifier input

The owning follow-on spec must version a manifest containing artifact identifier/version,
immutable upstream release/revision, byte length and digest for every file, format/runtime
compatibility, model/tokenizer/config dependencies, publisher identity and verifiable signature,
rights/license notices with redistribution and intended-use constraints, SBOM/native closure,
model card, evaluation dataset provenance and result digests, resource limits and supported
platforms. Evidence URLs alone are insufficient: retrieve under broker policy, pin content and
verify it. Existing request fields are a stub contract, not this complete verifier.

## Admission state machine

`DISCOVERED -> QUARANTINED -> VERIFIED -> QUALIFIED -> INSTALLED_DISABLED -> ENABLED`.
Any missing, malformed, expired, incompatible, untrusted or revoked prerequisite transitions
to `REJECTED` with a stable reason code. A interrupted download remains quarantined. Never
execute files to determine whether they are trustworthy. Digest checks establish byte identity,
not publisher trust, rights, quality or clinical safety.

Verification order: bounded manifest parsing; trust/signature policy; size/digest checks;
rights and SBOM completeness; compatible format/resource limits; qualified sandbox/runtime;
owned synthetic smoke/evaluation fixtures; atomic installation; explicit local enablement.
No shell installers, arbitrary hooks, ambient network, patient-data upload or database access.
Model output is a proposal with model/prompt/input lineage and uncertainty, never a canonical
assertion without the existing promotion path. Core record workflows work without this Pack.

## Acceptance evidence and recovery

Use synthetic manifests for missing asset, mismatched digest, bad signature, unknown signer,
expired metadata, rollback, unsupported format, oversized archive, missing rights/SBOM/evaluation,
interrupted installation and disabled network cases. Assert no canonical mutation or execution
on rejection. Test uninstall, revocation and restoration to the previous qualified version;
retain historical output provenance without retaining executable revoked assets unnecessarily.
Publisher key rotation and rollback policy must be explicit, not inferred from semantic version.

Gate clearance requires an actual immutable released asset and independently recorded verifier,
rights and platform evidence. Engineering preparation can proceed now; actual artifact admission
cannot. No upstream availability date or performance result is assumed.
