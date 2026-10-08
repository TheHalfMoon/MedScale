//! Core surface for the local model catalog and verified snapshot admission
//! (Spec 103). Surfaces (CLI, Desktop) reach `medscale-pack` only through
//! Core. Catalog queries are read-only metadata and need no vault; building a
//! Pack from a snapshot only writes the Pack directory, and admission then goes
//! through the normal `PacksInstallLocal` capability.

use std::collections::BTreeMap;
use std::path::Path;

use medscale_pack::{
    CatalogFilter, CatalogRow, CatalogSource, CatalogStatus, DeviceFit, HfRepoMetadata,
    ModelCatalog, TokenClassifierSnapshot, build_token_classifier_pack,
};
use serde::Serialize;

pub use medscale_pack::{CatalogError, SnapshotError};

/// Query over a catalog snapshot (all fields optional).
#[derive(Debug, Default, Clone)]
pub struct CatalogQuery {
    pub text: Option<String>,
    pub task: Option<String>,
    pub family: Option<String>,
    pub language: Option<String>,
    pub format: Option<String>,
    pub architecture: Option<String>,
    pub license_claim: Option<String>,
    /// `cpu_portable`, `apple_silicon_only` or `unknown`.
    pub device_fit: Option<String>,
    /// For example `DISCOVERABLE` or `RIGHTS_PENDING`.
    pub status: Option<String>,
    pub max_params: Option<u64>,
    pub max_disk_mb: Option<f64>,
    pub offset: usize,
    pub limit: usize,
}

#[derive(Debug, Serialize)]
pub struct CatalogQueryView {
    pub source: CatalogSource,
    pub total_rows: usize,
    /// Every count is a listing count; none implies download or execution.
    pub status_counts: BTreeMap<CatalogStatus, usize>,
    pub total_matching: usize,
    pub next_offset: Option<usize>,
    pub rows: Vec<CatalogRow>,
}

#[derive(Debug, thiserror::Error)]
pub enum ModelCatalogError {
    #[error(transparent)]
    Catalog(#[from] CatalogError),
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
    #[error("unknown {field}: {value}")]
    InvalidArgument { field: &'static str, value: String },
    #[error("repository {0} is not in the catalog")]
    NotInCatalog(String),
    #[error("io: {0}")]
    Io(String),
}

fn parse_device(value: &str) -> Result<DeviceFit, ModelCatalogError> {
    match value {
        "cpu_portable" => Ok(DeviceFit::CpuPortable),
        "apple_silicon_only" => Ok(DeviceFit::AppleSiliconOnly),
        "unknown" => Ok(DeviceFit::Unknown),
        other => Err(ModelCatalogError::InvalidArgument {
            field: "device_fit",
            value: other.into(),
        }),
    }
}

fn parse_status(value: &str) -> Result<CatalogStatus, ModelCatalogError> {
    serde_json::from_value(serde_json::Value::String(value.to_ascii_uppercase())).map_err(|_| {
        ModelCatalogError::InvalidArgument {
            field: "status",
            value: value.into(),
        }
    })
}

fn load(
    manifest: &Path,
    repository: &str,
    commit: &str,
) -> Result<ModelCatalog, ModelCatalogError> {
    let bytes = std::fs::read(manifest).map_err(|e| ModelCatalogError::Io(e.to_string()))?;
    Ok(ModelCatalog::import(repository, commit, &bytes)?)
}

/// Read-only query of a locally supplied catalog snapshot. No network access.
pub fn query_catalog_snapshot(
    manifest: &Path,
    repository: &str,
    commit: &str,
    query: &CatalogQuery,
) -> Result<CatalogQueryView, ModelCatalogError> {
    let catalog = load(manifest, repository, commit)?;
    let filter = CatalogFilter {
        text: query.text.clone(),
        task: query.task.clone(),
        family: query.family.clone(),
        language: query.language.clone(),
        format: query.format.clone(),
        architecture: query.architecture.clone(),
        license_claim: query.license_claim.clone(),
        device_fit: query.device_fit.as_deref().map(parse_device).transpose()?,
        status: query.status.as_deref().map(parse_status).transpose()?,
        max_params: query.max_params,
        max_disk_mb: query.max_disk_mb,
    };
    let page = catalog.query(&filter, query.offset, query.limit)?;
    Ok(CatalogQueryView {
        total_matching: page.total_matching,
        next_offset: page.next_offset,
        rows: page.rows.into_iter().cloned().collect(),
        status_counts: catalog.status_counts(),
        total_rows: catalog.len(),
        source: catalog.source.clone(),
    })
}

/// Verifies a locally supplied Hub snapshot against its catalog row and writes
/// a signed token-classification Pack to `out_dir`. Returns the Pack id.
/// Admission is a separate `PacksInstallLocal` call.
pub fn build_pack_from_snapshot(
    manifest: &Path,
    repository: &str,
    commit: &str,
    snapshot_dir: &Path,
    onnx_file: &str,
    fixed_sequence_length: u32,
    out_dir: &Path,
) -> Result<String, ModelCatalogError> {
    let catalog = load(manifest, repository, commit)?;
    let meta_bytes = std::fs::read(snapshot_dir.join("metadata.json"))
        .map_err(|e| ModelCatalogError::Io(e.to_string()))?;
    let meta: HfRepoMetadata = serde_json::from_slice(&meta_bytes)
        .map_err(|e| ModelCatalogError::Io(format!("metadata.json: {e}")))?;
    let row = catalog
        .get(&meta.id)
        .ok_or_else(|| ModelCatalogError::NotInCatalog(meta.id.clone()))?;
    Ok(build_token_classifier_pack(
        &TokenClassifierSnapshot {
            meta: &meta,
            row,
            snapshot_dir,
            onnx_file,
            fixed_sequence_length,
        },
        out_dir,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMIT: &str = "ea920f36fadd7b45935247d639f0ffa1ef493b23";

    fn manifest() -> std::path::PathBuf {
        let rows = concat!(
            r#"{"repo_id":"Org/ner-en","family":"NER","task":"token-classification","languages":["en"],"formats":["onnx"],"param_count":33000000,"architecture":"bert","license":"apache-2.0"}"#,
            "\n",
            r#"{"repo_id":"Org/pii-ar","family":"PII","task":"token-classification","languages":["ar"],"formats":["mlx-fp"],"param_count":560000000,"architecture":"xlm-roberta","license":"apache-2.0"}"#,
            "\n",
            r#"{"repo_id":"Org/gen","family":"General","task":"text-generation","languages":["en"],"formats":["pytorch"],"architecture":"qwen","license":"other"}"#,
            "\n",
        );
        let path = std::env::temp_dir().join(format!(
            "medscale-core-catalog-{}.jsonl",
            std::process::id()
        ));
        std::fs::write(&path, rows).unwrap();
        path
    }

    #[test]
    fn query_filters_and_reports_listing_counts_only() {
        let path = manifest();
        let all = query_catalog_snapshot(
            &path,
            "org/fixture",
            COMMIT,
            &CatalogQuery {
                limit: 10,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(all.total_rows, 3);
        assert_eq!(
            all.status_counts.get(&CatalogStatus::Discoverable),
            Some(&2)
        );
        assert_eq!(
            all.status_counts.get(&CatalogStatus::RightsPending),
            Some(&1)
        );
        let cpu = query_catalog_snapshot(
            &path,
            "org/fixture",
            COMMIT,
            &CatalogQuery {
                device_fit: Some("cpu_portable".into()),
                status: Some("discoverable".into()),
                limit: 10,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(
            cpu.rows
                .iter()
                .map(|r| r.repo_id.as_str())
                .collect::<Vec<_>>(),
            ["Org/ner-en"]
        );
        assert!(
            query_catalog_snapshot(
                &path,
                "org/fixture",
                COMMIT,
                &CatalogQuery {
                    device_fit: Some("gpu".into()),
                    limit: 10,
                    ..Default::default()
                }
            )
            .is_err()
        );
        assert!(
            query_catalog_snapshot(
                &path,
                "org/fixture",
                COMMIT,
                &CatalogQuery {
                    status: Some("installed".into()),
                    limit: 10,
                    ..Default::default()
                }
            )
            .is_err()
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn snapshot_for_unknown_repository_is_refused_before_any_write() {
        let path = manifest();
        let dir = std::env::temp_dir().join(format!("medscale-core-snap-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("metadata.json"),
            format!(
                r#"{{"id":"Other/model","sha":"{}","siblings":[]}}"#,
                "a".repeat(40)
            ),
        )
        .unwrap();
        let out = dir.join("pack");
        let err =
            build_pack_from_snapshot(&path, "org/fixture", COMMIT, &dir, "model.onnx", 128, &out)
                .unwrap_err();
        assert!(matches!(err, ModelCatalogError::NotInCatalog(_)));
        assert!(!out.exists());
        let _ = std::fs::remove_dir_all(dir);
        let _ = std::fs::remove_file(path);
    }
}
