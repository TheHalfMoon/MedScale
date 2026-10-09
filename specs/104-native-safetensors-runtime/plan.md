# Spec 104 plan

1. **Admission and runtime** (this slice):
   - workspace dependencies `candle-* =0.9.1` (CPU, `default-features = false`);
   - `PackArtifactKind::SafetensorsModel` (1 GiB bound);
   - `medscale-pack/src/candle_runtime.rs`: allowlisted `model_type`, verified-bytes loading, encoders plus a biased linear head, whole-document execution through `token_windows`;
   - unit tests on a randomly initialised tiny BERT.
2. **Snapshot Pack building and acquisition:**
   - `verify_file` accepts `.safetensors`;
   - the snapshot builder writes `model.safetensors` plus `config.json` and declares the candle runtime;
   - acquisition consent accepts a `.safetensors` weight file.
3. **Core integration:** prepared candle models in the residency pool; Model Fleet lanes on candle Packs.
4. **Qualification:** the Spec 103 workflow gains safetensors jobs per architecture. Compatibility expectations follow the evidence.
