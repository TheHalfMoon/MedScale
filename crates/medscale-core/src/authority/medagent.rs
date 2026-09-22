//! MedAgent Workbench Core authority paths (Spec 077).
//!
//! T077-03 scope only: `AgentIdentity` + `AgentCapabilityManifest`
//! register/get/list/revoke. `ContextManifest`/`AgentRun`/tool invocation/
//! receipt/proposal Core paths land in T077-04 through T077-08.
//!
//! `pack_version` is never client-supplied: it is captured here from the
//! currently admitted `PackManifestV0` for the given `pack_id`, so a caller
//! cannot register an identity against a version string that does not
//! match what is actually admitted (`security.md` T5). Registration against
//! a `pack_id` the local `PackStore` does not recognize fails closed.
//!
//! Audit split (mirrors `collaboration.rs`'s own T076-03 precedent):
//! `AgentIdentity` is scope-level, not run-level, and has no run to chain an
//! activity record into, so register/revoke reuse the existing in-memory
//! `ActionAuditRecord` trail rather than a dedicated hash chain.

use medscale_contracts::envelopes::AuthorityError;
use medscale_contracts::medagent::{
    AgentCapabilityManifest, AgentIdentity, AgentIdentityStatus, MEDAGENT_SCHEMA_VERSION, ToolKind,
};
use medscale_contracts::objects::{AuthorityScopeId, ObjectHeader, OpaqueId, RealmId, VaultId};
use medscale_storage::{MetaError, SqliteMetaStore};

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

fn header(realm: &RealmId, scope: &AuthorityScopeId, id: OpaqueId) -> ObjectHeader {
    ObjectHeader {
        id,
        schema_version: MEDAGENT_SCHEMA_VERSION,
        realm_id: realm.clone(),
        authority_scope_id: scope.clone(),
    }
}

/// Authenticated authority view for one request.
pub struct MedAgent<'a> {
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

impl MedAgent<'_> {
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

    /// Appends one scope-level audit row to the existing trail (identity
    /// lifecycle only; see module docs).
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

    fn scoped_agent_identity(&self, id: &OpaqueId) -> Result<AgentIdentity, AuthorityError> {
        let identity = self.meta.get_agent_identity(id).map_err(meta_err)?;
        if identity.header.realm_id != self.realm
            || identity.header.authority_scope_id != self.scope
        {
            return Err(AuthorityError::WrongScope);
        }
        Ok(identity)
    }

    fn scoped_project(&self, id: &OpaqueId) -> Result<(), AuthorityError> {
        let project = self.meta.get_project(id).map_err(meta_err)?;
        if project.header.realm_id != self.realm || project.header.authority_scope_id != self.scope
        {
            return Err(AuthorityError::WrongScope);
        }
        Ok(())
    }

    // ----- AgentIdentity + AgentCapabilityManifest -----

    /// Registers a new `AgentIdentity` bound to `pack_id`. The identity's
    /// `pack_version` is captured from the currently admitted
    /// `PackManifestV0`, never from caller input; a `pack_id` the local
    /// `PackStore` does not recognize fails closed.
    pub fn register_agent_identity(
        &mut self,
        project_id: OpaqueId,
        pack_id: OpaqueId,
        display_name: String,
        granted_tool_kinds: Vec<ToolKind>,
    ) -> Result<(AgentIdentity, AgentCapabilityManifest), AuthorityError> {
        self.scoped_project(&project_id)?;
        let manifest = self
            .packs
            .get(&pack_id)
            .ok_or_else(|| AuthorityError::InvalidArgument {
                message: format!("pack {} is not admitted", pack_id.as_str()),
            })?;
        let id = self
            .meta
            .alloc_medagent_id("medagent-identity-seq", "agent")
            .map_err(meta_err)?;
        let identity = AgentIdentity::new(
            header(&self.realm, &self.scope, id.clone()),
            project_id,
            pack_id,
            manifest.version,
            display_name,
        )
        .map_err(|message| AuthorityError::InvalidArgument { message })?;
        let capabilities = AgentCapabilityManifest::new(id.clone(), granted_tool_kinds)
            .map_err(|message| AuthorityError::InvalidArgument { message })?;
        self.meta
            .insert_agent_identity_with_capabilities(&identity, &capabilities)
            .map_err(meta_err)?;
        self.audit("medagent_identity.register", vec![id])?;
        Ok((identity, capabilities))
    }

    /// Scoped identity + capability-manifest read.
    pub fn get_agent_identity(
        &self,
        id: &OpaqueId,
    ) -> Result<(AgentIdentity, AgentCapabilityManifest), AuthorityError> {
        let identity = self.scoped_agent_identity(id)?;
        let capabilities = self.meta.get_capability_manifest(id).map_err(meta_err)?;
        Ok((identity, capabilities))
    }

    /// Lists agent identities in one Project. The Project itself is
    /// scope-checked before the storage read, so no cross-scope row can be
    /// named by a valid `project_id`.
    pub fn list_agent_identities(
        &self,
        project_id: &OpaqueId,
        limit: u32,
    ) -> Result<Vec<AgentIdentity>, AuthorityError> {
        self.scoped_project(project_id)?;
        self.meta
            .list_agent_identities(project_id, limit)
            .map_err(meta_err)
    }

    /// Revokes an agent identity (status only; `pack_id`/`pack_version`
    /// never change once registered).
    pub fn revoke_agent_identity(
        &mut self,
        id: &OpaqueId,
        expected: u64,
    ) -> Result<(AgentIdentity, AgentCapabilityManifest), AuthorityError> {
        let current = self.scoped_agent_identity(id)?;
        if current.status != AgentIdentityStatus::Active {
            return Err(AuthorityError::Conflict {
                message: "agent identity is not active".to_owned(),
            });
        }
        let revoked = self
            .meta
            .revoke_agent_identity(id, expected)
            .map_err(meta_err)?;
        let capabilities = self.meta.get_capability_manifest(id).map_err(meta_err)?;
        self.audit("medagent_identity.revoke", vec![id.clone()])?;
        Ok((revoked, capabilities))
    }
}
