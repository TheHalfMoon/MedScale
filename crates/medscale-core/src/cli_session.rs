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
}
