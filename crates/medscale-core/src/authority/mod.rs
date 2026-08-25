//! Authority facade, store, promotion, and identity operations.

mod facade;
mod handles;
mod identity;
mod promote;
mod source_ops;
mod store;

pub use facade::CoreFacade;
pub use handles::assert_no_secret_handles;
pub use source_ops::{create_source_record, overwrite_source_bytes, verify_source_digest};
pub use store::InMemoryAuthorityStore;
