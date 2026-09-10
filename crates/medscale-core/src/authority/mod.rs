//! Authority facade, store, promotion, and identity operations.

mod amend;
mod corpus;
mod document_ops;
mod durable;
mod facade;
mod handles;
mod identity;
mod ingest_ops;
mod presentation;
mod promote;
mod retrieval;
mod source_ops;
mod store;

pub use corpus::{
    DEFAULT_CORPUS_ID, DEFAULT_CORPUS_VERSION, SCALE_CORPUS_DOC_COUNT_10K, SCALE_CORPUS_ID,
    SCALE_CORPUS_VERSION_10K, admit_corpus_bytes, admit_corpus_dir,
    build_synthetic_lexical_scale_corpus, default_synthetic_corpus, scale_synthetic_corpus_10k,
};
pub use document_ops::{document_worker_policy, mime_decision, voice_worker_policy};
pub use facade::CoreFacade;
pub use handles::assert_no_secret_handles;
pub use source_ops::{create_source_record, overwrite_source_bytes, verify_source_digest};
pub use store::InMemoryAuthorityStore;
