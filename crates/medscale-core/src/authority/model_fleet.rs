//! Model Fleet + Compare Core authority paths (Spec 078).
//!
//! T078-03 scope: `AgentLane` + `LanePolicy` create/get/list/retire.
//!
//! A lane is a policy wrapper around an existing, unmodified Spec 077
//! `AgentIdentity`/`ContextManifest` pair. Every Spec 077 object this module
//! touches is read through Spec 077's own public, scope-checked `MedAgent`
//! entry points (`get_agent_identity`, `get_context_manifest`), never through
//! its storage rows or private helpers (`security.md` section 1).
//!
//! `LanePolicy` narrows, never widens, what the bound identity's
//! `AgentCapabilityManifest` and the bound `ContextManifest` already allow
//! (`security.md` T2): `LanePolicy::validate_within` runs against the
//! live-resolved manifests before any row is written, and a superset request
//! is `InvalidArgument` with nothing persisted.
//!
//! This module has no path to proposal promotion, amendment, effects, or
//! external actions (`security.md` T1); it imports none of them.

use medscale_contracts::envelopes::AuthorityError;
use medscale_contracts::medagent::{AgentIdentityStatus, ToolKind};
use medscale_contracts::model_fleet::{
    AgentLane, AgentLaneStatus, LanePolicy, MODEL_FLEET_SCHEMA_VERSION,
};
use medscale_contracts::objects::{AuthorityScopeId, ObjectHeader, OpaqueId, RealmId, VaultId};
use medscale_storage::{MetaError, SqliteMetaStore};

use super::medagent::MedAgent;
use super::store::{InMemoryAuthorityStore, StoredObject};
use crate::process::{LeaseRegistry, SessionRegistry};

fn meta_err(err: MetaError) -> AuthorityError {
    match err {
        MetaError::NotFound => AuthorityError::NotFound,
        MetaError::Conflict(message) => AuthorityError::Conflict { message },
        MetaError::UnsupportedSchema(message) => AuthorityError::UnsupportedSchema { message },
        MetaError::CorruptObjectBody(message) => AuthorityError::Corrupt { message },
        other => AuthorityError::Internal {
            message: other.to_string(),
        },
    }
}

fn invalid(message: String) -> AuthorityError {
    AuthorityError::InvalidArgument { message }
}

fn header(realm: &RealmId, scope: &AuthorityScopeId, id: OpaqueId) -> ObjectHeader {
    ObjectHeader {
        id,
        schema_version: MODEL_FLEET_SCHEMA_VERSION,
        realm_id: realm.clone(),
        authority_scope_id: scope.clone(),
    }
}

/// Authenticated Model Fleet authority view for one request.
pub struct ModelFleet<'a> {
    pub store: &'a mut InMemoryAuthorityStore,
    pub meta: &'a SqliteMetaStore,
    pub packs: &'a medscale_pack::PackStore,
    pub sessions: &'a SessionRegistry,
    pub leases: &'a LeaseRegistry,
    pub vault_id: &'a VaultId,
    pub realm: RealmId,
    pub scope: AuthorityScopeId,
    pub session_id: Option<OpaqueId>,
}

impl ModelFleet<'_> {
    /// A Spec 077 `MedAgent` view over the same request authority. This is
    /// the only way this module reaches Spec 077 behavior.
    fn medagent(&mut self) -> MedAgent<'_> {
        MedAgent {
            store: &mut *self.store,
            meta: self.meta,
            packs: self.packs,
            sessions: self.sessions,
            leases: self.leases,
            vault_id: self.vault_id,
            realm: self.realm.clone(),
            scope: self.scope.clone(),
            session_id: self.session_id.clone(),
        }
    }

    /// Resolves the request actor: session holder, else lease holder.
    fn actor(&self) -> Result<OpaqueId, AuthorityError> {
        if let Some(holder) = self
            .session_id
            .as_ref()
            .and_then(|session_id| self.sessions.holder_of(session_id))
        {
            return Ok(holder);
        }
        self.leases
            .holder(self.vault_id)
            .ok_or(AuthorityError::LeaseRequired)
    }

    /// Appends one scope-level audit row to the existing trail (the same
    /// `ActionAuditRecord` mechanism Spec 077 uses for its scope-level rows).
    fn audit(&mut self, action: &str, targets: Vec<OpaqueId>) -> Result<(), AuthorityError> {
        let actor = self.actor()?;
        let id = self.store.alloc_id("audit");
        let record = medscale_contracts::objects::ActionAuditRecord {
            header: ObjectHeader {
                id,
                schema_version: medscale_contracts::AUTHORITY_SCHEMA_VERSION,
                realm_id: self.realm.clone(),
                authority_scope_id: self.scope.clone(),
            },
            kind: medscale_contracts::objects::ActionAuditKind::Audit,
            actor,
            action: action.to_owned(),
            target_refs: targets,
            effect_state: None,
            payload_digest: None,
            detail: None,
        };
        self.store.insert(StoredObject::Audit(record));
        Ok(())
    }

    fn in_scope(&self, header: &ObjectHeader) -> Result<(), AuthorityError> {
        if header.realm_id != self.realm || header.authority_scope_id != self.scope {
            return Err(AuthorityError::WrongScope);
        }
        Ok(())
    }

    fn scoped_project(&self, id: &OpaqueId) -> Result<(), AuthorityError> {
        let project = self.meta.get_project(id).map_err(meta_err)?;
        self.in_scope(&project.header)
    }

    fn scoped_lane(&self, id: &OpaqueId) -> Result<AgentLane, AuthorityError> {
        let lane = self.meta.get_agent_lane(id).map_err(meta_err)?;
        self.in_scope(&lane.header)?;
        Ok(lane)
    }

    // ----- AgentLane + LanePolicy -----

    /// Creates an `Active` agent lane bound to an existing Spec 077
    /// identity/context pair in `project_id`. Refused before any write when
    /// the identity is revoked, either object belongs to another Project,
    /// or `policy` is not a genuine subset of the bound identity's
    /// capability manifest and the bound context manifest (`security.md` T2).
    pub fn create_agent_lane(
        &mut self,
        project_id: OpaqueId,
        agent_identity_id: OpaqueId,
        context_manifest_id: OpaqueId,
        role_label: String,
        granted_tool_kinds: Option<Vec<ToolKind>>,
        context_artifact_ids: Option<Vec<OpaqueId>>,
    ) -> Result<AgentLane, AuthorityError> {
        self.scoped_project(&project_id)?;
        let policy = LanePolicy::new(granted_tool_kinds, context_artifact_ids).map_err(invalid)?;
        let (identity, capabilities) = self.medagent().get_agent_identity(&agent_identity_id)?;
        if identity.status != AgentIdentityStatus::Active {
            return Err(invalid("agent identity is revoked".to_owned()));
        }
        if identity.project_id != project_id {
            return Err(invalid(
                "agent identity does not belong to this project".to_owned(),
            ));
        }
        let (context, _resolutions) = self.medagent().get_context_manifest(&context_manifest_id)?;
        if context.project_id != project_id {
            return Err(invalid(
                "context manifest does not belong to this project".to_owned(),
            ));
        }
        policy
            .validate_within(&capabilities, &context)
            .map_err(invalid)?;
        let id = self
            .meta
            .alloc_model_fleet_id("model-fleet-lane-seq", "lane")
            .map_err(meta_err)?;
        let lane = AgentLane::new(
            header(&self.realm, &self.scope, id.clone()),
            project_id,
            agent_identity_id,
            context_manifest_id,
            role_label,
            policy,
        )
        .map_err(invalid)?;
        self.meta.insert_agent_lane(&lane).map_err(meta_err)?;
        self.audit("model_fleet_lane.create", vec![id])?;
        Ok(lane)
    }

    /// Scoped lane read.
    pub fn get_agent_lane(&self, id: &OpaqueId) -> Result<AgentLane, AuthorityError> {
        self.scoped_lane(id)
    }

    /// Lists lanes in one Project, optionally by status. The Project is
    /// scope-checked first, so no cross-scope row can be named.
    pub fn list_agent_lanes(
        &self,
        project_id: &OpaqueId,
        status: Option<AgentLaneStatus>,
        limit: u32,
    ) -> Result<Vec<AgentLane>, AuthorityError> {
        self.scoped_project(project_id)?;
        self.meta
            .list_agent_lanes(project_id, status, limit)
            .map_err(meta_err)
    }

    /// Retires an active lane (status tombstone; CAS on `expected_revision`).
    /// Past fleet history bound to the lane is never rewritten.
    pub fn retire_agent_lane(
        &mut self,
        id: &OpaqueId,
        expected_revision: u64,
    ) -> Result<AgentLane, AuthorityError> {
        let lane = self.scoped_lane(id)?;
        lane.check_mutation(expected_revision)
            .map_err(|message| AuthorityError::Conflict { message })?;
        if lane.status == AgentLaneStatus::Retired {
            return Err(AuthorityError::Conflict {
                message: "agent lane is already retired".to_owned(),
            });
        }
        let retired = self
            .meta
            .retire_agent_lane(id, expected_revision)
            .map_err(meta_err)?;
        self.audit("model_fleet_lane.retire", vec![id.clone()])?;
        Ok(retired)
    }
}

#[cfg(test)]
mod tests {
    /// `security.md` T1: the non-test part of this module has no path to
    /// proposal promotion, amendment, effects, or external actions.
    #[test]
    fn module_has_no_path_to_promotion_amendment_effects_or_actions() {
        let source = include_str!("model_fleet.rs");
        let production = source.split("#[cfg(test)]").next().expect("module source");
        for forbidden in [
            "promote::",
            "amend::",
            "actions::",
            "PromoteProposal",
            "TransitionEffect",
            "ExternalActionIntent",
            "Outbox",
            "ClinicalAssertion",
        ] {
            assert!(
                !production.contains(forbidden),
                "authority::model_fleet must not reference `{forbidden}`"
            );
        }
    }
}
