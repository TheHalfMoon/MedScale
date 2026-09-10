//! Doctor report aggregation (Spec 006 / 028).

use std::path::Path;

use medscale_contracts::actions::ControlledActionsDoctorStatus;
use medscale_contracts::doctor::{
    AccessibilityDoctorStatus, DoctorReport, HostAuthorityDoctorStatus, KeyStoreAvailability,
    PrivacyFreshness, RecordSemanticsDoctorStatus, ReleaseQualificationDoctorStatus, SyncRisk,
    VaultPrivacyDoctorStatus,
};
use medscale_contracts::evidence::EvidenceCorpusDoctorStatus;
use medscale_contracts::fhir::{FhirInterchangeDoctorStatus, FhirSupportMatrix};
use medscale_contracts::mesc::MescArtifactDoctorStatus;
use medscale_contracts::mobile::MobileDoctorStatus;
use medscale_contracts::network::NetworkBrokerDoctorStatus;
use medscale_contracts::online_packs::OnlinePacksDoctorStatus;
use medscale_contracts::os_sandbox::OsSandboxDoctorStatus;
use medscale_contracts::packs::{PackSignerDoctorStatus, PacksRuntimeDoctorStatus};
use medscale_contracts::workflow::WorkflowDoctorStatus;
use medscale_contracts::{MEDSCALE_PRODUCT_NAME, MEDSCALE_VERSION};
use medscale_keys::KeyStoreDoctorPosture;
use medscale_storage::{assert_claim_path, default_vault_root};

use crate::authority::{DEFAULT_CORPUS_ID, DEFAULT_CORPUS_VERSION};

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

    let key_posture = KeyStoreDoctorPosture::detect();
    let key_store = if key_posture.os_keyring_used {
        KeyStoreAvailability::OsStoreAvailable
    } else if key_posture.os_keyring_available {
        // Available but forced to Memory via MEDSCALE_FORCE_MEMORY_KEYSTORE.
        KeyStoreAvailability::MemoryMockAvailable
    } else {
        KeyStoreAvailability::MemoryMockAvailable
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
        key_store,
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
        mesc_artifact: MescArtifactDoctorStatus::gate_blocked(),
        vault_privacy: VaultPrivacyDoctorStatus::spec_028_honest(
            key_posture.os_keyring_available,
            key_posture.os_keyring_used,
        ),
        host_authority: HostAuthorityDoctorStatus::ready_base(),
        record_semantics: RecordSemanticsDoctorStatus::ready_base(),
        fhir_interchange: FhirInterchangeDoctorStatus::ready_base(),
        fhir_support_matrix: FhirSupportMatrix::trusted_v1_ready_base(),
        workflow: WorkflowDoctorStatus::ready_base(),
        release_qualification: ReleaseQualificationDoctorStatus::prep_ready_base(),
        accessibility: AccessibilityDoctorStatus::ready_base(),
        evidence_corpus: EvidenceCorpusDoctorStatus::ready_base(
            DEFAULT_CORPUS_ID,
            DEFAULT_CORPUS_VERSION,
        ),
        pack_signer: PackSignerDoctorStatus::ready_base(),
        os_sandbox: OsSandboxDoctorStatus::ready_base(),
        notes: vec![
            "CLI and Desktop call Core Host authority facade only".to_owned(),
            "Tauri/WebView not admitted in Spec 006".to_owned(),
            "Product egress DEFAULT_DENY except Network Broker allowlist".to_owned(),
            "Packs offline-only; signer+anti-rollback READY_BASE; no ONNX/llama admitted".to_owned(),
            "Mobile READY_BASE: no apps shipped; Keychain sync forbidden".to_owned(),
            "Controlled actions READY_BASE: outbox + UNKNOWN reconcile; NPHIES gated".to_owned(),
            "Online packs READY_BASE: acquire denied; HF not a runtime dependency".to_owned(),
            "MESC ARTIFACT_IMPORT blocked: MESC_RELEASED_ARTIFACT not available".to_owned(),
            "Vault privacy Spec 028: OsKeyStore READY_BASE; PRIVATE_DATA_READY=false (swap/hibernate/snapshot)".to_owned(),
            "Host authority READY_BASE: localhost OS IPC + strict sessions; MULTI_CLIENT_RELEASE_READY=false".to_owned(),
            "Record semantics READY_BASE: precision-aware time, append-only amendments; RELEASE_READY=false".to_owned(),
            "FHIR interchange READY_BASE: honest support matrix; no full conformance; validator evidence != authority".to_owned(),
            "Workflow READY_BASE: synthetic import-review-export-backup journey; WORKFLOW_READY_BASE=true; RELEASE_READY=false".to_owned(),
            "Release qualification READY_BASE (022+027+029): locked CI + macOS CI present + perf/SBOM scaffolds; macos_qualified=false; RELEASE_READY=false; budgets not claimed; branch protection EXTERNAL_GATES".to_owned(),
            "Accessibility READY_BASE: fixture/CLI labels + disclosure clarity checked; WCAG not claimed; final v0 UI absent".to_owned(),
            "Evidence corpus READY_BASE: versioned synthetic-lexical@1.0.0; relevance != authority; clinical quality not claimed".to_owned(),
            "OS sandbox READY_BASE: Linux Landlock measured; platform_qualified=false; WORKER_OS_SANDBOX_PLATFORM_QUALIFIED OPEN".to_owned(),
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
