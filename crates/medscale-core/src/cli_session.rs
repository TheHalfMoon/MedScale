//! CLI session: in-process transient Core Host owner (Spec 006 + Spec 024 sessions).

use medscale_contracts::envelopes::{
    AuthorityError, AuthorityRequest, Capability, RequestBody, ResponseBody,
};
use medscale_contracts::objects::{AuthorityScopeId, OpaqueId, RealmId, VaultId};
use medscale_contracts::presentation::{SubjectBriefV1, SubjectCoverageV1, SubjectTimelineV1};

use crate::CoreFacade;

/// In-process CLI session owning one CoreFacade + lease + client session for a vault id.
pub struct CliSession {
    facade: CoreFacade,
    vault_id: VaultId,
    realm_id: RealmId,
    scope_id: AuthorityScopeId,
    holder_id: OpaqueId,
    session_id: OpaqueId,
    next_req: u64,
    vault_root: Option<String>,
    open: bool,
}

impl CliSession {
    /// Acquire lease and open the broad CLI operator session (vault not yet open).
    pub fn connect(vault_id: &str) -> Result<Self, AuthorityError> {
        Self::connect_with_grants(
            vault_id,
            "medscale-cli",
            "cli-holder",
            Capability::operator_grants(),
        )
    }

    /// Acquire the least-privilege session used by Desktop Model Center.
    ///
    /// The session may inspect admitted Packs and submit a local Pack for Core
    /// admission. It cannot open vaults, evaluate models, promote Packs, use the
    /// network broker, or invoke unrelated operator capabilities.
    pub fn connect_pack_operator(vault_id: &str) -> Result<Self, AuthorityError> {
        Self::connect_with_grants(
            vault_id,
            "medscale-model-center",
            "model-center-holder",
            vec![Capability::PacksList, Capability::PacksInstallLocal],
        )
    }

    fn connect_with_grants(
        vault_id: &str,
        client_id: &str,
        holder_id_hint: &str,
        granted: Vec<Capability>,
    ) -> Result<Self, AuthorityError> {
        let facade = CoreFacade::new();
        let vault_id = VaultId::new(vault_id);
        let realm_id = RealmId::new("cli-realm");
        let scope_id = AuthorityScopeId::new("cli-scope");
        let mut session = Self {
            facade,
            vault_id: vault_id.clone(),
            realm_id: realm_id.clone(),
            scope_id: scope_id.clone(),
            holder_id: OpaqueId::new("pending"),
            session_id: OpaqueId::new("pending"),
            next_req: 0,
            vault_root: None,
            open: false,
        };
        let resp = session.dispatch_bootstrap(
            Capability::AcquireLease,
            RequestBody::AcquireLease {
                client_id: OpaqueId::new(client_id),
                holder_id_hint: Some(OpaqueId::new(holder_id_hint)),
            },
        )?;
        let ResponseBody::Lease { holder_id, .. } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected lease".to_owned(),
            });
        };
        session.holder_id = holder_id.clone();
        let opened = session.dispatch_bootstrap(
            Capability::OpenSession,
            RequestBody::OpenSession {
                holder_id,
                granted,
                ttl_ticks: 1_000_000,
            },
        )?;
        let ResponseBody::Session { session_id, .. } = opened else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected session".to_owned(),
            });
        };
        session.session_id = session_id;
        Ok(session)
    }

    fn req_id(&mut self) -> OpaqueId {
        self.next_req += 1;
        OpaqueId::new(format!("cli-req-{}", self.next_req))
    }

    fn dispatch_bootstrap(
        &mut self,
        capability: Capability,
        body: RequestBody,
    ) -> Result<ResponseBody, AuthorityError> {
        let req = AuthorityRequest::new(
            self.req_id(),
            self.vault_id.clone(),
            self.realm_id.clone(),
            self.scope_id.clone(),
            capability,
            body,
        );
        self.facade.dispatch(req).result
    }

    fn dispatch(
        &mut self,
        capability: Capability,
        body: RequestBody,
    ) -> Result<ResponseBody, AuthorityError> {
        let mut req = AuthorityRequest::new(
            self.req_id(),
            self.vault_id.clone(),
            self.realm_id.clone(),
            self.scope_id.clone(),
            capability,
            body,
        );
        req.session_id = Some(self.session_id.clone());
        self.facade.dispatch(req).result
    }

    pub fn create_encrypted_vault(
        &mut self,
        vault_root: &str,
        passphrase: &str,
    ) -> Result<Vec<String>, AuthorityError> {
        let resp = self.dispatch(
            Capability::CreateEncryptedVault,
            RequestBody::CreateEncryptedVault {
                vault_root: vault_root.to_owned(),
                passphrase: passphrase.to_owned(),
            },
        )?;
        let ResponseBody::EncryptedVaultReady {
            vault_root,
            recovery_codes,
        } = resp
        else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected encrypted vault ready".to_owned(),
            });
        };
        self.vault_root = Some(vault_root);
        self.open = true;
        Ok(recovery_codes.unwrap_or_default())
    }

    pub fn open_encrypted_vault(
        &mut self,
        vault_root: &str,
        passphrase: &str,
    ) -> Result<(), AuthorityError> {
        let resp = self.dispatch(
            Capability::OpenEncryptedVault,
            RequestBody::OpenEncryptedVault {
                vault_root: vault_root.to_owned(),
                passphrase: Some(passphrase.to_owned()),
                recovery_code: None,
            },
        )?;
        let ResponseBody::EncryptedVaultReady { vault_root, .. } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected encrypted vault ready".to_owned(),
            });
        };
        self.vault_root = Some(vault_root);
        self.open = true;
        Ok(())
    }

    pub fn open_synthetic_vault(&mut self, vault_root: &str) -> Result<(), AuthorityError> {
        let _ = self.dispatch(
            Capability::OpenSyntheticVault,
            RequestBody::OpenSyntheticVault {
                vault_root: vault_root.to_owned(),
            },
        )?;
        self.vault_root = Some(vault_root.to_owned());
        self.open = true;
        Ok(())
    }

    /// Default local vault root for a vault id (platform data directory).
    ///
    /// Desktop resolves its Projects vault location through Core so surfaces
    /// never depend on storage layout directly.
    #[must_use]
    pub fn default_vault_root(vault_id: &str) -> std::path::PathBuf {
        medscale_storage::default_vault_root(vault_id)
    }

    pub fn ingest_fhir_file(&mut self, path: &str) -> Result<OpaqueId, AuthorityError> {
        let bytes = std::fs::read(path).map_err(|e| AuthorityError::InvalidArgument {
            message: e.to_string(),
        })?;
        let resp = self.dispatch(
            Capability::IngestFhirSynthetic,
            RequestBody::IngestFhirSynthetic {
                media_type: "application/fhir+json".to_owned(),
                bytes,
                fhir_version_hint: Some("4.0.1".to_owned()),
                attach_validator_fixture_id: None,
            },
        )?;
        let ResponseBody::Ingested { receipt } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected ingest receipt".to_owned(),
            });
        };
        receipt.source_id.ok_or(AuthorityError::InvalidArgument {
            message: format!("ingest outcome {:?}", receipt.outcome),
        })
    }

    pub fn promote_fixture_as_assertion(
        &mut self,
        source_id: OpaqueId,
        subject: &str,
        claim_kind: &str,
        resource_json: serde_json::Value,
    ) -> Result<OpaqueId, AuthorityError> {
        let created = self.dispatch(
            Capability::CreateProposal,
            RequestBody::CreateProposal {
                subject_ref: Some(OpaqueId::new(subject)),
                claim_kind: claim_kind.to_owned(),
                payload: serde_json::json!({ "resource": resource_json }),
                evidence_refs: vec![source_id],
            },
        )?;
        let ResponseBody::Created {
            object_id: proposal_id,
        } = created
        else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected proposal".to_owned(),
            });
        };
        let promoted = self.dispatch(
            Capability::PromoteProposal,
            RequestBody::PromoteProposal {
                proposal_id,
                authorized_by: OpaqueId::new("cli-operator"),
                subject_ref: OpaqueId::new(subject),
            },
        )?;
        let ResponseBody::Promoted { assertion_id, .. } = promoted else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected promote".to_owned(),
            });
        };
        Ok(assertion_id)
    }

    pub fn timeline(&mut self, subject: &str) -> Result<SubjectTimelineV1, AuthorityError> {
        let resp = self.dispatch(
            Capability::GetTimeline,
            RequestBody::GetTimeline {
                subject_ref: OpaqueId::new(subject),
            },
        )?;
        let ResponseBody::Timeline { body, .. } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected timeline".to_owned(),
            });
        };
        Ok(body)
    }

    pub fn brief(&mut self, subject: &str) -> Result<SubjectBriefV1, AuthorityError> {
        let resp = self.dispatch(
            Capability::GetBrief,
            RequestBody::GetBrief {
                subject_ref: OpaqueId::new(subject),
            },
        )?;
        let ResponseBody::Brief { body, .. } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected brief".to_owned(),
            });
        };
        Ok(body)
    }

    pub fn coverage(&mut self, subject: &str) -> Result<SubjectCoverageV1, AuthorityError> {
        let resp = self.dispatch(
            Capability::GetCoverage,
            RequestBody::GetCoverage {
                subject_ref: OpaqueId::new(subject),
            },
        )?;
        let ResponseBody::Coverage { body, .. } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected coverage".to_owned(),
            });
        };
        Ok(body)
    }

    #[must_use]
    pub fn vault_root(&self) -> Option<&str> {
        self.vault_root.as_deref()
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// This session's bound lease/session holder id -- the identity every
    /// Core authority check resolves `actor()` to for requests made through
    /// this session. A caller that needs to register its own
    /// `ParticipantIdentity` (Spec 076 collaboration) must register under
    /// this exact id, not an arbitrary string: `caller_participant()` looks
    /// up the participant by the session's real holder, so a mismatched
    /// `holder_id` would register a participant Core can never resolve as
    /// the caller.
    #[must_use]
    pub fn holder_id(&self) -> OpaqueId {
        self.holder_id.clone()
    }

    /// Offline local pack install (Spec 008).
    pub fn packs_install_local(
        &mut self,
        local_path: &str,
    ) -> Result<medscale_contracts::packs::PackAdmitResult, AuthorityError> {
        let resp = self.dispatch(
            Capability::PacksInstallLocal,
            RequestBody::PacksInstallLocal {
                local_path: local_path.to_owned(),
            },
        )?;
        let ResponseBody::PackAdmit { result } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected pack admit".to_owned(),
            });
        };
        Ok(result)
    }

    /// List admitted packs.
    pub fn packs_list(
        &mut self,
    ) -> Result<Vec<medscale_contracts::packs::PackManifestV0>, AuthorityError> {
        let resp = self.dispatch(Capability::PacksList, RequestBody::PacksList)?;
        let ResponseBody::PackList { packs } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected pack list".to_owned(),
            });
        };
        Ok(packs)
    }

    /// Read the durable external-action outbox through the same authority facade as Desktop.
    pub fn outbox(
        &mut self,
    ) -> Result<Vec<medscale_contracts::actions::OutboxEntry>, AuthorityError> {
        let resp = self.dispatch(Capability::ListOutbox, RequestBody::ListOutbox)?;
        let ResponseBody::Outbox { entries } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected outbox".to_owned(),
            });
        };
        Ok(entries)
    }

    /// Read append-only disclosure records through the authority facade.
    pub fn disclosures(
        &mut self,
    ) -> Result<Vec<medscale_contracts::workflow::DisclosureRecord>, AuthorityError> {
        let resp = self.dispatch(Capability::ListDisclosures, RequestBody::ListDisclosures)?;
        let ResponseBody::DisclosureList { records } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected disclosure list".to_owned(),
            });
        };
        Ok(records)
    }

    /// Read the honest FHIR support matrix; this does not claim full conformance.
    pub fn fhir_support_matrix(
        &mut self,
    ) -> Result<medscale_contracts::fhir::FhirSupportMatrix, AuthorityError> {
        let resp = self.dispatch(
            Capability::GetFhirSupportMatrix,
            RequestBody::GetFhirSupportMatrix,
        )?;
        let ResponseBody::FhirSupportMatrix { matrix } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected FHIR support matrix".to_owned(),
            });
        };
        Ok(matrix)
    }

    // ----- Spec 074 project graph helpers (CLI + Desktop share these) -----

    fn expect_project(
        resp: ResponseBody,
    ) -> Result<medscale_contracts::project_graph::Project, AuthorityError> {
        let ResponseBody::Project { project } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected project".to_owned(),
            });
        };
        Ok(project)
    }

    fn expect_experiment(
        resp: ResponseBody,
    ) -> Result<medscale_contracts::project_graph::Experiment, AuthorityError> {
        let ResponseBody::Experiment { experiment } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected experiment".to_owned(),
            });
        };
        Ok(experiment)
    }

    /// Creates a Project through Core authority.
    pub fn project_create(
        &mut self,
        name: String,
        description: Option<String>,
    ) -> Result<medscale_contracts::project_graph::Project, AuthorityError> {
        let resp = self.dispatch(
            Capability::ProjectCreate,
            RequestBody::ProjectCreate { name, description },
        )?;
        Self::expect_project(resp)
    }

    /// Reads one Project through Core authority.
    pub fn project_get(
        &mut self,
        project_id: OpaqueId,
    ) -> Result<medscale_contracts::project_graph::Project, AuthorityError> {
        let resp = self.dispatch(
            Capability::ProjectRead,
            RequestBody::ProjectGet { project_id },
        )?;
        Self::expect_project(resp)
    }

    /// Lists Projects in the session scope with summaries.
    pub fn project_list(
        &mut self,
        status: Option<medscale_contracts::project_graph::ProjectStatus>,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<
        (
            Vec<medscale_contracts::project_graph::ProjectSummary>,
            Option<String>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::ProjectRead,
            RequestBody::ProjectList {
                status,
                limit,
                cursor,
            },
        )?;
        let ResponseBody::ProjectList {
            projects,
            next_cursor,
        } = resp
        else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected project list".to_owned(),
            });
        };
        Ok((projects, next_cursor))
    }

    /// Updates Project metadata through Core authority.
    pub fn project_update(
        &mut self,
        project_id: OpaqueId,
        expected_revision: u64,
        name: Option<String>,
        description: Option<Option<String>>,
    ) -> Result<medscale_contracts::project_graph::Project, AuthorityError> {
        let resp = self.dispatch(
            Capability::ProjectUpdate,
            RequestBody::ProjectUpdate {
                project_id,
                expected_revision,
                name,
                description,
            },
        )?;
        Self::expect_project(resp)
    }

    /// Archives a Project (references never cascade).
    pub fn project_archive(
        &mut self,
        project_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<medscale_contracts::project_graph::Project, AuthorityError> {
        let resp = self.dispatch(
            Capability::ProjectArchive,
            RequestBody::ProjectArchive {
                project_id,
                expected_revision,
            },
        )?;
        Self::expect_project(resp)
    }

    /// Restores an archived Project.
    pub fn project_restore(
        &mut self,
        project_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<medscale_contracts::project_graph::Project, AuthorityError> {
        let resp = self.dispatch(
            Capability::ProjectArchive,
            RequestBody::ProjectRestore {
                project_id,
                expected_revision,
            },
        )?;
        Self::expect_project(resp)
    }

    /// Reads the Project summary through Core authority.
    pub fn project_summary(
        &mut self,
        project_id: OpaqueId,
    ) -> Result<medscale_contracts::project_graph::ProjectSummary, AuthorityError> {
        let resp = self.dispatch(
            Capability::ProjectRead,
            RequestBody::ProjectSummaryQuery { project_id },
        )?;
        let ResponseBody::ProjectSummary { summary } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected project summary".to_owned(),
            });
        };
        Ok(summary)
    }

    /// Resolves the bounded Project context through Core authority.
    pub fn project_context(
        &mut self,
        project_id: OpaqueId,
        experiment_id: Option<OpaqueId>,
        refs_limit: Option<u32>,
        graph_limit: Option<u32>,
    ) -> Result<medscale_contracts::project_graph::ProjectContext, AuthorityError> {
        let resp = self.dispatch(
            Capability::ProjectRead,
            RequestBody::ProjectContextResolve {
                project_id,
                experiment_id,
                refs_limit,
                graph_limit,
            },
        )?;
        let ResponseBody::ProjectContext { context } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected project context".to_owned(),
            });
        };
        Ok(context)
    }

    /// Creates an Experiment through Core authority.
    pub fn experiment_create(
        &mut self,
        project_id: OpaqueId,
        name: String,
        description: Option<String>,
    ) -> Result<medscale_contracts::project_graph::Experiment, AuthorityError> {
        let resp = self.dispatch(
            Capability::ExperimentCreate,
            RequestBody::ExperimentCreate {
                project_id,
                name,
                description,
            },
        )?;
        Self::expect_experiment(resp)
    }

    /// Reads one Experiment through Core authority.
    pub fn experiment_get(
        &mut self,
        experiment_id: OpaqueId,
    ) -> Result<medscale_contracts::project_graph::Experiment, AuthorityError> {
        let resp = self.dispatch(
            Capability::ExperimentRead,
            RequestBody::ExperimentGet { experiment_id },
        )?;
        Self::expect_experiment(resp)
    }

    /// Lists Experiments of one Project.
    pub fn experiment_list(
        &mut self,
        project_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<
        (
            Vec<medscale_contracts::project_graph::ExperimentSummary>,
            Option<String>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::ExperimentRead,
            RequestBody::ExperimentList {
                project_id,
                limit,
                cursor,
            },
        )?;
        let ResponseBody::ExperimentList {
            experiments,
            next_cursor,
        } = resp
        else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected experiment list".to_owned(),
            });
        };
        Ok((experiments, next_cursor))
    }

    /// Updates Experiment metadata through Core authority.
    pub fn experiment_update(
        &mut self,
        experiment_id: OpaqueId,
        expected_revision: u64,
        name: Option<String>,
        description: Option<Option<String>>,
    ) -> Result<medscale_contracts::project_graph::Experiment, AuthorityError> {
        let resp = self.dispatch(
            Capability::ExperimentUpdate,
            RequestBody::ExperimentUpdate {
                experiment_id,
                expected_revision,
                name,
                description,
            },
        )?;
        Self::expect_experiment(resp)
    }

    /// Archives an Experiment (referenced artifacts untouched).
    pub fn experiment_archive(
        &mut self,
        experiment_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<medscale_contracts::project_graph::Experiment, AuthorityError> {
        let resp = self.dispatch(
            Capability::ExperimentArchive,
            RequestBody::ExperimentArchive {
                experiment_id,
                expected_revision,
            },
        )?;
        Self::expect_experiment(resp)
    }

    /// Attaches an artifact reference through Core authority.
    pub fn project_attach(
        &mut self,
        project_id: OpaqueId,
        experiment_id: Option<OpaqueId>,
        artifact: medscale_contracts::project_graph::ArtifactDescriptor,
    ) -> Result<medscale_contracts::project_graph::ProjectArtifactRef, AuthorityError> {
        let resp = self.dispatch(
            Capability::ProjectArtifactAttach,
            RequestBody::ProjectAttach {
                project_id,
                experiment_id,
                artifact,
            },
        )?;
        let ResponseBody::ProjectRef { reference } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected project ref".to_owned(),
            });
        };
        Ok(reference)
    }

    /// Detaches an artifact reference (tombstone; canonical untouched).
    pub fn project_detach(
        &mut self,
        ref_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<medscale_contracts::project_graph::ProjectArtifactRef, AuthorityError> {
        let resp = self.dispatch(
            Capability::ProjectArtifactDetach,
            RequestBody::ProjectDetach {
                ref_id,
                expected_revision,
            },
        )?;
        let ResponseBody::ProjectRef { reference } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected project ref".to_owned(),
            });
        };
        Ok(reference)
    }

    /// Lists resolved artifact references through Core authority.
    pub fn project_list_refs(
        &mut self,
        project_id: OpaqueId,
        experiment_id: Option<OpaqueId>,
        active_only: bool,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<
        (
            Vec<medscale_contracts::project_graph::ResolvedArtifactRef>,
            Option<String>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::ProjectRead,
            RequestBody::ProjectListRefs {
                project_id,
                experiment_id,
                active_only,
                limit,
                cursor,
            },
        )?;
        let ResponseBody::ProjectRefList { refs, next_cursor } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected project ref list".to_owned(),
            });
        };
        Ok((refs, next_cursor))
    }

    /// Creates a typed graph edge through Core authority.
    pub fn graph_edge_create(
        &mut self,
        project_id: OpaqueId,
        subject: medscale_contracts::project_graph::GraphEndpoint,
        predicate: medscale_contracts::project_graph::ProjectGraphPredicate,
        object: medscale_contracts::project_graph::GraphEndpoint,
    ) -> Result<medscale_contracts::project_graph::ProjectGraphEdge, AuthorityError> {
        let resp = self.dispatch(
            Capability::ProjectGraphMutate,
            RequestBody::GraphEdgeCreate {
                project_id,
                subject,
                predicate,
                object,
            },
        )?;
        let ResponseBody::GraphEdge { edge } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected graph edge".to_owned(),
            });
        };
        Ok(edge)
    }

    /// Removes a graph edge (endpoints untouched).
    pub fn graph_edge_remove(
        &mut self,
        edge_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<medscale_contracts::project_graph::ProjectGraphEdge, AuthorityError> {
        let resp = self.dispatch(
            Capability::ProjectGraphMutate,
            RequestBody::GraphEdgeRemove {
                edge_id,
                expected_revision,
            },
        )?;
        let ResponseBody::GraphEdge { edge } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected graph edge".to_owned(),
            });
        };
        Ok(edge)
    }

    /// Runs a bounded neighbor query through Core authority.
    pub fn graph_neighbors(
        &mut self,
        project_id: OpaqueId,
        start: medscale_contracts::project_graph::GraphEndpoint,
        predicates: Option<Vec<medscale_contracts::project_graph::ProjectGraphPredicate>>,
        direction: Option<medscale_contracts::project_graph::GraphDirection>,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<medscale_contracts::project_graph::GraphNeighborPage, AuthorityError> {
        let resp = self.dispatch(
            Capability::ProjectGraphRead,
            RequestBody::GraphNeighbors {
                project_id,
                start,
                predicates,
                direction,
                limit,
                cursor,
            },
        )?;
        let ResponseBody::GraphNeighbors { page } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected graph neighbors".to_owned(),
            });
        };
        Ok(page)
    }

    /// Creates a data source through Core authority.
    pub fn data_source_create(
        &mut self,
        project_id: OpaqueId,
        display_name: String,
        locator: medscale_contracts::data_sources::SourceLocator,
        credential_ref: Option<OpaqueId>,
    ) -> Result<medscale_contracts::data_sources::DataSourceManifest, AuthorityError> {
        let resp = self.dispatch(
            Capability::DataSourceCreate,
            RequestBody::DataSourceCreate {
                project_id,
                display_name,
                locator,
                credential_ref,
            },
        )?;
        Self::expect_data_source(resp)
    }

    /// Reads one data source through Core authority.
    pub fn data_source_get(
        &mut self,
        source_id: OpaqueId,
    ) -> Result<medscale_contracts::data_sources::DataSourceManifest, AuthorityError> {
        let resp = self.dispatch(
            Capability::DataSourceRead,
            RequestBody::DataSourceGet { source_id },
        )?;
        Self::expect_data_source(resp)
    }

    /// Lists data sources for one project through Core authority.
    pub fn data_source_list(
        &mut self,
        project_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<
        (
            Vec<medscale_contracts::data_sources::DataSourceSummary>,
            Option<String>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::DataSourceRead,
            RequestBody::DataSourceList {
                project_id,
                limit,
                cursor,
            },
        )?;
        let ResponseBody::DataSourceList {
            sources,
            next_cursor,
        } = resp
        else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected data source list".to_owned(),
            });
        };
        Ok((sources, next_cursor))
    }

    /// Updates data-source metadata through Core authority.
    pub fn data_source_update(
        &mut self,
        source_id: OpaqueId,
        expected_revision: u64,
        display_name: Option<String>,
        credential_ref: Option<Option<OpaqueId>>,
    ) -> Result<medscale_contracts::data_sources::DataSourceManifest, AuthorityError> {
        let resp = self.dispatch(
            Capability::DataSourceUpdate,
            RequestBody::DataSourceUpdate {
                source_id,
                expected_revision,
                display_name,
                credential_ref,
            },
        )?;
        Self::expect_data_source(resp)
    }

    /// Archives a data source (snapshots and lineage are retained).
    pub fn data_source_archive(
        &mut self,
        source_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<medscale_contracts::data_sources::DataSourceManifest, AuthorityError> {
        let resp = self.dispatch(
            Capability::DataSourceArchive,
            RequestBody::DataSourceArchive {
                source_id,
                expected_revision,
            },
        )?;
        Self::expect_data_source(resp)
    }

    /// Imports current source bytes into an immutable snapshot.
    pub fn snapshot_import(
        &mut self,
        source_id: OpaqueId,
    ) -> Result<
        (
            medscale_contracts::data_sources::DataSnapshot,
            medscale_contracts::data_sources::ImportReceipt,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::SnapshotImport,
            RequestBody::SnapshotImport { source_id },
        )?;
        let ResponseBody::SnapshotImported { snapshot, receipt } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected imported snapshot".to_owned(),
            });
        };
        Ok((*snapshot, receipt))
    }

    /// Previews source schema plus leading rows without persisting.
    pub fn snapshot_preview(
        &mut self,
        source_id: OpaqueId,
        max_rows: Option<u32>,
    ) -> Result<
        (
            medscale_contracts::data_sources::SourceSchema,
            Vec<Vec<medscale_contracts::data_sources::CellValue>>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::SnapshotPreview,
            RequestBody::SnapshotPreview {
                source_id,
                max_rows,
            },
        )?;
        let ResponseBody::SnapshotPreview { schema, rows } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected snapshot preview".to_owned(),
            });
        };
        Ok((schema, rows))
    }

    /// Reads one snapshot record through Core authority.
    pub fn snapshot_get(
        &mut self,
        snapshot_id: OpaqueId,
    ) -> Result<medscale_contracts::data_sources::DataSnapshot, AuthorityError> {
        let resp = self.dispatch(
            Capability::SnapshotRead,
            RequestBody::SnapshotGet { snapshot_id },
        )?;
        let ResponseBody::Snapshot { snapshot } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected snapshot".to_owned(),
            });
        };
        Ok(*snapshot)
    }

    /// Lists snapshots for one source through Core authority.
    pub fn snapshot_list(
        &mut self,
        source_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<
        (
            Vec<medscale_contracts::data_sources::SnapshotSummary>,
            Option<String>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::SnapshotRead,
            RequestBody::SnapshotList {
                source_id,
                limit,
                cursor,
            },
        )?;
        let ResponseBody::SnapshotList {
            snapshots,
            next_cursor,
        } = resp
        else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected snapshot list".to_owned(),
            });
        };
        Ok((snapshots, next_cursor))
    }

    /// Queries snapshot rows with optional filters/sort and stable paging.
    pub fn snapshot_rows(
        &mut self,
        snapshot_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
        filters: Vec<medscale_contracts::data_sources::FilterExpr>,
        sort: Vec<medscale_contracts::data_sources::SortKey>,
    ) -> Result<medscale_contracts::data_sources::SnapshotRowPage, AuthorityError> {
        let resp = self.dispatch(
            Capability::SnapshotRead,
            RequestBody::SnapshotRows {
                snapshot_id,
                limit,
                cursor,
                filters,
                sort,
            },
        )?;
        let ResponseBody::SnapshotRows { page } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected snapshot rows".to_owned(),
            });
        };
        Ok(page)
    }

    /// Refreshes a source against current external state.
    pub fn snapshot_refresh(
        &mut self,
        source_id: OpaqueId,
        allow_schema_change: bool,
    ) -> Result<
        (
            medscale_contracts::data_sources::RefreshReceipt,
            Option<medscale_contracts::data_sources::DataSnapshot>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::SnapshotRefresh,
            RequestBody::SnapshotRefresh {
                source_id,
                allow_schema_change,
            },
        )?;
        let ResponseBody::SnapshotRefreshed { receipt, snapshot } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected refresh receipt".to_owned(),
            });
        };
        Ok((receipt, snapshot.map(|boxed| *boxed)))
    }

    /// Creates one saved view through Core authority.
    pub fn saved_view_create(
        &mut self,
        snapshot_id: OpaqueId,
        view_kind: medscale_contracts::data_sources::DataViewKind,
        state: medscale_contracts::data_sources::ViewState,
    ) -> Result<medscale_contracts::data_sources::SavedDataView, AuthorityError> {
        let resp = self.dispatch(
            Capability::SavedViewCreate,
            RequestBody::SavedViewCreate {
                snapshot_id,
                view_kind,
                state,
            },
        )?;
        Self::expect_saved_view(resp)
    }

    /// Reads one saved view through Core authority.
    pub fn saved_view_get(
        &mut self,
        view_id: OpaqueId,
    ) -> Result<medscale_contracts::data_sources::SavedDataView, AuthorityError> {
        let resp = self.dispatch(
            Capability::SavedViewRead,
            RequestBody::SavedViewGet { view_id },
        )?;
        Self::expect_saved_view(resp)
    }

    /// Lists saved views for one snapshot through Core authority.
    pub fn saved_view_list(
        &mut self,
        snapshot_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<
        (
            Vec<medscale_contracts::data_sources::SavedViewSummary>,
            Option<String>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::SavedViewRead,
            RequestBody::SavedViewList {
                snapshot_id,
                limit,
                cursor,
            },
        )?;
        let ResponseBody::SavedViewList { views, next_cursor } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected saved view list".to_owned(),
            });
        };
        Ok((views, next_cursor))
    }

    /// Updates one saved view state through Core authority.
    pub fn saved_view_update(
        &mut self,
        view_id: OpaqueId,
        expected_revision: u64,
        state: medscale_contracts::data_sources::ViewState,
    ) -> Result<medscale_contracts::data_sources::SavedDataView, AuthorityError> {
        let resp = self.dispatch(
            Capability::SavedViewUpdate,
            RequestBody::SavedViewUpdate {
                view_id,
                expected_revision,
                state,
            },
        )?;
        Self::expect_saved_view(resp)
    }

    /// Executes one deterministic transformation through Core authority.
    pub fn transform_execute(
        &mut self,
        input_snapshot_ids: Vec<OpaqueId>,
        ops: Vec<medscale_contracts::data_sources::TransformOp>,
    ) -> Result<
        (
            medscale_contracts::data_sources::DataSnapshot,
            medscale_contracts::data_sources::TransformationReceipt,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::TransformExecute,
            RequestBody::TransformExecute {
                input_snapshot_ids,
                ops,
            },
        )?;
        let ResponseBody::Transformed { snapshot, receipt } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected transformed snapshot".to_owned(),
            });
        };
        Ok((*snapshot, receipt))
    }

    /// Creates one dataset release through Core authority.
    pub fn dataset_release_create(
        &mut self,
        snapshot_id: OpaqueId,
        version: String,
        split_group: Option<String>,
        annotation_schema_ref: Option<String>,
        rights_state: medscale_contracts::data_sources::RightsState,
    ) -> Result<medscale_contracts::data_sources::ReleaseManifest, AuthorityError> {
        let resp = self.dispatch(
            Capability::DatasetReleaseCreate,
            RequestBody::DatasetReleaseCreate {
                snapshot_id,
                version,
                split_group,
                annotation_schema_ref,
                rights_state,
            },
        )?;
        let ResponseBody::DatasetRelease { release } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected dataset release".to_owned(),
            });
        };
        Ok(*release)
    }

    /// Reads one dataset release through Core authority.
    pub fn dataset_release_get(
        &mut self,
        release_id: OpaqueId,
    ) -> Result<medscale_contracts::data_sources::ReleaseManifest, AuthorityError> {
        let resp = self.dispatch(
            Capability::DatasetReleaseRead,
            RequestBody::DatasetReleaseGet { release_id },
        )?;
        let ResponseBody::DatasetRelease { release } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected dataset release".to_owned(),
            });
        };
        Ok(*release)
    }

    /// Lists dataset releases for one project through Core authority.
    pub fn dataset_release_list(
        &mut self,
        project_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<
        (
            Vec<medscale_contracts::data_sources::DatasetReleaseSummary>,
            Option<String>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::DatasetReleaseRead,
            RequestBody::DatasetReleaseList {
                project_id,
                limit,
                cursor,
            },
        )?;
        let ResponseBody::DatasetReleaseList {
            releases,
            next_cursor,
        } = resp
        else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected dataset release list".to_owned(),
            });
        };
        Ok((releases, next_cursor))
    }

    fn expect_data_source(
        resp: ResponseBody,
    ) -> Result<medscale_contracts::data_sources::DataSourceManifest, AuthorityError> {
        let ResponseBody::DataSource { source } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected data source".to_owned(),
            });
        };
        Ok(*source)
    }

    fn expect_saved_view(
        resp: ResponseBody,
    ) -> Result<medscale_contracts::data_sources::SavedDataView, AuthorityError> {
        let ResponseBody::SavedView { view } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected saved view".to_owned(),
            });
        };
        Ok(*view)
    }

    // ---------------------------------------------------------------------
    // Spec 076: Collaboration Substrate
    // ---------------------------------------------------------------------

    /// Registers a participant, or returns the existing one if already
    /// registered under the same `holder_id` (idempotent).
    pub fn collab_participant_register(
        &mut self,
        holder_id: OpaqueId,
        kind: medscale_contracts::collaboration::ParticipantKind,
        display_name: String,
        agent_profile_ref: Option<OpaqueId>,
    ) -> Result<medscale_contracts::collaboration::ParticipantIdentity, AuthorityError> {
        let resp = self.dispatch(
            Capability::ParticipantRegister,
            RequestBody::ParticipantRegister {
                holder_id,
                kind,
                display_name,
                agent_profile_ref,
            },
        )?;
        Self::expect_participant(resp)
    }

    /// Reads one participant through Core authority.
    pub fn collab_participant_get(
        &mut self,
        participant_id: OpaqueId,
    ) -> Result<medscale_contracts::collaboration::ParticipantIdentity, AuthorityError> {
        let resp = self.dispatch(
            Capability::ParticipantRead,
            RequestBody::ParticipantGet { participant_id },
        )?;
        Self::expect_participant(resp)
    }

    /// Revokes a participant (status only; `kind`/`holder_id` never change).
    pub fn collab_participant_revoke(
        &mut self,
        participant_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<medscale_contracts::collaboration::ParticipantIdentity, AuthorityError> {
        let resp = self.dispatch(
            Capability::ParticipantRevoke,
            RequestBody::ParticipantRevoke {
                participant_id,
                expected_revision,
            },
        )?;
        Self::expect_participant(resp)
    }

    // ---------------------------------------------------------------------
    // Spec 077: MedAgent Workbench (T077-03 slice)
    // ---------------------------------------------------------------------

    /// Registers a new `AgentIdentity` bound to `pack_id` (must already be
    /// admitted locally; a non-admitted pack fails closed).
    pub fn medagent_identity_register(
        &mut self,
        project_id: OpaqueId,
        pack_id: OpaqueId,
        display_name: String,
        granted_tool_kinds: Vec<medscale_contracts::medagent::ToolKind>,
    ) -> Result<
        (
            medscale_contracts::medagent::AgentIdentity,
            medscale_contracts::medagent::AgentCapabilityManifest,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::AgentIdentityRegister,
            RequestBody::AgentIdentityRegister {
                project_id,
                pack_id,
                display_name,
                granted_tool_kinds,
            },
        )?;
        Self::expect_medagent_identity(resp)
    }

    /// Reads one agent identity + its capability manifest through Core.
    pub fn medagent_identity_get(
        &mut self,
        agent_id: OpaqueId,
    ) -> Result<
        (
            medscale_contracts::medagent::AgentIdentity,
            medscale_contracts::medagent::AgentCapabilityManifest,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::AgentIdentityRead,
            RequestBody::AgentIdentityGet { agent_id },
        )?;
        Self::expect_medagent_identity(resp)
    }

    /// Lists agent identities in one Project.
    pub fn medagent_identity_list(
        &mut self,
        project_id: OpaqueId,
        limit: Option<u32>,
    ) -> Result<Vec<medscale_contracts::medagent::AgentIdentity>, AuthorityError> {
        let resp = self.dispatch(
            Capability::AgentIdentityRead,
            RequestBody::AgentIdentityList { project_id, limit },
        )?;
        let ResponseBody::MedAgentIdentityList { identities } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected medagent identity list".to_owned(),
            });
        };
        Ok(identities)
    }

    /// Revokes an agent identity (status only).
    pub fn medagent_identity_revoke(
        &mut self,
        agent_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<
        (
            medscale_contracts::medagent::AgentIdentity,
            medscale_contracts::medagent::AgentCapabilityManifest,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::AgentIdentityRevoke,
            RequestBody::AgentIdentityRevoke {
                agent_id,
                expected_revision,
            },
        )?;
        Self::expect_medagent_identity(resp)
    }

    /// Creates a new `ContextManifest` from an explicit artifact list.
    pub fn medagent_context_create(
        &mut self,
        project_id: OpaqueId,
        selected_artifacts: Vec<medscale_contracts::project_graph::ArtifactDescriptor>,
    ) -> Result<
        (
            medscale_contracts::medagent::ContextManifest,
            Vec<medscale_contracts::project_graph::ReferenceResolution>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::ContextManifestCreate,
            RequestBody::ContextManifestCreate {
                project_id,
                selected_artifacts,
            },
        )?;
        Self::expect_medagent_context(resp)
    }

    /// Reads one `ContextManifest`, with every selected artifact's
    /// `ReferenceResolution` recomputed live.
    pub fn medagent_context_get(
        &mut self,
        context_id: OpaqueId,
    ) -> Result<
        (
            medscale_contracts::medagent::ContextManifest,
            Vec<medscale_contracts::project_graph::ReferenceResolution>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::ContextManifestRead,
            RequestBody::ContextManifestGet { context_id },
        )?;
        Self::expect_medagent_context(resp)
    }

    fn expect_medagent_context(
        resp: ResponseBody,
    ) -> Result<
        (
            medscale_contracts::medagent::ContextManifest,
            Vec<medscale_contracts::project_graph::ReferenceResolution>,
        ),
        AuthorityError,
    > {
        let ResponseBody::MedAgentContextManifest {
            manifest,
            resolutions,
        } = resp
        else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected medagent context manifest".to_owned(),
            });
        };
        Ok((*manifest, resolutions))
    }

    /// Creates a new `Pending` `AgentRun`.
    pub fn medagent_run_create(
        &mut self,
        project_id: OpaqueId,
        agent_identity_id: OpaqueId,
        context_manifest_id: OpaqueId,
        prompt: String,
    ) -> Result<medscale_contracts::medagent::AgentRun, AuthorityError> {
        let resp = self.dispatch(
            Capability::AgentRunCreate,
            RequestBody::AgentRunCreate {
                project_id,
                agent_identity_id,
                context_manifest_id,
                prompt,
            },
        )?;
        Self::expect_medagent_run(resp)
    }

    /// Reads one `AgentRun`.
    pub fn medagent_run_get(
        &mut self,
        run_id: OpaqueId,
    ) -> Result<medscale_contracts::medagent::AgentRun, AuthorityError> {
        let resp = self.dispatch(
            Capability::AgentRunRead,
            RequestBody::AgentRunGet { run_id },
        )?;
        Self::expect_medagent_run(resp)
    }

    /// Lists runs in one Project, optionally filtered by agent identity.
    pub fn medagent_run_list(
        &mut self,
        project_id: OpaqueId,
        agent_id: Option<OpaqueId>,
        limit: Option<u32>,
    ) -> Result<Vec<medscale_contracts::medagent::AgentRun>, AuthorityError> {
        let resp = self.dispatch(
            Capability::AgentRunRead,
            RequestBody::AgentRunList {
                project_id,
                agent_id,
                limit,
            },
        )?;
        let ResponseBody::MedAgentRunList { runs } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected medagent run list".to_owned(),
            });
        };
        Ok(runs)
    }

    /// Starts a `Pending` run (`Pending -> Running`), returning the run
    /// plus its auto-appended initial `PromptSubmitted` turn.
    pub fn medagent_run_start(
        &mut self,
        run_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<
        (
            medscale_contracts::medagent::AgentRun,
            medscale_contracts::medagent::AgentTurn,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::AgentRunStart,
            RequestBody::AgentRunStart {
                run_id,
                expected_revision,
            },
        )?;
        let ResponseBody::MedAgentRunStarted { run, turn } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected medagent run started".to_owned(),
            });
        };
        Ok((*run, *turn))
    }

    /// Cancels a `Pending` or `Running` run, returning it plus its
    /// committed `RunReceipt`.
    pub fn medagent_run_cancel(
        &mut self,
        run_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<
        (
            medscale_contracts::medagent::AgentRun,
            medscale_contracts::medagent::RunReceipt,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::AgentRunCancel,
            RequestBody::AgentRunCancel {
                run_id,
                expected_revision,
            },
        )?;
        let ResponseBody::MedAgentRunTerminal { run, receipt } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected medagent run terminal".to_owned(),
            });
        };
        Ok((*run, *receipt))
    }

    /// Lists a run's turns in `seq` order.
    pub fn medagent_run_turns(
        &mut self,
        run_id: OpaqueId,
    ) -> Result<Vec<medscale_contracts::medagent::AgentTurn>, AuthorityError> {
        let resp = self.dispatch(
            Capability::AgentRunRead,
            RequestBody::AgentRunTurnList { run_id },
        )?;
        let ResponseBody::MedAgentTurnList { turns } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected medagent turn list".to_owned(),
            });
        };
        Ok(turns)
    }

    fn expect_medagent_run(
        resp: ResponseBody,
    ) -> Result<medscale_contracts::medagent::AgentRun, AuthorityError> {
        let ResponseBody::MedAgentRun { run } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected medagent run".to_owned(),
            });
        };
        Ok(*run)
    }

    /// Dispatches one typed tool invocation for a `Running` run.
    pub fn medagent_tool_invoke(
        &mut self,
        run_id: OpaqueId,
        kind: medscale_contracts::medagent::ToolKind,
        arguments: serde_json::Value,
    ) -> Result<
        (
            medscale_contracts::medagent::ToolInvocation,
            Option<medscale_contracts::medagent::ToolReceipt>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::AgentToolInvoke,
            RequestBody::AgentToolInvoke {
                run_id,
                kind,
                arguments,
            },
        )?;
        let ResponseBody::MedAgentToolInvocation {
            invocation,
            receipt,
        } = resp
        else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected medagent tool invocation".to_owned(),
            });
        };
        Ok((*invocation, receipt.map(|r| *r)))
    }

    /// Runs a `Running` run's prompt through its bound admitted local
    /// model Pack (zero network) and persists the result as an
    /// `AgentProposal`.
    pub fn medagent_run_execute(
        &mut self,
        run_id: OpaqueId,
        local_path: String,
        max_tokens: u32,
        synthetic_only: bool,
    ) -> Result<
        (
            medscale_contracts::medagent::AgentTurn,
            medscale_contracts::medagent::AgentProposal,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::AgentRunExecute,
            RequestBody::AgentRunExecute {
                run_id,
                local_path,
                max_tokens,
                synthetic_only,
            },
        )?;
        let ResponseBody::MedAgentRunExecuted { turn, proposal } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected medagent run executed".to_owned(),
            });
        };
        Ok((*turn, *proposal))
    }

    fn expect_medagent_identity(
        resp: ResponseBody,
    ) -> Result<
        (
            medscale_contracts::medagent::AgentIdentity,
            medscale_contracts::medagent::AgentCapabilityManifest,
        ),
        AuthorityError,
    > {
        let ResponseBody::MedAgentIdentity {
            identity,
            capabilities,
        } = resp
        else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected medagent identity".to_owned(),
            });
        };
        Ok((*identity, *capabilities))
    }

    fn expect_participant(
        resp: ResponseBody,
    ) -> Result<medscale_contracts::collaboration::ParticipantIdentity, AuthorityError> {
        let ResponseBody::Participant { participant } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected participant".to_owned(),
            });
        };
        Ok(*participant)
    }

    /// Creates a Room scoped to an existing Project; grants the caller
    /// `Owner` membership.
    pub fn collab_room_create(
        &mut self,
        project_id: OpaqueId,
        experiment_id: Option<OpaqueId>,
        name: String,
    ) -> Result<medscale_contracts::collaboration::Room, AuthorityError> {
        let resp = self.dispatch(
            Capability::RoomCreate,
            RequestBody::RoomCreate {
                project_id,
                experiment_id,
                name,
            },
        )?;
        Self::expect_room(resp)
    }

    /// Reads one Room through Core authority (membership-gated).
    pub fn collab_room_get(
        &mut self,
        room_id: OpaqueId,
    ) -> Result<medscale_contracts::collaboration::Room, AuthorityError> {
        let resp = self.dispatch(Capability::RoomRead, RequestBody::RoomGet { room_id })?;
        Self::expect_room(resp)
    }

    /// Lists Rooms in one Project that the caller is an active member of.
    pub fn collab_room_list(
        &mut self,
        project_id: OpaqueId,
        status: Option<medscale_contracts::collaboration::RoomStatus>,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<(Vec<medscale_contracts::collaboration::Room>, Option<String>), AuthorityError>
    {
        let resp = self.dispatch(
            Capability::RoomRead,
            RequestBody::RoomList {
                project_id,
                status,
                limit,
                cursor,
            },
        )?;
        let ResponseBody::CollabRoomList { rooms, next_cursor } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected room list".to_owned(),
            });
        };
        Ok((rooms, next_cursor))
    }

    /// Renames a Room (revision-guarded).
    pub fn collab_room_rename(
        &mut self,
        room_id: OpaqueId,
        expected_revision: u64,
        name: String,
    ) -> Result<medscale_contracts::collaboration::Room, AuthorityError> {
        let resp = self.dispatch(
            Capability::RoomUpdate,
            RequestBody::RoomRename {
                room_id,
                expected_revision,
                name,
            },
        )?;
        Self::expect_room(resp)
    }

    /// Archives a Room (revision-guarded; `Owner` role required).
    pub fn collab_room_archive(
        &mut self,
        room_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<medscale_contracts::collaboration::Room, AuthorityError> {
        let resp = self.dispatch(
            Capability::RoomArchive,
            RequestBody::RoomArchive {
                room_id,
                expected_revision,
            },
        )?;
        Self::expect_room(resp)
    }

    fn expect_room(
        resp: ResponseBody,
    ) -> Result<medscale_contracts::collaboration::Room, AuthorityError> {
        let ResponseBody::CollabRoom { room, .. } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected room".to_owned(),
            });
        };
        Ok(*room)
    }

    /// Adds a member to a Room (membership-gated: caller must already be an
    /// active member).
    pub fn collab_membership_add(
        &mut self,
        room_id: OpaqueId,
        participant_id: OpaqueId,
        role: medscale_contracts::collaboration::MembershipRole,
    ) -> Result<medscale_contracts::collaboration::RoomMembership, AuthorityError> {
        let resp = self.dispatch(
            Capability::RoomMembershipManage,
            RequestBody::RoomMembershipAdd {
                room_id,
                participant_id,
                role,
            },
        )?;
        Self::expect_membership(resp)
    }

    /// Lists active memberships for one Room.
    pub fn collab_membership_list(
        &mut self,
        room_id: OpaqueId,
    ) -> Result<Vec<medscale_contracts::collaboration::RoomMembership>, AuthorityError> {
        let resp = self.dispatch(
            Capability::RoomMembershipRead,
            RequestBody::RoomMembershipList { room_id },
        )?;
        let ResponseBody::CollabMembershipList { memberships } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected membership list".to_owned(),
            });
        };
        Ok(memberships)
    }

    /// Removes a member (a member may remove themself; removing another
    /// requires `Owner` role).
    pub fn collab_membership_remove(
        &mut self,
        room_id: OpaqueId,
        membership_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<medscale_contracts::collaboration::RoomMembership, AuthorityError> {
        let resp = self.dispatch(
            Capability::RoomMembershipManage,
            RequestBody::RoomMembershipRemove {
                room_id,
                membership_id,
                expected_revision,
            },
        )?;
        Self::expect_membership(resp)
    }

    fn expect_membership(
        resp: ResponseBody,
    ) -> Result<medscale_contracts::collaboration::RoomMembership, AuthorityError> {
        let ResponseBody::CollabMembership { membership } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected membership".to_owned(),
            });
        };
        Ok(*membership)
    }

    /// Opens a thread anchored to an exact artifact revision (membership-
    /// gated). Returns the thread plus its live `ReferenceResolution`.
    pub fn collab_thread_open(
        &mut self,
        room_id: OpaqueId,
        anchor: medscale_contracts::collaboration::AnchorTarget,
    ) -> Result<
        (
            medscale_contracts::collaboration::ThreadRef,
            medscale_contracts::project_graph::ReferenceResolution,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::ThreadCreate,
            RequestBody::ThreadOpen { room_id, anchor },
        )?;
        Self::expect_thread(resp)
    }

    /// Reads one thread with its live `ReferenceResolution`.
    pub fn collab_thread_get(
        &mut self,
        thread_id: OpaqueId,
    ) -> Result<
        (
            medscale_contracts::collaboration::ThreadRef,
            medscale_contracts::project_graph::ReferenceResolution,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(Capability::ThreadRead, RequestBody::ThreadGet { thread_id })?;
        Self::expect_thread(resp)
    }

    /// Lists threads in one room with each thread's live resolution.
    #[allow(clippy::type_complexity)]
    pub fn collab_thread_list(
        &mut self,
        room_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<
        (
            Vec<medscale_contracts::collaboration::ThreadRef>,
            Vec<medscale_contracts::project_graph::ReferenceResolution>,
            Option<String>,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::ThreadRead,
            RequestBody::ThreadList {
                room_id,
                limit,
                cursor,
            },
        )?;
        let ResponseBody::CollabThreadList {
            threads,
            resolutions,
            next_cursor,
        } = resp
        else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected thread list".to_owned(),
            });
        };
        Ok((threads, resolutions, next_cursor))
    }

    /// Transitions a thread's status (`Open<->Resolved<->Reopened` only).
    pub fn collab_thread_set_status(
        &mut self,
        thread_id: OpaqueId,
        expected_revision: u64,
        status: medscale_contracts::collaboration::ThreadStatus,
    ) -> Result<
        (
            medscale_contracts::collaboration::ThreadRef,
            medscale_contracts::project_graph::ReferenceResolution,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::ThreadResolve,
            RequestBody::ThreadSetStatus {
                thread_id,
                expected_revision,
                status,
            },
        )?;
        Self::expect_thread(resp)
    }

    fn expect_thread(
        resp: ResponseBody,
    ) -> Result<
        (
            medscale_contracts::collaboration::ThreadRef,
            medscale_contracts::project_graph::ReferenceResolution,
        ),
        AuthorityError,
    > {
        let ResponseBody::CollabThread { thread, resolution } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected thread".to_owned(),
            });
        };
        Ok((*thread, resolution))
    }

    /// Posts a message to a thread (membership-gated).
    pub fn collab_message_post(
        &mut self,
        thread_id: OpaqueId,
        body: String,
    ) -> Result<medscale_contracts::collaboration::Message, AuthorityError> {
        let resp = self.dispatch(
            Capability::MessagePost,
            RequestBody::MessagePost { thread_id, body },
        )?;
        let ResponseBody::CollabMessage { message } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected message".to_owned(),
            });
        };
        Ok(*message)
    }

    /// Lists messages in one thread, in `seq` order.
    pub fn collab_message_list(
        &mut self,
        thread_id: OpaqueId,
        limit: Option<u32>,
        after_seq: Option<u64>,
    ) -> Result<Vec<medscale_contracts::collaboration::Message>, AuthorityError> {
        let resp = self.dispatch(
            Capability::MessageRead,
            RequestBody::MessageList {
                thread_id,
                limit,
                after_seq,
            },
        )?;
        let ResponseBody::CollabMessageList { messages } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected message list".to_owned(),
            });
        };
        Ok(messages)
    }

    /// Edits a message's body (author-only).
    pub fn collab_message_edit(
        &mut self,
        message_id: OpaqueId,
        new_body: String,
    ) -> Result<medscale_contracts::collaboration::MessageEdit, AuthorityError> {
        let resp = self.dispatch(
            Capability::MessageEdit,
            RequestBody::MessageEditBody {
                message_id,
                new_body,
            },
        )?;
        Self::expect_message_edit(resp)
    }

    /// Deletes a message (append-only tombstone; author-only).
    pub fn collab_message_delete(
        &mut self,
        message_id: OpaqueId,
    ) -> Result<medscale_contracts::collaboration::MessageEdit, AuthorityError> {
        let resp = self.dispatch(
            Capability::MessageEdit,
            RequestBody::MessageDelete { message_id },
        )?;
        Self::expect_message_edit(resp)
    }

    fn expect_message_edit(
        resp: ResponseBody,
    ) -> Result<medscale_contracts::collaboration::MessageEdit, AuthorityError> {
        let ResponseBody::CollabMessageEdit { edit } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected message edit".to_owned(),
            });
        };
        Ok(*edit)
    }

    /// Creates a task, optionally anchored to an artifact (membership-gated).
    pub fn collab_task_create(
        &mut self,
        room_id: OpaqueId,
        anchor: Option<medscale_contracts::collaboration::AnchorTarget>,
        title: String,
        description: Option<String>,
    ) -> Result<medscale_contracts::collaboration::Task, AuthorityError> {
        let resp = self.dispatch(
            Capability::TaskCreate,
            RequestBody::TaskCreate {
                room_id,
                anchor,
                title,
                description,
            },
        )?;
        Self::expect_task(resp)
    }

    /// Reads one task.
    pub fn collab_task_get(
        &mut self,
        task_id: OpaqueId,
    ) -> Result<medscale_contracts::collaboration::Task, AuthorityError> {
        let resp = self.dispatch(Capability::TaskRead, RequestBody::TaskGet { task_id })?;
        Self::expect_task(resp)
    }

    /// Lists tasks in one room.
    pub fn collab_task_list(
        &mut self,
        room_id: OpaqueId,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<(Vec<medscale_contracts::collaboration::Task>, Option<String>), AuthorityError>
    {
        let resp = self.dispatch(
            Capability::TaskRead,
            RequestBody::TaskList {
                room_id,
                limit,
                cursor,
            },
        )?;
        let ResponseBody::CollabTaskList { tasks, next_cursor } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected task list".to_owned(),
            });
        };
        Ok((tasks, next_cursor))
    }

    /// Updates task status/assignee (revision-guarded).
    pub fn collab_task_update(
        &mut self,
        task_id: OpaqueId,
        expected_revision: u64,
        status: medscale_contracts::collaboration::TaskStatus,
        assignee_participant_id: Option<OpaqueId>,
    ) -> Result<medscale_contracts::collaboration::Task, AuthorityError> {
        let resp = self.dispatch(
            Capability::TaskUpdate,
            RequestBody::TaskUpdate {
                task_id,
                expected_revision,
                status,
                assignee_participant_id,
            },
        )?;
        Self::expect_task(resp)
    }

    fn expect_task(
        resp: ResponseBody,
    ) -> Result<medscale_contracts::collaboration::Task, AuthorityError> {
        let ResponseBody::CollabTask { task } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected task".to_owned(),
            });
        };
        Ok(*task)
    }

    /// Creates a note with its initial body (membership-gated).
    pub fn collab_note_create(
        &mut self,
        room_id: OpaqueId,
        title: String,
        body: String,
    ) -> Result<medscale_contracts::collaboration::NoteDocument, AuthorityError> {
        let resp = self.dispatch(
            Capability::NoteCreate,
            RequestBody::NoteCreate {
                room_id,
                title,
                body,
            },
        )?;
        let ResponseBody::CollabNote { note } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected note".to_owned(),
            });
        };
        Ok(*note)
    }

    /// Reads one note document (pointer only; see `collab_note_revisions`
    /// for body history).
    pub fn collab_note_get(
        &mut self,
        note_id: OpaqueId,
    ) -> Result<medscale_contracts::collaboration::NoteDocument, AuthorityError> {
        let resp = self.dispatch(Capability::NoteRead, RequestBody::NoteGet { note_id })?;
        let ResponseBody::CollabNote { note } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected note".to_owned(),
            });
        };
        Ok(*note)
    }

    /// Lists the full revision history of one note, including conflict
    /// copies.
    pub fn collab_note_revisions(
        &mut self,
        note_id: OpaqueId,
    ) -> Result<Vec<medscale_contracts::collaboration::NoteRevision>, AuthorityError> {
        let resp = self.dispatch(
            Capability::NoteRead,
            RequestBody::NoteListRevisions { note_id },
        )?;
        let ResponseBody::CollabNoteRevisionList { revisions } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected note revision list".to_owned(),
            });
        };
        Ok(revisions)
    }

    /// Edits a note. Fast-forward when `expected_revision` matches the
    /// current pointer; otherwise creates an explicit conflict copy that
    /// preserves the caller's content (`is_conflict_copy` in the result).
    pub fn collab_note_edit(
        &mut self,
        note_id: OpaqueId,
        expected_revision: u64,
        body: String,
    ) -> Result<
        (
            medscale_contracts::collaboration::NoteDocument,
            medscale_contracts::collaboration::NoteRevision,
            bool,
        ),
        AuthorityError,
    > {
        let resp = self.dispatch(
            Capability::NoteUpdate,
            RequestBody::NoteEdit {
                note_id,
                expected_revision,
                body,
            },
        )?;
        let ResponseBody::CollabNoteEdit {
            note,
            new_revision,
            is_conflict_copy,
        } = resp
        else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected note edit".to_owned(),
            });
        };
        Ok((*note, *new_revision, is_conflict_copy))
    }

    /// Creates an approval request (membership-gated). Supports multiple
    /// independent assignees and an optional `blind_until_closed` filter.
    pub fn collab_approval_request_create(
        &mut self,
        room_id: OpaqueId,
        anchor: medscale_contracts::collaboration::AnchorTarget,
        kind: medscale_contracts::collaboration::ApprovalKind,
        assignee_participant_ids: Vec<OpaqueId>,
        blind_until_closed: bool,
    ) -> Result<medscale_contracts::collaboration::ApprovalRequest, AuthorityError> {
        let resp = self.dispatch(
            Capability::ApprovalRequestCreate,
            RequestBody::ApprovalRequestCreate {
                room_id,
                anchor,
                kind,
                assignee_participant_ids,
                blind_until_closed,
            },
        )?;
        Self::expect_approval_request(resp)
    }

    /// Reads one approval request.
    pub fn collab_approval_request_get(
        &mut self,
        request_id: OpaqueId,
    ) -> Result<medscale_contracts::collaboration::ApprovalRequest, AuthorityError> {
        let resp = self.dispatch(
            Capability::ApprovalRequestRead,
            RequestBody::ApprovalRequestGet { request_id },
        )?;
        Self::expect_approval_request(resp)
    }

    /// Withdraws an open approval request (requester-only).
    pub fn collab_approval_request_withdraw(
        &mut self,
        request_id: OpaqueId,
        expected_revision: u64,
    ) -> Result<medscale_contracts::collaboration::ApprovalRequest, AuthorityError> {
        let resp = self.dispatch(
            Capability::ApprovalWithdraw,
            RequestBody::ApprovalRequestWithdraw {
                request_id,
                expected_revision,
            },
        )?;
        Self::expect_approval_request(resp)
    }

    fn expect_approval_request(
        resp: ResponseBody,
    ) -> Result<medscale_contracts::collaboration::ApprovalRequest, AuthorityError> {
        let ResponseBody::CollabApprovalRequest { request } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected approval request".to_owned(),
            });
        };
        Ok(*request)
    }

    /// Records a decision against an open approval request (assignee-only).
    /// Never triggers any effect/action/proposal-promotion path
    /// (`security.md` T1).
    pub fn collab_approval_decide(
        &mut self,
        request_id: OpaqueId,
        outcome: medscale_contracts::collaboration::ApprovalDecisionOutcome,
        rationale: Option<String>,
    ) -> Result<medscale_contracts::collaboration::ApprovalDecision, AuthorityError> {
        let resp = self.dispatch(
            Capability::ApprovalDecide,
            RequestBody::ApprovalDecide {
                request_id,
                outcome,
                rationale,
            },
        )?;
        let ResponseBody::CollabApprovalDecision { decision } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected approval decision".to_owned(),
            });
        };
        Ok(*decision)
    }

    /// Lists decisions recorded against one request, filtered by
    /// `blind_until_closed` relative to the caller.
    pub fn collab_approval_decision_list(
        &mut self,
        request_id: OpaqueId,
    ) -> Result<Vec<medscale_contracts::collaboration::ApprovalDecision>, AuthorityError> {
        let resp = self.dispatch(
            Capability::ApprovalRequestRead,
            RequestBody::ApprovalDecisionList { request_id },
        )?;
        let ResponseBody::CollabApprovalDecisionList { decisions } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected approval decision list".to_owned(),
            });
        };
        Ok(decisions)
    }

    /// Lists activity records for one room, in `seq` order (membership-
    /// gated); the same durable, hash-chained log the activity feed and
    /// tamper-evidence check both read.
    pub fn collab_activity_list(
        &mut self,
        room_id: OpaqueId,
        limit: Option<u32>,
        after_seq: Option<u64>,
    ) -> Result<Vec<medscale_contracts::collaboration::ActivityRecord>, AuthorityError> {
        let resp = self.dispatch(
            Capability::ActivityRead,
            RequestBody::ActivityList {
                room_id,
                limit,
                after_seq,
            },
        )?;
        let ResponseBody::CollabActivityList { records } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected activity list".to_owned(),
            });
        };
        Ok(records)
    }
}
