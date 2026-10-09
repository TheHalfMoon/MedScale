# Spec 103 rights ledger: executed OpenMed models

**Date:** 2026-10-09. **Scope:** the four models `EXECUTED_TESTED` in `specs/103-local-model-catalog/research.md` §9–10.
**Method:** each model card `README.md` read at the pinned Hub commit (text only; no weights), the base-model card, and the declared upstream base-model licenses from Hub metadata. This is a recorded review, not legal advice. It supports local qualification use; it is **not** a redistribution clearance.

| Executed model (pinned commit) | Card SHA-256 (prefix) | Card license | Derived from | Base license (Hub) | Training data named on card |
|---|---|---|---|---|---|
| `OpenMed/OpenMed-NER-DiseaseDetect-ElectraMed-33M-v1-onnx-android` @ `54c9cc11` | `344c3a3fd0a4dd5c` | apache-2.0 | `OpenMed/OpenMed-NER-DiseaseDetect-ElectraMed-33M` @ `05659fbd` (apache-2.0) ← `intfloat/e5-small-v2` @ `ffb93f3b` | MIT | BC5CDR-Disease |
| `OpenMed/OpenMed-NER-AnatomyDetect-TinyMed-65M-v1-onnx-android` @ `ad5e79cb` | `e43ec69329f881a3` | apache-2.0 | `OpenMed/OpenMed-NER-AnatomyDetect-TinyMed-65M` @ `1ab9359d` (apache-2.0) ← `distilbert/distilbert-base-cased` @ `6ea81172` | Apache-2.0 | ANATOMY |
| `OpenMed/OpenMed-NER-AnatomyDetect-TinyMed-82M-v1-onnx-android` @ `95f0bf74` | `e75bf32ef72931da` | apache-2.0 | `OpenMed/OpenMed-NER-AnatomyDetect-TinyMed-82M` @ `724a3702` (apache-2.0) ← `distilbert/distilroberta-base` @ `fb53ab88` | Apache-2.0 | ANATOMY |
| `OpenMed/OpenMed-NER-AnatomyDetect-ModernClinical-149M-v1-onnx-android` @ `66525717` | `7a63e35a01291fe3` | apache-2.0 | `OpenMed/OpenMed-NER-AnatomyDetect-ModernClinical-149M` @ `3c0bdcc3` (apache-2.0) ← `thomas-sounack/BioClinical-ModernBERT-base` @ `c3648aa8` | MIT | ANATOMY |

All repositories were non-gated at review time.

## Findings

1. **Weights and base models.** Every model, its OpenMed base and the upstream base model declare a permissive license (Apache-2.0 or MIT). No gated or `other` license appears in the lineage. MIT and Apache-2.0 both require notice preservation on redistribution.
2. **Training data (residual).** The cards name BC5CDR-Disease and ANATOMY as training data. Their terms were **not verified** in this review. A model trained on a dataset may carry dataset-terms questions for some uses even when the weights are permissively licensed. Recorded as `DATASET_TERMS_UNVERIFIED`.
3. **Use in MedScale.** Local, synthetic-only qualification is consistent with the declared licenses. **Not cleared:** bundling or redistributing these weights in a MedScale release, any clinical use, or any claim about the training data.

`CLINICAL_VALIDATION_STATUS=NOT_PERFORMED`. No model is `TASK_QUALIFIED`.
