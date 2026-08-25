//! Durable storage for synthetic H0-A vaults (unencrypted).

mod backup;
mod blob;
mod claim;
mod gc;
mod migrate;
mod sqlite_meta;
mod vault;

pub use backup::{backup_vault, restore_vault};
pub use blob::FsBlobStore;
pub use claim::{ClaimError, assert_claim_path};
pub use gc::{GcStats, run_gc};
pub use migrate::MigrationJournal;
pub use sqlite_meta::{SourceMeta, SqliteMetaStore};
pub use vault::SyntheticVault;
