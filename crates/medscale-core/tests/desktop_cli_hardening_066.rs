//! Spec 066 Desktop + CLI hardening regression binding.

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
fn every_advertised_route_is_honest_and_documents_is_real() {
    let root = repo_root();
    let ui = std::fs::read_to_string(root.join("crates/medscale-desktop/ui/app.slint"))
        .expect("Desktop UI");
    for route in [
        "Patients",
        "Insights",
        "Workflows",
        "Tasks",
        "Messages",
        "Documents",
        "Audit Trail",
        "Exports",
        "Settings",
        "Integrations",
        "About",
        "Home",
    ] {
        let needle = format!("root.active-route == \"{route}\"");
        assert!(ui.contains(&needle), "route is not handled: {route}");
    }
    assert!(ui.contains("Documents — bounded intake"));
    assert!(ui.contains("Custody is not understanding"));
    assert!(!ui.contains("This surface is scheduled in a later Desktop slice."));
}

#[test]
fn home_demo_does_not_invent_clinical_risk_or_real_roster() {
    let root = repo_root();
    let ui = std::fs::read_to_string(root.join("crates/medscale-desktop/ui/app.slint"))
        .expect("Desktop UI");
    assert!(!ui.contains("High risk"));
    for real_like in [
        "Sarah Chen",
        "Michael Torres",
        "Emily Watson",
        "James Miller",
    ] {
        assert!(
            !ui.contains(real_like),
            "real-looking demo name remains: {real_like}"
        );
    }
    assert!(ui.contains("not a clinical risk ranking"));
    assert!(ui.contains("Review before anything consequential"));
}

#[test]
fn window_floor_and_command_copy_are_hardened() {
    let root = repo_root();
    let ui = std::fs::read_to_string(root.join("crates/medscale-desktop/ui/app.slint"))
        .expect("Desktop UI");
    assert!(ui.contains("min-width: 1100px"));
    assert!(ui.contains("min-height: 720px"));
    assert!(ui.contains("label: \"Commands\""));
    assert!(!ui.contains("Search patients, notes, commands, or ask MedScale"));
}

#[test]
fn ci_keeps_pr_and_main_gates_without_duplicate_spec_push() {
    let root = repo_root();
    let ci = std::fs::read_to_string(root.join(".github/workflows/ci.yml")).expect("CI");
    assert!(ci.contains("branches: [main]"));
    assert!(ci.contains("pull_request:"));
    assert!(!ci.contains("spec/**"));
    for required in [
        "rust (${{ matrix.os }})",
        "perf delivery-plan scale (windows)",
        "cargo-deny",
        "supply-chain policy present",
    ] {
        assert!(
            ci.contains(required),
            "required job identity missing: {required}"
        );
    }
}

fn srgb_luminance(rgb: [u8; 3]) -> f64 {
    let channel = |value: u8| {
        let normalized = f64::from(value) / 255.0;
        if normalized <= 0.04045 {
            normalized / 12.92
        } else {
            ((normalized + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(rgb[0]) + 0.7152 * channel(rgb[1]) + 0.0722 * channel(rgb[2])
}

fn contrast_ratio(foreground: [u8; 3], background: [u8; 3]) -> f64 {
    let a = srgb_luminance(foreground);
    let b = srgb_luminance(background);
    let (lighter, darker) = if a >= b { (a, b) } else { (b, a) };
    (lighter + 0.05) / (darker + 0.05)
}

#[test]
fn primary_text_tokens_keep_engineering_contrast_floor_without_wcag_claim() {
    let root = repo_root();
    let theme = std::fs::read_to_string(root.join("crates/medscale-desktop/ui/theme.slint"))
        .expect("theme");
    let components =
        std::fs::read_to_string(root.join("crates/medscale-desktop/ui/components.slint"))
            .expect("components");
    let ui = std::fs::read_to_string(root.join("crates/medscale-desktop/ui/app.slint"))
        .expect("Desktop UI");

    for required in [
        "ink: #0D1420",
        "ink-subtle: #455468",
        "ink-quiet: #5E6D80",
        "signal-strong: #0047B3",
    ] {
        assert!(
            theme.contains(required),
            "missing hardened text token: {required}"
        );
    }

    let surface = [0xFF, 0xFF, 0xFF];
    let soft = [0xEA, 0xF2, 0xFF];
    for (name, rgb) in [
        ("ink", [0x0D, 0x14, 0x20]),
        ("ink-subtle", [0x45, 0x54, 0x68]),
        ("ink-quiet", [0x5E, 0x6D, 0x80]),
        ("signal-strong", [0x00, 0x47, 0xB3]),
    ] {
        assert!(
            contrast_ratio(rgb, surface) >= 4.5,
            "{name} below surface contrast floor"
        );
        assert!(
            contrast_ratio(rgb, soft) >= 4.5,
            "{name} below soft-surface contrast floor"
        );
    }

    assert!(components.contains("color: Theme.ink;"));
    assert!(!ui.contains("color: Theme.warning;"));
    assert!(!ui.contains("color: Theme.success;"));
    assert!(!ui.contains("color: Theme.danger;"));
}
