//! Offline Pack v0 admission (Spec 008) + MESC synthetic verifier (Spec 036).

mod candle_runtime;
mod catalog;
mod compatibility;
mod format;
mod fp16_widen;
mod hf_snapshot;
mod mesc_verify;
mod onnx_runtime;
mod residency;
mod runtime;
mod store;
mod token_windows;

pub use candle_runtime::{
    CANDLE_ARCHITECTURES, CANDLE_TOKEN_CLASSIFIER_RUNTIME_ID, CandleRunOptions, CandleRuntimeError,
    PreparedCandleTokenClassifier, candle_model_type, prepare_candle_token_classifier,
};
pub use catalog::{
    CatalogError, CatalogFilter, CatalogPage, CatalogRow, CatalogSource, CatalogStatus, DeviceFit,
    ModelCatalog, SizeMb,
};
pub use compatibility::{
    EVIDENCED_ARCHITECTURES, KNOWN_DEFECTIVE_EXPORTS, KNOWN_UNSUPPORTED, RuntimeExpectation,
    SAFETENSORS_EVIDENCED_ARCHITECTURES, SAFETENSORS_KNOWN_UNSUPPORTED, runtime_expectation,
};
pub use format::{admit_pack_dir, forbidden_reason};
pub use hf_snapshot::{
    HfRepoMetadata, HfSibling, SnapshotError, TokenClassifierSnapshot, build_token_classifier_pack,
    labels_from_config, reproducibility_hash, verify_against_catalog, verify_file,
};
pub use mesc_verify::{MescEpochStore, MescVerifyError, verify_mesc_release_dir};
pub use onnx_runtime::{
    DocumentRunOptions, ONNX_TOKEN_CLASSIFIER_RUNTIME_ID, OnnxRuntimeError,
    OnnxTokenClassifierRuntime, PreparedOnnxTokenClassifier,
};
pub use residency::{EvictionPlan, ResidencyError, ResidencyPool};
pub use runtime::{FixtureRuntime, PackRuntimeAdapter, RuntimeOutput};
pub use store::PackStore;
pub use token_windows::{
    BestContextVotes, CharSpan, DecodedEntity, MAX_WINDOWS, TokenLogits, TokenWindow, WindowError,
    decode_entities, default_stride, plan_windows,
};
