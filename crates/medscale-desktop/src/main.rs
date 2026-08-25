//! Thin Desktop scaffold — no WebView/Tauri (Spec 006).

use medscale_core::{CoreFacade, build_doctor_report, privacy_proof_artifact_present};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let smoke = env::args().any(|a| a == "--smoke");
    let report = build_doctor_report(None, false, privacy_proof_artifact_present());
    if smoke {
        println!("medscale-desktop smoke ok");
        println!("version={}", CoreFacade::version());
        println!("tauri_admitted={}", report.tauri_admitted);
        println!("desktop_shell={}", report.desktop_shell);
        return ExitCode::SUCCESS;
    }
    println!("{} desktop scaffold (non-WebView)", report.product_name);
    println!("version: {}", report.version);
    println!("shell: {}", report.desktop_shell);
    println!("tauri_admitted: {}", report.tauri_admitted);
    println!("note: final v0 UI deferred (FINAL_V0_UI_ARTIFACT); Tauri privacy deferred");
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_tauri_in_manifest() {
        let manifest =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")).unwrap();
        assert!(!manifest.contains("tauri"));
    }
}
