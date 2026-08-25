//! Shared `MedScale` contracts (Spec 001 bootstrap + Spec 002 object/authority foundation).

pub mod envelopes;
pub mod ffi_policy;
pub mod objects;
pub mod text;
pub mod worker_policy;

/// Workspace semantic version advertised by CLI and core reports.
pub const MEDSCALE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Stable product name used in operator-facing bootstrap output.
pub const MEDSCALE_PRODUCT_NAME: &str = "MedScale";

/// Authority-bearing envelope schema version for Spec 002.
pub const AUTHORITY_SCHEMA_VERSION: u32 = 1;

/// Minimal workspace identity for bootstrap health reporting.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct WorkspaceIdentity {
    /// Semantic version string for the workspace package set.
    pub version: &'static str,
    /// Human-readable product name.
    pub product_name: &'static str,
}

impl WorkspaceIdentity {
    /// Returns the current workspace identity constants.
    #[must_use]
    pub const fn current() -> Self {
        Self {
            version: MEDSCALE_VERSION,
            product_name: MEDSCALE_PRODUCT_NAME,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_identity_uses_package_version() {
        let identity = WorkspaceIdentity::current();
        assert_eq!(identity.product_name, "MedScale");
        assert!(!identity.version.is_empty());
        assert_eq!(identity.version, MEDSCALE_VERSION);
    }
}
