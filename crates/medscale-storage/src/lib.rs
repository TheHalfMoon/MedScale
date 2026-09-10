//! Durable storage: synthetic vaults (003) + encrypted vaults (005).

mod backup;
mod blob;
mod claim;
mod encrypted_vault;
mod gc;
mod migrate;
mod privacy_probes;
mod sealed_blob;
mod sqlite_meta;
mod vault;
mod writer_lock;

pub use backup::{backup_vault, restore_vault};
pub use blob::FsBlobStore;
pub use claim::{ClaimError, assert_claim_path};
pub use encrypted_vault::{EncryptedVault, EncryptedVaultError, default_vault_root};
pub use gc::{GcStats, run_gc};
pub use migrate::MigrationJournal;
pub use privacy_probes::{
    OsFileProbeResult, PrivacyProbeReport, crash_sidecar_leftovers_present,
    probe_os_privacy_surfaces, residual_risk_classes_open, scan_vault_work_leftovers,
};
pub use sealed_blob::SealedBlobStore;
pub use sqlite_meta::{AuthorityObjectRow, MetaError, SourceMeta, SqliteMetaStore};
pub use vault::{SyntheticVault, VaultError};
pub use writer_lock::{WriterLock, WriterLockError};
