//! Format gates and local-path admission.

use std::fs;
use std::path::Path;

use medscale_contracts::objects::{DigestSha256, OpaqueId};
use medscale_contracts::packs::{
    PackAdmitReason, PackArtifactEntry, PackArtifactKind, PackManifestV0, PackPromotionState,
};
use serde::Deserialize;
use thiserror::Error;

/// Admission errors before store insert.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AdmitError {
    #[error("invalid manifest: {0}")]
    InvalidManifest(String),
    #[error("missing rights_uri")]
    MissingRights,
    #[error("forbidden artifact kind")]
    ForbiddenKind,
    #[error("digest mismatch")]
    DigestMismatch,
    #[error("io: {0}")]
    Io(String),
}

impl AdmitError {
    #[must_use]
    pub fn reason(&self) -> PackAdmitReason {
        match self {
            Self::MissingRights => PackAdmitReason::MissingRights,
            Self::ForbiddenKind => PackAdmitReason::ForbiddenArtifactKind,
            Self::DigestMismatch => PackAdmitReason::DigestMismatch,
            Self::InvalidManifest(_) | Self::Io(_) => PackAdmitReason::InvalidManifest,
        }
    }
}

/// Returns deny reason if kind is forbidden by Spec 008 default policy.
#[must_use]
pub fn forbidden_reason(kind: PackArtifactKind) -> Option<PackAdmitReason> {
    kind.is_forbidden_by_default()
        .then_some(PackAdmitReason::ForbiddenArtifactKind)
}

fn parse_hex_digest(hex: &str) -> Result<DigestSha256, AdmitError> {
    let hex = hex.trim();
    if hex.len() != 64 {
        return Err(AdmitError::InvalidManifest(
            "digest must be 64 hex chars".into(),
        ));
    }
    let mut bytes = [0_u8; 32];
    for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
        let s = std::str::from_utf8(chunk)
            .map_err(|_| AdmitError::InvalidManifest("digest utf8".into()))?;
        bytes[i] = u8::from_str_radix(s, 16)
            .map_err(|_| AdmitError::InvalidManifest("digest hex".into()))?;
    }
    Ok(DigestSha256::from_bytes(bytes))
}

#[derive(Debug, Deserialize)]
struct WireArtifact {
    relative_path: String,
    kind: PackArtifactKind,
    digest: String,
}

#[derive(Debug, Deserialize)]
struct WireManifest {
    pack_id: String,
    version: String,
    content_digest: String,
    artifacts: Vec<WireArtifact>,
    rights_uri: String,
    sbom_ref: String,
    runtime_requirements: String,
    #[serde(default)]
    benchmark_links: Vec<String>,
    promotion_state: PackPromotionState,
}

/// Load and validate a pack directory containing `pack.manifest.json` + artifacts.
pub fn admit_pack_dir(path: &Path) -> Result<PackManifestV0, AdmitError> {
    let manifest_path = path.join("pack.manifest.json");
    let raw = fs::read(&manifest_path).map_err(|e| AdmitError::Io(e.to_string()))?;
    let wire: WireManifest =
        serde_json::from_slice(&raw).map_err(|e| AdmitError::InvalidManifest(e.to_string()))?;

    if wire.rights_uri.trim().is_empty() {
        return Err(AdmitError::MissingRights);
    }
    if wire.sbom_ref.trim().is_empty() {
        return Err(AdmitError::InvalidManifest("empty sbom_ref".into()));
    }

    let mut artifacts = Vec::new();
    for art in &wire.artifacts {
        if forbidden_reason(art.kind).is_some() {
            return Err(AdmitError::ForbiddenKind);
        }
        let file = path.join(&art.relative_path);
        let mut bytes = fs::read(&file).map_err(|e| AdmitError::Io(e.to_string()))?;
        // Text fixture kinds are LF-canonical (Windows checkouts must not change digests).
        if matches!(
            art.kind,
            PackArtifactKind::FixtureBytes | PackArtifactKind::TokenizerMeta
        ) {
            bytes = normalize_lf(bytes);
        }
        let dig = DigestSha256::of(&bytes);
        let expected = parse_hex_digest(&art.digest)?;
        if dig != expected {
            return Err(AdmitError::DigestMismatch);
        }
        artifacts.push(PackArtifactEntry {
            relative_path: art.relative_path.clone(),
            kind: art.kind,
            digest: dig,
        });
    }

    let mut arts = artifacts.clone();
    arts.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    let mut concat = String::new();
    for a in &arts {
        concat.push_str(&a.digest.to_hex());
        concat.push('\n');
    }
    let expected_content = DigestSha256::of(concat.as_bytes());
    let declared = parse_hex_digest(&wire.content_digest)?;
    if expected_content != declared {
        return Err(AdmitError::DigestMismatch);
    }

    let mut promotion_state = wire.promotion_state;
    if !matches!(
        promotion_state,
        PackPromotionState::Candidate | PackPromotionState::LastGreen
    ) {
        promotion_state = PackPromotionState::Candidate;
    }

    Ok(PackManifestV0 {
        pack_id: OpaqueId::new(wire.pack_id),
        version: wire.version,
        content_digest: declared,
        artifacts,
        rights_uri: wire.rights_uri,
        sbom_ref: wire.sbom_ref,
        runtime_requirements: wire.runtime_requirements,
        benchmark_links: wire.benchmark_links,
        promotion_state,
    })
}
