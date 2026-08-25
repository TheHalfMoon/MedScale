//! KeyProvider and vault key classes (Spec 005). Synthetic-only proofs; REAL_PHI unauthorized.

mod aead_wrap;
mod keystore;
mod provider;
mod recovery;

pub use aead_wrap::{WrappedBlob, seal, unseal};
pub use keystore::{KeyStore, KeyStoreError, MemoryKeyStore};
pub use provider::{
    ARGON2_M_KIB, ARGON2_P, ARGON2_T, ENCRYPTION_PROFILE_V1, EncryptionProfile, KeyError,
    KeyProvider, RECOVERY_CODE_COUNT, UnlockMethod, VaultDek, VaultKeyHeader,
};
pub use recovery::{RecoveryCodeSet, generate_recovery_codes};
