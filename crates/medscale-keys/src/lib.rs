//! KeyProvider and vault key classes (Spec 005). Synthetic-only proofs; REAL_PHI unauthorized.

mod aead_wrap;
mod keystore;
mod pack_trust;
mod provider;
mod recovery;

pub use aead_wrap::{WrappedBlob, seal, unseal};
pub use keystore::{KeyStore, KeyStoreError, MemoryKeyStore, MobileKeyStorePolicy};
pub use pack_trust::{
    PackTrustError, SYNTHETIC_PACK_TRUST_ROOT_ID, pack_signing_payload, sign_pack_payload,
    synthetic_verifying_key_hex, verify_pack_signature,
};
pub use provider::{
    ARGON2_M_KIB, ARGON2_P, ARGON2_T, ENCRYPTION_PROFILE_V1, EncryptionProfile, KeyError,
    KeyProvider, RECOVERY_CODE_COUNT, UnlockMethod, VaultDek, VaultKeyHeader,
};
pub use recovery::{RecoveryCodeSet, generate_recovery_codes};
