//! `MedScale` trusted core (Specs 001–006).
//!
//! OS IPC transports remain intentional for multi-process Desktop; Spec 006 ships
//! an in-process CLI session over the same logical authority facade.

pub mod authority;
pub mod cli_session;
pub mod doctor;
pub mod effects;
pub mod process;
pub mod text;
pub mod validate;
pub mod workflow;

pub use authority::CoreFacade;
pub use cli_session::CliSession;
pub use doctor::{
    build_doctor_report, build_doctor_report_full, build_doctor_report_with_allowlist,
    privacy_proof_artifact_present,
};
pub use medscale_contracts::{MEDSCALE_VERSION, WorkspaceIdentity};
pub use workflow::{JourneyConfig, run_minimum_lovable_journey};

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
