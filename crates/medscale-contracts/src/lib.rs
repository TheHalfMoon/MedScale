//! Shared bootstrap contracts for `MedScale`.
//!
//! Spec 001 intentionally contains no medical object semantics. Spec 002 owns
//! trusted object / source / authority types.

/// Workspace semantic version advertised by CLI and core reports.
pub const MEDSCALE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Stable product name used in operator-facing bootstrap output.
pub const MEDSCALE_PRODUCT_NAME: &str = "MedScale";

/// Minimal workspace identity for bootstrap health reporting.
#[derive(Debug, Clone, PartialEq, Eq)]
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
