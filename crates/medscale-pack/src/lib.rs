//! Offline Pack v0 admission (Spec 008) + MESC synthetic verifier (Spec 036).

mod catalog;
mod format;
mod hf_snapshot;
mod mesc_verify;
mod onnx_runtime;
mod residency;
mod runtime;
mod store;

pub use catalog::{
    CatalogError, CatalogFilter, CatalogPage, CatalogRow, CatalogSource, CatalogStatus, DeviceFit,
    ModelCatalog, SizeMb,
};
pub use format::{admit_pack_dir, forbidden_reason};
pub use hf_snapshot::{
    HfRepoMetadata, HfSibling, SnapshotError, TokenClassifierSnapshot, build_token_classifier_pack,
    labels_from_config, reproducibility_hash, verify_against_catalog, verify_file,
};
pub use mesc_verify::{MescEpochStore, MescVerifyError, verify_mesc_release_dir};
pub use onnx_runtime::{
    ONNX_TOKEN_CLASSIFIER_RUNTIME_ID, OnnxRuntimeError, OnnxTokenClassifierRuntime,
    PreparedOnnxTokenClassifier,
};
pub use residency::{EvictionPlan, ResidencyError, ResidencyPool};
pub use runtime::{FixtureRuntime, PackRuntimeAdapter, RuntimeOutput};
pub use store::PackStore;
