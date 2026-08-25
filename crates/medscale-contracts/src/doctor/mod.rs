//! Operator doctor report and PRIVACY_PROOF evidence types (Spec 006 / 013).

use serde::{Deserialize, Serialize};

use crate::mobile::MobileDoctorStatus;
use crate::network::NetworkBrokerDoctorStatus;
use crate::packs::PacksRuntimeDoctorStatus;

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
