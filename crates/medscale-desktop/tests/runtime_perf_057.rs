//! Spec 057 runtime performance coverage.
//! Measures only; never claims Trusted V1 budget attainment.

use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const WARMUP: usize = 3;
const RUNS_DEFAULT: usize = 30;
const IDLE_MS_DEFAULT: u64 = 1_500;
const RSS_SAMPLES: usize = 5;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate parent")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_medscale-desktop"))
}

fn timed_runs() -> usize {
    std::env::var("MEDSCALE_057_TIMED_RUNS")
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(RUNS_DEFAULT)
        .clamp(3, 30)
}

fn idle_ms() -> u64 {
    std::env::var("MEDSCALE_057_IDLE_MS")
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(IDLE_MS_DEFAULT)
        .clamp(500, 10_000)
}

fn percentile_ms(sorted: &[Duration], pct: usize) -> f64 {
    assert!(!sorted.is_empty());
    let index = ((sorted.len() * pct) / 100).min(sorted.len() - 1);
    sorted[index].as_secs_f64() * 1_000.0
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo_root())
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn cargo_lock_sha256() -> String {
    let bytes = fs::read(repo_root().join("Cargo.lock")).expect("read Cargo.lock");
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn rustc_version() -> String {
    Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_else(|| "rustc-version-unavailable".to_owned())
}

fn smoke_once() -> Duration {
    let started = Instant::now();
    let output = Command::new(binary())
        .arg("--smoke")
        .output()
        .expect("run desktop smoke");
    let elapsed = started.elapsed();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("medscale-desktop smoke ok"));
    elapsed
}

#[cfg(unix)]
fn sample_rss_kib(pid: u32) -> u64 {
    let output = Command::new("ps")
        .args(["-o", "rss=", "-p", &pid.to_string()])
        .output()
        .expect("sample RSS with ps");
    assert!(output.status.success(), "ps RSS sample failed");
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u64>()
        .expect("parse ps RSS KiB")
}

#[cfg(windows)]
fn sample_rss_kib(pid: u32) -> u64 {
    let output = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {pid}"), "/FO", "CSV", "/NH"])
        .output()
        .expect("sample RSS with tasklist");
    assert!(output.status.success(), "tasklist RSS sample failed");
    let line = String::from_utf8_lossy(&output.stdout);
    let field = line
        .trim()
        .rsplit(",\"")
        .next()
        .expect("tasklist memory field");
    let digits: String = field.chars().filter(char::is_ascii_digit).collect();
    digits.parse::<u64>().expect("parse tasklist RSS KiB")
}

fn measure_idle_rss_kib() -> Vec<u64> {
    let idle_ms = idle_ms();
    let mut child = Command::new(binary())
        .args(["--perf-idle-ms", &idle_ms.to_string()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn desktop idle probe");
    let stdout = child.stdout.take().expect("idle probe stdout");
    let mut reader = BufReader::new(stdout);
    let mut ready = String::new();
    reader.read_line(&mut ready).expect("read idle readiness");
    assert_eq!(ready.trim(), "medscale-desktop perf-idle-ready");

    let mut samples = Vec::with_capacity(RSS_SAMPLES);
    for _ in 0..RSS_SAMPLES {
        thread::sleep(Duration::from_millis(100));
        samples.push(sample_rss_kib(child.id()));
    }
    drop(reader);
    let status = child.wait().expect("wait idle probe");
    assert!(status.success());
    samples
}

fn evidence_dir() -> PathBuf {
    repo_root().join("target/medscale-evidence/057-release-qualification-residual-integrity")
}

#[test]
fn runtime_perf_057_measures_without_claiming_attainment() {
    for _ in 0..WARMUP {
        let _ = smoke_once();
    }

    let runs = timed_runs();
    let mut launch_samples: Vec<Duration> = (0..runs).map(|_| smoke_once()).collect();
    launch_samples.sort();
    let launch_p50 = percentile_ms(&launch_samples, 50);
    let launch_p95 = percentile_ms(&launch_samples, 95);
    let launch_raw: Vec<f64> = launch_samples
        .iter()
        .map(|sample| sample.as_secs_f64() * 1_000.0)
        .collect();

    let rss_samples = measure_idle_rss_kib();
    let idle_rss_kib = *rss_samples.iter().max().expect("RSS samples");
    assert!(launch_p50.is_finite() && launch_p95.is_finite());
    assert!(launch_p95 >= launch_p50);
    assert!(idle_rss_kib > 0);

    let report = json!({
        "spec_id": "057-release-qualification-residual-integrity",
        "schema_version": 1,
        "budgets_claimed_met": false,
        "release_ready": false,
        "binding": {
            "git_sha": git_output(&["rev-parse", "HEAD"]),
            "git_tree": git_output(&["rev-parse", "HEAD^{tree}"]),
            "cargo_lock_sha256": cargo_lock_sha256(),
            "rustc_version": rustc_version(),
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "binary": binary().display().to_string(),
        },
        "methodology": {
            "warmup_runs": WARMUP,
            "timed_runs": runs,
            "idle_probe_ms": idle_ms(),
            "idle_rss_samples": RSS_SAMPLES,
            "synthetic_only": true,
            "real_phi": false,
        },
        "delivery_plan_targets_not_claimed": {
            "cold_model_free_launch_p95_ms": 2000,
            "ui_interaction_response_ms": 100,
            "model_free_idle_memory_mib": 250,
        },
        "results": {
            "desktop_scaffold_cold_launch_ms": {
                "p50": launch_p50,
                "p95": launch_p95,
                "samples": launch_raw,
            },
            "desktop_scaffold_idle_rss": {
                "max_kib": idle_rss_kib,
                "max_mib": (idle_rss_kib as f64) / 1024.0,
                "samples_kib": rss_samples,
            },
        },
        "honesty": {
            "cold_launch_scope": "desktop scaffold process + model-free doctor init",
            "idle_memory_scope": "desktop scaffold engineering idle probe",
            "final_ui_response": "BLOCKED_BY_FINAL_V0_UI",
            "final_shell_remeasurement_required": true,
            "qualified_hardware_required_for_attainment": true,
        },
    });

    assert_eq!(report["budgets_claimed_met"], false);
    assert_eq!(report["release_ready"], false);
    assert_eq!(
        report["honesty"]["final_ui_response"],
        "BLOCKED_BY_FINAL_V0_UI"
    );

    let out_dir = evidence_dir();
    fs::create_dir_all(&out_dir).expect("create runtime perf evidence dir");
    let out_path = out_dir.join(format!("runtime_perf_{}.json", std::env::consts::OS));
    fs::write(
        &out_path,
        serde_json::to_string_pretty(&report).expect("serialize report"),
    )
    .expect("write runtime perf report");
    assert!(Path::new(&out_path).is_file());
}
