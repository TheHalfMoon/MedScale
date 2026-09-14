//! Native MedScale Desktop shell — Slint, no WebView/Tauri (Spec 060).

use medscale_core::{CoreFacade, build_doctor_report, privacy_proof_artifact_present};
use std::env;
use std::hint::black_box;
use std::io::{self, Write};
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

use slint::ComponentHandle;

slint::include_modules!();

const PERF_IDLE_MAX_MS: u64 = 10_000;

fn perf_idle_ms(args: &[String]) -> Result<Option<u64>, &'static str> {
    let Some(index) = args.iter().position(|arg| arg == "--perf-idle-ms") else {
        return Ok(None);
    };
    let Some(raw) = args.get(index + 1) else {
        return Err("--perf-idle-ms requires a millisecond value");
    };
    let value = raw
        .parse::<u64>()
        .map_err(|_| "invalid --perf-idle-ms value")?;
    if !(50..=PERF_IDLE_MAX_MS).contains(&value) {
        return Err("--perf-idle-ms must be between 50 and 10000");
    }
    Ok(Some(value))
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let smoke = args.iter().any(|arg| arg == "--smoke");
    let idle_ms = match perf_idle_ms(&args) {
        Ok(value) => value,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };

    let report = build_doctor_report(None, false, privacy_proof_artifact_present());
    if let Some(ms) = idle_ms {
        black_box(&report);
        println!("medscale-desktop perf-idle-ready");
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(ms));
        black_box(&report);
        return ExitCode::SUCCESS;
    }

    if smoke {
        println!("medscale-desktop smoke ok");
        println!("version={}", CoreFacade::version());
        println!("tauri_admitted={}", report.tauri_admitted);
        println!("desktop_shell={}", report.desktop_shell);
        return ExitCode::SUCCESS;
    }

    let ui = match AppWindow::new() {
        Ok(ui) => ui,
        Err(err) => {
            eprintln!("failed to initialize MedScale Desktop UI: {err}");
            return ExitCode::from(1);
        }
    };
    ui.set_product_version(report.version.into());

    // Spec 060 keeps shell actions useful without bypassing Core authority.
    // Consequential operations are routed to their owning review surfaces; no action is committed here.
    let weak = ui.as_weak();
    ui.on_ui_action(move |action| {
        let Some(ui) = weak.upgrade() else {
            return;
        };
        let action = action.as_str();
        let route = if action == "care-plan" {
            Some("Workflows")
        } else if action == "summarize" || action == "import" || action.starts_with("search:") {
            Some("Patients")
        } else if action == "privacy-status" {
            Some("Settings")
        } else {
            None
        };
        if let Some(route) = route {
            ui.set_active_route(route.into());
        }
    });

    match ui.run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("MedScale Desktop UI exited with an error: {err}");
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::perf_idle_ms;

    #[test]
    fn perf_idle_probe_is_bounded_and_explicit() {
        assert_eq!(
            perf_idle_ms(&["--perf-idle-ms".to_owned(), "250".to_owned()]),
            Ok(Some(250))
        );
        assert!(perf_idle_ms(&["--perf-idle-ms".to_owned(), "49".to_owned()]).is_err());
        assert!(perf_idle_ms(&["--perf-idle-ms".to_owned()]).is_err());
    }

    #[test]
    fn no_tauri_in_manifest() {
        let manifest =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")).unwrap();
        assert!(!manifest.contains("tauri"));
    }
}
