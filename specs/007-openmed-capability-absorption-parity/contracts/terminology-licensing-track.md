# Contract: Terminology / Licensing Track Start

**Spec**: 007  
**Status**: Track started (research)  
**Related**: SOURCE_ACQUISITION §5, OSS matrix terminology rows, EXTERNAL_GATES

## Purpose

Begin MedScale’s terminology rights/version/checksum discipline so Spec 011/013 can consume Packs without assuming bundled licensed content or copying OpenMed tables.

## Minimum systems in track

| system_id | Rights class (initial) | copy_from_openmed | Consume specs |
|---|---|---|---|
| `snomed-ct` | `gated_license` / `caller_supplied` | false | 011, 013, 014 |
| `loinc` | `gated_license` / `caller_supplied` (confirm) | false | 011, 013 |
| `ucum` | `open` (confirm NOTICE) | false | 011, 013 |
| `icd` | `gated_license` / `caller_supplied` | false | 011, 013, 014 |
| `atc` | `unknown_pending_counsel` | false | 011 |
| `umls-athena` | `gated_license` / `caller_supplied` | false | 011 |

Initial classes are **research defaults**; counsel may reclassify via EXTERNAL_GATES without reopening Spec 007 architecture.

## Pack metadata required (schema fields)

```text
system_id
version
checksum_sha256
rights_uri
notice_text_or_path
license_spdx_or_label
expiry_or_refresh_policy
revocation_hook
source_distributor
caller_supplied: bool
```

## Rules

1. Licensed/restricted content is **caller-supplied or separately licensed**—never assumed bundled in core.
2. **Never** copy terminology tables from OpenMed.
3. OpenMed grounding/snapshot **contracts and tests** may be REFERENCE / COPY_BOUNDED; payloads are not.
4. Missing counsel acceptance → EXTERNAL_GATES row; Spec 007 research may still close with `track_started`.
5. Local core MUST NOT require a terminology server (Snowstorm REFERENCE only).

## EXTERNAL_GATES hooks (to record at research implement)

Suggested ids (create if absent):

- `TERMINOLOGY_SNOMED_LICENSE_ACCEPTANCE`
- `TERMINOLOGY_LOINC_LICENSE_ACCEPTANCE`
- `TERMINOLOGY_ICD_LICENSE_ACCEPTANCE`
- `TERMINOLOGY_UMLS_ATHENA_LICENSE_ACCEPTANCE`

Status: `OPEN` / `NOT_REQUIRED_IF_CALLER_SUPPLIED` as applicable—do not block unrelated specs.

## Evidence path

`docs/matrices/terminology-rights-track.md` + EXTERNAL_GATES updates.
