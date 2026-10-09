//! Governed, opt-in model acquisition (Spec 103 ACQUIRE).
//!
//! Downloads exactly the files a catalog row needs for local execution
//! (`config.json`, `tokenizer.json` and one ONNX file) from the Hugging Face
//! Hub, only with an explicit per-model consent that names the repository, the
//! ONNX file and the maximum number of bytes. The egress seam is the existing
//! Governed Browse transport (`medscale_network::BrowseTransport`):
//! - HTTPS on port 443 only;
//! - public addresses only;
//! - redirects are never followed by the transport; every hop is re-validated
//!   here against a fixed Hub host allowlist.
//!
//! Integrity follows the ported OpenMed logic in `medscale_pack::hf_snapshot`:
//! - Hub metadata is bound to the catalog row by the reproducibility hash;
//! - files come only from that pinned commit;
//! - each file is checked for exact size and digest before it is kept.
//!
//! Files are written to a staging directory and moved into place only after
//! every file verified; any failure leaves nothing behind. No patient or
//! research input is ever sent: requests carry only public repository paths.

use std::path::{Path, PathBuf};
use std::time::Duration;

use medscale_network::{BrowseTransport, BrowseTransportError, resolve_redirect, validate_url};
use medscale_pack::{
    CatalogRow, HfRepoMetadata, ModelCatalog, SnapshotError, verify_against_catalog, verify_file,
};
use serde::Serialize;

/// Hub hosts an acquisition may contact (API, file resolution and LFS/Xet CDNs).
pub const HUB_HOSTS: &[&str] = &[
    "huggingface.co",
    "cdn-lfs.huggingface.co",
    "cdn-lfs-us-1.huggingface.co",
    "cdn-lfs-eu-1.huggingface.co",
    "cas-bridge.xethub.hf.co",
];
const MAX_HOPS: usize = 4;
const METADATA_MAX_BYTES: usize = 4 * 1024 * 1024;
const JSON_MAX_BYTES: u64 = 64 * 1024 * 1024;
/// Same ceiling as the Pack admission bound for ONNX models.
const ONNX_MAX_BYTES: u64 = 1024 * 1024 * 1024;
const TIMEOUT: Duration = Duration::from_secs(600);

/// Explicit, per-model consent captured from the person before any request.
#[derive(Debug, Clone)]
pub struct AcquisitionConsent {
    pub repo_id: String,
    pub onnx_file: String,
    /// Upper bound the person approved for the total download.
    pub approved_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AcquiredFile {
    pub name: String,
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AcquisitionReport {
    pub repo_id: String,
    pub revision: String,
    pub snapshot_dir: PathBuf,
    pub files: Vec<AcquiredFile>,
    pub total_bytes: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum AcquisitionError {
    #[error("catalog: {0}")]
    Catalog(String),
    #[error("{0} is not in the catalog")]
    NotInCatalog(String),
    #[error("consent does not cover {0}")]
    ConsentMismatch(String),
    #[error("download needs {needed} bytes; consent approved {approved}")]
    ExceedsConsent { needed: u64, approved: u64 },
    #[error("{0}: exact size unknown in Hub metadata; refusing")]
    UnknownSize(String),
    #[error("{name}: {bytes} bytes exceeds the {limit}-byte bound")]
    TooLarge {
        name: String,
        bytes: u64,
        limit: u64,
    },
    #[error("host {0} is not an allowed Hub host")]
    HostNotAllowed(String),
    #[error("URL refused: {0}")]
    UrlRefused(String),
    #[error("too many redirects")]
    TooManyRedirects,
    #[error("HTTP {status} for {url}")]
    Http { status: u16, url: String },
    #[error("transport: {0:?}")]
    Transport(BrowseTransportError),
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
    #[error("metadata: {0}")]
    Metadata(String),
    #[error("io: {0}")]
    Io(String),
}

fn io(e: &std::io::Error) -> AcquisitionError {
    AcquisitionError::Io(e.to_string())
}

/// One governed GET, following redirects only to allowed Hub hosts.
fn governed_get(
    transport: &dyn BrowseTransport,
    url: &str,
    max: usize,
) -> Result<Vec<u8>, AcquisitionError> {
    let mut next = url.to_string();
    for _ in 0..=MAX_HOPS {
        let validated =
            validate_url(&next).map_err(|r| AcquisitionError::UrlRefused(format!("{r:?}")))?;
        if !HUB_HOSTS.contains(&validated.host.as_str()) {
            return Err(AcquisitionError::HostNotAllowed(validated.host));
        }
        let response = transport
            .get(&validated, max, TIMEOUT)
            .map_err(AcquisitionError::Transport)?;
        match response.status {
            200 => {
                if response.body.len() > max {
                    return Err(AcquisitionError::TooLarge {
                        name: validated.path,
                        bytes: response.body.len() as u64,
                        limit: max as u64,
                    });
                }
                return Ok(response.body);
            }
            301 | 302 | 303 | 307 | 308 => {
                let location = response.location.ok_or(AcquisitionError::Http {
                    status: response.status,
                    url: validated.url.clone(),
                })?;
                next = resolve_redirect(&validated, &location);
            }
            status => {
                return Err(AcquisitionError::Http {
                    status,
                    url: validated.url,
                });
            }
        }
    }
    Err(AcquisitionError::TooManyRedirects)
}

fn expected_size(meta: &HfRepoMetadata, name: &str) -> Result<u64, AcquisitionError> {
    let sibling = meta
        .siblings
        .iter()
        .find(|s| s.rfilename == name)
        .ok_or_else(|| {
            AcquisitionError::Snapshot(SnapshotError::NotInSnapshot(name.to_string()))
        })?;
    sibling
        .lfs
        .as_ref()
        .and_then(|l| l.size)
        .or(sibling.size)
        .ok_or_else(|| AcquisitionError::UnknownSize(name.to_string()))
}

/// Acquires the snapshot files for one catalog row into `dest_dir`
/// (`metadata.json`, `config.json`, `tokenizer.json`, the ONNX file).
pub fn acquire_snapshot(
    transport: &dyn BrowseTransport,
    catalog: &ModelCatalog,
    consent: &AcquisitionConsent,
    dest_dir: &Path,
) -> Result<AcquisitionReport, AcquisitionError> {
    let row: &CatalogRow = catalog
        .get(&consent.repo_id)
        .ok_or_else(|| AcquisitionError::NotInCatalog(consent.repo_id.clone()))?;
    if !consent.onnx_file.to_ascii_lowercase().ends_with(".onnx") {
        return Err(AcquisitionError::ConsentMismatch(consent.onnx_file.clone()));
    }

    // 1. Metadata, bound to the catalog row (immutable commit and file list).
    let meta_bytes = governed_get(
        transport,
        &format!(
            "https://huggingface.co/api/models/{}?blobs=true",
            row.repo_id
        ),
        METADATA_MAX_BYTES,
    )?;
    let meta: HfRepoMetadata = serde_json::from_slice(&meta_bytes)
        .map_err(|e| AcquisitionError::Metadata(e.to_string()))?;
    verify_against_catalog(&meta, row)?;

    // 2. Exact sizes and the consent bound, before any file download.
    let files = ["config.json", "tokenizer.json", consent.onnx_file.as_str()];
    let mut total = 0u64;
    for name in files {
        let bytes = expected_size(&meta, name)?;
        let limit = if name == consent.onnx_file {
            ONNX_MAX_BYTES
        } else {
            JSON_MAX_BYTES
        };
        if bytes > limit {
            return Err(AcquisitionError::TooLarge {
                name: name.to_string(),
                bytes,
                limit,
            });
        }
        total += bytes;
    }
    if total > consent.approved_bytes {
        return Err(AcquisitionError::ExceedsConsent {
            needed: total,
            approved: consent.approved_bytes,
        });
    }

    // 3. Download into staging; verify each file; publish atomically.
    let staging = dest_dir.with_extension("partial");
    let _ = std::fs::remove_dir_all(&staging);
    std::fs::create_dir_all(&staging).map_err(|e| io(&e))?;
    let result: Result<Vec<AcquiredFile>, AcquisitionError> = (|| {
        let mut acquired = Vec::new();
        for name in files {
            let size = expected_size(&meta, name)?;
            let url = format!(
                "https://huggingface.co/{}/resolve/{}/{name}",
                meta.id, meta.sha
            );
            let max = usize::try_from(size).unwrap_or(usize::MAX);
            let body = governed_get(transport, &url, max)?;
            if body.len() as u64 != size {
                return Err(AcquisitionError::Snapshot(SnapshotError::SizeMismatch {
                    path: name.to_string(),
                }));
            }
            let sha256 = verify_file(&meta, name, &body)?;
            std::fs::write(staging.join(name), &body).map_err(|e| io(&e))?;
            acquired.push(AcquiredFile {
                name: name.to_string(),
                bytes: size,
                sha256,
            });
        }
        std::fs::write(staging.join("metadata.json"), &meta_bytes).map_err(|e| io(&e))?;
        Ok(acquired)
    })();
    let acquired = match result {
        Ok(a) => a,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&staging);
            return Err(e);
        }
    };
    let _ = std::fs::remove_dir_all(dest_dir);
    std::fs::rename(&staging, dest_dir).map_err(|e| io(&e))?;
    Ok(AcquisitionReport {
        repo_id: meta.id.clone(),
        revision: meta.sha.clone(),
        snapshot_dir: dest_dir.to_path_buf(),
        files: acquired,
        total_bytes: total,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use medscale_network::ScriptedBrowseTransport;
    use sha2::{Digest, Sha256};

    const COMMIT: &str = "ea920f36fadd7b45935247d639f0ffa1ef493b23";
    const SHA: &str = "54c9cc119d325ddad23300a1e9108bafa9643a06";
    const REPO: &str = "OpenMed/Synthetic-NER-onnx";

    fn hex(b: &[u8]) -> String {
        b.iter().map(|x| format!("{x:02x}")).collect()
    }
    fn git_blob(b: &[u8]) -> String {
        let mut h = sha1::Sha1::new();
        h.update(format!("blob {}\0", b.len()).as_bytes());
        h.update(b);
        hex(&h.finalize())
    }

    struct Fixture {
        config: Vec<u8>,
        tokenizer: Vec<u8>,
        onnx: Vec<u8>,
        meta: String,
        catalog: ModelCatalog,
    }

    fn fixture(gated: bool) -> Fixture {
        let config = br#"{"id2label":{"0":"O","1":"B-X"}}"#.to_vec();
        let tokenizer = br#"{"version":"1.0"}"#.to_vec();
        let onnx = vec![42u8; 4096];
        let meta = format!(
            r#"{{"id":"{REPO}","sha":"{SHA}","lastModified":"2026-07-13T07:43:16.000Z","gated":{},"private":false,"siblings":[{{"rfilename":"config.json","size":{},"blobId":"{}"}},{{"rfilename":"model.onnx","size":4096,"lfs":{{"sha256":"{}","size":4096}}}},{{"rfilename":"tokenizer.json","size":{},"blobId":"{}"}}]}}"#,
            if gated { "\"auto\"" } else { "false" },
            config.len(),
            git_blob(&config),
            hex(&Sha256::digest(&onnx)),
            tokenizer.len(),
            git_blob(&tokenizer)
        );
        let parsed: HfRepoMetadata =
            serde_json::from_str(&meta.replace("\"auto\"", "false")).unwrap();
        let hash = medscale_pack::reproducibility_hash(&parsed);
        let row = format!(
            r#"{{"repo_id":"{REPO}","family":"NER","task":"token-classification","formats":["onnx"],"license":"apache-2.0","reproducibility_hash":"{hash}"}}"#
        );
        let catalog = ModelCatalog::import("s/f", COMMIT, row.as_bytes()).unwrap();
        Fixture {
            config,
            tokenizer,
            onnx,
            meta,
            catalog,
        }
    }

    fn transport(f: &Fixture, onnx_body: &[u8], cdn: &str) -> ScriptedBrowseTransport {
        let base = format!("https://huggingface.co/{REPO}/resolve/{SHA}");
        ScriptedBrowseTransport::new()
            .route(
                &format!("https://huggingface.co/api/models/{REPO}?blobs=true"),
                ScriptedBrowseTransport::ok("application/json", f.meta.as_bytes()),
            )
            .route(
                &format!("{base}/config.json"),
                ScriptedBrowseTransport::ok("application/json", &f.config),
            )
            .route(
                &format!("{base}/tokenizer.json"),
                ScriptedBrowseTransport::ok("application/json", &f.tokenizer),
            )
            .route(
                &format!("{base}/model.onnx"),
                ScriptedBrowseTransport::redirect(&format!("https://{cdn}/blobs/abc")),
            )
            .route(
                &format!("https://{cdn}/blobs/abc"),
                ScriptedBrowseTransport::ok("application/octet-stream", onnx_body),
            )
    }

    fn consent(bytes: u64) -> AcquisitionConsent {
        AcquisitionConsent {
            repo_id: REPO.into(),
            onnx_file: "model.onnx".into(),
            approved_bytes: bytes,
        }
    }

    fn dest(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!("medscale-acq-{tag}-{}", std::process::id()))
    }

    #[test]
    fn acquires_verified_files_through_an_allowed_cdn_redirect() {
        let f = fixture(false);
        let t = transport(&f, &f.onnx, "cdn-lfs.huggingface.co");
        let d = dest("ok");
        let report = acquire_snapshot(&t, &f.catalog, &consent(1_000_000), &d).unwrap();
        assert_eq!(report.revision, SHA);
        assert_eq!(
            report.total_bytes,
            (f.config.len() + f.tokenizer.len() + 4096) as u64
        );
        for name in [
            "metadata.json",
            "config.json",
            "tokenizer.json",
            "model.onnx",
        ] {
            assert!(d.join(name).is_file(), "{name}");
        }
        assert!(!d.with_extension("partial").exists());
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn tampered_artifact_leaves_nothing_behind() {
        let f = fixture(false);
        let t = transport(&f, &vec![7u8; 4096], "cdn-lfs.huggingface.co");
        let d = dest("tamper");
        let err = acquire_snapshot(&t, &f.catalog, &consent(1_000_000), &d).unwrap_err();
        assert!(
            matches!(
                err,
                AcquisitionError::Snapshot(SnapshotError::DigestMismatch { .. })
            ),
            "{err}"
        );
        assert!(!d.exists() && !d.with_extension("partial").exists());
    }

    #[test]
    fn redirect_to_a_non_hub_host_is_refused() {
        let f = fixture(false);
        let t = transport(&f, &f.onnx, "evil.example.com");
        let err = acquire_snapshot(&t, &f.catalog, &consent(1_000_000), &dest("host")).unwrap_err();
        assert!(matches!(err, AcquisitionError::HostNotAllowed(h) if h == "evil.example.com"));
    }

    #[test]
    fn insufficient_consent_refuses_before_any_file_download() {
        let f = fixture(false);
        // Only metadata is scripted: a file request would fail as a transport error.
        let t = ScriptedBrowseTransport::new().route(
            &format!("https://huggingface.co/api/models/{REPO}?blobs=true"),
            ScriptedBrowseTransport::ok("application/json", f.meta.as_bytes()),
        );
        let err = acquire_snapshot(&t, &f.catalog, &consent(100), &dest("consent")).unwrap_err();
        assert!(matches!(
            err,
            AcquisitionError::ExceedsConsent { approved: 100, .. }
        ));
    }

    #[test]
    fn gated_repositories_and_unlisted_models_are_refused() {
        let f = fixture(true);
        let t = transport(&f, &f.onnx, "cdn-lfs.huggingface.co");
        let err =
            acquire_snapshot(&t, &f.catalog, &consent(1_000_000), &dest("gated")).unwrap_err();
        assert!(
            matches!(err, AcquisitionError::Snapshot(SnapshotError::Gated)),
            "{err}"
        );
        let mut other = consent(1_000_000);
        other.repo_id = "OpenMed/not-listed".into();
        assert!(matches!(
            acquire_snapshot(&t, &f.catalog, &other, &dest("unlisted")),
            Err(AcquisitionError::NotInCatalog(_))
        ));
    }
}
