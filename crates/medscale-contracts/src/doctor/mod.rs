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

/// Vault privacy posture (Spec 017). Never claims secrets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VaultPrivacyDoctorStatus {
    pub present: bool,
    pub private_data_ready: bool,
    pub sealed_at_close: bool,
    pub open_work_plaintext_risk: bool,
    pub work_wipe_on_close: bool,
    pub sqlcipher_enabled: bool,
}

impl VaultPrivacyDoctorStatus {
    #[must_use]
    pub fn spec_017_honest() -> Self {
        Self {
            present: true,
            private_data_ready: false,
            sealed_at_close: true,
            open_work_plaintext_risk: true,
            work_wipe_on_close: true,
            sqlcipher_enabled: false,
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
    /// Spec 018 READY_BASE: in-process sessions only; multi-client release not claimed.
    #[must_use]
    pub fn ready_base() -> Self {
        Self {
            present: true,
            ready_base: true,
            multi_client_release_ready: false,
            os_ipc_qualified: false,
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
