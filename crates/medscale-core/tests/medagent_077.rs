//! Spec 077 MedAgent Workbench Core authority integration tests (T077-03).
//!
//! Every mutation travels request -> session -> realm/scope -> `MedAgent`
//! authority -> validation -> revision -> transaction -> typed result
//! through `CoreFacade::dispatch`. Synthetic data only. T077-03 scope:
//! `AgentIdentity` + `AgentCapabilityManifest` register/get/list/revoke.

use std::fs;
use std::path::PathBuf;

use medscale_contracts::envelopes::{
    AuthorityError, AuthorityRequest, Capability, RequestBody, ResponseBody,
};
use medscale_contracts::medagent::ToolKind;
use medscale_contracts::objects::{AuthorityScopeId, OpaqueId, RealmId, VaultId};
use medscale_core::CoreFacade;

fn tmp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("medscale-077c-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn fixture_pack() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../evidence/008-local-ai-capability-fabric/fixtures/pack-fixture-ner-v0")
}

const REALM: &str = "realm-a";
const SCOPE: &str = "scope-a";
const VAULT: &str = "vault-1";

/// Single-actor Core harness (mirrors `collaboration_076.rs`'s `Harness`,
/// trimmed to what T077-03 needs: no multi-actor switching required since
/// agent identity registration/revocation is scope-level, not
/// membership-gated).
struct Harness {
    facade: CoreFacade,
    session: OpaqueId,
    dir: std::path::PathBuf,
    next_req: u64,
}

fn base_req(capability: Capability, body: RequestBody) -> AuthorityRequest {
    AuthorityRequest::new(
        OpaqueId::new("req"),
        VaultId::new(VAULT),
        RealmId::new(REALM),
        AuthorityScopeId::new(SCOPE),
        capability,
        body,
    )
}

impl Harness {
    fn setup(name: &str) -> Self {
        let dir = tmp_dir(name);
        let facade = CoreFacade::new();
        let mut h = Harness {
            facade,
            session: OpaqueId::new("pending"),
            dir,
            next_req: 0,
        };
        let mut vault_req = base_req(
            Capability::OpenSyntheticVault,
            RequestBody::OpenSyntheticVault {
                vault_root: h.dir.to_str().unwrap().to_owned(),
            },
        );
        // Bootstrap a lease + session before the vault open so later calls
        // carry a valid session_id (mirrors collaboration_076.rs).
        let lease = h
            .facade
            .dispatch(base_req(
                Capability::AcquireLease,
                RequestBody::AcquireLease {
                    client_id: OpaqueId::new("actor-a"),
                    holder_id_hint: Some(OpaqueId::new("actor-a")),
                },
            ))
            .result
            .expect("lease");
        let holder_id = match lease {
            ResponseBody::Lease { holder_id, .. } => holder_id,
            other => panic!("{other:?}"),
        };
        let session = h
            .facade
            .dispatch(base_req(
                Capability::OpenSession,
                RequestBody::OpenSession {
                    holder_id,
                    granted: Capability::operator_grants(),
                    ttl_ticks: 1_000_000,
                },
            ))
            .result
            .expect("session");
        h.session = match session {
            ResponseBody::Session { session_id, .. } => session_id,
            other => panic!("{other:?}"),
        };
        vault_req.session_id = Some(h.session.clone());
        h.facade.dispatch(vault_req).result.expect("vault");
        h
    }

    fn req_id(&mut self) -> OpaqueId {
        self.next_req += 1;
        OpaqueId::new(format!("req-{}", self.next_req))
    }

    fn call(
        &mut self,
        capability: Capability,
        body: RequestBody,
    ) -> Result<ResponseBody, AuthorityError> {
        let id = self.req_id();
        let mut request = AuthorityRequest::new(
            id,
            VaultId::new(VAULT),
            RealmId::new(REALM),
            AuthorityScopeId::new(SCOPE),
            capability,
            body,
        );
        request.session_id = Some(self.session.clone());
        self.facade.dispatch(request).result
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

    /// Installs the Spec 008 fixture pack and returns its admitted
    /// `pack_id`.
    fn install_fixture_pack(&mut self) -> OpaqueId {
        let path = fixture_pack().display().to_string();
        match self
            .call(
                Capability::PacksInstallLocal,
                RequestBody::PacksInstallLocal { local_path: path },
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
}

#[test]
fn register_get_list_revoke_roundtrip_through_core() {
    let mut h = Harness::setup("lifecycle");
    let project_id = h.project("agent-project");
    let pack_id = h.install_fixture_pack();

    let resp = h
        .call(
            Capability::AgentIdentityRegister,
            RequestBody::AgentIdentityRegister {
                project_id: project_id.clone(),
                pack_id: pack_id.clone(),
                display_name: "Research Assistant".to_owned(),
                granted_tool_kinds: vec![ToolKind::ReadContextArtifact],
            },
        )
        .expect("register");
    let (agent_id, pack_version) = match resp {
        ResponseBody::MedAgentIdentity {
            identity,
            capabilities,
        } => {
            assert_eq!(identity.project_id, project_id);
            assert_eq!(identity.pack_id, pack_id);
            assert!(!identity.pack_version.is_empty());
            assert!(capabilities.grants(ToolKind::ReadContextArtifact));
            assert!(!capabilities.grants(ToolKind::SearchContextArtifacts));
            (identity.header.id, identity.pack_version)
        }
        other => panic!("{other:?}"),
    };

    let resp = h
        .call(
            Capability::AgentIdentityRead,
            RequestBody::AgentIdentityGet {
                agent_id: agent_id.clone(),
            },
        )
        .expect("get");
    match resp {
        ResponseBody::MedAgentIdentity { identity, .. } => {
            assert_eq!(identity.header.id, agent_id);
            assert_eq!(identity.pack_version, pack_version);
            assert_eq!(identity.revision, 1);
        }
        other => panic!("{other:?}"),
    }

    let resp = h
        .call(
            Capability::AgentIdentityRead,
            RequestBody::AgentIdentityList {
                project_id: project_id.clone(),
                limit: None,
            },
        )
        .expect("list");
    match resp {
        ResponseBody::MedAgentIdentityList { identities } => {
            assert_eq!(identities.len(), 1);
            assert_eq!(identities[0].header.id, agent_id);
        }
        other => panic!("{other:?}"),
    }

    let resp = h
        .call(
            Capability::AgentIdentityRevoke,
            RequestBody::AgentIdentityRevoke {
                agent_id: agent_id.clone(),
                expected_revision: 1,
            },
        )
        .expect("revoke");
    match resp {
        ResponseBody::MedAgentIdentity { identity, .. } => {
            assert_eq!(identity.status.as_str(), "revoked");
            assert_eq!(identity.revision, 2);
        }
        other => panic!("{other:?}"),
    }

    // Revoking again with the now-stale expected_revision fails closed.
    let err = h
        .call(
            Capability::AgentIdentityRevoke,
            RequestBody::AgentIdentityRevoke {
                agent_id,
                expected_revision: 1,
            },
        )
        .unwrap_err();
    assert!(matches!(err, AuthorityError::Conflict { .. }));
}

/// `security.md` T5: registration against a `pack_id` the local
/// `PackStore` does not recognize must fail closed, never silently create
/// an identity with a fabricated `pack_version`.
#[test]
fn registration_against_non_admitted_pack_fails_closed() {
    let mut h = Harness::setup("non-admitted-pack");
    let project_id = h.project("agent-project");

    let err = h
        .call(
            Capability::AgentIdentityRegister,
            RequestBody::AgentIdentityRegister {
                project_id,
                pack_id: OpaqueId::new("never-admitted-pack"),
                display_name: "Rogue Agent".to_owned(),
                granted_tool_kinds: vec![ToolKind::ReadContextArtifact],
            },
        )
        .unwrap_err();
    assert!(
        matches!(err, AuthorityError::InvalidArgument { .. }),
        "expected InvalidArgument for a non-admitted pack, got {err:?}"
    );
}

/// Capability manifest is immutable once set: there is no update path, only
/// revoke-and-re-register. Re-registering the same holder concept (a fresh
/// `AgentIdentity`) is a distinct object, never an in-place capability
/// widening of the first.
#[test]
fn capability_manifest_is_immutable_and_registration_is_scoped_to_its_project() {
    let mut h = Harness::setup("scoped");
    let project_id = h.project("agent-project");
    let other_project_id = h.project("other-project");
    let pack_id = h.install_fixture_pack();

    let resp = h
        .call(
            Capability::AgentIdentityRegister,
            RequestBody::AgentIdentityRegister {
                project_id: project_id.clone(),
                pack_id: pack_id.clone(),
                display_name: "Agent One".to_owned(),
                granted_tool_kinds: vec![ToolKind::ReadContextArtifact],
            },
        )
        .expect("register");
    let agent_id = match resp {
        ResponseBody::MedAgentIdentity { identity, .. } => identity.header.id,
        other => panic!("{other:?}"),
    };

    // A second identity registered against a different Project, even with
    // the same pack, is a distinct object with its own id and manifest.
    let resp = h
        .call(
            Capability::AgentIdentityRegister,
            RequestBody::AgentIdentityRegister {
                project_id: other_project_id,
                pack_id,
                display_name: "Agent Two".to_owned(),
                granted_tool_kinds: vec![
                    ToolKind::ReadContextArtifact,
                    ToolKind::SearchContextArtifacts,
                ],
            },
        )
        .expect("register second");
    let other_agent_id = match resp {
        ResponseBody::MedAgentIdentity {
            identity,
            capabilities,
        } => {
            assert!(capabilities.grants(ToolKind::SearchContextArtifacts));
            identity.header.id
        }
        other => panic!("{other:?}"),
    };
    assert_ne!(agent_id, other_agent_id);

    // The first identity's own capability manifest is unaffected by the
    // second registration.
    let resp = h
        .call(
            Capability::AgentIdentityRead,
            RequestBody::AgentIdentityGet { agent_id },
        )
        .expect("get first");
    match resp {
        ResponseBody::MedAgentIdentity { capabilities, .. } => {
            assert!(!capabilities.grants(ToolKind::SearchContextArtifacts));
        }
        other => panic!("{other:?}"),
    }
}

/// Reopen durability: an agent identity registered before closing the
/// synthetic vault must read back identically after a fresh `CoreFacade`
/// reopens it (matches `collaboration_076.rs`'s own reopen precedent).
#[test]
fn agent_identity_survives_vault_reopen() {
    let mut h = Harness::setup("reopen");
    let project_id = h.project("agent-project");
    let pack_id = h.install_fixture_pack();
    let resp = h
        .call(
            Capability::AgentIdentityRegister,
            RequestBody::AgentIdentityRegister {
                project_id,
                pack_id,
                display_name: "Durable Agent".to_owned(),
                granted_tool_kinds: vec![ToolKind::ReadContextArtifact],
            },
        )
        .expect("register");
    let (agent_id, pack_version) = match resp {
        ResponseBody::MedAgentIdentity { identity, .. } => {
            (identity.header.id, identity.pack_version)
        }
        other => panic!("{other:?}"),
    };
    let dir = h.dir.clone();
    drop(h);

    // Fresh facade/session/vault-open against the same on-disk root.
    let facade = CoreFacade::new();
    let lease = facade
        .dispatch(base_req(
            Capability::AcquireLease,
            RequestBody::AcquireLease {
                client_id: OpaqueId::new("actor-b"),
                holder_id_hint: Some(OpaqueId::new("actor-b")),
            },
        ))
        .result
        .expect("lease");
    let holder_id = match lease {
        ResponseBody::Lease { holder_id, .. } => holder_id,
        other => panic!("{other:?}"),
    };
    let session = facade
        .dispatch(base_req(
            Capability::OpenSession,
            RequestBody::OpenSession {
                holder_id,
                granted: Capability::operator_grants(),
                ttl_ticks: 1_000_000,
            },
        ))
        .result
        .expect("session");
    let session_id = match session {
        ResponseBody::Session { session_id, .. } => session_id,
        other => panic!("{other:?}"),
    };
    let mut vault_req = base_req(
        Capability::OpenSyntheticVault,
        RequestBody::OpenSyntheticVault {
            vault_root: dir.to_str().unwrap().to_owned(),
        },
    );
    vault_req.session_id = Some(session_id.clone());
    facade.dispatch(vault_req).result.expect("reopen vault");

    let mut get_req = base_req(
        Capability::AgentIdentityRead,
        RequestBody::AgentIdentityGet { agent_id },
    );
    get_req.session_id = Some(session_id);
    let resp = facade.dispatch(get_req).result.expect("get after reopen");
    match resp {
        ResponseBody::MedAgentIdentity { identity, .. } => {
            assert_eq!(identity.pack_version, pack_version);
            assert_eq!(identity.revision, 1);
        }
        other => panic!("{other:?}"),
    }
}
