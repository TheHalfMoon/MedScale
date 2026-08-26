//! OS worker sandbox scaffold (EXTERNAL_GATES: WORKER_OS_SANDBOX_PLATFORM_QUALIFIED).
//!
//! Spec 008 closed on policy ambient-deny. This module records per-OS *plans* only.
//! It MUST NOT claim PLATFORM_QUALIFIED until measured evidence exists.

use serde::{Deserialize, Serialize};

/// Qualification claim for an OS confinement backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OsSandboxQualification {
    /// Scaffold / research only — Spec 008 exit posture.
    NotPlatformQualified,
    /// Reserved for later measured evidence (do not set without EXTERNAL_GATES close).
    PlatformQualified,
}

/// Target OS for confinement composition (GLM F-15).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OsSandboxTarget {
    LinuxLandlock,
    WindowsAppContainerJobObject,
    MacosSeatbeltSandbox,
}

/// Documented plan for one OS — no runtime apply in this scaffold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OsSandboxPlan {
    pub target: OsSandboxTarget,
    pub qualification: OsSandboxQualification,
    pub mechanisms: Vec<String>,
    pub evidence_path: String,
    pub limitations: Vec<String>,
}

impl OsSandboxPlan {
    #[must_use]
    pub fn linux_landlock_scaffold() -> Self {
        Self {
            target: OsSandboxTarget::LinuxLandlock,
            qualification: OsSandboxQualification::NotPlatformQualified,
            mechanisms: vec![
                "landlock_abi".to_owned(),
                "seccomp_network_deny_candidate".to_owned(),
                "rlimit_candidate".to_owned(),
            ],
            evidence_path: "evidence/008-local-ai-capability-fabric/OS_SANDBOX_SCAFFOLD.md"
                .to_owned(),
            limitations: vec![
                "Not PLATFORM_QUALIFIED".to_owned(),
                "No Landlock apply path in Spec 008 exit".to_owned(),
            ],
        }
    }

    #[must_use]
    pub fn windows_appcontainer_scaffold() -> Self {
        Self {
            target: OsSandboxTarget::WindowsAppContainerJobObject,
            qualification: OsSandboxQualification::NotPlatformQualified,
            mechanisms: vec![
                "appcontainer_profile_candidate".to_owned(),
                "job_object_candidate".to_owned(),
                "brokered_handles_candidate".to_owned(),
            ],
            evidence_path: "evidence/008-local-ai-capability-fabric/OS_SANDBOX_SCAFFOLD.md"
                .to_owned(),
            limitations: vec![
                "Not PLATFORM_QUALIFIED".to_owned(),
                "No AppContainer apply path in Spec 008 exit".to_owned(),
            ],
        }
    }

    #[must_use]
    pub fn macos_seatbelt_scaffold() -> Self {
        Self {
            target: OsSandboxTarget::MacosSeatbeltSandbox,
            qualification: OsSandboxQualification::NotPlatformQualified,
            mechanisms: vec![
                "seatbelt_profile_candidate".to_owned(),
                "app_sandbox_candidate".to_owned(),
                "xpc_pattern_candidate".to_owned(),
            ],
            evidence_path: "evidence/008-local-ai-capability-fabric/OS_SANDBOX_SCAFFOLD.md"
                .to_owned(),
            limitations: vec![
                "Not PLATFORM_QUALIFIED".to_owned(),
                "No Seatbelt apply path in Spec 008 exit".to_owned(),
            ],
        }
    }

    /// All scaffold plans; every entry must remain NotPlatformQualified until gate closes.
    #[must_use]
    pub fn all_scaffolds() -> Vec<Self> {
        vec![
            Self::linux_landlock_scaffold(),
            Self::windows_appcontainer_scaffold(),
            Self::macos_seatbelt_scaffold(),
        ]
    }

    #[must_use]
    pub fn claims_platform_qualified(&self) -> bool {
        matches!(
            self.qualification,
            OsSandboxQualification::PlatformQualified
        )
    }
}

/// Runtime apply is unavailable until PLATFORM_QUALIFIED evidence + gate close.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsSandboxApplyError {
    NotPlatformQualified,
}

/// Refuse apply while gate OPEN — fail closed.
pub fn try_apply_os_sandbox(plan: &OsSandboxPlan) -> Result<(), OsSandboxApplyError> {
    if plan.claims_platform_qualified() {
        // Even if someone flips the enum early, refuse without EXTERNAL_GATES evidence.
        return Err(OsSandboxApplyError::NotPlatformQualified);
    }
    Err(OsSandboxApplyError::NotPlatformQualified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffolds_are_not_platform_qualified() {
        for plan in OsSandboxPlan::all_scaffolds() {
            assert!(!plan.claims_platform_qualified());
            assert_eq!(
                try_apply_os_sandbox(&plan),
                Err(OsSandboxApplyError::NotPlatformQualified)
            );
        }
    }
}
