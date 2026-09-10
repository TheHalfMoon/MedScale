//! OS worker sandbox (Specs 008 / 026 / 030 / 031 / 033 / 038).
//!
//! EXTERNAL_GATES: `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED` remains OPEN until multi-OS
//! measured PLATFORM_QUALIFIED evidence exists (stronger composition than per-OS READY_BASE).
//! Linux Landlock, Windows Job Object + AppContainer FS/network, and macOS Seatbelt may report
//! ReadyBaseMeasured only — not PlatformQualified. LPAC + macOS App Sandbox entitlements remain
//! scaffold after Spec 038.

#[cfg(target_os = "macos")]
mod macos_seatbelt;
#[cfg(windows)]
mod windows_appcontainer;
#[cfg(windows)]
mod windows_job;

#[cfg(windows)]
pub use windows_appcontainer::{
    APPCONTAINER_FS_CHILD_ARG, APPCONTAINER_FS_PARENT_ARG, APPCONTAINER_NET_CHILD_ARG,
    APPCONTAINER_NET_PARENT_ARG, appcontainer_fs_child_exit_code, appcontainer_net_child_exit_code,
    measure_appcontainer_fs_deny, measure_appcontainer_net_deny, resolve_appcontainer_child_exe,
};

use serde::{Deserialize, Serialize};

#[cfg(target_os = "linux")]
use std::path::PathBuf;

/// Qualification claim for an OS confinement backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OsSandboxQualification {
    /// Scaffold / research only — Spec 008 exit posture for non-measured backends.
    NotPlatformQualified,
    /// Spec 026/030/031/033: measured READY_BASE — not full multi-OS PLATFORM_QUALIFIED.
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
    /// Spec 033: per-user AppContainer profile + measured host-file deny for child process.
    WindowsAppContainerFs,
    /// Spec 038: per-user AppContainer profile + measured TCP/network deny (zero capabilities).
    WindowsAppContainerNetwork,
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
    /// Absolute paths allowed for ReadyBaseMeasured Landlock apply (read+write under hierarchy).
    /// Unused for Windows Job Object / macOS Seatbelt network-deny READY_BASE.
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

    /// Spec 030 Windows READY_BASE: Job Object active-process limit measured.
    /// AppContainer profile/FS/network isolation remains scaffold-only.
    #[must_use]
    pub fn windows_job_object_ready_base() -> Self {
        Self {
            target: OsSandboxTarget::WindowsAppContainerJobObject,
            qualification: OsSandboxQualification::ReadyBaseMeasured,
            mechanisms: vec![
                "job_object_active_process_limit".to_owned(),
                "job_object_kill_on_job_close".to_owned(),
                "appcontainer_profile_scaffold".to_owned(),
            ],
            evidence_path: "evidence/030-windows-appcontainer-sandbox/WINDOWS_JOB_OBJECT_MEASURED.md"
                .to_owned(),
            limitations: vec![
                "ReadyBaseMeasured Job Object process-limit only — not multi-OS PLATFORM_QUALIFIED"
                    .to_owned(),
                "AppContainer FS ReadyBaseMeasured is Spec 033; network ReadyBaseMeasured is Spec 038; LPAC still scaffold"
                    .to_owned(),
                "Job Objects do not provide Landlock-equivalent path allowlists".to_owned(),
                "EXTERNAL_GATES WORKER_OS_SANDBOX_PLATFORM_QUALIFIED remains OPEN".to_owned(),
            ],
            allow_paths: vec![],
        }
    }

    /// Spec 033 Windows READY_BASE: AppContainer child FS deny measured.
    /// Complements Spec 030 Job Object; does not clear PLATFORM_QUALIFIED.
    #[must_use]
    pub fn windows_appcontainer_fs_ready_base() -> Self {
        Self {
            target: OsSandboxTarget::WindowsAppContainerFs,
            qualification: OsSandboxQualification::ReadyBaseMeasured,
            mechanisms: vec![
                "appcontainer_profile_create_derive".to_owned(),
                "proc_thread_security_capabilities".to_owned(),
                "host_temp_marker_fs_deny".to_owned(),
            ],
            evidence_path: "evidence/033-windows-appcontainer-fs/WINDOWS_APPCONTAINER_FS_MEASURED.md"
                .to_owned(),
            limitations: vec![
                "ReadyBaseMeasured AppContainer child FS deny only — not multi-OS PLATFORM_QUALIFIED"
                    .to_owned(),
                "Does not apply Landlock-equivalent allowlists to the parent process".to_owned(),
                "Network ReadyBaseMeasured is Spec 038; LPAC capability matrix not measured in Spec 033"
                    .to_owned(),
                "Requires medscale-os-sandbox-probe helper for CreateProcess child".to_owned(),
                "EXTERNAL_GATES WORKER_OS_SANDBOX_PLATFORM_QUALIFIED remains OPEN".to_owned(),
            ],
            allow_paths: vec![],
        }
    }

    /// Spec 038 Windows READY_BASE: AppContainer child network deny measured (zero capabilities).
    /// Complements Spec 033 FS; LPAC remains scaffold; does not clear PLATFORM_QUALIFIED.
    #[must_use]
    pub fn windows_appcontainer_network_ready_base() -> Self {
        Self {
            target: OsSandboxTarget::WindowsAppContainerNetwork,
            qualification: OsSandboxQualification::ReadyBaseMeasured,
            mechanisms: vec![
                "appcontainer_profile_create_derive".to_owned(),
                "proc_thread_security_capabilities_zero_caps".to_owned(),
                "tcp_connect_network_deny".to_owned(),
            ],
            evidence_path:
                "evidence/038-windows-appcontainer-network/WINDOWS_APPCONTAINER_NETWORK_MEASURED.md"
                    .to_owned(),
            limitations: vec![
                "ReadyBaseMeasured AppContainer child network deny only — not multi-OS PLATFORM_QUALIFIED"
                    .to_owned(),
                "LPAC capability matrix still scaffold (not measured in Spec 038)".to_owned(),
                "Requires medscale-os-sandbox-probe helper for CreateProcess child".to_owned(),
                "EXTERNAL_GATES WORKER_OS_SANDBOX_PLATFORM_QUALIFIED remains OPEN".to_owned(),
            ],
            allow_paths: vec![],
        }
    }

    /// AppContainer-oriented scaffold (NotPlatformQualified); Job Object / FS / network READY_BASE are separate.
    #[must_use]
    pub fn windows_appcontainer_scaffold() -> Self {
        Self {
            target: OsSandboxTarget::WindowsAppContainerJobObject,
            qualification: OsSandboxQualification::NotPlatformQualified,
            mechanisms: vec![
                "appcontainer_lpac_candidate".to_owned(),
                "brokered_handles_candidate".to_owned(),
            ],
            evidence_path: "evidence/038-windows-appcontainer-network/LIMITATIONS.md".to_owned(),
            limitations: vec![
                "Not PLATFORM_QUALIFIED".to_owned(),
                "LPAC AppContainer path remains scaffold after Spec 038 network ReadyBaseMeasured"
                    .to_owned(),
            ],
            allow_paths: vec![],
        }
    }

    /// Spec 031 macOS READY_BASE: Seatbelt `sandbox_init` network-deny measured.
    /// App Sandbox entitlements / container FS remain scaffold-only.
    #[must_use]
    pub fn macos_seatbelt_ready_base() -> Self {
        Self {
            target: OsSandboxTarget::MacosSeatbeltSandbox,
            qualification: OsSandboxQualification::ReadyBaseMeasured,
            mechanisms: vec![
                "sandbox_init_sbpl".to_owned(),
                "seatbelt_network_deny".to_owned(),
                "app_sandbox_entitlements_scaffold".to_owned(),
            ],
            evidence_path: "evidence/031-macos-seatbelt-sandbox/MACOS_SEATBELT_MEASURED.md"
                .to_owned(),
            limitations: vec![
                "ReadyBaseMeasured Seatbelt network-deny only — not multi-OS PLATFORM_QUALIFIED"
                    .to_owned(),
                "App Sandbox entitlements / container FS isolation still scaffold (not measured in Spec 031)"
                    .to_owned(),
                "sandbox_init is deprecated in headers but still ships; not entitlement App Sandbox"
                    .to_owned(),
                "EXTERNAL_GATES WORKER_OS_SANDBOX_PLATFORM_QUALIFIED remains OPEN".to_owned(),
            ],
            allow_paths: vec![],
        }
    }

    /// App Sandbox / XPC-oriented scaffold (NotPlatformQualified); Seatbelt READY_BASE is separate.
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
            evidence_path: "evidence/031-macos-seatbelt-sandbox/LIMITATIONS.md".to_owned(),
            limitations: vec![
                "Not PLATFORM_QUALIFIED".to_owned(),
                "App Sandbox entitlements apply path not measured in Spec 031 (Seatbelt ReadyBaseMeasured is separate)"
                    .to_owned(),
            ],
            allow_paths: vec![],
        }
    }

    /// Scaffold / READY_BASE plans; Linux + Windows + macOS may claim ReadyBaseMeasured.
    #[must_use]
    pub fn all_plans() -> Vec<Self> {
        vec![
            Self::linux_landlock_ready_base(vec![]),
            Self::windows_job_object_ready_base(),
            Self::windows_appcontainer_fs_ready_base(),
            Self::windows_appcontainer_network_ready_base(),
            Self::windows_appcontainer_scaffold(),
            Self::macos_seatbelt_ready_base(),
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

/// OS sandbox doctor axis (Specs 026 + 030 + 031 + 033 + 038).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OsSandboxDoctorStatus {
    pub present: bool,
    pub ready_base: bool,
    pub linux_measured: bool,
    /// Spec 030: Windows Job Object ReadyBaseMeasured evidence exists in-tree.
    pub windows_measured: bool,
    /// Spec 033: Windows AppContainer FS ReadyBaseMeasured evidence exists in-tree.
    pub windows_appcontainer_fs_measured: bool,
    /// Spec 038: Windows AppContainer network ReadyBaseMeasured evidence exists in-tree.
    pub windows_appcontainer_network_measured: bool,
    /// Spec 031: macOS Seatbelt ReadyBaseMeasured evidence exists in-tree.
    pub macos_measured: bool,
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
            windows_measured: true,
            windows_appcontainer_fs_measured: true,
            windows_appcontainer_network_measured: true,
            macos_measured: true,
            platform_qualified: false,
            release_ready: false,
        }
    }

    #[must_use]
    pub fn is_honest_ready_base(&self) -> bool {
        self.present
            && self.ready_base
            && self.linux_measured
            && self.windows_measured
            && self.windows_appcontainer_fs_measured
            && self.windows_appcontainer_network_measured
            && self.macos_measured
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
/// - `ReadyBaseMeasured` + WindowsAppContainerJobObject on Windows: Job Object limits.
/// - `ReadyBaseMeasured` + WindowsAppContainerFs on Windows: AppContainer child FS deny measure.
/// - `ReadyBaseMeasured` + WindowsAppContainerNetwork on Windows: AppContainer child network deny.
/// - `ReadyBaseMeasured` + MacosSeatbeltSandbox on macOS: Seatbelt network-deny.
/// - `PlatformQualified`: still refused while EXTERNAL_GATES remains OPEN.
/// - LPAC / App Sandbox scaffolds: NotPlatformQualified.
pub fn try_apply_os_sandbox(plan: &OsSandboxPlan) -> Result<(), OsSandboxApplyError> {
    if plan.claims_platform_qualified() {
        // Gate OPEN — never honor PlatformQualified without EXTERNAL_GATES close.
        return Err(OsSandboxApplyError::NotPlatformQualified);
    }

    if !plan.claims_ready_base_measured() {
        return Err(OsSandboxApplyError::NotPlatformQualified);
    }

    match plan.target {
        OsSandboxTarget::LinuxLandlock => apply_linux_ready_base(plan),
        OsSandboxTarget::WindowsAppContainerJobObject => apply_windows_job_ready_base(plan),
        OsSandboxTarget::WindowsAppContainerFs => apply_windows_appcontainer_fs_ready_base(plan),
        OsSandboxTarget::WindowsAppContainerNetwork => {
            apply_windows_appcontainer_net_ready_base(plan)
        }
        OsSandboxTarget::MacosSeatbeltSandbox => apply_macos_ready_base(plan),
    }
}

fn apply_linux_ready_base(plan: &OsSandboxPlan) -> Result<(), OsSandboxApplyError> {
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

fn apply_windows_job_ready_base(_plan: &OsSandboxPlan) -> Result<(), OsSandboxApplyError> {
    #[cfg(windows)]
    {
        windows_job::apply_job_object_windows()
    }

    #[cfg(not(windows))]
    {
        Err(OsSandboxApplyError::NotReadyOnThisHost)
    }
}

fn apply_windows_appcontainer_fs_ready_base(
    _plan: &OsSandboxPlan,
) -> Result<(), OsSandboxApplyError> {
    #[cfg(windows)]
    {
        windows_appcontainer::apply_appcontainer_fs_windows()
    }

    #[cfg(not(windows))]
    {
        Err(OsSandboxApplyError::NotReadyOnThisHost)
    }
}

fn apply_windows_appcontainer_net_ready_base(
    _plan: &OsSandboxPlan,
) -> Result<(), OsSandboxApplyError> {
    #[cfg(windows)]
    {
        windows_appcontainer::apply_appcontainer_net_windows()
    }

    #[cfg(not(windows))]
    {
        Err(OsSandboxApplyError::NotReadyOnThisHost)
    }
}

fn apply_macos_ready_base(_plan: &OsSandboxPlan) -> Result<(), OsSandboxApplyError> {
    #[cfg(target_os = "macos")]
    {
        macos_seatbelt::apply_seatbelt_macos()
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(OsSandboxApplyError::NotReadyOnThisHost)
    }
}

#[cfg(target_os = "linux")]
fn apply_landlock_linux(plan: &OsSandboxPlan) -> Result<(), OsSandboxApplyError> {
    use landlock::{
        ABI, Access, AccessFs, Ruleset, RulesetAttr, RulesetCreatedAttr, RulesetStatus,
        path_beneath_rules,
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
    let access = AccessFs::from_all(abi);
    let status = Ruleset::default()
        .handle_access(access)
        .map_err(|e| OsSandboxApplyError::ApplyFailed(e.to_string()))?
        .create()
        .map_err(|e| OsSandboxApplyError::ApplyFailed(e.to_string()))?
        .add_rules(path_beneath_rules(
            paths.iter().map(|p| p.as_path()),
            access,
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
    fn scaffolds_and_ready_base_claims() {
        assert!(!OsSandboxPlan::windows_appcontainer_scaffold().claims_platform_qualified());
        assert!(!OsSandboxPlan::windows_appcontainer_scaffold().claims_ready_base_measured());
        assert!(!OsSandboxPlan::macos_seatbelt_scaffold().claims_platform_qualified());
        assert!(!OsSandboxPlan::macos_seatbelt_scaffold().claims_ready_base_measured());
        assert!(!OsSandboxPlan::linux_landlock_ready_base(vec![]).claims_platform_qualified());
        assert!(OsSandboxPlan::linux_landlock_ready_base(vec![]).claims_ready_base_measured());
        assert!(OsSandboxPlan::windows_job_object_ready_base().claims_ready_base_measured());
        assert!(!OsSandboxPlan::windows_job_object_ready_base().claims_platform_qualified());
        assert!(OsSandboxPlan::windows_appcontainer_fs_ready_base().claims_ready_base_measured());
        assert!(!OsSandboxPlan::windows_appcontainer_fs_ready_base().claims_platform_qualified());
        assert!(
            OsSandboxPlan::windows_appcontainer_network_ready_base().claims_ready_base_measured()
        );
        assert!(
            !OsSandboxPlan::windows_appcontainer_network_ready_base().claims_platform_qualified()
        );
        assert!(OsSandboxPlan::macos_seatbelt_ready_base().claims_ready_base_measured());
        assert!(!OsSandboxPlan::macos_seatbelt_ready_base().claims_platform_qualified());
    }

    #[test]
    fn platform_qualified_still_refused() {
        let mut plan = OsSandboxPlan::linux_landlock_ready_base(vec!["/tmp".to_owned()]);
        plan.qualification = OsSandboxQualification::PlatformQualified;
        assert_eq!(
            try_apply_os_sandbox(&plan),
            Err(OsSandboxApplyError::NotPlatformQualified)
        );
        let mut win = OsSandboxPlan::windows_job_object_ready_base();
        win.qualification = OsSandboxQualification::PlatformQualified;
        assert_eq!(
            try_apply_os_sandbox(&win),
            Err(OsSandboxApplyError::NotPlatformQualified)
        );
        let mut win_fs = OsSandboxPlan::windows_appcontainer_fs_ready_base();
        win_fs.qualification = OsSandboxQualification::PlatformQualified;
        assert_eq!(
            try_apply_os_sandbox(&win_fs),
            Err(OsSandboxApplyError::NotPlatformQualified)
        );
        let mut mac = OsSandboxPlan::macos_seatbelt_ready_base();
        mac.qualification = OsSandboxQualification::PlatformQualified;
        assert_eq!(
            try_apply_os_sandbox(&mac),
            Err(OsSandboxApplyError::NotPlatformQualified)
        );
    }

    #[test]
    fn ready_base_linux_without_allowlist_fails() {
        let plan = OsSandboxPlan::linux_landlock_ready_base(vec![]);
        assert_eq!(
            try_apply_os_sandbox(&plan),
            Err(OsSandboxApplyError::EmptyAllowlist)
        );
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn linux_ready_base_on_non_linux_is_not_ready() {
        let plan = OsSandboxPlan::linux_landlock_ready_base(vec!["C:\\tmp".to_owned()]);
        assert_eq!(
            try_apply_os_sandbox(&plan),
            Err(OsSandboxApplyError::NotReadyOnThisHost)
        );
    }

    #[cfg(not(windows))]
    #[test]
    fn windows_ready_base_on_non_windows_is_not_ready() {
        let plan = OsSandboxPlan::windows_job_object_ready_base();
        assert_eq!(
            try_apply_os_sandbox(&plan),
            Err(OsSandboxApplyError::NotReadyOnThisHost)
        );
        let plan_fs = OsSandboxPlan::windows_appcontainer_fs_ready_base();
        assert_eq!(
            try_apply_os_sandbox(&plan_fs),
            Err(OsSandboxApplyError::NotReadyOnThisHost)
        );
        let plan_net = OsSandboxPlan::windows_appcontainer_network_ready_base();
        assert_eq!(
            try_apply_os_sandbox(&plan_net),
            Err(OsSandboxApplyError::NotReadyOnThisHost)
        );
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn macos_ready_base_on_non_macos_is_not_ready() {
        let plan = OsSandboxPlan::macos_seatbelt_ready_base();
        assert_eq!(
            try_apply_os_sandbox(&plan),
            Err(OsSandboxApplyError::NotReadyOnThisHost)
        );
    }

    #[test]
    fn doctor_axis_honest() {
        let d = OsSandboxDoctorStatus::ready_base();
        assert!(d.is_honest_ready_base());
        assert!(d.windows_measured);
        assert!(d.windows_appcontainer_fs_measured);
        assert!(d.windows_appcontainer_network_measured);
        assert!(d.linux_measured);
        assert!(d.macos_measured);
        assert!(!d.platform_qualified);
        assert!(!d.release_ready);
    }
}
