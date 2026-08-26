//! MESC artifact admission contracts (Spec 012 — fail-closed while gate open).
//!
//! ARTIFACT_FIRST only. Never Python import / shared DB / shared keys.

use serde::{Deserialize, Serialize};

use crate::objects::DigestSha256;

/// Doctor axis for MESC artifact integration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MescArtifactDoctorStatus {
    pub present: bool,
    pub artifact_admitted: bool,
    pub python_runtime_imported: bool,
    pub shared_db_or_keys: bool,
    pub disposition: String,
    pub gate: String,
}

impl MescArtifactDoctorStatus {
    /// Gate-open defaults: contracts present; nothing admitted.
    #[must_use]
    pub fn gate_blocked() -> Self {
        Self {
            present: true,
            artifact_admitted: false,
            python_runtime_imported: false,
            shared_db_or_keys: false,
            disposition: "ARTIFACT_IMPORT".to_owned(),
            gate: "MESC_RELEASED_ARTIFACT".to_owned(),
        }
    }
}

/// Request to admit a released MESC artifact through Pack (gated).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MescArtifactAdmitRequest {
    pub artifact_uri: String,
    pub content_digest: DigestSha256,
    pub rights_uri: String,
    pub sbom_digest: DigestSha256,
    pub evaluation_digest: DigestSha256,
    /// Must be true — Pack path only.
    pub pack_path_required: bool,
}

/// Explicit refuse of MESC Python / ambient service paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MescIntegrationMode {
    ArtifactImportOnly,
    PythonRuntimeForbidden,
    SharedDbForbidden,
    SharedKeysForbidden,
    AmbientServiceForbidden,
}
