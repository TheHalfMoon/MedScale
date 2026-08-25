//! MedScale CLI — facade-only authority surface (Spec 006).

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use medscale_contracts::doctor::PrivacyProof;
use medscale_core::{CliSession, build_doctor_report, privacy_proof_artifact_present};

#[derive(Debug, Parser)]
#[command(name = "medscale", version, about = "MedScale local-first CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Operator health / privacy / vault axes (no secrets).
    Doctor {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        vault_root: Option<PathBuf>,
    },
    /// Encrypted or synthetic vault operations via Core Host.
    Vault {
        #[command(subcommand)]
        action: VaultCmd,
    },
    /// Ingest synthetic FHIR R4 JSON through the authority facade.
    Ingest {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        vault_root: PathBuf,
        #[arg(long)]
        file: PathBuf,
        #[arg(long, default_value = "synthetic")]
        mode: String,
    },
    /// Deterministic subject timeline (Spec 004).
    Timeline {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        subject: String,
        #[arg(long)]
        json: bool,
    },
    /// Narrow LLM-free Brief.
    Brief {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        subject: String,
        #[arg(long)]
        json: bool,
    },
    /// Coverage accounting.
    Coverage {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        subject: String,
        #[arg(long)]
        json: bool,
    },
    /// Emit PRIVACY_PROOF JSON schema (baseline).
    PrivacyProof {
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum VaultCmd {
    Create {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        vault_root: PathBuf,
        /// Read passphrase from this environment variable (never argv).
        #[arg(long, default_value = "MEDSCALE_PASSPHRASE")]
        passphrase_env: String,
    },
    Open {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        vault_root: PathBuf,
        #[arg(long, default_value = "MEDSCALE_PASSPHRASE")]
        passphrase_env: String,
    },
    OpenSynthetic {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        vault_root: PathBuf,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Doctor { json, vault_root } => {
            let root = vault_root.map(|p| p.display().to_string());
            let report =
                build_doctor_report(root.as_deref(), false, privacy_proof_artifact_present());
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("{} doctor", report.product_name);
                println!("version: {}", report.version);
                println!("local_only: {}", report.local_only);
                println!("egress: {}", report.product_runtime_egress);
                println!("synthetic_only: {}", report.synthetic_only);
                println!("real_phi_authorized: {}", report.real_phi_authorized);
                println!(
                    "vault_root: {}",
                    report.vault_root.as_deref().unwrap_or("unset")
                );
                println!("vault_open: {}", report.vault_open);
                println!("sync_risk: {:?}", report.sync_risk);
                println!("filesystem_claim_ok: {}", report.filesystem_claim_ok);
                println!("key_store: {:?}", report.key_store);
                println!("privacy_proof: {:?}", report.privacy_proof_freshness);
                println!("desktop_shell: {}", report.desktop_shell);
                println!("tauri_admitted: {}", report.tauri_admitted);
                for note in &report.notes {
                    println!("note: {note}");
                }
            }
            let rendered = if json {
                serde_json::to_string(&report)?
            } else {
                format!("{report:?}")
            };
            assert_no_secret_markers(&rendered)?;
            Ok(())
        }
        Commands::Vault { action } => match action {
            VaultCmd::Create {
                vault_id,
                vault_root,
                passphrase_env,
            } => {
                let pw = std::env::var(&passphrase_env).with_context(|| {
                    format!("set passphrase in env var {passphrase_env} (not argv)")
                })?;
                let mut session = CliSession::connect(&vault_id).map_err(auth)?;
                let codes = session
                    .create_encrypted_vault(&vault_root.display().to_string(), &pw)
                    .map_err(auth)?;
                println!("vault_created: {}", vault_root.display());
                println!("recovery_codes_count: {}", codes.len());
                println!("note: store recovery codes offline; not re-printed by doctor");
                Ok(())
            }
            VaultCmd::Open {
                vault_id,
                vault_root,
                passphrase_env,
            } => {
                let pw = std::env::var(&passphrase_env)
                    .with_context(|| format!("set passphrase in env var {passphrase_env}"))?;
                let mut session = CliSession::connect(&vault_id).map_err(auth)?;
                session
                    .open_encrypted_vault(&vault_root.display().to_string(), &pw)
                    .map_err(auth)?;
                println!("vault_open: {}", vault_root.display());
                Ok(())
            }
            VaultCmd::OpenSynthetic {
                vault_id,
                vault_root,
            } => {
                let mut session = CliSession::connect(&vault_id).map_err(auth)?;
                session
                    .open_synthetic_vault(&vault_root.display().to_string())
                    .map_err(auth)?;
                println!("synthetic_vault_open: {}", vault_root.display());
                Ok(())
            }
        },
        Commands::Ingest {
            vault_id,
            vault_root,
            file,
            mode,
        } => {
            if mode != "synthetic" {
                bail!("only synthetic ingest is authorized (REAL_PHI not authorized)");
            }
            let mut session = CliSession::connect(&vault_id).map_err(auth)?;
            session
                .open_synthetic_vault(&vault_root.display().to_string())
                .map_err(auth)?;
            let source_id = session
                .ingest_fhir_file(&file.display().to_string())
                .map_err(auth)?;
            println!("ingested_source_id: {}", source_id.as_str());
            Ok(())
        }
        Commands::Timeline {
            vault_id,
            subject,
            json,
        } => {
            let mut session = CliSession::connect(&vault_id).map_err(auth)?;
            let body = session.timeline(&subject).map_err(auth)?;
            print_json_or_debug(&body, json)
        }
        Commands::Brief {
            vault_id,
            subject,
            json,
        } => {
            let mut session = CliSession::connect(&vault_id).map_err(auth)?;
            let body = session.brief(&subject).map_err(auth)?;
            print_json_or_debug(&body, json)
        }
        Commands::Coverage {
            vault_id,
            subject,
            json,
        } => {
            let mut session = CliSession::connect(&vault_id).map_err(auth)?;
            let body = session.coverage(&subject).map_err(auth)?;
            print_json_or_debug(&body, json)
        }
        Commands::PrivacyProof { json } => {
            let proof = PrivacyProof::spec_006_baseline();
            if json {
                println!("{}", serde_json::to_string_pretty(&proof)?);
            } else {
                println!("{proof:?}");
            }
            Ok(())
        }
    }
}

fn auth(err: medscale_contracts::envelopes::AuthorityError) -> anyhow::Error {
    anyhow::anyhow!("authority error: {err:?}")
}

fn print_json_or_debug<T: serde::Serialize + std::fmt::Debug>(value: &T, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(value)?);
    } else {
        println!("{value:?}");
    }
    Ok(())
}

fn assert_no_secret_markers(text: &str) -> Result<()> {
    for marker in [
        "BEGIN PRIVATE",
        "passphrase=",
        "VaultDek(",
        "MEDSCALE_PASSPHRASE=",
    ] {
        if text.contains(marker) {
            bail!("doctor/CLI output contained forbidden secret marker: {marker}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use medscale_contracts::objects::{AuthorityScopeId, OpaqueId, RealmId};
    use medscale_core::CliSession;
    use std::fs;

    #[test]
    fn doctor_json_has_required_axes_and_no_secrets() {
        let report = build_doctor_report(None, false, false);
        let json = serde_json::to_string(&report).unwrap();
        for key in [
            "product_name",
            "version",
            "local_only",
            "product_runtime_egress",
            "synthetic_only",
            "real_phi_authorized",
            "sync_risk",
            "key_store",
            "privacy_proof_freshness",
            "tauri_admitted",
        ] {
            assert!(json.contains(key), "missing {key}");
        }
        assert_no_secret_markers(&json).unwrap();
        assert!(!report.tauri_admitted);
        assert!(!report.real_phi_authorized);
    }

    #[test]
    fn privacy_proof_limitations_include_tauri_and_no_systemwide_claim() {
        let proof = PrivacyProof::spec_006_baseline();
        let blob = serde_json::to_string(&proof).unwrap();
        assert!(
            blob.contains("Tauri")
                || blob.contains("tauri")
                || proof.limitations.iter().any(|l| l.contains("Tauri"))
        );
        assert!(
            proof
                .limitations
                .iter()
                .any(|l| l.contains("system-wide") || l.contains("zero packets"))
        );
        assert_eq!(
            proof.webview_cache_scan,
            medscale_contracts::doctor::WebViewScanStatus::NotApplicable
        );
    }

    #[test]
    fn cli_does_not_depend_on_storage_or_rusqlite() {
        let manifest =
            fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")).unwrap();
        assert!(!manifest.contains("medscale-storage"));
        assert!(!manifest.contains("rusqlite"));
        assert!(!manifest.contains("tauri"));
    }

    #[test]
    fn cli_longitudinal_wedge() {
        let root = std::env::temp_dir().join(format!("medscale-006-wedge-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();

        let mut session = CliSession::connect("cli-wedge").unwrap();
        session
            .open_synthetic_vault(&root.display().to_string())
            .unwrap();

        let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/synthetic/fhir/r4/presentation/patient-golden.json");
        let bytes = fs::read(&fixture).unwrap();
        let resource: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        let source_id = session
            .ingest_fhir_file(&fixture.display().to_string())
            .unwrap();
        let subject = "subject-wedge";
        session
            .promote_fixture_as_assertion(source_id, subject, "patient", resource)
            .unwrap();

        let tl = session.timeline(subject).unwrap();
        assert!(!tl.events.is_empty());
        let brief = session.brief(subject).unwrap();
        assert!(!brief.sections.identity.is_empty());
        let cov = session.coverage(subject).unwrap();
        assert!(!cov.slots.is_empty());

        // Touch realm/scope types so wedge stays typed.
        let _ = RealmId::new("x");
        let _ = AuthorityScopeId::new("y");
        let _ = OpaqueId::new("z");
    }
}
