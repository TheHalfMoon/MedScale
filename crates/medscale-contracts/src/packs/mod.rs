//! Pack v0 contracts (Spec 008) — offline admission only.

use serde::{Deserialize, Serialize};

use crate::objects::{DigestSha256, OpaqueId};

/// Promotion lifecycle for admitted packs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackPromotionState {
    Candidate,
    Current,
    LastGreen,
    Canary,
}

/// Artifact class declared in a pack (fail-closed on forbidden kinds).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackArtifactKind {
    FixtureBytes,
    TokenizerMeta,
    OnnxModel,
    /// Forbidden by default.
    Pickle,
    /// Forbidden by default (code-bearing).
    CodeBin,
    /// Forbidden unless signed custom-op admission (not in Spec 008).
    OnnxCustomOp,
}

impl PackArtifactKind {
    /// Returns true when Spec 008 must deny admission.
    #[must_use]
    pub const fn is_forbidden_by_default(self) -> bool {
        matches!(self, Self::Pickle | Self::CodeBin | Self::OnnxCustomOp)
    }
}

/// One artifact entry in PackManifestV0.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackArtifactEntry {
    pub relative_path: String,
    pub kind: PackArtifactKind,
    pub digest: DigestSha256,
}

/// Content-addressed Pack v0 manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackManifestV0 {
    pub pack_id: OpaqueId,
    pub version: String,
    pub content_digest: DigestSha256,
    pub artifacts: Vec<PackArtifactEntry>,
    pub rights_uri: String,
    pub sbom_ref: String,
    pub runtime_requirements: String,
    pub benchmark_links: Vec<String>,
    pub promotion_state: PackPromotionState,
}

/// Doctor axis for packs / runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PacksRuntimeDoctorStatus {
    pub present: bool,
    pub offline_only: bool,
    pub admitted_count: u32,
    pub current_pack_id: Option<String>,
    pub confinement_claim: String,
    pub online_download_authorized: bool,
}

/// Admission decision reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackAdmitReason {
    Ok,
    MissingRights,
    DigestMismatch,
    ForbiddenArtifactKind,
    InvalidManifest,
    OnlinePathRefused,
}

/// Result of local pack install.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackAdmitResult {
    pub admitted: bool,
    pub reason: PackAdmitReason,
    pub pack_id: Option<OpaqueId>,
    pub audit_id: OpaqueId,
}
