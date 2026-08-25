//! `MedScale` core authority facade stub.
//!
//! Spec 001 establishes the crate boundary only. No canonical storage, keys,
//! network broker, or medical object graph is available yet.

use medscale_contracts::{MEDSCALE_VERSION, WorkspaceIdentity};

/// Non-authoritative bootstrap health report for operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReport {
    /// Workspace identity constants.
    pub identity: WorkspaceIdentity,
    /// Explicit statement that this process holds no vault/key/network handles.
    pub local_only: bool,
    /// Explicit statement that medical functionality is absent in Spec 001.
    pub medical_functionality: bool,
}

/// Authority facade entry point shared by future CLI/Desktop/mobile clients.
#[derive(Debug, Default, Clone, Copy)]
pub struct CoreFacade;

impl CoreFacade {
    /// Returns a bootstrap report proving the stub is local-only and non-medical.
    #[must_use]
    pub fn bootstrap_report() -> BootstrapReport {
        BootstrapReport {
            identity: WorkspaceIdentity::current(),
            local_only: true,
            medical_functionality: false,
        }
    }

    /// Returns the workspace version string.
    #[must_use]
    pub fn version() -> &'static str {
        MEDSCALE_VERSION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_report_is_local_and_non_medical() {
        let report = CoreFacade::bootstrap_report();
        assert!(report.local_only);
        assert!(!report.medical_functionality);
        assert_eq!(report.identity.version, CoreFacade::version());
    }
}
