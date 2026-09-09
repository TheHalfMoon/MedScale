//! Authority facade, store, promotion, and identity operations.

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

pub use document_ops::{document_worker_policy, mime_decision, voice_worker_policy};
pub use facade::CoreFacade;
pub use handles::assert_no_secret_handles;
pub use source_ops::{create_source_record, overwrite_source_bytes, verify_source_digest};
pub use store::InMemoryAuthorityStore;
