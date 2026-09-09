//! OS worker sandbox (Spec 008 scaffold + Spec 026 Linux ReadyBaseMeasured).
//!
//! EXTERNAL_GATES: `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED` remains OPEN until multi-OS
//! measured PLATFORM_QUALIFIED evidence exists. Linux may report ReadyBaseMeasured only.

use serde::{Deserialize, Serialize};

#[cfg(target_os = "linux")]
use std::path::{Path, PathBuf};

/// Qualification claim for an OS confinement backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OsSandboxQualification {
    /// Scaffold / research only — Spec 008 exit posture for non-Linux (and pre-026).
    NotPlatformQualified,
    /// Spec 026: Linux Landlock measured READY_BASE — not full multi-OS PLATFORM_QUALIFIED.
    ReadyBaseMeasured,
    /// Reserved for multi-OS measured evidence + EXTERNAL_GATES close.
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

/// Documented plan for one OS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OsSandboxPlan {
    pub target: OsSandboxTarget,
    pub qualification: OsSandboxQualification,
    pub mechanisms: Vec<String>,
    pub evidence_path: String,
    pub limitations: Vec<String>,
    /// Absolute paths allowed for ReadyBaseMeasured apply (read+write under hierarchy).
    #[serde(default)]
    pub allow_paths: Vec<String>,
}

impl OsSandboxPlan {
    /// Spec 026 Linux READY_BASE measured Landlock plan (allow_paths filled by caller).
    #[must_use]
    pub fn linux_landlock_ready_base(allow_paths: Vec<String>) -> Self {
        Self {
            target: OsSandboxTarget::LinuxLandlock,
            qualification: OsSandboxQualification::ReadyBaseMeasured,
            mechanisms: vec![
                "landlock_abi_v1".to_owned(),
                "path_beneath_allowlist".to_owned(),
            ],
            evidence_path: "evidence/026-pack-signer-os-sandbox/LINUX_LANDLOCK_MEASURED.md"
                .to_owned(),
            limitations: vec![
                "ReadyBaseMeasured on Linux only — not multi-OS PLATFORM_QUALIFIED".to_owned(),
                "EXTERNAL_GATES WORKER_OS_SANDBOX_PLATFORM_QUALIFIED remains OPEN".to_owned(),
                "No seccomp/network broker composition in Spec 026".to_owned(),
            ],
            allow_paths,
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
                "No AppContainer apply path in Spec 026".to_owned(),
            ],
            allow_paths: vec![],
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
                "No Seatbelt apply path in Spec 026".to_owned(),
            ],
            allow_paths: vec![],
        }
    }

    /// Scaffold / READY_BASE plans; only Linux may claim ReadyBaseMeasured.
    #[must_use]
    pub fn all_plans() -> Vec<Self> {
        vec![
            Self::linux_landlock_ready_base(vec![]),
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

    #[must_use]
    pub fn claims_ready_base_measured(&self) -> bool {
        matches!(
            self.qualification,
            OsSandboxQualification::ReadyBaseMeasured
        )
    }
}

/// OS sandbox doctor axis (Spec 026).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OsSandboxDoctorStatus {
    pub present: bool,
    pub ready_base: bool,
    pub linux_measured: bool,
    pub platform_qualified: bool,
    pub release_ready: bool,
}

impl OsSandboxDoctorStatus {
    #[must_use]
    pub fn ready_base() -> Self {
        Self {
            present: true,
            ready_base: true,
            linux_measured: true,
            platform_qualified: false,
            release_ready: false,
        }
    }

    #[must_use]
    pub fn is_honest_ready_base(&self) -> bool {
        self.present
            && self.ready_base
            && self.linux_measured
            && !self.platform_qualified
            && !self.release_ready
    }
}

/// Runtime apply errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OsSandboxApplyError {
    NotPlatformQualified,
    NotReadyOnThisHost,
    EmptyAllowlist,
    ApplyFailed(String),
}

/// Apply OS sandbox according to plan qualification.
///
/// - `ReadyBaseMeasured` + LinuxLandlock on Linux: Landlock allowlist apply.
/// - `PlatformQualified`: still refused while EXTERNAL_GATES remains OPEN.
/// - Windows/macOS scaffolds: NotPlatformQualified / NotReadyOnThisHost.
pub fn try_apply_os_sandbox(plan: &OsSandboxPlan) -> Result<(), OsSandboxApplyError> {
    if plan.claims_platform_qualified() {
        // Gate OPEN — never honor PlatformQualified without EXTERNAL_GATES close.
        return Err(OsSandboxApplyError::NotPlatformQualified);
    }

    if !plan.claims_ready_base_measured() {
        return Err(OsSandboxApplyError::NotPlatformQualified);
    }

    if plan.target != OsSandboxTarget::LinuxLandlock {
        return Err(OsSandboxApplyError::NotReadyOnThisHost);
    }

    if plan.allow_paths.is_empty() {
        return Err(OsSandboxApplyError::EmptyAllowlist);
    }

    #[cfg(target_os = "linux")]
    {
        apply_landlock_linux(plan)
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = plan;
        Err(OsSandboxApplyError::NotReadyOnThisHost)
    }
}

#[cfg(target_os = "linux")]
fn apply_landlock_linux(plan: &OsSandboxPlan) -> Result<(), OsSandboxApplyError> {
    use landlock::{
        ABI, AccessFs, Ruleset, RulesetAttr, RulesetCreatedAttr, RulesetStatus, path_beneath_rules,
    };

    let paths: Vec<PathBuf> = plan.allow_paths.iter().map(PathBuf::from).collect();
    for p in &paths {
        if !p.is_absolute() {
            return Err(OsSandboxApplyError::ApplyFailed(
                "allow_paths must be absolute".to_owned(),
            ));
        }
    }

    let abi = ABI::V1;
    let status = Ruleset::default()
        .handle_access(AccessFs::from_all(abi))
        .map_err(|e| OsSandboxApplyError::ApplyFailed(e.to_string()))?
        .create()
        .map_err(|e| OsSandboxApplyError::ApplyFailed(e.to_string()))?
        .add_rules(path_beneath_rules(
            paths.iter().map(Path::as_path),
            AccessFs::from_all(abi),
        ))
        .map_err(|e| OsSandboxApplyError::ApplyFailed(e.to_string()))?
        .restrict_self()
        .map_err(|e| OsSandboxApplyError::ApplyFailed(e.to_string()))?;

    match status.ruleset {
        RulesetStatus::NotEnforced => Err(OsSandboxApplyError::ApplyFailed(
            "landlock not enforced (kernel/ABI)".to_owned(),
        )),
        RulesetStatus::PartiallyEnforced | RulesetStatus::FullyEnforced => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_linux_plans_are_not_platform_qualified() {
        assert!(!OsSandboxPlan::windows_appcontainer_scaffold().claims_platform_qualified());
        assert!(!OsSandboxPlan::macos_seatbelt_scaffold().claims_platform_qualified());
        assert!(!OsSandboxPlan::linux_landlock_ready_base(vec![]).claims_platform_qualified());
        assert!(OsSandboxPlan::linux_landlock_ready_base(vec![]).claims_ready_base_measured());
    }

    #[test]
    fn platform_qualified_still_refused() {
        let mut plan = OsSandboxPlan::linux_landlock_ready_base(vec!["/tmp".to_owned()]);
        plan.qualification = OsSandboxQualification::PlatformQualified;
        assert_eq!(
            try_apply_os_sandbox(&plan),
            Err(OsSandboxApplyError::NotPlatformQualified)
        );
    }

    #[test]
    fn ready_base_without_allowlist_fails() {
        let plan = OsSandboxPlan::linux_landlock_ready_base(vec![]);
        assert_eq!(
            try_apply_os_sandbox(&plan),
            Err(OsSandboxApplyError::EmptyAllowlist)
        );
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn ready_base_on_non_linux_is_not_ready() {
        let plan = OsSandboxPlan::linux_landlock_ready_base(vec!["C:\\tmp".to_owned()]);
        assert_eq!(
            try_apply_os_sandbox(&plan),
            Err(OsSandboxApplyError::NotReadyOnThisHost)
        );
    }

    #[test]
    fn doctor_axis_honest() {
        let d = OsSandboxDoctorStatus::ready_base();
        assert!(d.is_honest_ready_base());
    }
}
