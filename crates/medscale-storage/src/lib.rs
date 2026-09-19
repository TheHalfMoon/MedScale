//! Durable storage: synthetic vaults (003) + encrypted vaults (005).

mod backup;
mod blob;
mod claim;
mod data_sources;
mod encrypted_vault;
mod gc;
mod migrate;
mod privacy_probes;
mod project_graph;
mod sealed_blob;
mod sqlite_meta;
mod vault;
mod writer_lock;

pub use backup::{backup_vault, restore_vault};
pub use blob::FsBlobStore;
pub use claim::{ClaimError, assert_claim_path};
pub use data_sources::{
    EXTERNAL_TABLES_MAX, ExternalCell, ExternalTable, RECEIPT_KIND_IMPORT, RECEIPT_KIND_REFRESH,
    RECEIPT_KIND_TRANSFORMATION, SnapshotRecord, TransformationRecord, list_external_sqlite_tables,
    read_external_sqlite_table, validate_external_identifier,
};
pub use encrypted_vault::{EncryptedVault, EncryptedVaultError, default_vault_root};
pub use gc::{GcStats, run_gc};
pub use migrate::MigrationJournal;
pub use privacy_probes::{
    ClassifiedResidualSurface, OsFileProbeResult, PrivacyProbeReport, ProbeHonestyClass,
    crash_sidecar_leftovers_present, probe_os_privacy_surfaces, residual_risk_classes_open,
    scan_vault_work_leftovers,
};
pub use sealed_blob::SealedBlobStore;
pub use sqlite_meta::{AuthorityObjectRow, MetaError, SourceMeta, SqliteMetaStore};
pub use vault::{SyntheticVault, VaultError};
pub use writer_lock::{WriterLock, WriterLockError};
