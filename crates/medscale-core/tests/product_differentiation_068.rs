//! Spec 068 product differentiation and honesty regression.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate parent")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn medscale_signal_identity_replaces_rejected_multicolor_brand() {
    let root = repo_root();
    let theme = std::fs::read_to_string(root.join("crates/medscale-desktop/ui/theme.slint"))
        .expect("theme");
    let design = std::fs::read_to_string(root.join("DESIGN.md")).expect("design");

    for rejected_hex in ["#5B5CF6", "#FF785C", "#D8B4FE", "#2F4F46"] {
        assert!(
            !theme.contains(rejected_hex),
            "rejected palette returned: {rejected_hex}"
        );
    }
    assert!(theme.contains("signal: #0A66FF"));
    assert!(theme.contains("obsidian: #0A0E14"));
    for retired_name in ["blurple", "coral", "lavender", "pine", "mint", "amber"] {
        assert!(
            !theme.contains(retired_name),
            "retired palette token returned: {retired_name}"
        );
    }
    assert!(design.contains("SUPERSEDED_BY_SPEC_068"));
    assert!(!design.contains("Cohere-inspired"));
}

#[test]
fn models_and_competitive_evidence_are_first_class_and_truthful() {
    let root = repo_root();
    let app =
        std::fs::read_to_string(root.join("crates/medscale-desktop/ui/app.slint")).expect("app");
    let product =
        std::fs::read_to_string(root.join("crates/medscale-desktop/src/product_intelligence.rs"))
            .expect("product intelligence");

    assert!(app.contains("label: \"Models\""));
    assert!(app.contains("label: \"Evidence\""));
    assert!(app.contains("EVIDENCE, NOT MARKETING"));
    assert!(app.contains("AI is visible, not implied."));
    assert!(product.contains("real portable ONNX runtime admitted"));
    assert!(product.contains("production clinical model not promoted"));
    assert!(product.contains("OPENMED AHEAD"));
    assert!(product.contains("PROVEN ADVANTAGE"));
    assert!(product.contains("OPENMED AHEAD"));
    assert!(!product.contains("beats OpenMed"));
}

#[test]
fn home_is_a_clinical_workspace_not_an_admin_dashboard() {
    let root = repo_root();
    let app =
        std::fs::read_to_string(root.join("crates/medscale-desktop/ui/app.slint")).expect("app");
    let review = std::fs::read_to_string(
        root.join("evidence/068-product-differentiation-rebuild/ABRIDGE_PRODUCT_REVIEW.md"),
    )
    .expect("Abridge product review");

    assert!(app.contains("Clinical Workspace"));
    assert!(app.contains("CURRENT CLINICAL WORKSPACE"));
    assert!(app.contains("Clinical flow"));
    assert!(app.contains("01  PREPARE"));
    assert!(app.contains("02  UNDERSTAND"));
    assert!(app.contains("03  ACT"));
    assert!(!app.contains("Operational snapshot"));
    assert!(review.contains("the clinical work itself is the interface"));
    assert!(review.contains("Prepare → Understand → Act"));
}

#[test]
fn canonical_status_reopens_until_product_differentiation_sequence_closes() {
    let status =
        std::fs::read_to_string(repo_root().join("docs/planning/PROJECT_COMPLETION_STATUS.md"))
            .expect("completion status");
    assert!(status.contains("STATUS = PRODUCT_DIFFERENTIATION_REBUILD_IN_PROGRESS"));
    assert!(status.contains("MEDSCALE_IMPLEMENTATION_COMPLETE = FALSE"));
    assert!(status.contains("NEXT_PROMOTED_SPEC = 071"));
    assert!(!status.contains("NEXT_PROMOTED_SPEC = 072"));
    assert!(status.contains("MEDSCALE_IMPLEMENTATION_COMPLETE = FALSE"));
}

#[test]
fn brand_identity_is_canonical_and_runtime_bound() {
    let root = repo_root();
    let app =
        std::fs::read_to_string(root.join("crates/medscale-desktop/ui/app.slint")).expect("app");
    let mark =
        std::fs::read_to_string(root.join("crates/medscale-desktop/ui/assets/medscale-mark.svg"))
            .expect("mark");
    let brand =
        std::fs::read_to_string(root.join("docs/brand/BRAND_IDENTITY_SYSTEM.md")).expect("brand");
    let typography =
        std::fs::read_to_string(root.join("docs/brand/TYPOGRAPHY_SYSTEM.md")).expect("typography");
    let logo = std::fs::read_to_string(root.join("docs/brand/LOGO_SPEC.md")).expect("logo");

    assert!(app.contains("default-font-family: \"Geist\""));
    assert!(brand.contains("Clinical Intelligence OS"));
    assert!(brand.contains("Evidence first. Action second."));
    assert!(typography.contains("Geist Sans"));
    assert!(typography.contains("Geist Mono"));
    assert!(logo.contains("core mark must work without a container"));
    assert!(
        !mark.contains("<rect"),
        "master mark must not bake in an app-icon container"
    );
    assert!(mark.contains("#0A66FF"));
}
