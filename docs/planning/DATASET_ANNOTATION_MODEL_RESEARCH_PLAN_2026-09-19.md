# Dataset Annotation + Model Research Plan — 2026-09-19

**Status:** `PLANNING_CANDIDATE_ONLY`  
**Purpose:** make MedScale useful for medical AI/data-science teams that need to curate datasets, annotate data, evaluate models and run reproducible research without making model training or real-PHI use an implicit product authority.

## 1. Dataset lifecycle

Target:

```text
Source
 -> DataSnapshot
 -> Privacy/eligibility gate
 -> Curation
 -> Annotation task
 -> Review/adjudication
 -> Dataset release
 -> Train/evaluate experiment
 -> Model/result artifacts
 -> Research report
```

Every transition is versioned and evidence-linked.

## 2. Dataset card

Each dataset/release should record:

- title/version;
- source/provenance;
- rights/license/DUA state;
- subject/population scope;
- modalities;
- schema/data dictionary;
- row/item counts;
- inclusion/exclusion;
- missingness;
- label ontology;
- de-identification/privacy state;
- known biases/limitations;
- splits;
- transformations;
- checksums;
- intended/prohibited uses;
- maintainers/review state.

A dataset card is metadata, not legal or privacy clearance.

## 3. Annotation schema

Support versioned schemas for:
- classification;
- multi-label;
- span/NER;
- relation;
- document/page region;
- image region/segmentation when later qualified;
- audio/transcript spans;
- structured abstraction;
- ranking/preference;
- adjudicated clinical concepts.

Schema changes create explicit version transitions.

## 4. Annotation tasks

Each task binds:
- exact dataset snapshot/item;
- annotation schema version;
- instructions/guideline;
- reviewer/annotator;
- assignment state;
- annotation revision;
- source evidence;
- timestamps;
- blind/review settings.

## 5. Review/adjudication

Team workflows should support:
- single review;
- dual independent review;
- consensus view;
- conflict queue;
- adjudicator decision;
- comments;
- rationale.

Model-generated labels are a distinct annotator identity and never silently become human labels.

## 6. Inter-rater agreement

Analytics may calculate appropriate agreement metrics, for example:
- Cohen's kappa;
- Fleiss' kappa;
- Krippendorff's alpha;
- span/entity agreement metrics.

The metric chosen depends on annotation type and assumptions.

## 7. Active learning / model-assisted labeling

A local admitted model may:
- pre-label;
- prioritize uncertain examples;
- propose entities/relations;
- suggest duplicate/outlier items.

Required boundaries:
- model identity/version recorded;
- suggestions distinguishable from human labels;
- human review policy explicit;
- no training feedback loop silently overwrites the evaluation split.

## 8. Split integrity

Dataset splits should preserve:
- exact item identity;
- grouping constraints such as patient/site/episode when appropriate;
- random seed;
- stratification rules;
- temporal split logic;
- leakage checks.

Patient-level leakage is a critical failure in medical ML evaluation.

## 9. De-identification and consent

Before a dataset can enter a less restricted research workspace:
- source authority;
- consent/purpose;
- de-identification policy;
- DeidReceipt;
- residual risk state;
- data-use restrictions

must be explicit.

Real PHI remains prohibited until canonical authority changes.

## 10. Experiment artifact

A model experiment should bind:
- dataset release/splits;
- code/config;
- model base artifact/revision;
- environment;
- seed;
- hyperparameters;
- hardware;
- metrics;
- logs/receipts;
- outputs/checkpoints;
- failures;
- reviewer.

This can extend the existing Spec 074 Experiment concept rather than create a second experiment identity.

## 11. Fine-tuning / training

Training is a later Compute capability, not implied by the Model Fleet.

Candidate modes:
- local small-model fine-tuning;
- parameter-efficient tuning;
- external/self-hosted compute adapter;
- evaluation-only experiments.

Before any training workflow is promoted, define:
- model license;
- dataset rights;
- data class;
- hardware/resource budget;
- output model rights/provenance;
- memorization/privacy tests;
- rollback/cleanup;
- benchmark protocol.

No production clinical model is admitted merely because training completes.

## 12. Model evaluation

Use task-specific test sets and preserve:
- frozen benchmark version;
- hidden/held-out split governance;
- no train/eval contamination;
- confidence intervals where meaningful;
- subgroup analysis;
- calibration;
- abstention;
- safety/error taxonomy;
- resource metrics.

For clinical generation also use evidence/validator tests rather than only text similarity.

## 13. Model registry relationship

Spec 078 Model Fleet should show:
- candidate;
- admitted;
- rejected;
- superseded;
- experimental

states.

Training output begins as a candidate Pack and must pass normal Pack admission/qualification.

## 14. Dataset Workbench integration

Data Workbench provides:
- browse/filter;
- schema;
- quality;
- sample inspection;
- annotations;
- label distribution;
- split membership;
- lineage.

Do not show sensitive source values to an annotator whose assignment lacks authority.

## 15. HumanSignal/Label Studio reference

Label Studio and similar annotation products are useful UX/reference sources for:
- task queues;
- labeling interfaces;
- reviewer workflows;
- ML-assisted annotation.

They do not become MedScale's storage/identity/authority plane. Exact licensing/source qualification is required before code transfer.

## 16. Research reproducibility

A published model research artifact should be reconstructable from:
- dataset release;
- experiment manifest;
- code/config digest;
- environment;
- model base/output identity;
- evaluation Pack;
- results.

Failed experiments remain first-class records when useful to interpretation.

## 17. Research OS ownership

- 075 — dataset snapshots, curation views, dataset releases;
- 076 — annotation assignment/review/adjudication;
- 078 — model catalog/comparison/admission;
- 079 — privacy/de-identification;
- 082 — quality/agreement/statistical analysis;
- 083 — research/project relationships;
- 085 — bounded training/evaluation compute;
- 089 — dataset/evaluation/model-method Packs;
- 092 — reproducibility, leakage, safety and release truth.

## 18. Qualification

A future annotation/model-research vertical slice should prove:

```text
source snapshot
 -> privacy-approved synthetic/permitted dataset
 -> annotation schema
 -> independent labels
 -> adjudication
 -> versioned dataset release
 -> experiment
 -> model candidate
 -> held-out evaluation
 -> report
 -> restart/replay
```

No real PHI, model quality or clinical readiness claim is implied.
