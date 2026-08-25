//! KeyStore trait + in-process memory store (CI default).

use std::collections::HashMap;
use std::sync::Mutex;

use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum KeyStoreError {
    #[error("not found")]
    NotFound,
    #[error("store error: {0}")]
    Other(String),
}

/// Stores wrapped unlock material only — never ambient secrets.
pub trait KeyStore: Send + Sync {
    fn put(&self, account: &str, secret: &[u8]) -> Result<(), KeyStoreError>;
    fn get(&self, account: &str) -> Result<Vec<u8>, KeyStoreError>;
    fn delete(&self, account: &str) -> Result<(), KeyStoreError>;
}

/// In-process mock / CI store.
#[derive(Debug, Default)]
pub struct MemoryKeyStore {
    inner: Mutex<HashMap<String, Vec<u8>>>,
}

impl MemoryKeyStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Wipe all entries (keyring loss simulation).
    pub fn wipe(&self) {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear();
    }
}

impl KeyStore for MemoryKeyStore {
    fn put(&self, account: &str, secret: &[u8]) -> Result<(), KeyStoreError> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(account.to_owned(), secret.to_vec());
        Ok(())
    }

    fn get(&self, account: &str) -> Result<Vec<u8>, KeyStoreError> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(account)
            .cloned()
            .ok_or(KeyStoreError::NotFound)
    }

    fn delete(&self, account: &str) -> Result<(), KeyStoreError> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(account);
        Ok(())
    }
}
