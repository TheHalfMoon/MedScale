//! Spec 078 Model Fleet + Compare Core authority integration tests.
//!
//! Every mutation travels request -> session -> realm/scope -> `ModelFleet`
//! authority -> Spec 077 public reads -> validation -> revision -> typed
//! result through `CoreFacade::dispatch`. Synthetic data only.
//!
//! T078-03 scope: `AgentLane` + `LanePolicy` create/get/list/retire,
//! `security.md` T2 (subset-only policy), T5 (stale revision) and T7
//! (bounded text), plus reopen durability.

use std::fs;
use std::path::PathBuf;

use medscale_contracts::envelopes::{
    AuthorityError, AuthorityRequest, Capability, RequestBody, ResponseBody,
};
use medscale_contracts::medagent::ToolKind;
use medscale_contracts::model_fleet::{AgentLane, AgentLaneStatus, ROLE_LABEL_MAX_CHARS};
use medscale_contracts::objects::{AuthorityScopeId, OpaqueId, RealmId, VaultId};
use medscale_contracts::project_graph::{ArtifactDescriptor, ArtifactKind, ArtifactVersionBinding};
use medscale_core::CoreFacade;

const REALM: &str = "realm-a";
const SCOPE: &str = "scope-a";
const VAULT: &str = "vault-1";

fn tmp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("medscale-078c-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn fixture_pack() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../evidence/008-local-ai-capability-fabric/fixtures/pack-fixture-ner-v0")
}

fn request(
    id: OpaqueId,
    scope: &str,
    capability: Capability,
    body: RequestBody,
) -> AuthorityRequest {
    AuthorityRequest::new(
        id,
        VaultId::new(VAULT),
        RealmId::new(REALM),
        AuthorityScopeId::new(scope),
        capability,
        body,
    )
}

/// Single-actor Core harness over one synthetic vault directory.
struct Harness {
    facade: CoreFacade,
    session: OpaqueId,
    dir: PathBuf,
    next_req: u64,
}

impl Harness {
    fn setup(name: &str) -> Self {
        Self::open_at(tmp_dir(name))
    }

    /// Opens a fresh `CoreFacade` (a new process, as far as Core state is
    /// concerned) over an existing vault directory.
    fn open_at(dir: PathBuf) -> Self {
        let mut h = Harness {
            facade: CoreFacade::new(),
            session: OpaqueId::new("pending"),
            dir,
            next_req: 0,
        };
        let lease = h
            .facade
            .dispatch(request(
                OpaqueId::new("req-lease"),
                SCOPE,
                Capability::AcquireLease,
                RequestBody::AcquireLease {
                    client_id: OpaqueId::new("actor-a"),
                    holder_id_hint: Some(OpaqueId::new("actor-a")),
                },
            ))
            .result
            .expect("lease");
        let ResponseBody::Lease { holder_id, .. } = lease else {
            panic!("{lease:?}");
        };
        let session = h
            .facade
            .dispatch(request(
                OpaqueId::new("req-session"),
                SCOPE,
                Capability::OpenSession,
                RequestBody::OpenSession {
                    holder_id,
                    granted: Capability::operator_grants(),
                    ttl_ticks: 1_000_000,
                },
            ))
            .result
            .expect("session");
        let ResponseBody::Session { session_id, .. } = session else {
            panic!("{session:?}");
        };
        h.session = session_id;
        let vault_root = h.dir.to_str().unwrap().to_owned();
        h.call(
            Capability::OpenSyntheticVault,
            RequestBody::OpenSyntheticVault { vault_root },
        )
        .expect("vault");
        h
    }

    fn reopen(self) -> Self {
        let dir = self.dir.clone();
        drop(self);
        Self::open_at(dir)
    }

    fn call_in(
        &mut self,
        scope: &str,
        capability: Capability,
        body: RequestBody,
    ) -> Result<ResponseBody, AuthorityError> {
        self.next_req += 1;
        let mut req = request(
            OpaqueId::new(format!("req-{}", self.next_req)),
            scope,
            capability,
            body,
        );
        req.session_id = Some(self.session.clone());
        self.facade.dispatch(req).result
    }

    fn call(
        &mut self,
        capability: Capability,
        body: RequestBody,
    ) -> Result<ResponseBody, AuthorityError> {
        self.call_in(SCOPE, capability, body)
    }

    fn project(&mut self, name: &str) -> OpaqueId {
        match self
            .call(
                Capability::ProjectCreate,
                RequestBody::ProjectCreate {
                    name: name.to_owned(),
                    description: None,
                },
            )
            .expect("project")
        {
            ResponseBody::Project { project } => project.header.id,
            other => panic!("{other:?}"),
        }
    }

    fn install_fixture_pack(&mut self) -> OpaqueId {
        let local_path = fixture_pack().display().to_string();
        match self
            .call(
                Capability::PacksInstallLocal,
                RequestBody::PacksInstallLocal { local_path },
            )
            .expect("pack install")
        {
            ResponseBody::PackAdmit { result } => {
                assert!(result.admitted, "fixture pack must admit cleanly");
                result.pack_id.expect("admitted pack carries a pack_id")
            }
            other => panic!("{other:?}"),
        }
    }

    fn register_identity(
        &mut self,
        project_id: &OpaqueId,
        pack_id: &OpaqueId,
        granted_tool_kinds: Vec<ToolKind>,
    ) -> OpaqueId {
        match self
            .call(
                Capability::AgentIdentityRegister,
                RequestBody::AgentIdentityRegister {
                    project_id: project_id.clone(),
                    pack_id: pack_id.clone(),
                    display_name: "Lane Agent".to_owned(),
                    granted_tool_kinds,
                },
            )
            .expect("register identity")
        {
            ResponseBody::MedAgentIdentity { identity, .. } => identity.header.id,
            other => panic!("{other:?}"),
        }
    }

    fn create_context(&mut self, project_id: &OpaqueId, artifact_ids: &[&str]) -> OpaqueId {
        let selected_artifacts = artifact_ids
            .iter()
            .map(|id| ArtifactDescriptor {
                object_id: OpaqueId::new(*id),
                kind: ArtifactKind::SourceRecord,
                binding: ArtifactVersionBinding::IdentityOnly,
            })
            .collect();
        match self
            .call(
                Capability::ContextManifestCreate,
                RequestBody::ContextManifestCreate {
                    project_id: project_id.clone(),
                    selected_artifacts,
                },
            )
            .expect("create context")
        {
            ResponseBody::MedAgentContextManifest { manifest, .. } => manifest.header.id,
            other => panic!("{other:?}"),
        }
    }

    fn create_lane(
        &mut self,
        project_id: &OpaqueId,
        agent_id: &OpaqueId,
        context_id: &OpaqueId,
        granted_tool_kinds: Option<Vec<ToolKind>>,
        context_artifact_ids: Option<Vec<OpaqueId>>,
    ) -> Result<AgentLane, AuthorityError> {
        self.call(
            Capability::AgentLaneCreate,
            RequestBody::AgentLaneCreate {
                project_id: project_id.clone(),
                agent_identity_id: agent_id.clone(),
                context_manifest_id: context_id.clone(),
                role_label: "reviewer".to_owned(),
                granted_tool_kinds,
                context_artifact_ids,
            },
        )
        .map(|resp| match resp {
            ResponseBody::ModelFleetLane { lane } => *lane,
            other => panic!("{other:?}"),
        })
    }

    fn list_lanes(
        &mut self,
        project_id: &OpaqueId,
        status: Option<AgentLaneStatus>,
    ) -> Vec<AgentLane> {
        match self
            .call(
                Capability::AgentLaneRead,
                RequestBody::AgentLaneList {
                    project_id: project_id.clone(),
                    status,
                    limit: None,
                },
            )
            .expect("list lanes")
        {
            ResponseBody::ModelFleetLaneList { lanes } => lanes,
            other => panic!("{other:?}"),
        }
    }

    fn get_lane(&mut self, lane_id: &OpaqueId) -> Result<AgentLane, AuthorityError> {
        self.call(
            Capability::AgentLaneRead,
            RequestBody::AgentLaneGet {
                lane_id: lane_id.clone(),
            },
        )
        .map(|resp| match resp {
            ResponseBody::ModelFleetLane { lane } => *lane,
            other => panic!("{other:?}"),
        })
    }

    fn retire_lane(
        &mut self,
        lane_id: &OpaqueId,
        expected_revision: u64,
    ) -> Result<AgentLane, AuthorityError> {
        self.call(
            Capability::AgentLaneRetire,
            RequestBody::AgentLaneRetire {
                lane_id: lane_id.clone(),
                expected_revision,
            },
        )
        .map(|resp| match resp {
            ResponseBody::ModelFleetLane { lane } => *lane,
            other => panic!("{other:?}"),
        })
    }
}

/// A Project with one identity (granting read + search) and one context
/// manifest selecting two artifacts.
struct LaneFixture {
    project_id: OpaqueId,
    pack_id: OpaqueId,
    agent_id: OpaqueId,
    context_id: OpaqueId,
}

fn lane_fixture(h: &mut Harness) -> LaneFixture {
    let project_id = h.project("fleet-project");
    let pack_id = h.install_fixture_pack();
    let agent_id = h.register_identity(
        &project_id,
        &pack_id,
        vec![
            ToolKind::ReadContextArtifact,
            ToolKind::SearchContextArtifacts,
        ],
    );
    let context_id = h.create_context(&project_id, &["artifact-1", "artifact-2"]);
    LaneFixture {
        project_id,
        pack_id,
        agent_id,
        context_id,
    }
}

fn assert_invalid(result: Result<AgentLane, AuthorityError>, why: &str) {
    match result {
        Err(AuthorityError::InvalidArgument { .. }) => {}
        other => panic!("{why}: expected InvalidArgument, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// T078-03: lane vertical slice + reopen durability
// ---------------------------------------------------------------------------

#[test]
fn lane_create_get_list_retire_roundtrip_and_survives_reopen() {
    let mut h = Harness::setup("lane-lifecycle");
    let f = lane_fixture(&mut h);

    let narrowed = h
        .create_lane(
            &f.project_id,
            &f.agent_id,
            &f.context_id,
            Some(vec![ToolKind::ReadContextArtifact]),
            Some(vec![OpaqueId::new("artifact-1")]),
        )
        .expect("narrowed lane");
    assert_eq!(narrowed.status, AgentLaneStatus::Active);
    assert_eq!(narrowed.revision, 1);
    assert_eq!(narrowed.agent_identity_id, f.agent_id);
    assert_eq!(narrowed.context_manifest_id, f.context_id);
    assert_eq!(narrowed.header.realm_id.as_opaque().as_str(), REALM);

    // `None` inherits the full underlying grant.
    let inherited = h
        .create_lane(&f.project_id, &f.agent_id, &f.context_id, None, None)
        .expect("inheriting lane");
    assert_ne!(inherited.header.id, narrowed.header.id);

    assert_eq!(h.get_lane(&narrowed.header.id).unwrap(), narrowed);
    assert_eq!(h.list_lanes(&f.project_id, None).len(), 2);

    let retired = h.retire_lane(&narrowed.header.id, 1).expect("retire");
    assert_eq!(retired.status, AgentLaneStatus::Retired);
    assert_eq!(retired.revision, 2);
    assert_eq!(
        retired.policy, narrowed.policy,
        "retire never rewrites policy"
    );

    let mut h = h.reopen();
    assert_eq!(h.get_lane(&narrowed.header.id).unwrap(), retired);
    assert_eq!(h.get_lane(&inherited.header.id).unwrap(), inherited);
    let active = h.list_lanes(&f.project_id, Some(AgentLaneStatus::Active));
    assert_eq!(active, vec![inherited.clone()]);
    let retired_list = h.list_lanes(&f.project_id, Some(AgentLaneStatus::Retired));
    assert_eq!(retired_list, vec![retired]);

    // Ids keep allocating from the durable sequence after reopen.
    let third = h
        .create_lane(&f.project_id, &f.agent_id, &f.context_id, None, None)
        .expect("lane after reopen");
    assert_ne!(third.header.id, narrowed.header.id);
    assert_ne!(third.header.id, inherited.header.id);
}

// ---------------------------------------------------------------------------
// security.md T2: policy narrows, never widens
// ---------------------------------------------------------------------------

#[test]
fn lane_policy_naming_an_ungranted_tool_kind_is_refused_before_any_write() {
    let mut h = Harness::setup("lane-t2-tool");
    let project_id = h.project("fleet-project");
    let pack_id = h.install_fixture_pack();
    let agent_id = h.register_identity(&project_id, &pack_id, vec![ToolKind::ReadContextArtifact]);
    let context_id = h.create_context(&project_id, &["artifact-1"]);

    assert_invalid(
        h.create_lane(
            &project_id,
            &agent_id,
            &context_id,
            Some(vec![ToolKind::SearchContextArtifacts]),
            None,
        ),
        "a tool kind the identity does not grant",
    );
    assert_invalid(
        h.create_lane(
            &project_id,
            &agent_id,
            &context_id,
            Some(vec![
                ToolKind::ReadContextArtifact,
                ToolKind::SearchContextArtifacts,
            ]),
            None,
        ),
        "a superset of the identity's grant",
    );
    assert!(h.list_lanes(&project_id, None).is_empty());
}

#[test]
fn lane_policy_naming_an_out_of_manifest_artifact_is_refused_before_any_write() {
    let mut h = Harness::setup("lane-t2-context");
    let f = lane_fixture(&mut h);
    assert_invalid(
        h.create_lane(
            &f.project_id,
            &f.agent_id,
            &f.context_id,
            None,
            Some(vec![OpaqueId::new("artifact-outside")]),
        ),
        "an artifact outside the bound context manifest",
    );
    assert_invalid(
        h.create_lane(
            &f.project_id,
            &f.agent_id,
            &f.context_id,
            None,
            Some(vec![
                OpaqueId::new("artifact-1"),
                OpaqueId::new("artifact-outside"),
            ]),
        ),
        "a partial overlap with the bound context manifest",
    );
    assert_invalid(
        h.create_lane(
            &f.project_id,
            &f.agent_id,
            &f.context_id,
            Some(vec![]),
            None,
        ),
        "an empty tool-kind subset",
    );
    assert_invalid(
        h.create_lane(
            &f.project_id,
            &f.agent_id,
            &f.context_id,
            None,
            Some(vec![]),
        ),
        "an empty artifact subset",
    );
    assert!(h.list_lanes(&f.project_id, None).is_empty());
}

#[test]
fn lane_binding_to_revoked_missing_or_foreign_objects_is_refused() {
    let mut h = Harness::setup("lane-binding");
    let f = lane_fixture(&mut h);
    let other_project = h.project("other-project");
    let foreign_agent = h.register_identity(
        &other_project,
        &f.pack_id,
        vec![ToolKind::ReadContextArtifact],
    );
    let foreign_context = h.create_context(&other_project, &["artifact-9"]);

    assert_invalid(
        h.create_lane(&f.project_id, &foreign_agent, &f.context_id, None, None),
        "an identity from another Project",
    );
    assert_invalid(
        h.create_lane(&f.project_id, &f.agent_id, &foreign_context, None, None),
        "a context manifest from another Project",
    );
    assert!(matches!(
        h.create_lane(
            &f.project_id,
            &OpaqueId::new("agent-missing"),
            &f.context_id,
            None,
            None
        ),
        Err(AuthorityError::NotFound)
    ));
    assert!(matches!(
        h.create_lane(
            &f.project_id,
            &f.agent_id,
            &OpaqueId::new("ctx-missing"),
            None,
            None
        ),
        Err(AuthorityError::NotFound)
    ));

    h.call(
        Capability::AgentIdentityRevoke,
        RequestBody::AgentIdentityRevoke {
            agent_id: f.agent_id.clone(),
            expected_revision: 1,
        },
    )
    .expect("revoke identity");
    assert_invalid(
        h.create_lane(&f.project_id, &f.agent_id, &f.context_id, None, None),
        "a revoked identity",
    );
    assert!(h.list_lanes(&f.project_id, None).is_empty());
}

// ---------------------------------------------------------------------------
// security.md T5 / T7 and scope
// ---------------------------------------------------------------------------

#[test]
fn lane_retire_is_stale_revision_safe_and_not_repeatable() {
    let mut h = Harness::setup("lane-cas");
    let f = lane_fixture(&mut h);
    let lane = h
        .create_lane(&f.project_id, &f.agent_id, &f.context_id, None, None)
        .unwrap();
    assert!(matches!(
        h.retire_lane(&lane.header.id, 7),
        Err(AuthorityError::Conflict { .. })
    ));
    h.retire_lane(&lane.header.id, 1).unwrap();
    assert!(matches!(
        h.retire_lane(&lane.header.id, 1),
        Err(AuthorityError::Conflict { .. })
    ));
    assert!(matches!(
        h.retire_lane(&lane.header.id, 2),
        Err(AuthorityError::Conflict { .. })
    ));
    assert!(matches!(
        h.retire_lane(&OpaqueId::new("lane-missing"), 1),
        Err(AuthorityError::NotFound)
    ));
}

#[test]
fn lane_role_label_is_bounded_and_stored_as_inert_text() {
    let mut h = Harness::setup("lane-t7");
    let f = lane_fixture(&mut h);
    for role_label in [
        String::new(),
        "   ".to_owned(),
        "x".repeat(ROLE_LABEL_MAX_CHARS + 1),
    ] {
        let result = h
            .call(
                Capability::AgentLaneCreate,
                RequestBody::AgentLaneCreate {
                    project_id: f.project_id.clone(),
                    agent_identity_id: f.agent_id.clone(),
                    context_manifest_id: f.context_id.clone(),
                    role_label: role_label.clone(),
                    granted_tool_kinds: None,
                    context_artifact_ids: None,
                },
            )
            .map(|_| ());
        assert!(
            matches!(result, Err(AuthorityError::InvalidArgument { .. })),
            "role label of {} chars must be refused, got {result:?}",
            role_label.chars().count()
        );
    }
    let hostile = "'); DROP TABLE model_fleet_lanes; -- <script>\u{202e}";
    let lane = match h
        .call(
            Capability::AgentLaneCreate,
            RequestBody::AgentLaneCreate {
                project_id: f.project_id.clone(),
                agent_identity_id: f.agent_id.clone(),
                context_manifest_id: f.context_id.clone(),
                role_label: hostile.to_owned(),
                granted_tool_kinds: None,
                context_artifact_ids: None,
            },
        )
        .expect("hostile-but-bounded label is stored verbatim")
    {
        ResponseBody::ModelFleetLane { lane } => *lane,
        other => panic!("{other:?}"),
    };
    assert_eq!(h.get_lane(&lane.header.id).unwrap().role_label, hostile);
    assert_eq!(h.list_lanes(&f.project_id, None).len(), 1);
}

#[test]
fn lane_reads_are_scope_checked() {
    let mut h = Harness::setup("lane-scope");
    let f = lane_fixture(&mut h);
    let lane = h
        .create_lane(&f.project_id, &f.agent_id, &f.context_id, None, None)
        .unwrap();
    let foreign = h.call_in(
        "scope-b",
        Capability::AgentLaneRead,
        RequestBody::AgentLaneGet {
            lane_id: lane.header.id.clone(),
        },
    );
    assert!(
        foreign.is_err(),
        "a lane must not be readable from another scope: {foreign:?}"
    );
    let foreign_list = h.call_in(
        "scope-b",
        Capability::AgentLaneRead,
        RequestBody::AgentLaneList {
            project_id: f.project_id.clone(),
            status: None,
            limit: None,
        },
    );
    assert!(
        foreign_list.is_err(),
        "a Project's lanes must not be listable from another scope: {foreign_list:?}"
    );
}

#[test]
fn lane_capabilities_are_distinct_from_spec_077_capabilities() {
    let mut h = Harness::setup("lane-capability");
    let f = lane_fixture(&mut h);
    // A request whose declared capability does not match its body is
    // refused by the facade's capability matrix before any authority code.
    let result = h.call(
        Capability::AgentRunCreate,
        RequestBody::AgentLaneCreate {
            project_id: f.project_id.clone(),
            agent_identity_id: f.agent_id.clone(),
            context_manifest_id: f.context_id.clone(),
            role_label: "reviewer".to_owned(),
            granted_tool_kinds: None,
            context_artifact_ids: None,
        },
    );
    assert!(
        result.is_err(),
        "capability/body mismatch must be refused: {result:?}"
    );
    assert!(h.list_lanes(&f.project_id, None).is_empty());
}
