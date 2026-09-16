# Spec 069 Limitations

- The pinned Hugging Face model is a general-domain NER qualification model, not a promoted clinical model.
- Real PHI inference remains explicitly denied by `REAL_PHI_MODEL_RUNTIME`.
- Product online model acquisition remains denied by `HF_ONLINE_PACK_DISTRIBUTION`; Spec 069 qualification uses external pinned source files and offline Pack admission.
- The portable tract CPU runtime is a correctness baseline, not an accelerated-runtime winner.
- No MLX, Core ML, GPU, or ONNX Runtime accelerated adapter is promoted by Spec 069.
- PII/de-identification recall, clinical NER quality, multilingual model quality, and OpenMed comparative performance remain unproven until the later promoted specs.
- The synthetic Pack trust root used by repository fixtures/qualification is not production release signing authority.
