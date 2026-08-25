# Contract: Saudi / Arabic Benchmark Program Design

**Spec**: 007  
**Status**: Design-canonical; measurement in Spec 008  
**Related**: OPENMED_PARITY row `Saudi/Arabic PII + clinical NER`, [data-model.md](../data-model.md)

## Classification

`REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE`

Pinned floor: OpenMed `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837`.

## Trap classes (minimum ≥4)

| trap_class | Intent |
|---|---|
| `msa_vs_dialect` | MSA vs Gulf/Saudi dialect surface forms |
| `code_switch_en_ar` | Intra-sentence EN↔AR clinical/PII mix |
| `national_id_synthetic` | Synthetic Iqama/national-ID-like patterns (not real IDs) |
| `critical_number` | Dose, lab, date, MRN-like digits; RTL/LTR hazards |
| `named_entity_arabic` | Person/org/location in Arabic script |
| `unicode_offset_rtl` | Span/offset fidelity under RTL + normalization |

## Metrics (design)

| Metric | Required for parity | Notes |
|---|---|---|
| `pii_recall_high_risk` | yes | False-negative analysis |
| `ner_span_f1` | yes | Entity spans |
| `offset_fidelity` | yes | Byte/scalar tagged coords |
| `unsupported_language_honesty` | yes | Fail closed / explicit unsupported |
| `latency_rss_placeholders` | no (008 fills) | Hardware-bound |

## Surpass policy

```text
SURPASS := forbidden until BenchmarkManifest exists with:
  openmed_commit == 59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837
  medscale_commit == exact measured head
  corpus_hash bound
  metrics show superiority on pre-registered primary metrics
```

## PHI / rights policy

- Spec 007 admits **synthetic_only** (and later public rights-cleared if separately gated).
- REAL_PHI Arabic clinical notes remain EXTERNAL_GATES NOT_AUTHORIZED.
- Synthetic national-ID traps MUST NOT be real identifiers.

## Handoff

| Artifact | Owner |
|---|---|
| Program design + seed fixtures | Spec 007 |
| Runtime + measured runs | Spec 008 |
| Document/OCR Arabic expansion | Spec 010 |
| Voice/ASR Arabic | Spec 010 |

## Evidence path

`evidence/007-openmed-capability-absorption-parity/SAUDI_ARABIC_PROGRAM.md`
