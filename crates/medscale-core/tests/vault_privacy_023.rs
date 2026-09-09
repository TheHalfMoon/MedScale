//! Spec 023 doctor honesty for vault privacy axis.

use medscale_core::build_doctor_report;

#[test]
fn doctor_vault_privacy_023() {
    let report = build_doctor_report(None, false, false);
    let v = &report.vault_privacy;
    assert!(v.present);
    assert!(!v.private_data_ready);
    assert!(v.sealed_at_close);
    assert!(v.work_wipe_on_close);
    assert!(v.sqlcipher_enabled);
    assert!(v.open_work_page_encrypted);
    // Residual OS risk remains honest.
    assert!(v.open_work_plaintext_risk);
    let note = report
        .notes
        .iter()
        .any(|n| n.contains("SQLCipher") && n.contains("PRIVATE_DATA_READY=false"));
    assert!(
        note,
        "doctor notes must mention SQLCipher + PRIVATE_DATA_READY=false"
    );
}
