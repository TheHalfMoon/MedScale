# OpenMed / OSS absorption dispositions (Spec 007)

**Baseline**: OpenMed `v2.2.0` / `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837`  
**Rule**: No wholesale OpenMed fork; no Python trusted core; provenance before bytes.

## OpenMed capability families

| disposition_id | capability_family | operation | owning_spec | placement | forbidden |
|---|---|---|---|---|---|
| om-term-grounding | Terminology candidate/grounding contracts | COPY_BOUNDED tests + PORT_TO_RUST | 007/011 | P1 | Licensed tables; hidden authority |
| om-term-snapshot | Terminology snapshot/cache/provenance | COPY_BOUNDED ideas + PORT_TO_RUST | 007/011 | P1 | Table payloads from OpenMed |
| om-term-rank | Terminology conflict/ranking/calibration | COPY_BOUNDED tests + PORT_TO_RUST | 007/011 | P1 | Silent clinical conclusions |
| om-term-fhir | Terminology binding to FHIR | REFERENCE_ONLY + selected tests | 011/013 | N/A | Canonical DB replacement |
| om-grounding-audit | Grounding/audit provenance | COPY_BOUNDED patterns + PORT_TO_RUST | 007/011 | P1 | PHI in logs |
| om-fhir-r4 | FHIR R4 / profile / integrity / SDC | COPY_BOUNDED synthetic tests + port/reference | 007/013 | P1 | Canonical DB model |
| om-fhir-omop | FHIR→OMOP 5.4 | REFERENCE_ONLY | 016+ | N/A | Authority projection |
| om-docs-mime | Document MIME/quarantine/office formats | COPY_BOUNDED fixtures/policy | 007/010 | P1 | Unsandboxed parse |
| om-pii-ner-deid | PII/NER/de-ID eval taxonomy | COPY_BOUNDED tests/rules; models via Pack | 007/008 | P1 | Python runtime import |
| om-sdc-privacy | Structured privacy / SDC | COPY_BOUNDED tests + port selected | 007/011/015 | P1 | Silent legal conclusions |
| om-service-mcp | Service / MCP / GraphQL / mTLS | REFERENCE_ONLY | 013/014+ | N/A | Service plane as Network Broker |
| om-unicode | Unicode/offset projection | COPY_BOUNDED tests + PORT_TO_RUST | 002/010 | P0/P1 | Untagged coordinates |
| om-model-cache | Local model cache quota | REFERENCE_ONLY + selected tests | 008 | N/A | Ambient network for packs |
| om-timeline | Timeline provenance | COPY_BOUNDED value-free tests | 004/007 | N/A | Weaker than MedScale source timeline |
| om-cpu-int8 | CPU INT8 / perf ideas | REFERENCE_ONLY | 007/008 | N/A | Python runtime inherit |
| om-hardneg | Hard-negative / multilingual traps | COPY_BOUNDED synthetic gen ideas | 007 | N/A | REAL_PHI traps |
| om-release | Release evidence / promotion pointers | REFERENCE_ONLY + port lifecycle | 007/008/015 | N/A | Unsigned pack promotion |
| om-mobile-privacy | Swift/Android privacy patterns | REFERENCE_ONLY; port tests | 007/009 | N/A | Tauri/WebView without privacy gate |

All rows: `provenance_required_before_bytes=true` when COPY/VENDOR; `status=dispositioned`.

## Related OSS donors

| Donor | Disposition | Spec | Notes |
|---|---|---|---|
| Presidio | REFERENCE_ONLY + selective ABSORB rules/tests | 007/010 | No authority/service architecture import |
| medSpaCy | REFERENCE_ONLY + selective ABSORB | 010/011 | |
| scispaCy / QuickUMLS | REFERENCE_ONLY | 007/011 | |
| SNOMED/LOINC/UCUM/ICD/ATC/UMLS | STANDARD/PACK INPUT; licensing track | 007→011/013 | Never copy tables from OpenMed |
| Snowstorm | REFERENCE_ONLY / optional adapter later | 007/011/013 | No local terminology server required in V2 |

## Explicit DO_NOT_COPY

1. Entire OpenMed repository as MedScale tree  
2. OpenMed Python clinical domain model into trusted core  
3. Restricted terminology payloads from OpenMed  
4. OpenMed service plane as Network Broker  
5. Treating OpenMed/model output as ClinicalAssertion  

## Placement handoff (hostile / model paths)

| Path | Placement | Owning consume spec |
|---|---|---|
| Model inference workers | P1 confined | 008 |
| Hostile document workers | P1 sandbox | 010 |
| Network Broker (already Spec 013) | Core Host only | 013 |

No runtime import of OpenMed is authorized by this disposition map.
