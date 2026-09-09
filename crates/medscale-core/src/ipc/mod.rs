//! Localhost OS IPC for CoreFacade (Spec 024 READY_BASE).
//!
//! Framing: little-endian u32 length prefix + JSON AuthorityRequest/Response.
//! Transport: `interprocess` local sockets (Windows named pipe / Unix domain socket).

mod framing;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use interprocess::local_socket::prelude::*;
use interprocess::local_socket::{GenericNamespaced, Listener, ListenerOptions, Stream, ToNsName};
use medscale_contracts::envelopes::{AuthorityRequest, AuthorityResponse};
use medscale_storage::{WriterLock, WriterLockError};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::CoreFacade;

pub use framing::{MAX_FRAME_BYTES, read_frame, write_frame};

/// Errors from host OS IPC bind/serve/connect.
#[derive(Debug, Error)]
pub enum HostIpcError {
    #[error(transparent)]
    Writer(#[from] WriterLockError),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("ipc name: {0}")]
    Name(String),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("frame too large: {0} bytes")]
    FrameTooLarge(usize),
    #[error("server stopped")]
    ServerStopped,
}

/// Bound localhost IPC host: holds Spec 016 writer lock + Strict CoreFacade.
pub struct HostIpcServer {
    facade: Arc<CoreFacade>,
    _writer: WriterLock,
    listener: Listener,
    endpoint: String,
    vault_root: PathBuf,
}

impl HostIpcServer {
    /// Acquire writer lock and listen on a namespaced local-socket endpoint.
    pub fn bind(vault_root: &Path, endpoint: &str) -> Result<Self, HostIpcError> {
        let endpoint = sanitize_endpoint(endpoint)?;
        let writer = WriterLock::try_acquire(vault_root)?;
        let name = endpoint
            .clone()
            .to_ns_name::<GenericNamespaced>()
            .map_err(|e| HostIpcError::Name(e.to_string()))?;
        let listener = ListenerOptions::new()
            .name(name)
            .create_sync()
            .map_err(HostIpcError::Io)?;
        Ok(Self {
            facade: Arc::new(CoreFacade::new()),
            _writer: writer,
            listener,
            endpoint,
            vault_root: vault_root.to_path_buf(),
        })
    }

    /// Endpoint name clients must use.
    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Vault root this host exclusively locks.
    #[must_use]
    pub fn vault_root(&self) -> &Path {
        &self.vault_root
    }

    /// Shared Strict facade (for in-process assertions alongside IPC).
    #[must_use]
    pub fn facade(&self) -> &Arc<CoreFacade> {
        &self.facade
    }

    /// Accept one client and serve request/response frames until EOF.
    pub fn serve_connection(&self) -> Result<(), HostIpcError> {
        let stream = self.listener.accept().map_err(HostIpcError::Io)?;
        serve_stream(&self.facade, stream)
    }

    /// Move this server into a background accept loop (one connection handler thread each).
    pub fn into_background(self) -> (String, Arc<CoreFacade>, JoinHandle<()>) {
        let endpoint = self.endpoint.clone();
        let facade = Arc::clone(&self.facade);
        let handle = thread::spawn(move || {
            while let Ok(stream) = self.listener.accept() {
                let facade = Arc::clone(&self.facade);
                let _ = thread::spawn(move || {
                    let _ = serve_stream(&facade, stream);
                });
            }
        });
        (endpoint, facade, handle)
    }
}

fn serve_stream(facade: &CoreFacade, mut stream: Stream) -> Result<(), HostIpcError> {
    loop {
        let req_bytes = match read_frame(&mut stream) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(()),
            Err(e) => return Err(HostIpcError::Io(e)),
        };
        let req: AuthorityRequest = serde_json::from_slice(&req_bytes)?;
        let resp = facade.dispatch(req);
        let resp_bytes = serde_json::to_vec(&resp)?;
        write_frame(&mut stream, &resp_bytes)?;
    }
}

/// Client connected to a `HostIpcServer` endpoint.
pub struct HostIpcClient {
    stream: Stream,
    endpoint: String,
}

impl HostIpcClient {
    /// Connect to a namespaced local-socket endpoint.
    pub fn connect(endpoint: &str) -> Result<Self, HostIpcError> {
        let endpoint = sanitize_endpoint(endpoint)?;
        let stream = connect_with_retry(&endpoint, 50, Duration::from_millis(20))?;
        Ok(Self { stream, endpoint })
    }

    /// Endpoint this client is connected to.
    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Send one authority request and await the response.
    pub fn dispatch(&mut self, req: AuthorityRequest) -> Result<AuthorityResponse, HostIpcError> {
        let req_bytes = serde_json::to_vec(&req)?;
        write_frame(&mut self.stream, &req_bytes)?;
        let resp_bytes = read_frame(&mut self.stream).map_err(HostIpcError::Io)?;
        Ok(serde_json::from_slice(&resp_bytes)?)
    }
}

fn connect_with_retry(
    endpoint: &str,
    attempts: u32,
    delay: Duration,
) -> Result<Stream, HostIpcError> {
    let mut last = None;
    for _ in 0..attempts {
        let name = endpoint
            .to_ns_name::<GenericNamespaced>()
            .map_err(|e| HostIpcError::Name(e.to_string()))?;
        match Stream::connect(name) {
            Ok(s) => return Ok(s),
            Err(e) => {
                last = Some(e);
                thread::sleep(delay);
            }
        }
    }
    Err(HostIpcError::Io(last.unwrap_or_else(|| {
        std::io::Error::other("ipc connect failed")
    })))
}

fn sanitize_endpoint(endpoint: &str) -> Result<String, HostIpcError> {
    let trimmed = endpoint.trim();
    if trimmed.is_empty()
        || !trimmed
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(HostIpcError::Name(
            "endpoint must be non-empty [A-Za-z0-9_-]+".to_owned(),
        ));
    }
    Ok(trimmed.to_owned())
}

/// Derive a stable test/operator endpoint token from a vault root path.
#[must_use]
pub fn endpoint_for_vault_root(vault_root: &Path) -> String {
    let mut hasher = Sha256::new();
    hasher.update(vault_root.to_string_lossy().as_bytes());
    let digest = hasher.finalize();
    format!(
        "medscale-ipc-{}",
        digest
            .iter()
            .take(8)
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
    )
}
