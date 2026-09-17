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
fn historical_spec_068_rejected_multicolor_palette_remains_retired() {
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
    assert!(theme.contains("Mist Blue"));
    assert!(theme.contains("obsidian: #0D0F0E"));
    for retired_name in ["blurple", "coral", "lavender", "pine", "mint", "amber"] {
        assert!(
            !theme.contains(retired_name),
            "retired palette token returned: {retired_name}"
        );
    }
    assert!(design.contains("SUPERSEDED_BY_SPEC_068"));
    assert!(design.contains("SUPERSEDED_BY_SPEC_073"));
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
    assert!(app.contains("MedScale × OpenMed"));
    assert!(app.contains("Pinned OpenMed evidence · measured claims only · limitations explicit"));
    assert!(app.contains("AI is visible, not implied."));
    assert!(product.contains("real portable ONNX runtime admitted"));
    assert!(product.contains("production clinical model not promoted"));
    assert!(product.contains("verdict: \"UNMEASURED\""));
    assert!(product.contains("verdict: \"STRUCTURAL ONLY\""));
    assert!(product.contains("verdict: \"ANTI-METRIC\""));
    assert!(!product.contains("verdict: \"PROVEN ADVANTAGE\""));
    assert!(!product.contains("verdict: \"OPENMED AHEAD\""));
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
    assert!(app.contains("Clinical flow"));
    assert!(app.contains("Know the record before the encounter"));
    assert!(app.contains("Keep intelligence tied to evidence"));
    assert!(app.contains("Review before anything consequential"));
    assert!(!app.contains("Operational snapshot"));
    assert!(review.contains("the clinical work itself is the interface"));
    assert!(review.contains("Prepare → Understand → Act"));
}

#[test]
fn spec_068_closure_remains_historical_after_spec_073_closure() {
    let status =
        std::fs::read_to_string(repo_root().join("docs/planning/PROJECT_COMPLETION_STATUS.md"))
            .expect("completion status");
    assert!(status.contains("STATUS = REPOSITORY_IMPLEMENTATION_COMPLETE_PENDING_EXTERNAL_GATES"));
    assert!(status.contains("MEDSCALE_IMPLEMENTATION_COMPLETE = TRUE"));
    assert!(status.contains("KNOWN_REPOSITORY_OWNED_DESKTOP_CLI_RESIDUALS = 0"));
    assert!(status.contains("NEXT_PROMOTED_SPEC = NONE"));
    assert!(!status.contains("NEXT_PROMOTED_SPEC = 073"));
    assert!(status.contains("MEDSCALE_RELEASE_READY = FALSE"));
    assert!(status.contains("## Spec 068 canonical closure"));
    assert!(status.contains("## Spec 073 canonical closure"));
    assert!(status.contains("Specs 068–073 remain `CLOSED_CANONICAL`"));
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

    assert!(app.contains("default-font-family: \"Instrument Sans\""));
    assert!(brand.contains("Clinical Intelligence OS"));
    assert!(brand.contains("Evidence first. Action second."));
    assert!(typography.contains("Instrument Sans"));
    assert!(typography.contains("Source Serif 4"));
    assert!(logo.contains("FOUNDER_APPROVED_SIGNATURE_MARK"));
    assert!(logo.contains("MedScale Shelf"));
    assert!(mark.contains("<circle"));
    assert!(mark.contains("#0A0A0A"));
    assert!(mark.contains("#F4F4F1"));
    assert!(mark.contains("id=\"medscale-signature-m\""));
    assert!(!mark.contains("#0A66FF"));
}
