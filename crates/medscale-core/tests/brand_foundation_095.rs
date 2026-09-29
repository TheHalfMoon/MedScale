//! Spec 095 asset, redistribution, and non-color presentation contracts.
//! These deterministic checks do not establish rendered UI or WCAG qualification.

use medscale_contracts::objects::DigestSha256;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate parent")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative))
        .unwrap_or_else(|error| panic!("read {relative}: {error}"))
}

fn attribute<'a>(element: &'a str, name: &str) -> &'a str {
    element
        .split_once(&format!("{name}=\""))
        .unwrap_or_else(|| panic!("missing {name} attribute"))
        .1
        .split_once('"')
        .expect("closing attribute quote")
        .0
}

fn paired_path(svg: &str) -> &str {
    let id = "id=\"medscale-paired-m\"";
    assert_eq!(svg.matches(id).count(), 1, "one approved paired mark");
    let position = svg.find(id).expect("paired mark id");
    let start = svg[..position].rfind('<').expect("path start");
    let end = position + svg[position..].find('>').expect("path end") + 1;
    let path = &svg[start..end];
    assert!(path.starts_with("<path"));
    path
}

fn numbers(value: &str) -> Vec<f64> {
    value
        .split(|character: char| {
            character.is_ascii_alphabetic()
                || character.is_whitespace()
                || matches!(character, ',' | '(' | ')')
        })
        .filter(|part| !part.is_empty())
        .map(|part| part.parse().expect("finite numeric geometry"))
        .inspect(|value: &f64| assert!(value.is_finite()))
        .collect()
}

fn near(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 0.00001,
        "geometry {actual} differs from approved {expected}"
    );
}

#[test]
fn approved_pair_preserves_equilateral_peaks_contact_and_negative_space() {
    let black = read("assets/brand/mark-black.svg");
    let white = read("assets/brand/mark-white.svg");
    let black_path = paired_path(&black);
    let white_path = paired_path(&white);
    assert_eq!(attribute(black_path, "fill"), "#000000");
    assert_eq!(attribute(white_path, "fill"), "#FFFFFF");
    let geometry = attribute(black_path, "d");
    assert_eq!(
        geometry,
        attribute(white_path, "d"),
        "inverse geometry drift"
    );
    for svg in [&black, &white] {
        for forbidden in [
            "<circle",
            "stroke=",
            "<linearGradient",
            "<radialGradient",
            "<image",
        ] {
            assert!(
                !svg.contains(forbidden),
                "unapproved mark treatment: {forbidden}"
            );
        }
    }
    assert_eq!(geometry.matches('M').count(), 2, "two geometric lobes");
    assert_eq!(
        geometry.matches('Z').count(),
        2,
        "two closed geometric lobes"
    );
    assert!(
        geometry
            .chars()
            .filter(|value| value.is_ascii_alphabetic())
            .all(|value| matches!(value, 'M' | 'L' | 'Z'))
    );
    let coordinates = numbers(geometry);
    assert_eq!(coordinates.len(), 16);
    let height = 50.0 * 3.0_f64.sqrt();
    // Two side-100 equilateral primitives, offset 60; their overlap is removed.
    let expected = [
        [0.0, height],
        [50.0, 0.0],
        [80.0, height * 0.6],
        [60.0, height],
        [80.0, height * 0.6],
        [110.0, 0.0],
        [160.0, height],
        [100.0, height],
    ];
    for (point, approved) in coordinates.chunks_exact(2).zip(expected) {
        near(point[0], approved[0]);
        near(point[1], approved[1]);
    }
    let view_box = numbers(attribute(&black, "viewBox"));
    assert_eq!(view_box.len(), 4);
    near(view_box[0], 0.0);
    near(view_box[1], 0.0);
    near(view_box[2], 160.0);
    near(view_box[3], height);
    // The common center is above the baseline, leaving the lower triangular gap.
    assert!(coordinates[5] < coordinates[1]);
    assert!(coordinates[6] < coordinates[14]);
}

#[test]
fn native_assets_and_lockup_inverses_reuse_the_same_approved_geometry() {
    let canonical = read("assets/brand/mark-black.svg");
    let geometry = attribute(paired_path(&canonical), "d");
    for (master, native) in [
        (
            "assets/brand/mark-black.svg",
            "crates/medscale-desktop/ui/assets/medscale-mark.svg",
        ),
        (
            "assets/brand/mark-white.svg",
            "crates/medscale-desktop/ui/assets/medscale-mark-white.svg",
        ),
        (
            "assets/brand/medscale-app-icon-dark.svg",
            "crates/medscale-desktop/ui/assets/medscale-app-icon.svg",
        ),
        (
            "assets/brand/medscale-app-icon-light.svg",
            "crates/medscale-desktop/ui/assets/medscale-app-icon-light.svg",
        ),
    ] {
        assert_eq!(read(master), read(native), "runtime asset drift: {native}");
    }
    for layout in ["horizontal", "stacked"] {
        let black = read(&format!("assets/brand/medscale-{layout}-black.svg"));
        let white = read(&format!("assets/brand/medscale-{layout}-white.svg"));
        assert_eq!(attribute(paired_path(&black), "d"), geometry);
        assert_eq!(attribute(paired_path(&white), "d"), geometry);
        assert_eq!(attribute(paired_path(&black), "fill"), "#000000");
        assert_eq!(attribute(paired_path(&white), "fill"), "#FFFFFF");
        assert!(
            !black.contains("<text"),
            "lockup must retain actual vector glyphs"
        );
        assert!(
            !white.contains("<text"),
            "inverse must retain actual vector glyphs"
        );
    }
}

#[test]
fn application_icons_preserve_half_height_clear_space_and_visible_size() {
    for variant in ["light", "dark"] {
        let svg = read(&format!("assets/brand/medscale-app-icon-{variant}.svg"));
        let view_box = numbers(attribute(&svg, "viewBox"));
        let transform = numbers(attribute(&svg, "transform"));
        assert_eq!(view_box, vec![0.0, 0.0, 128.0, 128.0]);
        assert_eq!(transform.len(), 3, "translation plus uniform scale");
        let [x, y, scale] = [transform[0], transform[1], transform[2]];
        let height = 50.0 * 3.0_f64.sqrt() * scale;
        let width = 160.0 * scale;
        assert!(height >= 16.0, "visible mark below normal digital minimum");
        for padding in [x, y, view_box[2] - x - width, view_box[3] - y - height] {
            assert!(
                padding + 0.00001 >= height / 2.0,
                "app icon clear space below h/2"
            );
        }
        let canonical = read("assets/brand/mark-black.svg");
        assert_eq!(
            attribute(paired_path(&svg), "d"),
            attribute(paired_path(&canonical), "d")
        );
    }
    let provenance = read("third_party/provenance/095-approved-medscale-identity.md");
    assert!(provenance.to_lowercase().contains("favicon"));
    assert!(
        provenance.contains("16"),
        "tiny platform-slot exception must stay explicit"
    );
}

#[test]
fn complete_font_and_license_bytes_match_the_immutable_admission() {
    let provenance = read("third_party/provenance/095-native-typography.md");
    for (file, expected) in [
        (
            "InterVariable.ttf",
            "4989b125924991b90d05b2d16e0e388c48f7d5bb8b30539bbf9c755278d0ccaf",
        ),
        (
            "Inter-OFL.txt",
            "262481e844521b326f5ecd053e59b98c8b2da78c8ee1bdbb6e8174305e54935a",
        ),
        (
            "JetBrainsMonoNL-Regular.ttf",
            "fb3b2575d7b0657359707993288f12a7360344d39387bb26050e276d61f6bd2a",
        ),
        (
            "JetBrainsMono-OFL.txt",
            "30f0c136e3c88e422d0791acd97238870f9054a9729bc34cf2ff0d4ed8cac4ad",
        ),
    ] {
        let bytes = std::fs::read(repo_root().join("assets/brand/fonts").join(file))
            .expect("admitted font or license bytes");
        assert_eq!(
            DigestSha256::of(&bytes).to_hex(),
            expected,
            "asset drift: {file}"
        );
        assert!(provenance.contains(file));
        assert!(provenance.contains(expected));
    }
    for license in ["Inter-OFL.txt", "JetBrainsMono-OFL.txt"] {
        assert!(read(&format!("assets/brand/fonts/{license}")).contains("SIL OPEN FONT LICENSE"));
    }
    let app = read("crates/medscale-desktop/ui/app.slint");
    for font in ["InterVariable.ttf", "JetBrainsMonoNL-Regular.ttf"] {
        assert!(app.contains(&format!("import \"../../../assets/brand/fonts/{font}\";")));
    }
    let font_notice = read("assets/brand/fonts/FONT_NOTICE.md");
    assert!(font_notice.contains("Inter Project Authors"));
    assert!(font_notice.contains("JetBrains Mono Project Authors"));
    let builder = read("scripts/build-portable-release-package.ps1");
    let verifier = read("scripts/verify-portable-release-package.ps1");
    for file in ["Inter-OFL.txt", "JetBrainsMono-OFL.txt", "FONT_NOTICE.md"] {
        let packaged = format!("licenses/fonts/{file}");
        assert!(builder.contains(&packaged), "license not packaged: {file}");
        assert!(
            verifier.contains(&packaged),
            "license inventory not required: {file}"
        );
    }
    // Actual ZIP-mutation refusal proofs run in required CI's portable qualification step.
    let qualifier = read("scripts/qualify-portable-release-package.ps1");
    for failure in [
        "package file inventory mismatch",
        "payload hash mismatch: licenses/fonts/Inter-OFL.txt",
        "packaged font attribution missing from NOTICE.md",
    ] {
        assert!(qualifier.contains(failure));
    }
}

fn state_mapping(components: &str, property: &str) -> (BTreeMap<String, String>, String) {
    let prefix = format!("out property <string> {property}:");
    let expression = components
        .lines()
        .find_map(|line| line.trim().strip_prefix(&prefix))
        .expect("state mapping expression")
        .trim()
        .trim_end_matches(';');
    let mut mapping = BTreeMap::new();
    for branch in expression.split("state == \"").skip(1) {
        let (state, value) = branch.split_once('"').expect("state key");
        let label = value
            .split_once("? \"")
            .expect("state value")
            .1
            .split_once('"')
            .expect("state value end")
            .0;
        assert!(mapping.insert(state.to_owned(), label.to_owned()).is_none());
    }
    let fallback = expression
        .rsplit_once(" : \"")
        .expect("fallback value")
        .1
        .trim_end_matches('"')
        .to_owned();
    (mapping, fallback)
}

#[test]
fn every_product_state_has_distinct_literal_text_and_a_non_color_marker() {
    let components = read("crates/medscale-desktop/ui/components.slint");
    let (labels, label_fallback) = state_mapping(&components, "state-label");
    let (markers, marker_fallback) = state_mapping(&components, "state-marker");
    let states: BTreeSet<_> = [
        "current",
        "reviewed",
        "unknown",
        "stale",
        "conflicting",
        "partial",
        "denied",
        "unavailable",
        "unsupported",
        "unmeasured",
        "corrupt",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert_eq!(labels.keys().cloned().collect::<BTreeSet<_>>(), states);
    assert_eq!(markers.keys().cloned().collect::<BTreeSet<_>>(), states);
    assert_eq!(labels.values().collect::<BTreeSet<_>>().len(), states.len());
    assert_eq!(
        markers.values().collect::<BTreeSet<_>>().len(),
        states.len()
    );
    for (state, label) in &labels {
        assert_eq!(
            label.to_ascii_lowercase(),
            *state,
            "state collapsed: {state}"
        );
    }
    for marker in markers.values() {
        assert_eq!(marker.len(), 1, "single readable marker");
        assert!(marker.is_ascii() && !marker.chars().any(char::is_whitespace));
    }
    assert_eq!(label_fallback, "Unknown");
    assert_eq!(marker_fallback, markers["unknown"]);
    assert!(components.contains("state: \"unknown\""));
    assert!(components.contains("accessible-label: root.display-label"));
    assert!(components.contains("state-label + \" · \" + label"));
    assert!(components.contains("text: root.display-label"));
    assert!(components.contains("root.state-marker"));
}
