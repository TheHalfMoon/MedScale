//! MedAgent Workbench Core authority paths (Spec 077).
//!
//! T077-03/T077-04 scope: `AgentIdentity` + `AgentCapabilityManifest`
//! register/get/list/revoke, and `ContextManifest` create/get plus the
//! single read-boundary check (`require_artifact_in_context`) every future
//! tool-dispatch path (T077-06) must call. `AgentRun`/tool invocation/
//! receipt/proposal Core paths land in T077-05 through T077-08.
//!
//! `pack_version` is never client-supplied: it is captured here from the
//! currently admitted `PackManifestV0` for the given `pack_id`, so a caller
//! cannot register an identity against a version string that does not
//! match what is actually admitted (`security.md` T5). Registration against
//! a `pack_id` the local `PackStore` does not recognize fails closed.
//!
//! `ContextManifest.selected_artifacts` are validated for shape only at
//! write time; their live resolution is always recomputed at read time
//! through `resolve_context_artifact` (`migration.md` section 4), never
//! cached -- an artifact deleted or superseded after a `ContextManifest`
//! was created shows up as `Missing`/`Stale` on the next read, it does not
//! silently keep resolving `Current`.
//!
//! Audit split (mirrors `collaboration.rs`'s own T076-03 precedent):
//! `AgentIdentity`/`ContextManifest` are scope-level, not run-level, and
//! have no run to chain an activity record into, so their mutations reuse
//! the existing in-memory `ActionAuditRecord` trail rather than a
//! dedicated hash chain.

use medscale_contracts::envelopes::AuthorityError;
use medscale_contracts::medagent::{
    AgentCapabilityManifest, AgentIdentity, AgentIdentityStatus, ContextManifest,
    MEDAGENT_SCHEMA_VERSION, ToolKind,
};
use medscale_contracts::objects::{AuthorityScopeId, ObjectHeader, OpaqueId, RealmId, VaultId};
use medscale_contracts::project_graph::{
    ArtifactDescriptor, ArtifactKind, ArtifactVersionBinding, ReferenceResolution,
};
use medscale_storage::{MetaError, SqliteMetaStore};

use super::store::{InMemoryAuthorityStore, ScopeError, StoredObject};
use crate::process::{LeaseRegistry, SessionRegistry};

/// Mirrors `project_graph::stored_class_matches` (deliberately duplicated,
/// not imported, per this spec's own module-independence convention -- see
/// `crates/medscale-storage/src/medagent.rs`'s header comment).
fn stored_class_matches(stored: &StoredObject, kind: &ArtifactKind) -> bool {
    matches!(
        (stored, kind),
        (StoredObject::Source(_), ArtifactKind::SourceRecord)
            | (
                StoredObject::Derived(_),
                ArtifactKind::DerivedSourceArtifact
            )
            | (StoredObject::Proposal(_), ArtifactKind::Proposal)
            | (StoredObject::Assertion(_), ArtifactKind::ClinicalAssertion)
            | (StoredObject::Evaluation(_), ArtifactKind::EvaluationRecord)
            | (StoredObject::Identity(_), ArtifactKind::IdentityAssertion)
            | (StoredObject::Amendment(_), ArtifactKind::AmendmentRecord)
    )
}

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

    // ----- ContextManifest -----

    fn scoped_context_manifest(&self, id: &OpaqueId) -> Result<ContextManifest, AuthorityError> {
        let manifest = self.meta.get_context_manifest(id).map_err(meta_err)?;
        if manifest.header.realm_id != self.realm
            || manifest.header.authority_scope_id != self.scope
        {
            return Err(AuthorityError::WrongScope);
        }
        Ok(manifest)
    }

    /// Resolves one artifact descriptor against canonical owners in this
    /// realm/scope (mirrors `project_graph::ProjectGraph::resolve_descriptor`
    /// and `collaboration::Collab`'s own duplicated `resolve_anchor` --
    /// deliberately not imported, per this spec's module-independence
    /// convention). Always live; never cached (`migration.md` section 4).
    fn resolve_context_artifact(&self, descriptor: &ArtifactDescriptor) -> ReferenceResolution {
        match &descriptor.kind {
            ArtifactKind::PackManifest => {
                let Some(manifest) = self.packs.get(&descriptor.object_id) else {
                    return ReferenceResolution::Missing;
                };
                match &descriptor.binding {
                    ArtifactVersionBinding::IdentityOnly => ReferenceResolution::Current,
                    ArtifactVersionBinding::Digest(digest) => {
                        if manifest.content_digest == *digest {
                            ReferenceResolution::Current
                        } else {
                            ReferenceResolution::Stale
                        }
                    }
                    ArtifactVersionBinding::Revision(rev) => {
                        if manifest.version == *rev {
                            ReferenceResolution::Current
                        } else {
                            ReferenceResolution::Stale
                        }
                    }
                    ArtifactVersionBinding::DigestAndRevision { digest, revision } => {
                        if manifest.content_digest == *digest && manifest.version == *revision {
                            ReferenceResolution::Current
                        } else {
                            ReferenceResolution::Stale
                        }
                    }
                }
            }
            ArtifactKind::EvidenceDocument | ArtifactKind::OtherExplicit(_) => {
                ReferenceResolution::UnsupportedKind
            }
            _ => {
                let stored =
                    match self
                        .store
                        .get_scoped(&descriptor.object_id, &self.realm, &self.scope)
                    {
                        Ok(obj) => obj,
                        Err(ScopeError::NotFound) => return ReferenceResolution::Missing,
                        Err(ScopeError::WrongScope) => return ReferenceResolution::Denied,
                    };
                if !stored_class_matches(stored, &descriptor.kind) {
                    return ReferenceResolution::Corrupt;
                }
                match &descriptor.binding {
                    ArtifactVersionBinding::IdentityOnly => ReferenceResolution::Current,
                    _ => ReferenceResolution::Stale,
                }
            }
        }
    }

    /// Creates a new `ContextManifest`. Artifacts are validated for shape
    /// only (`ArtifactDescriptor::validate`); existence/staleness is never
    /// checked here -- it is always recomputed live on read
    /// (`migration.md` section 4), so a `ContextManifest` may legitimately
    /// name an artifact that does not exist yet or has since gone stale.
    pub fn create_context_manifest(
        &mut self,
        project_id: OpaqueId,
        selected_artifacts: Vec<ArtifactDescriptor>,
    ) -> Result<ContextManifest, AuthorityError> {
        self.scoped_project(&project_id)?;
        for artifact in &selected_artifacts {
            artifact
                .validate()
                .map_err(|message| AuthorityError::InvalidArgument { message })?;
        }
        let id = self
            .meta
            .alloc_medagent_id("medagent-context-seq", "context")
            .map_err(meta_err)?;
        let manifest = ContextManifest::new(
            header(&self.realm, &self.scope, id.clone()),
            project_id,
            selected_artifacts,
        )
        .map_err(|message| AuthorityError::InvalidArgument { message })?;
        self.meta
            .insert_context_manifest(&manifest)
            .map_err(meta_err)?;
        self.audit("medagent_context_manifest.create", vec![id])?;
        Ok(manifest)
    }

    /// Scoped context-manifest read, with every selected artifact's
    /// `ReferenceResolution` recomputed live (never cached) alongside it, in
    /// the same order as `selected_artifacts`.
    pub fn get_context_manifest(
        &self,
        id: &OpaqueId,
    ) -> Result<(ContextManifest, Vec<ReferenceResolution>), AuthorityError> {
        let manifest = self.scoped_context_manifest(id)?;
        let resolutions = manifest
            .selected_artifacts
            .iter()
            .map(|artifact| self.resolve_context_artifact(artifact))
            .collect();
        Ok((manifest, resolutions))
    }

    /// THE single Core-internal check for whether `object_id` is readable
    /// under `context_id`'s bound `ContextManifest` (`security.md` T4).
    /// Every future tool-dispatch path (T077-06) must call this before
    /// resolving any artifact content; there is no second, unchecked read
    /// path for agent-run code. Returns the manifest itself on success so
    /// the caller never needs a second, separately-scoped fetch.
    ///
    /// No production caller exists yet: T077-06's tool dispatch is the
    /// first one, and lands in a later commit. Proven directly by this
    /// module's own tests in the meantime (`#[allow(dead_code)]` is
    /// temporary and must be removed the moment T077-06 adds its caller).
    #[allow(dead_code)]
    pub fn require_artifact_in_context(
        &self,
        context_id: &OpaqueId,
        object_id: &OpaqueId,
    ) -> Result<ContextManifest, AuthorityError> {
        let context = self.scoped_context_manifest(context_id)?;
        if !context.allows(object_id) {
            return Err(AuthorityError::Unauthorized);
        }
        Ok(context)
    }
}

#[cfg(test)]
mod tests {
    use super::super::source_ops::create_source_record;
    use super::*;
    use medscale_contracts::project_graph::Project;

    #[allow(clippy::too_many_arguments)]
    fn harness<'a>(
        meta: &'a SqliteMetaStore,
        store: &'a mut InMemoryAuthorityStore,
        packs: &'a medscale_pack::PackStore,
        sessions: &'a SessionRegistry,
        leases: &'a LeaseRegistry,
        vault_id: &'a VaultId,
        realm: &str,
        scope: &str,
    ) -> MedAgent<'a> {
        MedAgent {
            store,
            meta,
            packs,
            sessions,
            leases,
            vault_id,
            realm: RealmId::new(realm),
            scope: AuthorityScopeId::new(scope),
            session_id: None,
        }
    }

    fn temp_meta(name: &str) -> SqliteMetaStore {
        let dir =
            std::env::temp_dir().join(format!("medscale-077cu-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        SqliteMetaStore::open_at(&dir.join("meta.sqlite3")).unwrap()
    }

    fn project_header(realm: &str, scope: &str, id: &str) -> ObjectHeader {
        ObjectHeader {
            id: OpaqueId::new(id),
            schema_version: 1,
            realm_id: RealmId::new(realm),
            authority_scope_id: AuthorityScopeId::new(scope),
        }
    }

    #[test]
    fn resolve_context_artifact_matrix_matches_project_graph_semantics() {
        let meta = temp_meta("resolve");
        let mut store = InMemoryAuthorityStore::default();
        let packs = medscale_pack::PackStore::default();
        let sessions = SessionRegistry::new();
        let leases = LeaseRegistry::new();
        let vault_id = VaultId::new("vault-1");
        let record = create_source_record(
            &mut store,
            RealmId::new("realm-a"),
            AuthorityScopeId::new("scope-a"),
            "text/plain".to_owned(),
            b"hello".to_vec(),
        );
        let medagent = harness(
            &meta, &mut store, &packs, &sessions, &leases, &vault_id, "realm-a", "scope-a",
        );

        assert_eq!(
            medagent.resolve_context_artifact(&ArtifactDescriptor {
                object_id: record.header.id.clone(),
                kind: ArtifactKind::SourceRecord,
                binding: ArtifactVersionBinding::IdentityOnly,
            }),
            ReferenceResolution::Current
        );
        assert_eq!(
            medagent.resolve_context_artifact(&ArtifactDescriptor {
                object_id: OpaqueId::new("src-nope"),
                kind: ArtifactKind::SourceRecord,
                binding: ArtifactVersionBinding::IdentityOnly,
            }),
            ReferenceResolution::Missing
        );
        assert_eq!(
            medagent.resolve_context_artifact(&ArtifactDescriptor {
                object_id: record.header.id.clone(),
                kind: ArtifactKind::EvidenceDocument,
                binding: ArtifactVersionBinding::IdentityOnly,
            }),
            ReferenceResolution::UnsupportedKind
        );
    }

    #[test]
    fn create_and_get_context_manifest_resolves_live_never_cached() {
        let meta = temp_meta("create-get");
        meta.insert_project(
            &Project::new(
                project_header("realm-a", "scope-a", "proj-1"),
                "proj-1".to_owned(),
                None,
            )
            .unwrap(),
        )
        .unwrap();
        let mut store = InMemoryAuthorityStore::default();
        let packs = medscale_pack::PackStore::default();
        let sessions = SessionRegistry::new();
        let leases = LeaseRegistry::new();
        let vault_id = VaultId::new("vault-1");
        let record = create_source_record(
            &mut store,
            RealmId::new("realm-a"),
            AuthorityScopeId::new("scope-a"),
            "text/plain".to_owned(),
            b"hello".to_vec(),
        );
        let mut medagent = harness(
            &meta, &mut store, &packs, &sessions, &leases, &vault_id, "realm-a", "scope-a",
        );

        let manifest = medagent
            .create_context_manifest(
                OpaqueId::new("proj-1"),
                vec![
                    ArtifactDescriptor {
                        object_id: record.header.id.clone(),
                        kind: ArtifactKind::SourceRecord,
                        binding: ArtifactVersionBinding::IdentityOnly,
                    },
                    // Not yet existing at creation time -- shape-only
                    // validation must still accept this (migration.md
                    // section 4: existence is never checked at write time).
                    ArtifactDescriptor {
                        object_id: OpaqueId::new("src-not-yet-created"),
                        kind: ArtifactKind::SourceRecord,
                        binding: ArtifactVersionBinding::IdentityOnly,
                    },
                ],
            )
            .unwrap();

        let (read_back, resolutions) = medagent.get_context_manifest(&manifest.header.id).unwrap();
        assert_eq!(read_back, manifest);
        assert_eq!(
            resolutions,
            vec![ReferenceResolution::Current, ReferenceResolution::Missing,]
        );
    }

    #[test]
    fn require_artifact_in_context_allows_named_denies_everything_else() {
        let meta = temp_meta("boundary");
        meta.insert_project(
            &Project::new(
                project_header("realm-a", "scope-a", "proj-1"),
                "proj-1".to_owned(),
                None,
            )
            .unwrap(),
        )
        .unwrap();
        let mut store = InMemoryAuthorityStore::default();
        let packs = medscale_pack::PackStore::default();
        let sessions = SessionRegistry::new();
        let leases = LeaseRegistry::new();
        let vault_id = VaultId::new("vault-1");
        let allowed = create_source_record(
            &mut store,
            RealmId::new("realm-a"),
            AuthorityScopeId::new("scope-a"),
            "text/plain".to_owned(),
            b"allowed".to_vec(),
        );
        let outside = create_source_record(
            &mut store,
            RealmId::new("realm-a"),
            AuthorityScopeId::new("scope-a"),
            "text/plain".to_owned(),
            b"outside".to_vec(),
        );
        let mut medagent = harness(
            &meta, &mut store, &packs, &sessions, &leases, &vault_id, "realm-a", "scope-a",
        );
        let manifest = medagent
            .create_context_manifest(
                OpaqueId::new("proj-1"),
                vec![ArtifactDescriptor {
                    object_id: allowed.header.id.clone(),
                    kind: ArtifactKind::SourceRecord,
                    binding: ArtifactVersionBinding::IdentityOnly,
                }],
            )
            .unwrap();

        // The named artifact resolves through the boundary check.
        assert!(
            medagent
                .require_artifact_in_context(&manifest.header.id, &allowed.header.id)
                .is_ok()
        );
        // A real, existing artifact that simply was never named by this
        // ContextManifest is refused -- direct access outside the bound
        // context is denied even though the object itself is perfectly
        // readable to a human operator with full Project access
        // (security.md T4).
        let err = medagent
            .require_artifact_in_context(&manifest.header.id, &outside.header.id)
            .unwrap_err();
        assert!(matches!(err, AuthorityError::Unauthorized));
        // A fabricated id that never existed anywhere is refused the same
        // way -- the boundary check never needs to consult storage at all.
        let err = medagent
            .require_artifact_in_context(&manifest.header.id, &OpaqueId::new("src-fabricated"))
            .unwrap_err();
        assert!(matches!(err, AuthorityError::Unauthorized));
    }

    #[test]
    fn context_manifest_is_scope_isolated() {
        let meta = temp_meta("scope-isolation");
        meta.insert_project(
            &Project::new(
                project_header("realm-a", "scope-a", "proj-1"),
                "proj-1".to_owned(),
                None,
            )
            .unwrap(),
        )
        .unwrap();
        let mut store = InMemoryAuthorityStore::default();
        let packs = medscale_pack::PackStore::default();
        let sessions = SessionRegistry::new();
        let leases = LeaseRegistry::new();
        let vault_id = VaultId::new("vault-1");
        let record = create_source_record(
            &mut store,
            RealmId::new("realm-a"),
            AuthorityScopeId::new("scope-a"),
            "text/plain".to_owned(),
            b"hello".to_vec(),
        );
        let context_id = {
            let mut medagent_a = harness(
                &meta, &mut store, &packs, &sessions, &leases, &vault_id, "realm-a", "scope-a",
            );
            medagent_a
                .create_context_manifest(
                    OpaqueId::new("proj-1"),
                    vec![ArtifactDescriptor {
                        object_id: record.header.id.clone(),
                        kind: ArtifactKind::SourceRecord,
                        binding: ArtifactVersionBinding::IdentityOnly,
                    }],
                )
                .unwrap()
                .header
                .id
        };

        // A second caller scoped to a different realm/scope cannot read
        // scope-a's ContextManifest at all -- WrongScope, not NotFound
        // (which would leak existence) and not a successful read.
        let medagent_b = harness(
            &meta, &mut store, &packs, &sessions, &leases, &vault_id, "realm-b", "scope-b",
        );
        let err = medagent_b.get_context_manifest(&context_id).unwrap_err();
        assert!(matches!(err, AuthorityError::WrongScope));
        let err = medagent_b
            .require_artifact_in_context(&context_id, &record.header.id)
            .unwrap_err();
        assert!(matches!(err, AuthorityError::WrongScope));
    }
}
