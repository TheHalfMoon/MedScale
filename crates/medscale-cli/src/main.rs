//! MedScale CLI — facade-only authority surface (Spec 006).

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use medscale_contracts::doctor::PrivacyProof;
use medscale_contracts::fixture_ui::{FixtureUiSurface, FixtureUiViewModel};
use medscale_contracts::workflow::CliJsonError;
use medscale_core::{
    CliSession, CoreFacade, HostIpcClient, HostIpcServer, JourneyConfig, build_doctor_report,
    endpoint_for_vault_root, privacy_proof_artifact_present, run_minimum_lovable_journey,
};

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
    /// Offline Pack v0 operations (Spec 008; local path only).
    Packs {
        #[command(subcommand)]
        action: PacksCmd,
    },
    /// Deterministic fixture UI view-models (no final visual design).
    FixtureUi {
        #[command(subcommand)]
        action: FixtureUiCmd,
    },
    /// Spec 021 minimum lovable trusted workflow (synthetic).
    Journey {
        #[command(subcommand)]
        action: JourneyCmd,
    },
    /// Localhost OS IPC host/client (Spec 024 READY_BASE).
    HostIpc {
        #[command(subcommand)]
        action: HostIpcCmd,
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

#[derive(Debug, Subcommand)]
enum PacksCmd {
    /// Admit a Pack from a local directory (offline only).
    Install {
        #[arg(long)]
        vault_id: String,
        path: PathBuf,
    },
    /// List admitted packs in the current Core Host session.
    List {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum FixtureUiCmd {
    /// Emit doctor FixtureUiViewModel JSON (synthetic-only; no visual design).
    Doctor {
        #[arg(long)]
        vault_root: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum JourneyCmd {
    /// Run the documented synthetic end-to-end journey via CoreFacade.
    Run {
        #[arg(long)]
        vault_root: PathBuf,
        #[arg(long)]
        fixture: PathBuf,
        #[arg(long, default_value = "journey-subject")]
        subject: String,
        #[arg(long, default_value = "journey-vault")]
        vault_id: String,
        #[arg(long)]
        backup_dir: PathBuf,
        #[arg(long)]
        restore_dir: PathBuf,
        /// Reject the previewed proposal instead of accepting.
        #[arg(long, default_value_t = false)]
        reject: bool,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum HostIpcCmd {
    /// Serve CoreFacade over localhost OS IPC (holds Spec 016 writer lock).
    Serve {
        #[arg(long)]
        vault_root: PathBuf,
        /// Namespaced endpoint ([A-Za-z0-9_-]+). Default: hash of vault_root.
        #[arg(long)]
        endpoint: Option<String>,
    },
    /// Dispatch one JSON AuthorityRequest file via IPC and print the response.
    Call {
        #[arg(long)]
        endpoint: String,
        #[arg(long)]
        request_json: PathBuf,
    },
}

/// Stable CLI exit codes (Spec 021).
const EXIT_OK: u8 = 0;
const EXIT_ERROR: u8 = 1;
const EXIT_JSON_ERROR: u8 = 2;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::from(EXIT_OK),
        Err(err) => {
            if let Some(json_err) = err.downcast_ref::<JsonCliFailure>() {
                eprintln!("{}", json_err.0);
                ExitCode::from(EXIT_JSON_ERROR)
            } else {
                eprintln!("error: {err:#}");
                ExitCode::from(EXIT_ERROR)
            }
        }
    }
}

#[derive(Debug)]
struct JsonCliFailure(String);

impl std::fmt::Display for JsonCliFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for JsonCliFailure {}

fn fail_json(code: &str, message: impl Into<String>, as_json: bool) -> anyhow::Error {
    let err = CliJsonError::new(code, message);
    if as_json {
        match serde_json::to_string_pretty(&err) {
            Ok(s) => JsonCliFailure(s).into(),
            Err(e) => anyhow::anyhow!("json error encode failed: {e}"),
        }
    } else {
        anyhow::anyhow!("{}: {}", err.code, err.message)
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
                println!(
                    "network_broker: present={} default_deny={} allowlist_entries={} live_partner={}",
                    report.network_broker.present,
                    report.network_broker.default_deny,
                    report.network_broker.allowlist_entries,
                    report.network_broker.live_partner_authorized
                );
                println!(
                    "packs_runtime: present={} offline_only={} admitted={} online_download={}",
                    report.packs_runtime.present,
                    report.packs_runtime.offline_only,
                    report.packs_runtime.admitted_count,
                    report.packs_runtime.online_download_authorized
                );
                println!(
                    "mobile: present={} apps_shipped={} 16kb={}",
                    report.mobile.present,
                    report.mobile.apps_shipped,
                    report.mobile.page_size_16kb_claim
                );
                println!(
                    "controlled_actions: present={} outbox_restart_qualified={} nphies={} blind_retry={}",
                    report.controlled_actions.present,
                    report.controlled_actions.outbox_restart_qualified,
                    report.controlled_actions.nphies_authorized,
                    report.controlled_actions.unknown_blind_retry
                );
                println!(
                    "online_packs: present={} authorized={} broker={}",
                    report.online_packs.present,
                    report.online_packs.online_download_authorized,
                    report.online_packs.broker_required
                );
                println!(
                    "mesc_artifact: present={} admitted={} gate={}",
                    report.mesc_artifact.present,
                    report.mesc_artifact.artifact_admitted,
                    report.mesc_artifact.gate
                );
                println!(
                    "vault_privacy: ready={} sealed_at_close={} open_work_risk={} page_encrypted={} sqlcipher={} wipe_on_close={} os_keyring_available={} os_keyring_used={} probes_present={} encrypted_authority_sync_qualified={} residual_open={} pagefile={:?} hibernate={:?}",
                    report.vault_privacy.private_data_ready,
                    report.vault_privacy.sealed_at_close,
                    report.vault_privacy.open_work_plaintext_risk,
                    report.vault_privacy.open_work_page_encrypted,
                    report.vault_privacy.sqlcipher_enabled,
                    report.vault_privacy.work_wipe_on_close,
                    report.vault_privacy.os_keyring_available,
                    report.vault_privacy.os_keyring_used,
                    report.vault_privacy.probes_present,
                    report.vault_privacy.encrypted_authority_sync_qualified,
                    report.vault_privacy.residual_risk_classes_open.join(","),
                    report.vault_privacy.pagefile_existence,
                    report.vault_privacy.hibernate_file_existence
                );
                println!(
                    "host_authority: ready_base={} multi_client_release={} os_ipc={}",
                    report.host_authority.ready_base,
                    report.host_authority.multi_client_release_ready,
                    report.host_authority.os_ipc_qualified
                );
                println!(
                    "record_semantics: ready_base={} precision_time={} amendments={} release_ready={}",
                    report.record_semantics.ready_base,
                    report.record_semantics.precision_aware_time,
                    report.record_semantics.append_only_amendments,
                    report.record_semantics.release_ready
                );
                println!(
                    "fhir_interchange: ready_base={} fhir={} full_conformance={} validator_authority={} release_ready={}",
                    report.fhir_interchange.ready_base,
                    report.fhir_interchange.fhir_version,
                    report.fhir_interchange.full_conformance_claimed,
                    report.fhir_interchange.validator_is_authority,
                    report.fhir_interchange.release_ready
                );
                println!(
                    "fhir_support_matrix: resources={} patient_structural={:?} conformance_claimed={}",
                    report.fhir_support_matrix.resources.len(),
                    report.fhir_support_matrix.status_for("Patient").structural,
                    report.fhir_support_matrix.full_conformance_claimed
                );
                println!(
                    "workflow: ready_base={} release_ready={} disclosure={}",
                    report.workflow.workflow_ready_base,
                    report.workflow.release_ready,
                    report.workflow.disclosure_append_supported
                );
                println!(
                    "release_qualification: prep_ready_base={} release_ready={} locked={} macos_ci={} macos_qualified={} branch_protection={} perf_harness={} sbom_scaffold={} notice_inventory={} rights_license_decision={} missing={}",
                    report.release_qualification.prep_ready_base,
                    report.release_qualification.release_ready,
                    report.release_qualification.locked_builds,
                    report.release_qualification.macos_ci_present,
                    report.release_qualification.macos_qualified,
                    report.release_qualification.branch_protection_configured,
                    report.release_qualification.perf_harness_present,
                    report.release_qualification.sbom_scaffold_present,
                    report.release_qualification.notice_inventory_present,
                    report.release_qualification.rights_license_decision,
                    report.release_qualification.missing_evidence_classes.len()
                );
                println!(
                    "accessibility: ready_base={} fixture_labels={} cli_keyboard={} disclosure={} wcag_claimed={} final_v0={} release_ready={}",
                    report.accessibility.ready_base,
                    report.accessibility.fixture_cli_labels_checked,
                    report.accessibility.cli_keyboard_path_documented,
                    report.accessibility.disclosure_clarity_checked,
                    report.accessibility.wcag_conformance_claimed,
                    report.accessibility.final_v0_ui_present,
                    report.accessibility.release_ready
                );
                println!(
                    "evidence_corpus: ready_base={} versioned={} synthetic_owned={} clinical_quality={} release_ready={} corpus={:?}@{:?}",
                    report.evidence_corpus.ready_base,
                    report.evidence_corpus.versioned_corpus,
                    report.evidence_corpus.synthetic_owned_only,
                    report.evidence_corpus.clinical_quality_claimed,
                    report.evidence_corpus.release_ready,
                    report.evidence_corpus.current_corpus_id,
                    report.evidence_corpus.current_version
                );
                println!(
                    "pack_signer: ready_base={} synthetic_trust_root={} anti_rollback={} release_ready={}",
                    report.pack_signer.ready_base,
                    report.pack_signer.synthetic_trust_root,
                    report.pack_signer.anti_rollback,
                    report.pack_signer.release_ready
                );
                println!(
                    "os_sandbox: ready_base={} linux_measured={} windows_measured={} windows_appcontainer_fs_measured={} macos_measured={} platform_qualified={} release_ready={}",
                    report.os_sandbox.ready_base,
                    report.os_sandbox.linux_measured,
                    report.os_sandbox.windows_measured,
                    report.os_sandbox.windows_appcontainer_fs_measured,
                    report.os_sandbox.macos_measured,
                    report.os_sandbox.platform_qualified,
                    report.os_sandbox.release_ready
                );
                for missing in &report.release_qualification.missing_evidence_classes {
                    println!("release_qualification_missing: {missing}");
                }
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
        Commands::Packs { action } => match action {
            PacksCmd::Install { vault_id, path } => {
                let mut session = CliSession::connect(&vault_id).map_err(auth)?;
                let result = session
                    .packs_install_local(&path.display().to_string())
                    .map_err(auth)?;
                println!("{}", serde_json::to_string_pretty(&result)?);
                if !result.admitted {
                    bail!("pack admission denied: {:?}", result.reason);
                }
                Ok(())
            }
            PacksCmd::List { vault_id, json } => {
                let mut session = CliSession::connect(&vault_id).map_err(auth)?;
                // Note: list is session-local; install then list in same process for CLI demos.
                let packs = session.packs_list().map_err(auth)?;
                print_json_or_debug(&packs, json)
            }
        },
        Commands::FixtureUi { action } => match action {
            FixtureUiCmd::Doctor { vault_root } => {
                let root = vault_root.map(|p| p.display().to_string());
                let report =
                    build_doctor_report(root.as_deref(), false, privacy_proof_artifact_present());
                let vm = FixtureUiViewModel::from_doctor(&report);
                if vm.surface != FixtureUiSurface::Doctor {
                    bail!("unexpected fixture UI surface");
                }
                if !vm.respects_phi_boundary() {
                    bail!("fixture UI PHI boundary violated");
                }
                assert_no_secret_markers(&serde_json::to_string(&vm)?)?;
                println!("{}", serde_json::to_string_pretty(&vm)?);
                Ok(())
            }
        },
        Commands::Journey { action } => match action {
            JourneyCmd::Run {
                vault_root,
                fixture,
                subject,
                vault_id,
                backup_dir,
                restore_dir,
                reject,
                json,
            } => {
                if !fixture.is_file() {
                    return Err(fail_json(
                        "fixture_missing",
                        format!("fixture not found: {}", fixture.display()),
                        json,
                    ));
                }
                let _ = std::fs::remove_dir_all(&backup_dir);
                let _ = std::fs::remove_dir_all(&restore_dir);
                let facade = CoreFacade::new();
                let cfg = JourneyConfig {
                    vault_id,
                    vault_root: vault_root.display().to_string(),
                    fixture_path: fixture.display().to_string(),
                    subject,
                    backup_dir: backup_dir.display().to_string(),
                    restore_dir: restore_dir.display().to_string(),
                    accept: !reject,
                };
                match run_minimum_lovable_journey(&facade, &cfg) {
                    Ok(report) => {
                        if !report.is_honest_ready_base() {
                            return Err(fail_json(
                                "honesty_failed",
                                "journey report must be synthetic READY_BASE with release_ready=false",
                                json,
                            ));
                        }
                        if json {
                            println!("{}", serde_json::to_string_pretty(&report)?);
                        } else {
                            println!("journey_ok: steps={}", report.steps.len());
                            println!("workflow_ready_base: {}", report.workflow_ready_base);
                            println!("release_ready: {}", report.release_ready);
                            if let Some(sid) = &report.source_id {
                                println!("source_id: {sid}");
                            }
                            if let Some(aid) = &report.assertion_id {
                                println!("assertion_id: {aid}");
                            }
                            if let Some(did) = &report.disclosure_id {
                                println!("disclosure_id: {did}");
                            }
                            for step in &report.steps {
                                println!("step {:?}: ok={}", step.step, step.ok);
                            }
                        }
                        Ok(())
                    }
                    Err(e) => Err(fail_json("journey_failed", format!("{e:?}"), json)),
                }
            }
        },
        Commands::HostIpc { action } => match action {
            HostIpcCmd::Serve {
                vault_root,
                endpoint,
            } => {
                std::fs::create_dir_all(&vault_root)
                    .with_context(|| format!("create vault_root {}", vault_root.display()))?;
                let endpoint = endpoint.unwrap_or_else(|| endpoint_for_vault_root(&vault_root));
                let server = HostIpcServer::bind(&vault_root, &endpoint)
                    .with_context(|| format!("bind host-ipc endpoint={endpoint}"))?;
                println!(
                    "host_ipc_listening endpoint={} vault_root={} os_ipc_qualified=true multi_client_release_ready=false",
                    server.endpoint(),
                    server.vault_root().display()
                );
                loop {
                    if let Err(e) = server.serve_connection() {
                        eprintln!("host_ipc_connection_error: {e}");
                    }
                }
            }
            HostIpcCmd::Call {
                endpoint,
                request_json,
            } => {
                let bytes = std::fs::read(&request_json)
                    .with_context(|| format!("read {}", request_json.display()))?;
                let req: medscale_contracts::envelopes::AuthorityRequest =
                    serde_json::from_slice(&bytes).context("parse AuthorityRequest JSON")?;
                let mut client = HostIpcClient::connect(&endpoint)
                    .with_context(|| format!("connect endpoint={endpoint}"))?;
                let resp = client.dispatch(req).context("ipc dispatch")?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
                Ok(())
            }
        },
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
            "network_broker",
            "default_deny",
            "packs_runtime",
            "offline_only",
            "mobile",
            "apps_shipped",
            "controlled_actions",
            "outbox_restart_qualified",
            "nphies_authorized",
            "online_packs",
            "broker_required",
            "mesc_artifact",
            "artifact_admitted",
            "record_semantics",
            "precision_aware_time",
            "fhir_interchange",
            "fhir_support_matrix",
            "full_conformance_claimed",
            "validator_is_authority",
            "workflow",
            "workflow_ready_base",
            "release_qualification",
            "prep_ready_base",
            "macos_ci_present",
            "missing_evidence_classes",
            "perf_harness_present",
            "sbom_scaffold_present",
            "accessibility",
            "fixture_cli_labels_checked",
            "cli_keyboard_path_documented",
            "disclosure_clarity_checked",
            "wcag_conformance_claimed",
            "pack_signer",
            "synthetic_trust_root",
            "anti_rollback",
            "os_sandbox",
            "linux_measured",
            "windows_measured",
            "windows_appcontainer_fs_measured",
            "macos_measured",
            "platform_qualified",
            "os_keyring_available",
            "os_keyring_used",
            "private_data_ready",
            "probes_present",
            "encrypted_authority_sync_qualified",
            "residual_risk_classes_open",
            "pagefile_existence",
            "hibernate_file_existence",
            "notice_inventory_present",
            "rights_license_decision",
        ] {
            assert!(json.contains(key), "missing {key}");
        }
        assert_no_secret_markers(&json).unwrap();
        assert!(!report.tauri_admitted);
        assert!(!report.real_phi_authorized);
        assert!(report.network_broker.default_deny);
        assert!(!report.network_broker.live_partner_authorized);
        assert!(report.packs_runtime.offline_only);
        assert!(!report.packs_runtime.online_download_authorized);
        assert!(!report.mobile.apps_shipped);
        assert!(report.controlled_actions.is_honest_ready_base());
        assert!(!report.controlled_actions.nphies_authorized);
        assert!(!report.controlled_actions.unknown_blind_retry);
        assert!(!report.online_packs.online_download_authorized);
        assert!(report.online_packs.broker_required);
        assert!(!report.online_packs.hf_runtime_required);
        assert!(!report.mesc_artifact.artifact_admitted);
        assert!(!report.mesc_artifact.python_runtime_imported);
        assert!(!report.fhir_interchange.full_conformance_claimed);
        assert!(!report.fhir_interchange.validator_is_authority);
        assert!(!report.fhir_interchange.release_ready);
        assert!(report.fhir_support_matrix.is_honest_ready_base());
        assert!(report.workflow.workflow_ready_base);
        assert!(!report.workflow.release_ready);
        assert!(report.release_qualification.is_honest_prep());
        assert!(report.release_qualification.macos_ci_present);
        assert!(!report.release_qualification.macos_qualified);
        assert!(!report.release_qualification.release_ready);
        assert!(report.accessibility.is_honest_ready_base());
        assert!(!report.accessibility.wcag_conformance_claimed);
        assert!(report.pack_signer.is_honest_ready_base());
        assert!(report.os_sandbox.is_honest_ready_base());
    }

    #[test]
    fn fixture_ui_and_cli_help_surface_required_a11y_labels_without_wcag_claim() {
        use clap::CommandFactory;

        let report = build_doctor_report(None, false, false);
        let vm = FixtureUiViewModel::from_doctor(&report);
        assert!(vm.has_required_a11y_labels());
        assert!(FixtureUiViewModel::required_surface_labels().contains(&vm.title.as_str()));
        assert!(!report.accessibility.wcag_conformance_claimed);
        assert!(!report.accessibility.release_ready);

        let mut help = Vec::new();
        Cli::command()
            .write_long_help(&mut help)
            .expect("render CLI help");
        let help_text = String::from_utf8(help).expect("utf8 help").to_lowercase();
        for required in [
            "doctor",
            "vault",
            "ingest",
            "timeline",
            "brief",
            "coverage",
            "privacy-proof",
            "fixture-ui",
            "journey",
            "host-ipc",
        ] {
            assert!(
                help_text.contains(required),
                "CLI help missing required label/path: {required}"
            );
        }

        // --json lives on subcommands (keyboard/operator path), not top-level about.
        let mut doctor_help = Vec::new();
        Cli::command()
            .find_subcommand_mut("doctor")
            .expect("doctor subcommand")
            .write_long_help(&mut doctor_help)
            .expect("render doctor help");
        let doctor_help = String::from_utf8(doctor_help).expect("utf8").to_lowercase();
        assert!(
            doctor_help.contains("--json"),
            "doctor help must expose --json for operator keyboard path"
        );

        let err = CliJsonError::new("vault_required", "open a vault first");
        assert_eq!(err.code, "vault_required");
        assert!(err.synthetic_only);
        assert_eq!(err.error, "cli_error");
    }

    #[test]
    fn fixture_ui_doctor_respects_phi_and_has_no_secrets() {
        let report = build_doctor_report(None, false, false);
        let vm = FixtureUiViewModel::from_doctor(&report);
        assert_eq!(vm.surface, FixtureUiSurface::Doctor);
        assert!(vm.respects_phi_boundary());
        let json = serde_json::to_string(&vm).unwrap();
        assert_no_secret_markers(&json).unwrap();
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
