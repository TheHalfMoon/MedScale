//! KeyProvider: passphrase Argon2id wraps + recovery + keystore.

use argon2::{Algorithm, Argon2, Params, Version};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::aead_wrap::{WrappedBlob, seal, unseal};
use crate::keystore::{KeyStore, KeyStoreError};
use crate::recovery::{
    DEFAULT_RECOVERY_COUNT, RecoveryCodeSet, generate_recovery_codes, unwrap_with_code,
    wrap_with_code,
};

/// Encryption profile id for vault header.
pub const ENCRYPTION_PROFILE_V1: &str = "medscale.vault.aesgcm.sealed.v1";

/// Argon2id memory (KiB) — CI/test-friendly; production may raise via profile bump.
pub const ARGON2_M_KIB: u32 = 19_456;
pub const ARGON2_T: u32 = 2;
pub const ARGON2_P: u32 = 1;
pub const RECOVERY_CODE_COUNT: usize = DEFAULT_RECOVERY_COUNT;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptionProfile {
    pub id: String,
    pub argon2_m_kib: u32,
    pub argon2_t: u32,
    pub argon2_p: u32,
}

impl EncryptionProfile {
    #[must_use]
    pub fn v1() -> Self {
        Self {
            id: ENCRYPTION_PROFILE_V1.to_owned(),
            argon2_m_kib: ARGON2_M_KIB,
            argon2_t: ARGON2_T,
            argon2_p: ARGON2_P,
        }
    }
}

/// Vault data encryption key (32 bytes).
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct VaultDek {
    bytes: [u8; 32],
}

impl VaultDek {
    #[must_use]
    pub fn generate() -> Self {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self { bytes }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KeyError> {
        if bytes.len() != 32 {
            return Err(KeyError::Crypto);
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        Ok(Self { bytes: arr })
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

impl std::fmt::Debug for VaultDek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("VaultDek([REDACTED])")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultKeyHeader {
    pub vault_id: String,
    pub profile: EncryptionProfile,
    pub salt: Vec<u8>,
    pub passphrase_wrap: WrappedBlob,
    pub recovery_wraps: Vec<WrappedBlob>,
    /// Revoked recovery wrap indexes (optional).
    pub revoked_recovery: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnlockMethod {
    Passphrase,
    RecoveryCode,
    KeyStore,
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum KeyError {
    #[error("missing key material")]
    MissingKeyMaterial,
    #[error("crypto failure")]
    Crypto,
    #[error("keystore: {0}")]
    KeyStore(String),
    #[error("invalid argument: {0}")]
    Invalid(String),
}

impl From<KeyStoreError> for KeyError {
    fn from(value: KeyStoreError) -> Self {
        Self::KeyStore(value.to_string())
    }
}

/// Creates and unlocks vault key material.
pub struct KeyProvider;

impl KeyProvider {
    fn argon2(profile: &EncryptionProfile) -> Result<Argon2<'static>, KeyError> {
        let params = Params::new(
            profile.argon2_m_kib,
            profile.argon2_t,
            profile.argon2_p,
            None,
        )
        .map_err(|_| KeyError::Crypto)?;
        Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
    }

    fn derive_kek(
        passphrase: &str,
        salt: &[u8],
        profile: &EncryptionProfile,
    ) -> Result<[u8; 32], KeyError> {
        let argon = Self::argon2(profile)?;
        let mut out = [0u8; 32];
        argon
            .hash_password_into(passphrase.as_bytes(), salt, &mut out)
            .map_err(|_| KeyError::Crypto)?;
        Ok(out)
    }

    fn aad(vault_id: &str) -> Vec<u8> {
        format!("medscale.vault.aad.v1:{vault_id}").into_bytes()
    }

    /// Create new DEK + header + recovery codes (codes returned once).
    pub fn create_vault_keys(
        vault_id: &str,
        passphrase: &str,
    ) -> Result<(VaultDek, VaultKeyHeader, RecoveryCodeSet), KeyError> {
        if passphrase.is_empty() {
            return Err(KeyError::Invalid("empty passphrase".to_owned()));
        }
        let profile = EncryptionProfile::v1();
        let dek = VaultDek::generate();
        let mut salt = vec![0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt);
        let aad = Self::aad(vault_id);
        let kek = Self::derive_kek(passphrase, &salt, &profile)?;
        let passphrase_wrap = seal(&kek, dek.as_bytes(), &aad).map_err(|_| KeyError::Crypto)?;

        let codes = generate_recovery_codes(RECOVERY_CODE_COUNT);
        let mut recovery_wraps = Vec::new();
        for code in &codes.codes {
            recovery_wraps.push(wrap_with_code(&dek, code, &aad)?);
        }

        let header = VaultKeyHeader {
            vault_id: vault_id.to_owned(),
            profile,
            salt,
            passphrase_wrap,
            recovery_wraps,
            revoked_recovery: vec![],
        };
        Ok((dek, header, codes))
    }

    pub fn unlock_passphrase(
        header: &VaultKeyHeader,
        passphrase: &str,
    ) -> Result<VaultDek, KeyError> {
        let aad = Self::aad(&header.vault_id);
        let kek = Self::derive_kek(passphrase, &header.salt, &header.profile)?;
        let bytes = unseal(&kek, &header.passphrase_wrap, &aad)
            .map_err(|_| KeyError::MissingKeyMaterial)?;
        VaultDek::from_bytes(&bytes)
    }

    pub fn unlock_recovery(header: &VaultKeyHeader, code: &str) -> Result<VaultDek, KeyError> {
        let aad = Self::aad(&header.vault_id);
        for (idx, wrap) in header.recovery_wraps.iter().enumerate() {
            if header.revoked_recovery.contains(&idx) {
                continue;
            }
            if let Ok(dek) = unwrap_with_code(wrap, code, &aad) {
                return Ok(dek);
            }
        }
        Err(KeyError::MissingKeyMaterial)
    }

    pub fn keystore_account(vault_id: &str) -> String {
        format!("medscale.vault.{vault_id}")
    }

    /// Store wrapped DEK bytes in keystore (wrapped under a random OS wrap key stored alongside — MVP stores passphrase_wrap ciphertext only as unlock token is the sealed DEK under a keystore-local random key).
    pub fn store_wrapped_dek(
        store: &dyn KeyStore,
        vault_id: &str,
        dek: &VaultDek,
    ) -> Result<(), KeyError> {
        let mut wrap_key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut wrap_key);
        let aad = Self::aad(vault_id);
        let sealed = seal(&wrap_key, dek.as_bytes(), &aad).map_err(|_| KeyError::Crypto)?;
        let payload =
            serde_json::to_vec(&(wrap_key.to_vec(), sealed)).map_err(|_| KeyError::Crypto)?;
        wrap_key.zeroize();
        store.put(&Self::keystore_account(vault_id), &payload)?;
        Ok(())
    }

    pub fn unlock_keystore(
        store: &dyn KeyStore,
        header: &VaultKeyHeader,
    ) -> Result<VaultDek, KeyError> {
        let payload = store.get(&Self::keystore_account(&header.vault_id))?;
        let (wrap_key, sealed): (Vec<u8>, WrappedBlob) =
            serde_json::from_slice(&payload).map_err(|_| KeyError::MissingKeyMaterial)?;
        let aad = Self::aad(&header.vault_id);
        let bytes = unseal(&wrap_key, &sealed, &aad).map_err(|_| KeyError::MissingKeyMaterial)?;
        VaultDek::from_bytes(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keystore::MemoryKeyStore;

    #[test]
    fn passphrase_and_recovery_roundtrip() {
        let (dek, header, codes) =
            KeyProvider::create_vault_keys("v1", "correct horse battery").unwrap();
        let u1 = KeyProvider::unlock_passphrase(&header, "correct horse battery").unwrap();
        assert_eq!(u1.as_bytes(), dek.as_bytes());
        let u2 = KeyProvider::unlock_recovery(&header, &codes.codes[0]).unwrap();
        assert_eq!(u2.as_bytes(), dek.as_bytes());
        assert!(KeyProvider::unlock_passphrase(&header, "wrong").is_err());
    }

    #[test]
    fn keystore_roundtrip_and_wipe() {
        let store = MemoryKeyStore::new();
        let (dek, header, _) = KeyProvider::create_vault_keys("v2", "pw").unwrap();
        KeyProvider::store_wrapped_dek(&store, "v2", &dek).unwrap();
        let u = KeyProvider::unlock_keystore(&store, &header).unwrap();
        assert_eq!(u.as_bytes(), dek.as_bytes());
        store.wipe();
        assert!(KeyProvider::unlock_keystore(&store, &header).is_err());
    }
}
