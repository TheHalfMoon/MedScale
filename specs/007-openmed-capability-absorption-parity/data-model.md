# Data Model: Spec 007 Parity Matrix & Research Schemas

**Date**: 2026-08-25  
**Status**: Planning-canonical for Spec 007 research implement  
**Persistence**: Research matrices and evidence JSON/Markdown. **No** new durable clinical object classes. **No** canonical DB schema from this unit.

## Principles

1. Parity rows are planning/evidence entities—not ClinicalAssertions.
2. Corpus descriptors are synthetic-first; REAL_PHI unauthorized.
3. Donor dispositions never authorize wholesale OpenMed fork.
4. Terminology rights entries never embed licensed table payloads.
5. BenchmarkManifest is required before any PARITY/SURPASS claim language in evidence.

## Entity: OpenMedBaselinePin

| Field | Required | Type / values | Description |
|---|---|---|---|
| `upstream_url` | yes | URL | `https://github.com/maziyarpanahi/openmed` |
| `baseline_tag` | yes | string | `v2.2.0` |
| `baseline_commit` | yes | git SHA | `59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837` |
| `baseline_tree` | yes | git tree SHA | `1c949e35b2b8f2ea69da4284b370074fc4bf84ab` |
| `verified_at` | optional | ISO-8601 | When local/archive verification ran |
| `verification_method` | optional | enum | `git_clone` \| `archive` \| `documented_pin_only` |
| `notes` | optional | string | Tip deltas labeled non-authoritative |

## Entity: ParityCapabilityRow

Schema for each OPENMED_PARITY_SURPASS_MATRIX_V2 row (and any Spec 007 extension rows).

| Field | Required | Type / values | Description |
|---|---|---|---|
| `capability_id` | yes | slug | e.g. `clinical-ner`, `saudi-arabic-pii-ner` |
| `capability_name` | yes | string | Human label |
| `v2_classification` | yes | enum | `REQUIRED_PARITY` \| `REQUIRED_PARITY_AND_SURPASS_PENDING_EVIDENCE` \| `USEFUL_PARITY` \| `REQUIRED_ABSORB_PATTERN` \| `MEDSCALE_SUPERSEDES_CONCEPT` \| `DEFER` |
| `owning_specs` | yes | string[] | e.g. `["007","008"]` |
| `openmed_v22_significance` | yes | string | Short note from matrix |
| `medscale_qualification_target` | yes | string | Metrics / contracts expected |
| `corpus_ids` | optional | string[] | Links to ParityCorpusDescriptor |
| `waiver` | optional | object | Required when DEFER or explicit non-block |
| `evidence_status` | yes | enum | `not_started` \| `designed` \| `corpus_ready` \| `measured` \| `waived` |
| `surpass_allowed` | yes | bool | Default false unless classification + evidence permit |
| `baseline_commit` | yes | git SHA | Must equal OpenMedBaselinePin.baseline_commit |

### Waiver object

| Field | Required | Description |
|---|---|---|
| `reason` | yes | Why not blocking |
| `scope` | yes | Claim envelope excluded |
| `recorded_in` | yes | Path to doc / EXTERNAL_GATES id |

## Entity: ParityCorpusDescriptor

| Field | Required | Type / values | Description |
|---|---|---|---|
| `corpus_id` | yes | slug | Stable id |
| `title` | yes | string | |
| `capability_ids` | yes | string[] | Linked rows |
| `data_class` | yes | enum | `synthetic` \| `public_rights_cleared` \| `gated` (gated forbidden in 007 admit) |
| `phi_class` | yes | enum | `synthetic_only` \| `real_phi` — **must be synthetic_only for 007** |
| `languages` | yes | string[] | BCP-47-ish tags, e.g. `en`, `ar`, `ar-SA` |
| `trap_classes` | optional | string[] | e.g. `code_switch`, `dialect`, `national_id_synthetic`, `critical_number` |
| `metrics` | yes | MetricSpec[] | See below |
| `anti_metrics` | optional | string[] | Explicit non-metrics (e.g. `raw_model_count`) |
| `fixture_paths` | optional | string[] | Repo-relative when seeded |
| `content_hash` | optional | string | When fixtures exist |
| `rights_note` | yes | string | Provenance / license posture |
| `harness_owner_spec` | yes | string | Usually `008` for execution |
| `status` | yes | enum | `design` \| `seeded` \| `frozen` |

### MetricSpec

| Field | Required | Description |
|---|---|---|
| `name` | yes | e.g. `span_f1`, `phi_recall`, `latency_p50_ms` |
| `higher_is_better` | yes | bool |
| `required_for_parity` | yes | bool |
| `notes` | optional | string |

## Entity: DonorAbsorptionDisposition

| Field | Required | Type / values | Description |
|---|---|---|---|
| `disposition_id` | yes | slug | |
| `donor` | yes | string | e.g. `OpenMed`, `Presidio` |
| `capability_family` | yes | string | |
| `upstream_loci` | yes | string | Paths / PR refs from SOURCE |
| `operation` | yes | enum | `COPY_BOUNDED` \| `PORT_TO_RUST` \| `DEPENDENCY` \| `VENDOR_SNAPSHOT` \| `FFI` \| `SIDECAR` \| `REFERENCE_ONLY` \| `ARTIFACT_IMPORT` \| `USER_SUPPLIED_UI` \| `DO_NOT_COPY` |
| `placement_class` | optional | enum | `P0` \| `P1` \| `P2` \| `P3` \| `N/A` |
| `owning_spec` | yes | string | |
| `medscale_intent` | yes | string | |
| `forbidden` | yes | string[] | Explicit anti-patterns |
| `provenance_required_before_bytes` | yes | bool | true if COPY/VENDOR |
| `status` | yes | enum | `dispositioned` \| `provenance_started` \| `absorbed` |

## Entity: TerminologyRightsTrackEntry

| Field | Required | Type / values | Description |
|---|---|---|---|
| `system_id` | yes | slug | `snomed-ct`, `loinc`, `ucum`, `icd`, `atc`, `umls-athena` |
| `display_name` | yes | string | |
| `rights_class` | yes | enum | `open` \| `caller_supplied` \| `gated_license` \| `unknown_pending_counsel` |
| `copy_from_openmed` | yes | bool | **must be false** |
| `pack_metadata_required` | yes | string[] | `version`, `checksum`, `rights_uri`, `notice`, … |
| `external_gate_id` | optional | string | EXTERNAL_GATES row when gated |
| `owning_consume_specs` | yes | string[] | e.g. `011`, `013` |
| `status` | yes | enum | `track_started` \| `awaiting_counsel` \| `pack_ready` \| `blocked` |

## Entity: SaudiArabicBenchmarkProgram

| Field | Required | Description |
|---|---|---|
| `program_id` | yes | e.g. `saudi-arabic-pii-ner-v0` |
| `baseline_commit` | yes | OpenMed pin SHA |
| `trap_classes` | yes | ≥4 named classes |
| `metrics` | yes | MetricSpec[] |
| `corpus_ids` | yes | Linked corpora |
| `surpass_policy` | yes | `forbidden_until_exact_head_benchmark` |
| `phi_policy` | yes | `synthetic_only` |
| `measurement_owner_spec` | yes | `008` |
| `status` | yes | `designed` \| `seeded` \| `measured` |

## Entity: BenchmarkManifest

Exact-head evidence envelope (OPENMED_PARITY §3).

| Field | Required | Description |
|---|---|---|
| `manifest_id` | yes | |
| `claim_type` | yes | `PARITY` \| `SURPASS` \| `ABSORB_PATTERN` \| `WAIVER` |
| `capability_id` | yes | |
| `openmed_commit` | yes | Must match pin for floor claims |
| `medscale_commit` | yes | |
| `artifact_hashes` | yes | model/pack/tool hashes |
| `corpus_hash` | yes | |
| `rights` | yes | |
| `hardware_os_runtime` | yes | |
| `quality_metrics` | yes | map |
| `latency_cold_start_throughput` | optional | map |
| `memory` | optional | RAM/VRAM |
| `network_privacy_observation` | yes | |
| `failure_behavior` | yes | |
| `limitations` | yes | string[] |
| `harness_revision` | yes | |
| `recorded_at` | yes | ISO-8601 |

## Seed matrix rows (capability_id catalog)

Must cover OPENMED_PARITY_SURPASS_MATRIX_V2 §2. Canonical ids:

```text
clinical-ner
pii-detection
de-identification
multilingual-medical-nlp
saudi-arabic-pii-clinical-ner
terminology-grounding
unicode-offset-semantics
local-offline-useful-operation
cpu-inference
apple-silicon-mlx
android-local-inference
ios-local-capability
nvidia-gpu
browser-webgpu
python-api
cli
rest-grpc-graphql-service-plane
mcp-agent-skills
model-catalogue-breadth
model-packaging-provenance
staged-model-promotion
structured-privacy-sdc
fhir-r4-summaries-documents-profile-integrity
omop-cdm-5-4
openehr-export
document-format-breadth
deterministic-clinical-document-processing
radiology-specialty-helpers
national-id-validation
voice-asr
watchos-visionos
source-custody
identity
time-semantics
longitudinal-truth-conflict
proposal-promotion-authority
privacy-observability
cross-surface-consistency
controlled-external-actions
```

## Validation rules

1. `baseline_commit` on every ParityCapabilityRow == OpenMedBaselinePin.
2. `phi_class` on any Spec 007-admitted corpus == `synthetic_only`.
3. `copy_from_openmed` on TerminologyRightsTrackEntry == false.
4. `claim_type` SURPASS requires `saudi-arabic-pii-clinical-ner` (and peers) to have measured BenchmarkManifest—not design-only.
5. No DonorAbsorptionDisposition may set operation that implies wholesale OpenMed product fork.

## Relationships

```text
OpenMedBaselinePin 1──* ParityCapabilityRow
ParityCapabilityRow *──* ParityCorpusDescriptor
ParityCorpusDescriptor *──1 SaudiArabicBenchmarkProgram (for Arabic program corpora)
DonorAbsorptionDisposition (independent; links owning_spec)
TerminologyRightsTrackEntry (independent; EXTERNAL_GATES)
BenchmarkManifest *──1 ParityCapabilityRow (when measured)
```
