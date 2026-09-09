//! EncryptedVault: AES-GCM sealed metadata + sealed blobs (Spec 005 D4 primary ship path).

use std::fs;
use std::path::{Path, PathBuf};

use medscale_contracts::objects::DigestSha256;
use medscale_keys::{
    KeyProvider, KeyStore, RecoveryCodeSet, VaultDek, VaultKeyHeader, seal, unseal,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::claim::{ClaimError, assert_claim_path};
use crate::sealed_blob::{SealedBlobError, SealedBlobStore};
use crate::sqlite_meta::{MetaError, SourceMeta, SqliteMetaStore};

const HEADER_FILE: &str = "vault_header.json";
const META_SEALED: &str = "meta.sealed";
const META_WORK: &str = "meta.work.sqlite3";
const LEASE_FILE: &str = ".writer.lock";

#[derive(Debug, Error)]
pub enum EncryptedVaultError {
    #[error(transparent)]
    Claim(#[from] ClaimError),
    #[error(transparent)]
    Meta(#[from] MetaError),
    #[error(transparent)]
    Sealed(#[from] SealedBlobError),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("missing key material")]
    MissingKeyMaterial,
    #[error("lease held by {0}")]
    LeaseHeld(String),
    #[error("crypto")]
    Crypto,
    #[error("invalid: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OnDiskHeader {
    keys: VaultKeyHeader,
}

/// Open encrypted vault handle (DEK held only while unlocked).
#[derive(Debug)]
pub struct EncryptedVault {
    pub vault_id: String,
    pub root: PathBuf,
    pub header: VaultKeyHeader,
    dek: VaultDek,
    pub meta: SqliteMetaStore,
    pub blobs: SealedBlobStore,
    #[allow(dead_code)]
    lease_holder: String,
}

impl EncryptedVault {
    /// Create a new encrypted vault; returns recovery codes once.
    pub fn create(
        vault_id: &str,
        vault_root: &Path,
        passphrase: &str,
        holder_id: &str,
        keystore: Option<&dyn KeyStore>,
    ) -> Result<(Self, RecoveryCodeSet), EncryptedVaultError> {
        let root = assert_claim_path(vault_root)?;
        fs::create_dir_all(&root)?;
        if root.join(HEADER_FILE).exists() {
            return Err(EncryptedVaultError::Invalid(
                "vault already exists".to_owned(),
            ));
        }
        Self::acquire_lease(&root, holder_id)?;

        let (dek, header, codes) = KeyProvider::create_vault_keys(vault_id, passphrase)
            .map_err(|_| EncryptedVaultError::Crypto)?;
        if let Some(store) = keystore {
            KeyProvider::store_wrapped_dek(store, vault_id, &dek)
                .map_err(|_| EncryptedVaultError::MissingKeyMaterial)?;
        }

        let on_disk = OnDiskHeader {
            keys: header.clone(),
        };
        fs::write(
            root.join(HEADER_FILE),
            serde_json::to_vec_pretty(&on_disk).map_err(|_| EncryptedVaultError::Crypto)?,
        )?;

        let work = root.join(META_WORK);
        {
            let _meta = SqliteMetaStore::open_at(&work)?;
        }
        Self::seal_meta_file(&root, vault_id, &dek)?;
        Self::unseal_meta_file(&root, vault_id, &dek)?;
        let meta = SqliteMetaStore::open_at(&work)?;
        let blobs = SealedBlobStore::open(&root, vault_id)?;

        Ok((
            Self {
                vault_id: vault_id.to_owned(),
                root,
                header,
                dek,
                meta,
                blobs,
                lease_holder: holder_id.to_owned(),
            },
            codes,
        ))
    }

    pub fn open_with_passphrase(
        vault_root: &Path,
        passphrase: &str,
        holder_id: &str,
    ) -> Result<Self, EncryptedVaultError> {
        let root = assert_claim_path(vault_root)?;
        Self::acquire_lease(&root, holder_id)?;
        let header = Self::load_header(&root)?;
        let dek = KeyProvider::unlock_passphrase(&header, passphrase)
            .map_err(|_| EncryptedVaultError::MissingKeyMaterial)?;
        Self::finish_open(root, header, dek, holder_id)
    }

    pub fn open_with_recovery(
        vault_root: &Path,
        code: &str,
        holder_id: &str,
    ) -> Result<Self, EncryptedVaultError> {
        let root = assert_claim_path(vault_root)?;
        Self::acquire_lease(&root, holder_id)?;
        let header = Self::load_header(&root)?;
        let dek = KeyProvider::unlock_recovery(&header, code)
            .map_err(|_| EncryptedVaultError::MissingKeyMaterial)?;
        Self::finish_open(root, header, dek, holder_id)
    }

    pub fn open_with_keystore(
        vault_root: &Path,
        store: &dyn KeyStore,
        holder_id: &str,
    ) -> Result<Self, EncryptedVaultError> {
        let root = assert_claim_path(vault_root)?;
        Self::acquire_lease(&root, holder_id)?;
        let header = Self::load_header(&root)?;
        let dek = KeyProvider::unlock_keystore(store, &header)
            .map_err(|_| EncryptedVaultError::MissingKeyMaterial)?;
        Self::finish_open(root, header, dek, holder_id)
    }

    fn finish_open(
        root: PathBuf,
        header: VaultKeyHeader,
        dek: VaultDek,
        holder_id: &str,
    ) -> Result<Self, EncryptedVaultError> {
        // Crash leftovers are not treated as sealed; wipe then restore from meta.sealed.
        if Self::leftover_work_present(&root) {
            Self::wipe_work_sidecars(&root)?;
        }
        Self::unseal_meta_file(&root, &header.vault_id, &dek)?;
        let work = root.join(META_WORK);
        let meta = SqliteMetaStore::open_at(&work)?;
        let blobs = SealedBlobStore::open(&root, &header.vault_id)?;
        Ok(Self {
            vault_id: header.vault_id.clone(),
            root,
            header,
            dek,
            meta,
            blobs,
            lease_holder: holder_id.to_owned(),
        })
    }

    fn load_header(root: &Path) -> Result<VaultKeyHeader, EncryptedVaultError> {
        let bytes = fs::read(root.join(HEADER_FILE))?;
        let on_disk: OnDiskHeader =
            serde_json::from_slice(&bytes).map_err(|_| EncryptedVaultError::Crypto)?;
        Ok(on_disk.keys)
    }

    fn meta_aad(vault_id: &str) -> Vec<u8> {
        format!("meta:{vault_id}").into_bytes()
    }

    fn seal_meta_file(
        root: &Path,
        vault_id: &str,
        dek: &VaultDek,
    ) -> Result<(), EncryptedVaultError> {
        let work = root.join(META_WORK);
        if !work.exists() {
            return Err(EncryptedVaultError::Invalid(
                "missing meta work file".to_owned(),
            ));
        }
        let plain = fs::read(&work)?;
        let sealed = seal(dek.as_bytes(), &plain, &Self::meta_aad(vault_id))
            .map_err(|_| EncryptedVaultError::Crypto)?;
        fs::write(
            root.join(META_SEALED),
            serde_json::to_vec(&sealed).map_err(|_| EncryptedVaultError::Crypto)?,
        )?;
        Self::wipe_work_sidecars(root)?;
        Ok(())
    }

    fn unseal_meta_file(
        root: &Path,
        vault_id: &str,
        dek: &VaultDek,
    ) -> Result<(), EncryptedVaultError> {
        let sealed_path = root.join(META_SEALED);
        if !sealed_path.exists() {
            return Err(EncryptedVaultError::MissingKeyMaterial);
        }
        let encoded = fs::read(&sealed_path)?;
        let sealed: medscale_keys::WrappedBlob = serde_json::from_slice(&encoded)
            .map_err(|_| EncryptedVaultError::MissingKeyMaterial)?;
        let plain = unseal(dek.as_bytes(), &sealed, &Self::meta_aad(vault_id))
            .map_err(|_| EncryptedVaultError::MissingKeyMaterial)?;
        fs::write(root.join(META_WORK), plain)?;
        Ok(())
    }

    fn acquire_lease(root: &Path, holder_id: &str) -> Result<(), EncryptedVaultError> {
        let lease = root.join(LEASE_FILE);
        if lease.exists() {
            let existing = fs::read_to_string(&lease).unwrap_or_default();
            if existing != holder_id {
                return Err(EncryptedVaultError::LeaseHeld(existing));
            }
        }
        fs::write(&lease, holder_id)?;
        Ok(())
    }

    /// Persist sealed meta and release lease; zeros DEK via drop.
    pub fn close(self) -> Result<(), EncryptedVaultError> {
        let EncryptedVault {
            vault_id,
            root,
            dek,
            meta,
            ..
        } = self;
        drop(meta);
        Self::seal_meta_file(&root, &vault_id, &dek)?;
        Self::wipe_work_sidecars(&root)?;
        let _ = fs::remove_file(root.join(LEASE_FILE));
        Ok(())
    }

    /// Remove plaintext work DB and SQLite sidecar files if present.
    pub fn wipe_work_sidecars(root: &Path) -> Result<(), EncryptedVaultError> {
        let work = root.join(META_WORK);
        for path in [
            work.clone(),
            sqlite_sidecar(&work, "-wal"),
            sqlite_sidecar(&work, "-shm"),
            sqlite_sidecar(&work, "-journal"),
        ] {
            if path.exists() {
                // Best-effort overwrite then remove (not a secure OS wipe claim).
                if let Ok(meta) = fs::metadata(&path) {
                    let len = meta.len() as usize;
                    let _ = fs::write(&path, vec![0_u8; len.min(1024 * 1024)]);
                }
                let _ = fs::remove_file(&path);
            }
        }
        Ok(())
    }

    /// True when plaintext work or journal sidecars remain on disk.
    #[must_use]
    pub fn leftover_work_present(root: &Path) -> bool {
        let work = root.join(META_WORK);
        work.exists()
            || sqlite_sidecar(&work, "-wal").exists()
            || sqlite_sidecar(&work, "-shm").exists()
            || sqlite_sidecar(&work, "-journal").exists()
    }

    pub fn put_blob(&self, bytes: &[u8]) -> Result<DigestSha256, EncryptedVaultError> {
        Ok(self.blobs.put(&self.dek, bytes)?)
    }

    pub fn get_blob(&self, digest: &DigestSha256) -> Result<Vec<u8>, EncryptedVaultError> {
        Ok(self.blobs.get(&self.dek, digest)?)
    }

    pub fn insert_source(&self, meta: &SourceMeta) -> Result<(), EncryptedVaultError> {
        self.meta.insert_source(meta)?;
        Ok(())
    }

    /// Destroy key wraps (retention); leaves ciphertext unreadable.
    pub fn destroy_key_material(vault_root: &Path) -> Result<(), EncryptedVaultError> {
        let root = assert_claim_path(vault_root)?;
        let header_path = root.join(HEADER_FILE);
        if header_path.exists() {
            fs::remove_file(header_path)?;
        }
        Ok(())
    }

    /// Backup sealed vault tree (header + sealed meta + sealed blobs).
    pub fn backup(vault_root: &Path, dest: &Path) -> Result<(), EncryptedVaultError> {
        let root = assert_claim_path(vault_root)?;
        let dest = assert_claim_path(dest)?;
        fs::create_dir_all(&dest)?;
        for name in [HEADER_FILE, META_SEALED] {
            let src = root.join(name);
            if src.exists() {
                fs::copy(&src, dest.join(name))?;
            }
        }
        let blobs_src = root.join("sealed_blobs");
        if blobs_src.exists() {
            copy_dir(&blobs_src, &dest.join("sealed_blobs"))?;
        }
        Ok(())
    }

    pub fn restore(source: &Path, dest: &Path) -> Result<(), EncryptedVaultError> {
        let source = assert_claim_path(source)?;
        let dest = assert_claim_path(dest)?;
        fs::create_dir_all(&dest)?;
        for name in [HEADER_FILE, META_SEALED] {
            let src = source.join(name);
            if src.exists() {
                fs::copy(&src, dest.join(name))?;
            }
        }
        let blobs_src = source.join("sealed_blobs");
        if blobs_src.exists() {
            copy_dir(&blobs_src, &dest.join("sealed_blobs"))?;
        }
        Ok(())
    }

    /// Migrate Spec 003 SyntheticVault plaintext into a new EncryptedVault.
    pub fn migrate_from_synthetic(
        synthetic_root: &Path,
        encrypted_root: &Path,
        vault_id: &str,
        passphrase: &str,
        holder_id: &str,
    ) -> Result<(Self, RecoveryCodeSet), EncryptedVaultError> {
        let synth = crate::SyntheticVault::open(vault_id, synthetic_root)
            .map_err(|e| EncryptedVaultError::Invalid(e.to_string()))?;
        let (enc, codes) = Self::create(vault_id, encrypted_root, passphrase, holder_id, None)?;
        for src in synth.meta.list_sources()? {
            let bytes = synth
                .blobs
                .get_blob(&src.digest)
                .map_err(|e| EncryptedVaultError::Invalid(e.to_string()))?;
            let digest = enc.put_blob(&bytes)?;
            if digest != src.digest {
                return Err(EncryptedVaultError::Crypto);
            }
            enc.insert_source(&src)?;
        }
        Ok((enc, codes))
    }

    pub fn plaintext_marker_absent_on_disk(&self, marker: &[u8]) -> bool {
        if let Ok(bytes) = fs::read(self.root.join(META_SEALED)) {
            if bytes.windows(marker.len()).any(|w| w == marker) {
                return false;
            }
        }
        self.blobs.plaintext_marker_absent(marker)
    }
}

fn sqlite_sidecar(work: &Path, suffix: &str) -> PathBuf {
    let mut os = work.as_os_str().to_owned();
    os.push(suffix);
    PathBuf::from(os)
}

fn copy_dir(src: &Path, dest: &Path) -> Result<(), EncryptedVaultError> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dest.join(entry.file_name());
        if ty.is_dir() {
            copy_dir(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}

/// Resolve default local app-data vault path (not yet claim-checked).
#[must_use]
pub fn default_vault_root(vault_id: &str) -> PathBuf {
    let base = if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("MedScale")
            .join("vaults")
    } else {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local").join("share"))
            })
            .unwrap_or_else(|| PathBuf::from("."))
            .join("medscale")
            .join("vaults")
    };
    base.join(vault_id)
}
