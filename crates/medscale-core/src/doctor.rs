//! Doctor report aggregation (Spec 006).

use std::path::Path;

use medscale_contracts::actions::ControlledActionsDoctorStatus;
use medscale_contracts::doctor::{DoctorReport, KeyStoreAvailability, PrivacyFreshness, SyncRisk};
use medscale_contracts::mobile::MobileDoctorStatus;
use medscale_contracts::network::NetworkBrokerDoctorStatus;
use medscale_contracts::online_packs::OnlinePacksDoctorStatus;
use medscale_contracts::packs::PacksRuntimeDoctorStatus;
use medscale_contracts::{MEDSCALE_PRODUCT_NAME, MEDSCALE_VERSION};
use medscale_storage::{assert_claim_path, default_vault_root};

/// Build a doctor report for operators (never includes secrets).
#[must_use]
pub fn build_doctor_report(
    vault_root: Option<&str>,
    vault_open: bool,
    privacy_proof_present: bool,
) -> DoctorReport {
    build_doctor_report_with_allowlist(vault_root, vault_open, privacy_proof_present, 0)
}

/// Doctor report with allowlist entry count (Spec 013; count-only, no destinations).
#[must_use]
pub fn build_doctor_report_with_allowlist(
    vault_root: Option<&str>,
    vault_open: bool,
    privacy_proof_present: bool,
    allowlist_entries: u32,
) -> DoctorReport {
    build_doctor_report_full(
        vault_root,
        vault_open,
        privacy_proof_present,
        allowlist_entries,
        0,
        None,
    )
}

/// Doctor report including packs runtime counts (Spec 008).
#[must_use]
pub fn build_doctor_report_full(
    vault_root: Option<&str>,
    vault_open: bool,
    privacy_proof_present: bool,
    allowlist_entries: u32,
    admitted_packs: u32,
    current_pack_id: Option<String>,
) -> DoctorReport {
    let (sync_risk, filesystem_claim_ok, resolved_root) = match vault_root {
        Some(root) => match assert_claim_path(Path::new(root)) {
            Ok(p) => (SyncRisk::Clear, true, Some(p.display().to_string())),
            Err(_) => (SyncRisk::RefusedMarker, false, Some(root.to_owned())),
        },
        None => {
            let def = default_vault_root("default");
            match assert_claim_path(&def) {
                Ok(p) => (SyncRisk::Clear, true, Some(p.display().to_string())),
                Err(_) => (SyncRisk::Unknown, false, Some(def.display().to_string())),
            }
        }
    };

    DoctorReport {
        product_name: MEDSCALE_PRODUCT_NAME.to_owned(),
        version: MEDSCALE_VERSION.to_owned(),
        local_only: true,
        product_runtime_egress: "DEFAULT_DENY".to_owned(),
        synthetic_only: true,
        real_phi_authorized: false,
        vault_root: resolved_root,
        vault_open,
        sync_risk,
        filesystem_claim_ok,
        key_store: KeyStoreAvailability::MemoryMockAvailable,
        privacy_proof_freshness: if privacy_proof_present {
            PrivacyFreshness::Present
        } else {
            PrivacyFreshness::Missing
        },
        desktop_shell: "thin_scaffold_non_webview".to_owned(),
        tauri_admitted: false,
        network_broker: NetworkBrokerDoctorStatus {
            present: true,
            default_deny: true,
            allowlist_entries,
            live_partner_authorized: false,
            http_client: "ureq_behind_broker_fixture_default".to_owned(),
        },
        packs_runtime: PacksRuntimeDoctorStatus {
            present: true,
            offline_only: true,
            admitted_count: admitted_packs,
            current_pack_id,
            confinement_claim: "policy_ambient_deny_v0".to_owned(),
            online_download_authorized: false,
        },
        mobile: MobileDoctorStatus::ready_base(),
        controlled_actions: ControlledActionsDoctorStatus::ready_base(),
        online_packs: OnlinePacksDoctorStatus::ready_base(),
        notes: vec![
            "CLI and Desktop call Core Host authority facade only".to_owned(),
            "Tauri/WebView not admitted in Spec 006".to_owned(),
            "Product egress DEFAULT_DENY except Network Broker allowlist".to_owned(),
            "Packs offline-only; no ONNX/llama admitted in Spec 008".to_owned(),
            "Mobile READY_BASE: no apps shipped; Keychain sync forbidden".to_owned(),
            "Controlled actions READY_BASE: outbox + UNKNOWN reconcile; NPHIES gated".to_owned(),
            "Online packs READY_BASE: acquire denied; HF not a runtime dependency".to_owned(),
        ],
    }
}

/// Detect whether Spec 006 PRIVACY_PROOF evidence file exists relative to CWD/repo.
#[must_use]
pub fn privacy_proof_artifact_present() -> bool {
    Path::new("evidence/006-cli-desktop-foundation/PRIVACY_PROOF.json").is_file()
        || Path::new("../evidence/006-cli-desktop-foundation/PRIVACY_PROOF.json").is_file()
        || Path::new("../../evidence/006-cli-desktop-foundation/PRIVACY_PROOF.json").is_file()
}
