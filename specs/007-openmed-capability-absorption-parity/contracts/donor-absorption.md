# Contract: Donor Absorption Dispositions

**Spec**: 007  
**Status**: Canonical for research  
**Related**: SOURCE_ACQUISITION_AND_COPY_PLAN, OSS_CODE_ABSORPTION_MATRIX_V2, [data-model.md](../data-model.md)

## Global rules

1. **No wholesale OpenMed copy/fork** as MedScale product.
2. **No OpenMed Python package** as MedScale trusted runtime DEPENDENCY.
3. **Code permission ≠ weights/datasets/terminology** permission.
4. Provenance record **before** first donor-derived line (`third_party/provenance/...`).
5. Prefer tests/fixtures/deterministic algorithms; port authority semantics to Rust in owning specs.

## OpenMed family dispositions (from SOURCE §2)

| Capability family | Operation | Owning spec | Notes |
|---|---|---|---|
| Terminology candidate/grounding contracts | `COPY_BOUNDED` tests + `PORT_TO_RUST` | 007/011 | No table copy |
| Terminology snapshot/cache/provenance | `COPY_BOUNDED` ideas + `PORT_TO_RUST` | 007/011 | Checksum/rights manifests |
| Terminology conflict/ranking/calibration | `COPY_BOUNDED` tests + `PORT_TO_RUST` | 007/011 | Never hidden authority |
| Terminology binding to FHIR | `REFERENCE_ONLY` + selected tests | 011/013 | Adapter in Rust |
| Grounding/audit provenance | `COPY_BOUNDED` patterns + `PORT_TO_RUST` | 007/011 | PHI-safe envelopes |
| FHIR R4 / profile / integrity / SDC patterns | `COPY_BOUNDED` synthetic tests + port/reference | 007/013 | Not canonical DB |
| FHIR→OMOP 5.4 | `REFERENCE_ONLY` (+ selected tests later) | 016+ | Assistive only |
| Document MIME/quarantine/office formats | `COPY_BOUNDED` fixtures/policy | 007/010 | P1 workers later |
| PII/NER/de-ID eval taxonomy | `COPY_BOUNDED` tests/rules; models via Pack | 007/008 | Runtime in 008 |
| Structured privacy / SDC | `COPY_BOUNDED` tests + port selected | 007/011/015 | No silent legal conclusions |
| Service / MCP / GraphQL / mTLS patterns | `REFERENCE_ONLY` | 013/014+ | Do not import service plane |
| Unicode/offset projection | `COPY_BOUNDED` tests + `PORT_TO_RUST` | 002/010 | |
| Local model cache quota | `REFERENCE_ONLY` + selected tests | 008 | |
| Timeline provenance | `COPY_BOUNDED` value-free tests | 004/007 | MedScale supersedes |
| CPU INT8 / perf ideas | `REFERENCE_ONLY` | 007/008 | No Python runtime inherit |
| Hard-negative / multilingual traps | `COPY_BOUNDED` synthetic gen ideas | 007 | |
| Release evidence / promotion pointers | `REFERENCE_ONLY` + port lifecycle | 007/008/015 | Pack states |
| Swift/Android privacy patterns | `REFERENCE_ONLY`; port tests | 007/009 | |

## Related OSS donors (007-relevant)

| Donor | Disposition | Spec |
|---|---|---|
| Presidio | REFERENCE + selective ABSORB rules/tests | 007/010 |
| medSpaCy | REFERENCE + selective ABSORB | 010/011 |
| scispaCy / QuickUMLS | REFERENCE | 007/011 |
| SNOMED/LOINC/UCUM/ICD/ATC/UMLS | STANDARD/PACK INPUT; licensing starts 007 | 007→011/013 |
| Snowstorm | REFERENCE / optional adapter later | 007/011/013 |

## Explicit DO_NOT_COPY

- Entire OpenMed repository as MedScale tree
- OpenMed Python clinical domain model into trusted core
- Restricted terminology payloads from OpenMed
- OpenMed service plane as Network Broker
- Treating OpenMed/model output as ClinicalAssertion

## Evidence path

`docs/matrices/openmed-absorption-dispositions.md` (research implement) and package contract copy here.
