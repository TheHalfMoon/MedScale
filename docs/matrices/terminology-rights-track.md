# Terminology rights track (Spec 007)

**copy_from_openmed**: always `false`  
**Snowstorm**: REFERENCE_ONLY — no local terminology server requirement in V2  
**Counsel**: EXTERNAL_GATES rows OPEN; non-blocking for Spec 007 research close

## Systems

| system_id | display_name | rights_class | pack_metadata_required | owning_consume_specs | external_gate_id | status |
|---|---|---|---|---|---|---|
| snomed-ct | SNOMED CT | gated_license | version, checksum, rights_uri, notice, expiry_or_revocation | 011, 013 | TERMINOLOGY_SNOMED_LICENSE | track_started |
| loinc | LOINC | gated_license | version, checksum, rights_uri, notice, expiry_or_revocation | 011, 013 | TERMINOLOGY_LOINC_LICENSE | track_started |
| ucum | UCUM | open | version, checksum, rights_uri, notice | 004, 011 | — | track_started |
| icd | ICD | gated_license | version, checksum, rights_uri, notice, expiry_or_revocation | 011 | TERMINOLOGY_ICD_LICENSE | track_started |
| atc | ATC | gated_license | version, checksum, rights_uri, notice, expiry_or_revocation | 011 | TERMINOLOGY_ATC_LICENSE | track_started |
| umls-athena | UMLS / Athena | gated_license | version, checksum, rights_uri, notice, expiry_or_revocation | 011 | TERMINOLOGY_UMLS_LICENSE | track_started |

## Pack metadata field list

Required on every terminology Pack admission:

1. `version`  
2. `checksum` (content identity)  
3. `rights_uri`  
4. `NOTICE` / attribution blob  
5. `expiry_or_revocation` (when gated)  
6. `source_distributor`  
7. `copy_from_openmed=false` assertion  

## Never-copy-from-OpenMed

MedScale must not copy terminology tables, snapshots, or restricted payloads from OpenMed. Code/test permission ≠ terminology redistribution permission.
