//! Operator doctor report and PRIVACY_PROOF evidence types (Spec 006 / 013).

use serde::{Deserialize, Serialize};

use crate::actions::ControlledActionsDoctorStatus;
use crate::fhir::{FhirInterchangeDoctorStatus, FhirSupportMatrix};
use crate::mesc::MescArtifactDoctorStatus;
use crate::mobile::MobileDoctorStatus;
use crate::network::NetworkBrokerDoctorStatus;
use crate::online_packs::OnlinePacksDoctorStatus;
use crate::packs::PacksRuntimeDoctorStatus;
use crate::workflow::WorkflowDoctorStatus;

/// Release-qualification prep posture (Spec 022 / Trusted V1 Q05 remnants).
///
/// Always reports `release_ready = false` until a separate qualification package
/// and external gates close. Lists missing evidence classes without claiming pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseQualificationDoctorStatus {
    pub present: bool,
    /// Spec 022 prep paths/evidence/locked CI documented and wired.
    pub prep_ready_base: bool,
    /// Never true in Spec 022.
    pub release_ready: bool,
    pub locked_builds: bool,
    pub immutable_ci_action_pins: bool,
    pub cargo_lock_committed: bool,
    pub windows_linux_ci_baseline: bool,
    pub macos_qualified: bool,
    pub mobile_release_qualified: bool,
    /// Owner settings EXTERNAL_GATES; not configured by MedScale code.
    pub branch_protection_configured: bool,
    pub missing_evidence_classes: Vec<String>,
}

impl ReleaseQualificationDoctorStatus {
    /// Spec 022 READY_BASE prep: locked builds + evidence; RELEASE_READY remains false.
    #[must_use]
    pub fn prep_ready_base() -> Self {
        Self {
            present: true,
            prep_ready_base: true,
            release_ready: false,
            locked_builds: true,
            immutable_ci_action_pins: true,
            cargo_lock_committed: true,
            windows_linux_ci_baseline: true,
            macos_qualified: false,
            mobile_release_qualified: false,
            branch_protection_configured: false,
            missing_evidence_classes: vec![
                "qualified_os_matrix_macos".to_owned(),
                "mobile_app_release_qualification".to_owned(),
                "repo_branch_protection_required_checks".to_owned(),
                "reproducible_release_package_contents".to_owned(),
                "release_sbom_native_model_assets".to_owned(),
                "public_source_license_choice".to_owned(),
                "checksums_provenance_signing_verification".to_owned(),
                "release_bar_migration_recovery_proof".to_owned(),
                "unresolved_material_findings_clearance".to_owned(),
            ],
        }
    }

    #[must_use]
    pub fn is_honest_prep(&self) -> bool {
        self.present
            && self.prep_ready_base
            && !self.release_ready
            && !self.macos_qualified
            && !self.mobile_release_qualified
            && !self.branch_protection_configured
            && !self.missing_evidence_classes.is_empty()
    }
}

/// Sync / remote filesystem risk assessment for a vault path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncRisk {
    Clear,
    RefusedMarker,
    Unknown,
}

/// Key-store availability for wrapped DEK material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyStoreAvailability {
    MemoryMockAvailable,
    OsStoreDeferred,
    Unavailable,
}

/// Privacy evidence freshness relative to repository evidence artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyFreshness {
    Present,
    Missing,
    Stale,
}

/// WebView cache scan applicability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebViewScanStatus {
    NotApplicable,
    Pass,
    Fail,
    Deferred,
}

/// Vault privacy posture (Specs 017/023). Never claims secrets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VaultPrivacyDoctorStatus {
    pub present: bool,
    pub private_data_ready: bool,
    pub sealed_at_close: bool,
    pub open_work_plaintext_risk: bool,
    /// Spec 023: EncryptedVault open-work pages encrypted via SQLCipher.
    pub open_work_page_encrypted: bool,
    pub work_wipe_on_close: bool,
    pub sqlcipher_enabled: bool,
}

impl VaultPrivacyDoctorStatus {
    /// Spec 017 posture before SQLCipher open-work (historical).
    #[must_use]
    pub fn spec_017_honest() -> Self {
        Self {
            present: true,
            private_data_ready: false,
            sealed_at_close: true,
            open_work_plaintext_risk: true,
            open_work_page_encrypted: false,
            work_wipe_on_close: true,
            sqlcipher_enabled: false,
        }
    }

    /// Spec 023 READY_BASE: page-encrypted open work; PRIVATE_DATA_READY still false.
    #[must_use]
    pub fn spec_023_honest() -> Self {
        Self {
            present: true,
            private_data_ready: false,
            sealed_at_close: true,
            // Residual OS swap/hibernate/snapshot risk; work file itself is page-encrypted.
            open_work_plaintext_risk: true,
            open_work_page_encrypted: true,
            work_wipe_on_close: true,
            sqlcipher_enabled: true,
        }
    }
}

/// Host / client session authority posture (Spec 018).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostAuthorityDoctorStatus {
    pub present: bool,
    pub ready_base: bool,
    pub multi_client_release_ready: bool,
    pub os_ipc_qualified: bool,
}

impl HostAuthorityDoctorStatus {
    /// Spec 024 READY_BASE: localhost OS IPC + strict sessions; multi-client release not claimed.
    #[must_use]
    pub fn ready_base() -> Self {
        Self {
            present: true,
            ready_base: true,
            multi_client_release_ready: false,
            os_ipc_qualified: true,
        }
    }
}

/// Record semantics posture (Spec 019 Q06). READY_BASE; not release/privacy claims.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordSemanticsDoctorStatus {
    pub present: bool,
    pub ready_base: bool,
    pub precision_aware_time: bool,
    pub append_only_amendments: bool,
    pub explicit_identity_reconciliation: bool,
    pub release_ready: bool,
}

impl RecordSemanticsDoctorStatus {
    #[must_use]
    pub fn ready_base() -> Self {
        Self {
            present: true,
            ready_base: true,
            precision_aware_time: true,
            append_only_amendments: true,
            explicit_identity_reconciliation: true,
            release_ready: false,
        }
    }
}

/// `medscale doctor` structured report (no secrets).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DoctorReport {
    pub product_name: String,
    pub version: String,
    pub local_only: bool,
    pub product_runtime_egress: String,
    pub synthetic_only: bool,
    pub real_phi_authorized: bool,
    pub vault_root: Option<String>,
    pub vault_open: bool,
    pub sync_risk: SyncRisk,
    pub filesystem_claim_ok: bool,
    pub key_store: KeyStoreAvailability,
    pub privacy_proof_freshness: PrivacyFreshness,
    pub desktop_shell: String,
    pub tauri_admitted: bool,
    pub network_broker: NetworkBrokerDoctorStatus,
    pub packs_runtime: PacksRuntimeDoctorStatus,
    pub mobile: MobileDoctorStatus,
    pub controlled_actions: ControlledActionsDoctorStatus,
    pub online_packs: OnlinePacksDoctorStatus,
    pub mesc_artifact: MescArtifactDoctorStatus,
    pub vault_privacy: VaultPrivacyDoctorStatus,
    pub host_authority: HostAuthorityDoctorStatus,
    pub record_semantics: RecordSemanticsDoctorStatus,
    pub fhir_interchange: FhirInterchangeDoctorStatus,
    pub fhir_support_matrix: FhirSupportMatrix,
    pub workflow: WorkflowDoctorStatus,
    pub release_qualification: ReleaseQualificationDoctorStatus,
    pub notes: Vec<String>,
}

/// Typed PRIVACY_PROOF evidence artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrivacyProof {
    pub schema_version: u32,
    pub spec_id: String,
    pub claim_scope: String,
    pub synthetic_only: bool,
    pub real_phi_authorized: bool,
    pub attributable_egress: String,
    pub network_capability_audit: String,
    pub log_crash_marker_scan: String,
    pub webview_cache_scan: WebViewScanStatus,
    pub vault_sync_root_check: String,
    pub tauri_status: String,
    pub limitations: Vec<String>,
}

impl PrivacyProof {
    /// Spec 006 baseline proof (CLI + non-WebView desktop scaffold).
    #[must_use]
    pub fn spec_006_baseline() -> Self {
        Self {
            schema_version: 1,
            spec_id: "006-cli-desktop-foundation".to_owned(),
            claim_scope: "MedScale CLI + in-process Core Host on synthetic fixtures; Desktop thin scaffold without WebView".to_owned(),
            synthetic_only: true,
            real_phi_authorized: false,
            attributable_egress: "DEFAULT_DENY; no product Network Broker calls in Spec 006 wedge".to_owned(),
            network_capability_audit: "workspace crates have no runtime HTTP client for product egress".to_owned(),
            log_crash_marker_scan: "doctor/CLI output secret-marker scan required in tests".to_owned(),
            webview_cache_scan: WebViewScanStatus::NotApplicable,
            vault_sync_root_check: "assert_claim_path refuse markers (Spec 003/005)".to_owned(),
            tauri_status: "DEFERRED — not admitted; TAURI_WEBVIEW_PRIVACY_QUALIFICATION external gate".to_owned(),
            limitations: vec![
                "Does not claim zero packets system-wide outside MedScale process boundary".to_owned(),
                "Tauri/WebView privacy qualification deferred; Desktop is non-WebView scaffold only".to_owned(),
                "REAL_PHI remains unauthorized; synthetic-only".to_owned(),
                "OS keyring stores deferred; MemoryKeyStore used for CI proofs".to_owned(),
            ],
        }
    }
}

#[cfg(test)]
mod release_qualification_tests {
    use super::ReleaseQualificationDoctorStatus;

    #[test]
    fn prep_ready_base_never_claims_release_ready() {
        let s = ReleaseQualificationDoctorStatus::prep_ready_base();
        assert!(s.is_honest_prep());
        assert!(!s.release_ready);
        assert!(
            s.missing_evidence_classes
                .contains(&"repo_branch_protection_required_checks".to_owned())
        );
    }
}
