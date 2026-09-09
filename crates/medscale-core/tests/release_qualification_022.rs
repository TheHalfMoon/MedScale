//! Spec 022 release-qualification prep honesty.

use medscale_core::build_doctor_report;

#[test]
fn release_qualification_axis_is_honest_prep_not_release_ready() {
    let report = build_doctor_report(None, false, false);
    let rq = &report.release_qualification;
    assert!(rq.present);
    assert!(rq.prep_ready_base);
    assert!(!rq.release_ready);
    assert!(rq.locked_builds);
    assert!(rq.immutable_ci_action_pins);
    assert!(rq.cargo_lock_committed);
    assert!(rq.windows_linux_ci_baseline);
    assert!(!rq.macos_qualified);
    assert!(!rq.mobile_release_qualified);
    assert!(!rq.branch_protection_configured);
    assert!(!rq.missing_evidence_classes.is_empty());
    assert!(rq.is_honest_prep());
    assert!(rq.perf_harness_present);
    assert!(rq.sbom_scaffold_present);
    assert!(
        rq.missing_evidence_classes
            .contains(&"perf_budgets_attained_on_qualified_hardware".to_owned())
    );
    assert!(
        report
            .notes
            .iter()
            .any(|n| n.contains("RELEASE_READY=false") && n.contains("Release qualification"))
    );
}
