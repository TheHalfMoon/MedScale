//! Deterministic fixture UI view-models (V0_UI_INTEGRATION_CONTRACT).
//!
//! These bind CLI/Desktop/mobile shells to typed authority surfaces without inventing
//! final visual design. No canonical DB/key handles.
//! Spec 029: accessible_label supports fixture/CLI a11y honesty (not WCAG conformance).

use serde::{Deserialize, Serialize};

use crate::doctor::DoctorReport;
use crate::presentation::{SubjectBriefV1, SubjectCoverageV1, SubjectTimelineV1};

/// Fixture UI surface identity (not a visual theme).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FixtureUiSurface {
    Doctor,
    Timeline,
    Brief,
    Coverage,
    PrivacyProof,
}

/// Shell-agnostic fixture adapter payload for one screen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureUiViewModel {
    pub surface: FixtureUiSurface,
    pub synthetic_only: bool,
    pub real_phi_authorized: bool,
    pub title: String,
    /// Operator / assistive-tech oriented label (fixture honesty; not a WCAG claim).
    pub accessible_label: String,
    pub body_json: serde_json::Value,
}

impl FixtureUiViewModel {
    /// Stable surface labels required for Spec 029 fixture accessibility honesty.
    #[must_use]
    pub fn required_surface_labels() -> &'static [&'static str] {
        &["doctor", "timeline", "brief", "coverage", "privacy_proof"]
    }

    #[must_use]
    pub fn from_doctor(report: &DoctorReport) -> Self {
        Self {
            surface: FixtureUiSurface::Doctor,
            synthetic_only: report.synthetic_only,
            real_phi_authorized: report.real_phi_authorized,
            title: "doctor".to_owned(),
            accessible_label: "MedScale doctor status".to_owned(),
            body_json: serde_json::to_value(report).unwrap_or(serde_json::Value::Null),
        }
    }

    #[must_use]
    pub fn from_timeline(timeline: &SubjectTimelineV1) -> Self {
        Self {
            surface: FixtureUiSurface::Timeline,
            synthetic_only: true,
            real_phi_authorized: false,
            title: "timeline".to_owned(),
            accessible_label: "MedScale subject timeline".to_owned(),
            body_json: serde_json::to_value(timeline).unwrap_or(serde_json::Value::Null),
        }
    }

    #[must_use]
    pub fn from_brief(brief: &SubjectBriefV1) -> Self {
        Self {
            surface: FixtureUiSurface::Brief,
            synthetic_only: true,
            real_phi_authorized: false,
            title: "brief".to_owned(),
            accessible_label: "MedScale subject brief".to_owned(),
            body_json: serde_json::to_value(brief).unwrap_or(serde_json::Value::Null),
        }
    }

    #[must_use]
    pub fn from_coverage(coverage: &SubjectCoverageV1) -> Self {
        Self {
            surface: FixtureUiSurface::Coverage,
            synthetic_only: true,
            real_phi_authorized: false,
            title: "coverage".to_owned(),
            accessible_label: "MedScale subject coverage".to_owned(),
            body_json: serde_json::to_value(coverage).unwrap_or(serde_json::Value::Null),
        }
    }

    /// UI must never claim live PHI while REAL_PHI unauthorized.
    #[must_use]
    pub fn respects_phi_boundary(&self) -> bool {
        !(self.real_phi_authorized && !self.synthetic_only)
            && (self.synthetic_only || !self.real_phi_authorized)
    }

    /// Spec 029: title + accessible_label present and non-empty (honesty, not WCAG).
    #[must_use]
    pub fn has_required_a11y_labels(&self) -> bool {
        !self.title.trim().is_empty() && !self.accessible_label.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::ControlledActionsDoctorStatus;
    use crate::doctor::{
        AccessibilityDoctorStatus, HostAuthorityDoctorStatus, KeyStoreAvailability,
        PrivacyFreshness, RecordSemanticsDoctorStatus, SyncRisk, VaultPrivacyDoctorStatus,
    };
    use crate::fhir::{FhirInterchangeDoctorStatus, FhirSupportMatrix};
    use crate::mobile::MobileDoctorStatus;
    use crate::network::NetworkBrokerDoctorStatus;
    use crate::online_packs::OnlinePacksDoctorStatus;
    use crate::packs::PacksRuntimeDoctorStatus;

    #[test]
    fn doctor_view_model_is_synthetic() {
        let report = DoctorReport {
            product_name: "MedScale".to_owned(),
            version: "0.1.0".to_owned(),
            local_only: true,
            product_runtime_egress: "DEFAULT_DENY".to_owned(),
            synthetic_only: true,
            real_phi_authorized: false,
            vault_root: None,
            vault_open: false,
            sync_risk: SyncRisk::Unknown,
            filesystem_claim_ok: false,
            key_store: KeyStoreAvailability::MemoryMockAvailable,
            privacy_proof_freshness: PrivacyFreshness::Missing,
            desktop_shell: "thin_scaffold_non_webview".to_owned(),
            tauri_admitted: false,
            network_broker: NetworkBrokerDoctorStatus {
                present: true,
                default_deny: true,
                allowlist_entries: 0,
                live_partner_authorized: false,
                http_client: "fixture".to_owned(),
            },
            packs_runtime: PacksRuntimeDoctorStatus {
                present: true,
                offline_only: true,
                admitted_count: 0,
                current_pack_id: None,
                confinement_claim: "policy".to_owned(),
                online_download_authorized: false,
            },
            mobile: MobileDoctorStatus::ready_base(),
            controlled_actions: ControlledActionsDoctorStatus::ready_base(),
            online_packs: OnlinePacksDoctorStatus::ready_base(),
            mesc_artifact: crate::mesc::MescArtifactDoctorStatus::gate_blocked(),
            vault_privacy: VaultPrivacyDoctorStatus::spec_023_honest(),
            host_authority: HostAuthorityDoctorStatus::ready_base(),
            record_semantics: RecordSemanticsDoctorStatus::ready_base(),
            fhir_interchange: FhirInterchangeDoctorStatus::ready_base(),
            fhir_support_matrix: FhirSupportMatrix::trusted_v1_ready_base(),
            workflow: crate::workflow::WorkflowDoctorStatus::ready_base(),
            release_qualification: crate::doctor::ReleaseQualificationDoctorStatus::prep_ready_base(
            ),
            accessibility: AccessibilityDoctorStatus::ready_base(),
            evidence_corpus: crate::evidence::EvidenceCorpusDoctorStatus::ready_base(
                "synthetic-lexical",
                "1.0.0",
            ),
            pack_signer: crate::packs::PackSignerDoctorStatus::ready_base(),
            os_sandbox: crate::os_sandbox::OsSandboxDoctorStatus::ready_base(),
            notes: vec![],
        };
        let vm = FixtureUiViewModel::from_doctor(&report);
        assert!(vm.respects_phi_boundary());
        assert!(vm.has_required_a11y_labels());
        assert_eq!(vm.surface, FixtureUiSurface::Doctor);
        assert_eq!(vm.title, "doctor");
        assert!(!vm.accessible_label.is_empty());
    }

    #[test]
    fn required_surface_labels_cover_fixture_surfaces() {
        let labels = FixtureUiViewModel::required_surface_labels();
        assert!(labels.contains(&"doctor"));
        assert!(labels.contains(&"timeline"));
        assert!(labels.contains(&"brief"));
        assert!(labels.contains(&"coverage"));
        assert!(labels.contains(&"privacy_proof"));
    }
}
