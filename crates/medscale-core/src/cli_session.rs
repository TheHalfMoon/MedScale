//! CLI session: in-process transient Core Host owner (Spec 006).

use medscale_contracts::envelopes::{
    AuthorityError, AuthorityRequest, Capability, RequestBody, ResponseBody,
};
use medscale_contracts::objects::{AuthorityScopeId, OpaqueId, RealmId, VaultId};
use medscale_contracts::presentation::{SubjectBriefV1, SubjectCoverageV1, SubjectTimelineV1};

use crate::CoreFacade;

/// In-process CLI session owning one CoreFacade + lease for a vault id.
pub struct CliSession {
    facade: CoreFacade,
    vault_id: VaultId,
    realm_id: RealmId,
    scope_id: AuthorityScopeId,
    holder_id: OpaqueId,
    next_req: u64,
    vault_root: Option<String>,
    open: bool,
}

impl CliSession {
    /// Acquire lease and create session (vault not yet open).
    pub fn connect(vault_id: &str) -> Result<Self, AuthorityError> {
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
            next_req: 0,
            vault_root: None,
            open: false,
        };
        let resp = session.dispatch(
            Capability::AcquireLease,
            RequestBody::AcquireLease {
                client_id: OpaqueId::new("medscale-cli"),
                holder_id_hint: Some(OpaqueId::new("cli-holder")),
            },
        )?;
        let ResponseBody::Lease { holder_id, .. } = resp else {
            return Err(AuthorityError::InvalidArgument {
                message: "expected lease".to_owned(),
            });
        };
        session.holder_id = holder_id;
        Ok(session)
    }

    fn req_id(&mut self) -> OpaqueId {
        self.next_req += 1;
        OpaqueId::new(format!("cli-req-{}", self.next_req))
    }

    fn dispatch(
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
}
