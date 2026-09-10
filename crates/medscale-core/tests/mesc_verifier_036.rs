//! Spec 036 MESC synthetic verifier READY_BASE (does not clear MESC_RELEASED_ARTIFACT).

use medscale_contracts::envelopes::{
    AuthorityError, AuthorityRequest, Capability, RequestBody, ResponseBody,
};
use medscale_contracts::mesc::{
    MescAdmissionState, MescArtifactAdmitRequest, MescArtifactVerifyRequest, MescVerifyReason,
};
use medscale_contracts::objects::{AuthorityScopeId, DigestSha256, OpaqueId, RealmId, VaultId};
use medscale_core::{CoreFacade, build_doctor_report};
use std::path::PathBuf;

fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../evidence/012-mesc-artifact-integration/fixtures")
}

fn req(capability: Capability, body: RequestBody) -> AuthorityRequest {
    AuthorityRequest::new(
        OpaqueId::new("req"),
        VaultId::new("v"),
        RealmId::new("r"),
        AuthorityScopeId::new("s"),
        capability,
        body,
    )
}

fn verify(facade: &CoreFacade, dir: &str) -> ResponseBody {
    let path = fixtures_root().join(dir);
    let out = facade.dispatch(req(
        Capability::MescArtifactVerify,
        RequestBody::MescArtifactVerify {
            request: MescArtifactVerifyRequest {
                release_dir: path.to_string_lossy().into_owned(),
            },
        },
    ));
    out.result.expect("dispatch ok")
}

#[test]
fn synthetic_good_verifies_without_product_admit() {
    let facade = CoreFacade::new_legacy_lease_only_engineering();
    match verify(&facade, "synthetic-good") {
        ResponseBody::MescVerify { report } => {
            assert_eq!(report.state, MescAdmissionState::Verified);
            assert_eq!(report.reason, MescVerifyReason::Ok);
            assert!(!report.product_admit_authorized);
            assert_eq!(
                report.producer_id.as_deref(),
                Some("synthetic.medscale.mesc")
            );
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn adversarial_fixtures_reject_with_stable_reasons() {
    let facade = CoreFacade::new_legacy_lease_only_engineering();
    let cases = [
        ("missing-manifest", MescVerifyReason::MissingManifest),
        ("digest-mismatch", MescVerifyReason::DigestMismatch),
        ("size-mismatch", MescVerifyReason::SizeMismatch),
        ("missing-file", MescVerifyReason::MissingArtifactFile),
        ("bad-schema", MescVerifyReason::UnsupportedSchema),
        ("unknown-field", MescVerifyReason::UnsupportedSchema),
        ("duplicate-path", MescVerifyReason::DuplicateArtifactPath),
        ("missing-rights", MescVerifyReason::MissingRights),
        ("missing-sbom", MescVerifyReason::MissingSbom),
    ];
    for (dir, reason) in cases {
        match verify(&facade, dir) {
            ResponseBody::MescVerify { report } => {
                assert_eq!(report.state, MescAdmissionState::Rejected, "{dir}");
                assert_eq!(report.reason, reason, "{dir}");
                assert!(!report.product_admit_authorized);
            }
            other => panic!("{dir}: unexpected {other:?}"),
        }
    }
}

#[test]
fn replay_rejects_second_identical_verified_epoch() {
    let facade = CoreFacade::new_legacy_lease_only_engineering();
    match verify(&facade, "synthetic-good") {
        ResponseBody::MescVerify { report } => {
            assert_eq!(report.reason, MescVerifyReason::Ok);
        }
        other => panic!("unexpected {other:?}"),
    }
    match verify(&facade, "synthetic-good") {
        ResponseBody::MescVerify { report } => {
            assert_eq!(report.reason, MescVerifyReason::ReplayRejected);
            assert!(!report.product_admit_authorized);
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn admit_still_gate_blocked_after_synthetic_verify() {
    let facade = CoreFacade::new_legacy_lease_only_engineering();
    let _ = verify(&facade, "synthetic-good");
    let out = facade.dispatch(req(
        Capability::MescArtifactAdmit,
        RequestBody::MescArtifactAdmit {
            request: MescArtifactAdmitRequest {
                artifact_uri: "synthetic://not-real".to_owned(),
                content_digest: DigestSha256::of(b"x"),
                rights_uri: "synthetic://rights".to_owned(),
                sbom_digest: DigestSha256::of(b"s"),
                evaluation_digest: DigestSha256::of(b"e"),
                pack_path_required: true,
            },
        },
    ));
    assert_eq!(
        out.result,
        Err(AuthorityError::ExternalGateRequired {
            gate: "MESC_RELEASED_ARTIFACT".to_owned()
        })
    );
}

#[test]
fn doctor_reports_verifier_ready_base_without_admit() {
    let report = build_doctor_report(None, false, false);
    assert!(report.mesc_artifact.verifier_ready_base);
    assert!(!report.mesc_artifact.artifact_admitted);
    assert!(report.mesc_artifact.is_honest_ready_base());
}
