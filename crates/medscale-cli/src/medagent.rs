//! Spec 077 MedAgent Workbench commands (CLI vertical slice through Core).
//!
//! T077-03 scope only: `AgentIdentity` + `AgentCapabilityManifest`
//! register/show/list/revoke. Every command opens the session scope,
//! dispatches one typed Core request via `CliSession`, and renders the
//! typed result as human lines or stable JSON. The CLI never writes
//! medagent storage directly.

use std::path::PathBuf;

use clap::Subcommand;
use medscale_contracts::envelopes::AuthorityError;
use medscale_contracts::medagent::{AgentCapabilityManifest, AgentIdentity, ToolKind};
use medscale_contracts::objects::OpaqueId;
use medscale_core::CliSession;

use super::{fail_json, print_json_or_debug};

fn medagent_fail(err: &AuthorityError, json: bool) -> anyhow::Error {
    let debug = format!("{err:?}");
    let (code, message) = match err {
        AuthorityError::Unauthorized
        | AuthorityError::SessionRequired
        | AuthorityError::SessionExpired
        | AuthorityError::SessionRevoked
        | AuthorityError::SessionDenied
        | AuthorityError::WrongScope => ("denied", debug),
        AuthorityError::NotFound => ("not_found", debug),
        AuthorityError::Conflict { message } => ("conflict", message.clone()),
        AuthorityError::InvalidArgument { message } => ("invalid", message.clone()),
        AuthorityError::Corrupt { message } => ("corrupt", message.clone()),
        AuthorityError::UnsupportedSchema { message } => ("unsupported_schema", message.clone()),
        AuthorityError::LeaseRequired
        | AuthorityError::VaultRequired
        | AuthorityError::MissingKeyMaterial => ("unavailable", debug),
        _ => ("internal", debug),
    };
    fail_json(code, message, json)
}

fn invalid(message: String, json: bool) -> anyhow::Error {
    fail_json("invalid", message, json)
}

fn open_medagent_session(
    vault_id: &str,
    vault_root: &std::path::Path,
    json: bool,
) -> anyhow::Result<CliSession> {
    let mut session = CliSession::connect(vault_id).map_err(|err| medagent_fail(&err, json))?;
    session
        .open_synthetic_vault(&vault_root.display().to_string())
        .map_err(|err| medagent_fail(&err, json))?;
    Ok(session)
}

fn parse_tool_kinds(value: &str) -> Result<Vec<ToolKind>, String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToolKind::parse)
        .collect()
}

fn print_identity_human(identity: &AgentIdentity, capabilities: &AgentCapabilityManifest) {
    println!("agent_id: {}", identity.header.id.as_str());
    println!("project_id: {}", identity.project_id.as_str());
    println!("pack_id: {}", identity.pack_id.as_str());
    println!("pack_version: {}", identity.pack_version);
    println!("display_name: {}", identity.display_name);
    println!("status: {}", identity.status.as_str());
    println!("revision: {}", identity.revision);
    let kinds: Vec<&str> = capabilities
        .granted_tool_kinds
        .iter()
        .map(|k| k.as_str())
        .collect();
    println!("granted_tool_kinds: {}", kinds.join(","));
}

#[derive(Debug, serde::Serialize)]
struct IdentityJson {
    identity: AgentIdentity,
    capabilities: AgentCapabilityManifest,
}

/// Spec 077 MedAgent Workbench commands (T077-03: agent identity only).
#[derive(Debug, Subcommand)]
pub enum MedAgentCmd {
    /// Register a new agent identity bound to an admitted local model Pack.
    Register {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        vault_root: PathBuf,
        #[arg(long)]
        project_id: String,
        #[arg(long)]
        pack_id: String,
        #[arg(long)]
        display_name: String,
        /// Comma-separated tool kinds, e.g. `read_context_artifact,search_context_artifacts`.
        #[arg(long)]
        granted_tool_kinds: String,
        #[arg(long)]
        json: bool,
    },
    /// Show one agent identity.
    Show {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        vault_root: PathBuf,
        #[arg(long)]
        agent_id: String,
        #[arg(long)]
        json: bool,
    },
    /// List agent identities in one Project.
    List {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        vault_root: PathBuf,
        #[arg(long)]
        project_id: String,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        json: bool,
    },
    /// Revoke an agent identity.
    Revoke {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        vault_root: PathBuf,
        #[arg(long)]
        agent_id: String,
        #[arg(long)]
        expected_revision: u64,
        #[arg(long)]
        json: bool,
    },
}

pub fn run_medagent(action: MedAgentCmd) -> anyhow::Result<()> {
    match action {
        MedAgentCmd::Register {
            vault_id,
            vault_root,
            project_id,
            pack_id,
            display_name,
            granted_tool_kinds,
            json,
        } => {
            let granted_tool_kinds =
                parse_tool_kinds(&granted_tool_kinds).map_err(|m| invalid(m, json))?;
            let mut session = open_medagent_session(&vault_id, &vault_root, json)?;
            let (identity, capabilities) = session
                .medagent_identity_register(
                    OpaqueId::new(project_id),
                    OpaqueId::new(pack_id),
                    display_name,
                    granted_tool_kinds,
                )
                .map_err(|err| medagent_fail(&err, json))?;
            if json {
                print_json_or_debug(
                    &IdentityJson {
                        identity,
                        capabilities,
                    },
                    true,
                )?;
            } else {
                print_identity_human(&identity, &capabilities);
            }
            Ok(())
        }
        MedAgentCmd::Show {
            vault_id,
            vault_root,
            agent_id,
            json,
        } => {
            let mut session = open_medagent_session(&vault_id, &vault_root, json)?;
            let (identity, capabilities) = session
                .medagent_identity_get(OpaqueId::new(agent_id))
                .map_err(|err| medagent_fail(&err, json))?;
            if json {
                print_json_or_debug(
                    &IdentityJson {
                        identity,
                        capabilities,
                    },
                    true,
                )?;
            } else {
                print_identity_human(&identity, &capabilities);
            }
            Ok(())
        }
        MedAgentCmd::List {
            vault_id,
            vault_root,
            project_id,
            limit,
            json,
        } => {
            let mut session = open_medagent_session(&vault_id, &vault_root, json)?;
            let identities = session
                .medagent_identity_list(OpaqueId::new(project_id), limit)
                .map_err(|err| medagent_fail(&err, json))?;
            if json {
                print_json_or_debug(&identities, true)?;
            } else {
                for identity in &identities {
                    println!(
                        "{}\t{}\t{}\t{}",
                        identity.header.id.as_str(),
                        identity.display_name,
                        identity.status.as_str(),
                        identity.revision
                    );
                }
            }
            Ok(())
        }
        MedAgentCmd::Revoke {
            vault_id,
            vault_root,
            agent_id,
            expected_revision,
            json,
        } => {
            let mut session = open_medagent_session(&vault_id, &vault_root, json)?;
            let (identity, capabilities) = session
                .medagent_identity_revoke(OpaqueId::new(agent_id), expected_revision)
                .map_err(|err| medagent_fail(&err, json))?;
            if json {
                print_json_or_debug(
                    &IdentityJson {
                        identity,
                        capabilities,
                    },
                    true,
                )?;
            } else {
                print_identity_human(&identity, &capabilities);
            }
            Ok(())
        }
    }
}
