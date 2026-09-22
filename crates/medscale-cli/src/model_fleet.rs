//! Spec 078 Model Fleet + Compare commands (CLI vertical slice through Core).
//!
//! T078-03 scope: `AgentLane` create/show/list/retire. Every command opens
//! the session scope, dispatches one typed Core request via `CliSession`,
//! and renders the typed result as human lines or stable JSON. The CLI never
//! reads or writes model_fleet storage directly.

use std::path::PathBuf;

use clap::Subcommand;
use medscale_contracts::envelopes::AuthorityError;
use medscale_contracts::medagent::ToolKind;
use medscale_contracts::model_fleet::{AgentLane, AgentLaneStatus};
use medscale_contracts::objects::OpaqueId;
use medscale_core::CliSession;

use super::{fail_json, print_json_or_debug};

fn fleet_fail(err: &AuthorityError, json: bool) -> anyhow::Error {
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

fn open_fleet_session(
    vault_id: &str,
    vault_root: &std::path::Path,
    json: bool,
) -> anyhow::Result<CliSession> {
    let mut session = CliSession::connect(vault_id).map_err(|err| fleet_fail(&err, json))?;
    session
        .open_synthetic_vault(&vault_root.display().to_string())
        .map_err(|err| fleet_fail(&err, json))?;
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

fn parse_ids(value: &str) -> Vec<OpaqueId> {
    value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(OpaqueId::new)
        .collect()
}

fn print_lane_human(lane: &AgentLane) {
    println!("lane_id: {}", lane.header.id.as_str());
    println!("project_id: {}", lane.project_id.as_str());
    println!("agent_identity_id: {}", lane.agent_identity_id.as_str());
    println!("context_manifest_id: {}", lane.context_manifest_id.as_str());
    println!("role_label: {}", lane.role_label.escape_debug());
    println!("status: {}", lane.status.as_str());
    println!("revision: {}", lane.revision);
    match &lane.policy.granted_tool_kinds {
        Some(kinds) => {
            let kinds: Vec<&str> = kinds.iter().map(|k| k.as_str()).collect();
            println!("policy_tool_kinds: {}", kinds.join(","));
        }
        None => println!("policy_tool_kinds: (inherit identity grant)"),
    }
    match &lane.policy.context_artifact_ids {
        Some(ids) => {
            let ids: Vec<&str> = ids.iter().map(OpaqueId::as_str).collect();
            println!("policy_context_artifacts: {}", ids.join(","));
        }
        None => println!("policy_context_artifacts: (inherit context manifest)"),
    }
}

/// Spec 078 Model Fleet + Compare commands.
#[derive(Debug, Subcommand)]
pub enum ModelFleetCmd {
    /// Create an agent lane over an existing agent identity + context manifest.
    LaneCreate {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        vault_root: PathBuf,
        #[arg(long)]
        project_id: String,
        #[arg(long)]
        agent_identity_id: String,
        #[arg(long)]
        context_manifest_id: String,
        #[arg(long)]
        role_label: String,
        /// Comma-separated subset of the identity's granted tool kinds;
        /// omit to inherit the full grant.
        #[arg(long)]
        tool_kinds: Option<String>,
        /// Comma-separated subset of the context manifest's artifact ids;
        /// omit to inherit the full manifest.
        #[arg(long)]
        context_artifacts: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// Show one agent lane.
    LaneShow {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        vault_root: PathBuf,
        #[arg(long)]
        lane_id: String,
        #[arg(long)]
        json: bool,
    },
    /// List agent lanes in one Project.
    LaneList {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        vault_root: PathBuf,
        #[arg(long)]
        project_id: String,
        /// `active` or `retired`.
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        json: bool,
    },
    /// Retire an agent lane (status tombstone; history is kept).
    LaneRetire {
        #[arg(long)]
        vault_id: String,
        #[arg(long)]
        vault_root: PathBuf,
        #[arg(long)]
        lane_id: String,
        #[arg(long)]
        expected_revision: u64,
        #[arg(long)]
        json: bool,
    },
}

fn print_lane(lane: &AgentLane, json: bool) -> anyhow::Result<()> {
    if json {
        print_json_or_debug(lane, true)?;
    } else {
        print_lane_human(lane);
    }
    Ok(())
}

pub fn run_model_fleet(action: ModelFleetCmd) -> anyhow::Result<()> {
    match action {
        ModelFleetCmd::LaneCreate {
            vault_id,
            vault_root,
            project_id,
            agent_identity_id,
            context_manifest_id,
            role_label,
            tool_kinds,
            context_artifacts,
            json,
        } => {
            let granted_tool_kinds = tool_kinds
                .as_deref()
                .map(parse_tool_kinds)
                .transpose()
                .map_err(|m| invalid(m, json))?;
            let context_artifact_ids = context_artifacts.as_deref().map(parse_ids);
            let mut session = open_fleet_session(&vault_id, &vault_root, json)?;
            let lane = session
                .model_fleet_lane_create(
                    OpaqueId::new(project_id),
                    OpaqueId::new(agent_identity_id),
                    OpaqueId::new(context_manifest_id),
                    role_label,
                    granted_tool_kinds,
                    context_artifact_ids,
                )
                .map_err(|err| fleet_fail(&err, json))?;
            print_lane(&lane, json)
        }
        ModelFleetCmd::LaneShow {
            vault_id,
            vault_root,
            lane_id,
            json,
        } => {
            let mut session = open_fleet_session(&vault_id, &vault_root, json)?;
            let lane = session
                .model_fleet_lane_get(OpaqueId::new(lane_id))
                .map_err(|err| fleet_fail(&err, json))?;
            print_lane(&lane, json)
        }
        ModelFleetCmd::LaneList {
            vault_id,
            vault_root,
            project_id,
            status,
            limit,
            json,
        } => {
            let status = status
                .as_deref()
                .map(AgentLaneStatus::parse)
                .transpose()
                .map_err(|m| invalid(m, json))?;
            let mut session = open_fleet_session(&vault_id, &vault_root, json)?;
            let lanes = session
                .model_fleet_lane_list(OpaqueId::new(project_id), status, limit)
                .map_err(|err| fleet_fail(&err, json))?;
            if json {
                print_json_or_debug(&lanes, true)?;
            } else {
                for lane in &lanes {
                    println!(
                        "{}\t{}\t{}\t{}",
                        lane.header.id.as_str(),
                        lane.role_label.escape_debug(),
                        lane.status.as_str(),
                        lane.revision
                    );
                }
            }
            Ok(())
        }
        ModelFleetCmd::LaneRetire {
            vault_id,
            vault_root,
            lane_id,
            expected_revision,
            json,
        } => {
            let mut session = open_fleet_session(&vault_id, &vault_root, json)?;
            let lane = session
                .model_fleet_lane_retire(OpaqueId::new(lane_id), expected_revision)
                .map_err(|err| fleet_fail(&err, json))?;
            print_lane(&lane, json)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use medscale_contracts::project_graph::{
        ArtifactDescriptor, ArtifactKind, ArtifactVersionBinding,
    };

    const VAULT: &str = "cli-fleet";

    /// Seeds a Project, an identity and a context manifest through Core in
    /// one session, then drops it: every lane command below runs in its own
    /// fresh CLI session over the persisted vault, exactly as the binary does.
    fn seed(root: &std::path::Path) -> (OpaqueId, OpaqueId, OpaqueId) {
        let mut session = CliSession::connect(VAULT).unwrap();
        session
            .open_synthetic_vault(&root.display().to_string())
            .unwrap();
        let project = session
            .project_create("cli fleet".to_owned(), None)
            .unwrap();
        let pack_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../evidence/008-local-ai-capability-fabric/fixtures/pack-fixture-ner-v0");
        let admitted = session
            .packs_install_local(&pack_dir.display().to_string())
            .unwrap();
        let (identity, _) = session
            .medagent_identity_register(
                project.header.id.clone(),
                admitted.pack_id.unwrap(),
                "cli lane agent".to_owned(),
                vec![ToolKind::ReadContextArtifact],
            )
            .unwrap();
        let (context, _) = session
            .medagent_context_create(
                project.header.id.clone(),
                vec![ArtifactDescriptor {
                    object_id: OpaqueId::new("artifact-1"),
                    kind: ArtifactKind::SourceRecord,
                    binding: ArtifactVersionBinding::IdentityOnly,
                }],
            )
            .unwrap();
        (project.header.id, identity.header.id, context.header.id)
    }

    #[test]
    fn lane_commands_run_through_core_across_fresh_sessions() {
        let root = std::env::temp_dir().join(format!("medscale-078-cli-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let (project_id, agent_id, context_id) = seed(&root);

        let create = |tool_kinds: Option<&str>, json: bool| {
            run_model_fleet(ModelFleetCmd::LaneCreate {
                vault_id: VAULT.to_owned(),
                vault_root: root.clone(),
                project_id: project_id.as_str().to_owned(),
                agent_identity_id: agent_id.as_str().to_owned(),
                context_manifest_id: context_id.as_str().to_owned(),
                role_label: "reviewer".to_owned(),
                tool_kinds: tool_kinds.map(str::to_owned),
                context_artifacts: Some("artifact-1".to_owned()),
                json,
            })
        };
        create(Some("read_context_artifact"), true).expect("json lane create");
        create(None, false).expect("human lane create");
        // T2 refusal surfaces as a CLI error, not a created lane.
        assert!(create(Some("search_context_artifacts"), true).is_err());
        assert!(create(Some("not_a_tool_kind"), true).is_err());

        let mut session = CliSession::connect(VAULT).unwrap();
        session
            .open_synthetic_vault(&root.display().to_string())
            .unwrap();
        let lanes = session
            .model_fleet_lane_list(project_id.clone(), None, None)
            .unwrap();
        assert_eq!(lanes.len(), 2);
        let lane_id = lanes[0].header.id.as_str().to_owned();
        drop(session);

        for json in [true, false] {
            run_model_fleet(ModelFleetCmd::LaneShow {
                vault_id: VAULT.to_owned(),
                vault_root: root.clone(),
                lane_id: lane_id.clone(),
                json,
            })
            .expect("lane show");
            run_model_fleet(ModelFleetCmd::LaneList {
                vault_id: VAULT.to_owned(),
                vault_root: root.clone(),
                project_id: project_id.as_str().to_owned(),
                status: Some("active".to_owned()),
                limit: None,
                json,
            })
            .expect("lane list");
        }
        assert!(
            run_model_fleet(ModelFleetCmd::LaneList {
                vault_id: VAULT.to_owned(),
                vault_root: root.clone(),
                project_id: project_id.as_str().to_owned(),
                status: Some("winner".to_owned()),
                limit: None,
                json: true,
            })
            .is_err(),
            "an unknown status is refused, never coerced"
        );
        run_model_fleet(ModelFleetCmd::LaneRetire {
            vault_id: VAULT.to_owned(),
            vault_root: root.clone(),
            lane_id: lane_id.clone(),
            expected_revision: 1,
            json: true,
        })
        .expect("lane retire");
        assert!(
            run_model_fleet(ModelFleetCmd::LaneRetire {
                vault_id: VAULT.to_owned(),
                vault_root: root.clone(),
                lane_id: lane_id.clone(),
                expected_revision: 1,
                json: true,
            })
            .is_err(),
            "stale retire is a conflict"
        );

        let mut session = CliSession::connect(VAULT).unwrap();
        session
            .open_synthetic_vault(&root.display().to_string())
            .unwrap();
        let retired = session
            .model_fleet_lane_get(OpaqueId::new(lane_id))
            .unwrap();
        assert_eq!(retired.status, AgentLaneStatus::Retired);
        assert_eq!(retired.revision, 2);
    }
}
