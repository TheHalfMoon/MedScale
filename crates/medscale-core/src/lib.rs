//! `MedScale` trusted core (Spec 001 bootstrap + Spec 002 authority foundation).
//!
//! OS IPC transports are intentional for Spec 006; Spec 002 proves the same logical
//! API in-process. See `specs/002-*/contracts/authority-facade.md`.

pub mod authority;
pub mod effects;
pub mod process;
pub mod text;
pub mod validate;

pub use authority::CoreFacade;
pub use medscale_contracts::{MEDSCALE_VERSION, WorkspaceIdentity};

/// Non-authoritative bootstrap health report for operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReport {
    /// Workspace identity constants.
    pub identity: WorkspaceIdentity,
    /// Explicit statement that this process holds no vault/key/network handles.
    pub local_only: bool,
    /// Spec 002 adds object/authority semantics but no clinical product surface.
    pub medical_functionality: bool,
}

impl CoreFacade {
    /// Returns a bootstrap report proving local-only operation.
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
