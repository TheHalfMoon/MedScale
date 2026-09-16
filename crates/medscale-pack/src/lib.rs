//! Offline Pack v0 admission (Spec 008) + MESC synthetic verifier (Spec 036).

mod format;
mod mesc_verify;
mod onnx_runtime;
mod runtime;
mod store;

pub use format::{admit_pack_dir, forbidden_reason};
pub use mesc_verify::{MescEpochStore, MescVerifyError, verify_mesc_release_dir};
pub use onnx_runtime::{
    ONNX_TOKEN_CLASSIFIER_RUNTIME_ID, OnnxRuntimeError, OnnxTokenClassifierRuntime,
    PreparedOnnxTokenClassifier,
};
pub use runtime::{FixtureRuntime, PackRuntimeAdapter, RuntimeOutput};
pub use store::PackStore;
