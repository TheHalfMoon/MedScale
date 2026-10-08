//! Bounded local ONNX token-classification runtime (Spec 069).
//!
//! Runtime code consumes only an already-admitted local MedScale Pack. It never
//! performs network I/O and its output remains proposal/evaluation evidence.

use std::fmt;
use std::fs;
use std::io::{Cursor, Read};
use std::path::{Component, Path, PathBuf};

use medscale_contracts::objects::{DigestSha256, OpaqueId};
use medscale_contracts::packs::{ModelSourceProvenance, PackArtifactKind, PackManifestV0};
use serde_json::json;
use thiserror::Error;
use tokenizers::{PaddingDirection, Tokenizer};
use tract_hir::internal::ElementWiseIntoHir as _;
use tract_hir::tract_core::ops::element_wise::{ElementWiseMiniOp, ElementWiseOp};
use tract_onnx::prelude::*;
use tract_onnx::tract_hir;

use crate::RuntimeOutput;
use crate::format::artifact_size_limit;

pub const ONNX_TOKEN_CLASSIFIER_RUNTIME_ID: &str = "tract_onnx_token_classification_v1";
const MAX_LABELS: usize = 512;
const MAX_INPUT_BYTES: usize = 65_536;

/// Fail-closed runtime errors. Input text is never embedded in an error.
#[derive(Debug, Error)]
pub enum OnnxRuntimeError {
    #[error("pack runtime requirement does not admit tract ONNX token classification")]
    RuntimeNotAdmitted,
    #[error("required pack artifact is missing or ambiguous: {0}")]
    ArtifactSelection(&'static str),
    #[error("unsafe pack artifact path")]
    UnsafeArtifactPath,
    #[error("pack artifact digest changed after admission")]
    ArtifactDigestMismatch,
    #[error("pack artifact exceeds the admitted byte bound")]
    ArtifactBound,
    #[error("tokenizer failed")]
    Tokenizer,
    #[error("input is outside the admitted bound")]
    InputBound,
    #[error("token count is outside the admitted bound")]
    TokenBound,
    #[error("unsupported required model input: {0}")]
    UnsupportedModelInput(String),
    #[error("model execution failed")]
    ModelExecution,
    #[error("model output shape is outside the token-classification contract")]
    OutputShape,
    #[error("model output contained a non-finite logit")]
    NonFiniteLogit,
    #[error("label metadata is invalid")]
    InvalidLabels,
    #[error("model provenance metadata is invalid")]
    InvalidProvenance,
}

/// One bounded runtime evaluation and the signed source provenance that was executed.
#[derive(Debug, Clone, PartialEq)]
pub struct OnnxRuntimeEvaluation {
    pub output: RuntimeOutput,
    pub provenance: ModelSourceProvenance,
}

/// Portable CPU runtime for HF-exported ONNX token-classification models.
#[derive(Debug, Clone, Copy)]
pub struct OnnxTokenClassifierRuntime {
    max_tokens: usize,
}

/// Prepared local model session. Model weights and execution plan are reusable;
/// no user input is retained between evaluations.
pub struct PreparedOnnxTokenClassifier {
    content_digest: DigestSha256,
    runtime_contract_digest: DigestSha256,
    provenance: ModelSourceProvenance,
    tokenizer: Tokenizer,
    pad_token: String,
    pad_id: u32,
    labels: Vec<String>,
    input_names: Vec<String>,
    fixed_sequence_length: usize,
    runnable: TypedSimplePlan<TypedModel>,
}

impl fmt::Debug for PreparedOnnxTokenClassifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PreparedOnnxTokenClassifier")
            .field("content_digest", &self.content_digest)
            .field("runtime_contract_digest", &self.runtime_contract_digest)
            .field("provenance", &self.provenance)
            .field("input_names", &self.input_names)
            .field("fixed_sequence_length", &self.fixed_sequence_length)
            .finish_non_exhaustive()
    }
}

impl OnnxTokenClassifierRuntime {
    /// Construct a runtime with an explicit token ceiling.
    pub fn new(max_tokens: usize) -> Result<Self, OnnxRuntimeError> {
        if !(1..=4096).contains(&max_tokens) {
            return Err(OnnxRuntimeError::TokenBound);
        }
        Ok(Self { max_tokens })
    }

    /// Verify artifacts and prepare an optimized reusable execution plan.
    pub fn prepare(
        &self,
        pack_dir: &Path,
        pack: &PackManifestV0,
    ) -> Result<PreparedOnnxTokenClassifier, OnnxRuntimeError> {
        let model_bytes = select_artifact(pack_dir, pack, PackArtifactKind::OnnxModel, None)?;
        let tokenizer_bytes = select_artifact(
            pack_dir,
            pack,
            PackArtifactKind::TokenizerMeta,
            Some("tokenizer.json"),
        )?;
        let labels_bytes = select_artifact(
            pack_dir,
            pack,
            PackArtifactKind::TokenizerMeta,
            Some("labels.json"),
        )?;
        let provenance_bytes = select_artifact(
            pack_dir,
            pack,
            PackArtifactKind::ModelMetadata,
            Some("model.meta.json"),
        )?;
        let provenance: ModelSourceProvenance = serde_json::from_slice(&provenance_bytes)
            .map_err(|_| OnnxRuntimeError::InvalidProvenance)?;
        validate_provenance(&provenance)?;
        let declared_runtime = pack
            .runtime_requirements
            .split(';')
            .any(|requirement| requirement == ONNX_TOKEN_CLASSIFIER_RUNTIME_ID);
        if !declared_runtime || provenance.runtime_family != ONNX_TOKEN_CLASSIFIER_RUNTIME_ID {
            return Err(OnnxRuntimeError::RuntimeNotAdmitted);
        }
        let fixed_sequence_length = usize::try_from(provenance.fixed_sequence_length)
            .map_err(|_| OnnxRuntimeError::TokenBound)?;
        if fixed_sequence_length == 0 || fixed_sequence_length > self.max_tokens {
            return Err(OnnxRuntimeError::TokenBound);
        }

        let mut tokenizer =
            Tokenizer::from_bytes(&tokenizer_bytes).map_err(|_| OnnxRuntimeError::Tokenizer)?;
        // The runtime owns padding (to the fixed sequence length) and refuses
        // over-length input instead of truncating it. Exported tokenizers may
        // embed their own fixed padding (OpenMed: 512) or truncation, which would
        // break that contract, so both are disabled here. The pad token is taken
        // from the tokenizer's own padding settings when present, else the
        // BERT (`[PAD]`) or RoBERTa/XLM-R (`<pad>`) convention.
        let (pad_token, pad_id) = tokenizer
            .get_padding()
            .map(|p| (p.pad_token.clone(), p.pad_id))
            .filter(|(token, id)| tokenizer.token_to_id(token) == Some(*id))
            .or_else(|| {
                ["[PAD]", "<pad>"]
                    .into_iter()
                    .find_map(|t| tokenizer.token_to_id(t).map(|id| (t.to_string(), id)))
            })
            .ok_or(OnnxRuntimeError::Tokenizer)?;
        tokenizer.with_padding(None);
        tokenizer
            .with_truncation(None)
            .map_err(|_| OnnxRuntimeError::Tokenizer)?;
        let labels: Vec<String> =
            serde_json::from_slice(&labels_bytes).map_err(|_| OnnxRuntimeError::InvalidLabels)?;
        if labels.is_empty() || labels.len() > MAX_LABELS {
            return Err(OnnxRuntimeError::InvalidLabels);
        }

        let mut model_reader = Cursor::new(model_bytes);
        let mut model = tract_onnx::onnx()
            .model_for_read(&mut model_reader)
            .map_err(|_| OnnxRuntimeError::ModelExecution)?;
        let input_outlets = model
            .input_outlets()
            .map_err(|_| OnnxRuntimeError::ModelExecution)?
            .to_vec();
        let input_names: Vec<String> = input_outlets
            .iter()
            .map(|outlet| {
                model
                    .outlet_label(*outlet)
                    .unwrap_or(&model.node(outlet.node).name)
                    .to_owned()
            })
            .collect();
        for (index, name) in input_names.iter().enumerate() {
            if !matches!(
                name.as_str(),
                "input_ids" | "attention_mask" | "token_type_ids"
            ) {
                return Err(OnnxRuntimeError::UnsupportedModelInput(name.clone()));
            }
            model
                .set_input_fact(
                    index,
                    InferenceFact::dt_shape(i64::datum_type(), tvec!(1, fixed_sequence_length)),
                )
                .map_err(|_| OnnxRuntimeError::ModelExecution)?;
        }
        normalize_static_shape_ops(&mut model);
        let optimized = model
            .into_optimized()
            .map_err(|_| OnnxRuntimeError::ModelExecution)?;
        if optimized
            .output_outlets()
            .map_err(|_| OnnxRuntimeError::OutputShape)?
            .len()
            != 1
        {
            return Err(OnnxRuntimeError::OutputShape);
        }
        let output_fact = optimized
            .output_fact(0)
            .map_err(|_| OnnxRuntimeError::OutputShape)?;
        let expected_output_shape = [1, fixed_sequence_length, labels.len()];
        if output_fact.datum_type != f32::datum_type()
            || output_fact.shape.as_concrete() != Some(expected_output_shape.as_slice())
        {
            return Err(OnnxRuntimeError::OutputShape);
        }
        let runnable = optimized
            .into_runnable()
            .map_err(|_| OnnxRuntimeError::ModelExecution)?;
        let runtime_contract_digest = runtime_contract_digest(pack)?;

        Ok(PreparedOnnxTokenClassifier {
            content_digest: pack.content_digest.clone(),
            runtime_contract_digest,
            provenance,
            tokenizer,
            labels,
            input_names,
            fixed_sequence_length,
            runnable,
            pad_token,
            pad_id,
        })
    }

    /// Convenience one-shot path; callers handling repeated traffic should keep
    /// the prepared session returned by [`Self::prepare`].
    pub fn run(
        &self,
        pack_dir: &Path,
        pack: &PackManifestV0,
        input: &str,
    ) -> Result<OnnxRuntimeEvaluation, OnnxRuntimeError> {
        self.prepare(pack_dir, pack)?.run(&pack.pack_id, input)
    }
}

impl PreparedOnnxTokenClassifier {
    #[must_use]
    pub const fn fixed_sequence_length(&self) -> usize {
        self.fixed_sequence_length
    }

    /// True only when the current admitted manifest has the same executable contract.
    #[must_use]
    pub fn matches_runtime_contract(&self, pack: &PackManifestV0) -> bool {
        runtime_contract_digest(pack)
            .map(|digest| digest == self.runtime_contract_digest)
            .unwrap_or(false)
    }

    #[must_use]
    pub fn provenance(&self) -> &ModelSourceProvenance {
        &self.provenance
    }

    /// Run one bounded evaluation using the already-prepared local model.
    pub fn run(
        &self,
        pack_id: &OpaqueId,
        input: &str,
    ) -> Result<OnnxRuntimeEvaluation, OnnxRuntimeError> {
        if input.len() > MAX_INPUT_BYTES {
            return Err(OnnxRuntimeError::InputBound);
        }
        let mut encoding = self
            .tokenizer
            .encode(input, true)
            .map_err(|_| OnnxRuntimeError::Tokenizer)?;
        if encoding.get_ids().is_empty() || encoding.get_ids().len() > self.fixed_sequence_length {
            return Err(OnnxRuntimeError::TokenBound);
        }
        encoding.pad(
            self.fixed_sequence_length,
            self.pad_id,
            0,
            &self.pad_token,
            PaddingDirection::Right,
        );
        let ids = encoding.get_ids();
        let ids_i64: Vec<i64> = ids.iter().map(|id| i64::from(*id)).collect();
        let mask_i64: Vec<i64> = encoding
            .get_attention_mask()
            .iter()
            .map(|value| i64::from(*value))
            .collect();
        let type_ids_i64: Vec<i64> = encoding
            .get_type_ids()
            .iter()
            .map(|value| i64::from(*value))
            .collect();
        let mut model_inputs = tvec![];
        for name in &self.input_names {
            let values = match name.as_str() {
                "input_ids" => &ids_i64,
                "attention_mask" => &mask_i64,
                "token_type_ids" => &type_ids_i64,
                other => return Err(OnnxRuntimeError::UnsupportedModelInput(other.to_owned())),
            };
            let tensor = Tensor::from_shape(&[1, ids.len()], values)
                .map_err(|_| OnnxRuntimeError::ModelExecution)?;
            model_inputs.push(tensor.into_tvalue());
        }
        let outputs = self
            .runnable
            .run(model_inputs)
            .map_err(|_| OnnxRuntimeError::ModelExecution)?;
        if outputs.len() != 1 {
            return Err(OnnxRuntimeError::OutputShape);
        }
        let output = &outputs[0];
        if output.shape() != [1, ids.len(), self.labels.len()] {
            return Err(OnnxRuntimeError::OutputShape);
        }
        let logits = output
            .as_slice::<f32>()
            .map_err(|_| OnnxRuntimeError::OutputShape)?;
        let mut predictions = Vec::new();
        let token_strings = encoding.get_tokens();
        for (token_index, token) in token_strings.iter().enumerate().take(ids.len()) {
            if encoding.get_attention_mask()[token_index] == 0 {
                continue;
            }
            let offset = token_index * self.labels.len();
            let row = &logits[offset..offset + self.labels.len()];
            if row.iter().any(|value| !value.is_finite()) {
                return Err(OnnxRuntimeError::NonFiniteLogit);
            }
            let (label_index, _) = row
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.total_cmp(b))
                .ok_or(OnnxRuntimeError::OutputShape)?;
            predictions.push(json!({
                "token": token,
                "label": self.labels[label_index],
                "label_index": label_index,
            }));
        }
        Ok(OnnxRuntimeEvaluation {
            output: RuntimeOutput {
                proposal_payload: json!({
                    "kind": "onnx_token_classification_v1",
                    "runtime": ONNX_TOKEN_CLASSIFIER_RUNTIME_ID,
                    "content_digest": self.content_digest.to_hex(),
                    "source_kind": self.provenance.source_kind,
                    "source_repository": self.provenance.repository,
                    "source_revision": self.provenance.revision,
                    "token_count": predictions.len(),
                    "predictions": predictions,
                    "note": "Proposal/evaluation evidence only; never ClinicalAssertion",
                }),
                evidence_only: true,
                pack_id: pack_id.clone(),
            },
            provenance: self.provenance.clone(),
        })
    }
}

fn safe_join(root: &Path, relative: &str) -> Result<PathBuf, OnnxRuntimeError> {
    let path = Path::new(relative);
    if path.is_absolute()
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(OnnxRuntimeError::UnsafeArtifactPath);
    }
    let canonical_root = root
        .canonicalize()
        .map_err(|_| OnnxRuntimeError::ArtifactDigestMismatch)?;
    let candidate = root.join(path);
    let canonical_candidate = candidate
        .canonicalize()
        .map_err(|_| OnnxRuntimeError::ArtifactDigestMismatch)?;
    if !canonical_candidate.starts_with(&canonical_root) {
        return Err(OnnxRuntimeError::UnsafeArtifactPath);
    }
    Ok(canonical_candidate)
}

fn normalized_artifact_bytes(
    path: &Path,
    kind: PackArtifactKind,
) -> Result<Vec<u8>, OnnxRuntimeError> {
    let limit = artifact_size_limit(kind);
    let metadata = fs::metadata(path).map_err(|_| OnnxRuntimeError::ArtifactDigestMismatch)?;
    if !metadata.is_file() || metadata.len() > limit {
        return Err(OnnxRuntimeError::ArtifactBound);
    }
    let file = fs::File::open(path).map_err(|_| OnnxRuntimeError::ArtifactDigestMismatch)?;
    let mut bytes = Vec::new();
    file.take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| OnnxRuntimeError::ArtifactDigestMismatch)?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > limit {
        return Err(OnnxRuntimeError::ArtifactBound);
    }
    if !matches!(
        kind,
        PackArtifactKind::FixtureBytes
            | PackArtifactKind::TokenizerMeta
            | PackArtifactKind::ModelMetadata
    ) {
        return Ok(bytes);
    }
    let mut normalized = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\r' {
            if index + 1 < bytes.len() && bytes[index + 1] == b'\n' {
                normalized.push(b'\n');
                index += 2;
                continue;
            }
            normalized.push(b'\n');
            index += 1;
            continue;
        }
        normalized.push(bytes[index]);
        index += 1;
    }
    Ok(normalized)
}

fn select_artifact(
    root: &Path,
    pack: &PackManifestV0,
    kind: PackArtifactKind,
    required_name: Option<&str>,
) -> Result<Vec<u8>, OnnxRuntimeError> {
    let mut selected = None;
    for artifact in &pack.artifacts {
        let path = safe_join(root, &artifact.relative_path)?;
        let bytes = normalized_artifact_bytes(&path, artifact.kind)?;
        let digest = medscale_contracts::objects::DigestSha256::of(&bytes);
        if digest != artifact.digest {
            return Err(OnnxRuntimeError::ArtifactDigestMismatch);
        }
        let name_matches = required_name
            .map(|name| artifact.relative_path == name)
            .unwrap_or(true);
        if artifact.kind == kind && name_matches {
            if selected.is_some() {
                return Err(OnnxRuntimeError::ArtifactSelection("ambiguous artifact"));
            }
            selected = Some(bytes);
        }
    }
    selected.ok_or(OnnxRuntimeError::ArtifactSelection(match required_name {
        Some("tokenizer.json") => "tokenizer.json",
        Some("labels.json") => "labels.json",
        Some("model.meta.json") => "model.meta.json",
        _ => "onnx_model",
    }))
}

fn runtime_contract_digest(pack: &PackManifestV0) -> Result<DigestSha256, OnnxRuntimeError> {
    let bytes = serde_json::to_vec(&(pack.runtime_requirements.as_str(), &pack.artifacts))
        .map_err(|_| OnnxRuntimeError::RuntimeNotAdmitted)?;
    Ok(DigestSha256::of(&bytes))
}

fn validate_provenance(provenance: &ModelSourceProvenance) -> Result<(), OnnxRuntimeError> {
    let source_ok = matches!(
        provenance.source_kind.as_str(),
        "hugging_face" | "synthetic_fixture"
    );
    if !source_ok
        || provenance.repository.trim().is_empty()
        || provenance.revision.trim().is_empty()
        || provenance.original_file.trim().is_empty()
        || provenance.source_sha256.len() != 64
        || !provenance
            .source_sha256
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
        || provenance.transform.trim().is_empty()
        || provenance.fixed_sequence_length == 0
        || provenance.fixed_sequence_length > 4096
        || provenance.task != "token-classification"
        || provenance.export_format != "onnx"
        || provenance.runtime_family != ONNX_TOKEN_CLASSIFIER_RUNTIME_ID
        || provenance.license_id.trim().is_empty()
        || provenance.upstream_rights_uri.trim().is_empty()
    {
        return Err(OnnxRuntimeError::InvalidProvenance);
    }
    Ok(())
}

/// Stateless cast to `i64`, usable during inference-time constant folding.
#[derive(Debug, Clone)]
struct CastToI64;

impl ElementWiseMiniOp for CastToI64 {
    fn name(&self) -> String {
        "medscale.CastToI64".into()
    }

    fn output_type(&self, _input_type: DatumType) -> Option<DatumType> {
        Some(i64::datum_type())
    }

    fn eval_out_of_place(&self, t: &Tensor, _out_dt: Option<DatumType>) -> TractResult<Tensor> {
        Ok(t.cast_to::<i64>()?.into_owned())
    }
}

/// Spec 103: `tract-onnx` loads ONNX `Shape` and `Cast(to=INT64)` as symbolic
/// `TDim` operations. In Transformer exports (for example OpenMed BERT token
/// classifiers) shape arithmetic then meets `i64` constants and analysis fails
/// ("Impossible to unify TDim with I64" at the position-id `Range`). This
/// runtime always fixes every input to a concrete `[1, fixed_sequence_length]`
/// shape, so those values are concrete integers and can be typed as `i64`
/// without changing semantics. Only full-range `Shape` nodes and casts to
/// `TDim` are rewritten; returns the number of rewritten nodes.
fn normalize_static_shape_ops(model: &mut InferenceModel) -> usize {
    let mut rewritten = 0;
    for node in model.nodes_mut() {
        let op = format!("{:?}", node.op);
        if node.op.name() == "Shape" && op.contains("start: 0, end: None") {
            node.op = tract_hir::ops::expandable::expand(tract_hir::ops::array::Shape::new(
                i64::datum_type(),
            ));
            rewritten += 1;
        } else if op == "ElementWiseOp(Cast { to: TDim })" {
            node.op = ElementWiseOp(Box::new(CastToI64), None).into_hir();
            rewritten += 1;
        }
    }
    rewritten
}
