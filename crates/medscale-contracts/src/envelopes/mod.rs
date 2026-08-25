//! Versioned authority facade request/response envelopes.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::AUTHORITY_SCHEMA_VERSION;
use crate::objects::{EffectState, OpaqueId, VaultId};

/// Capability required to execute a facade operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    AcquireLease,
    ReleaseLease,
    Ping,
    CreateSourceRecord,
    CreateDerivedArtifact,
    CreateProposal,
    PromoteProposal,
    CreateIdentityAssertion,
    DecideIdentityMerge,
    AppendAudit,
    TransitionEffect,
    ReadObject,
}

/// Request body variants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum RequestBody {
    AcquireLease {
        client_id: OpaqueId,
        holder_id_hint: Option<OpaqueId>,
    },
    ReleaseLease {
        holder_id: OpaqueId,
    },
    Ping,
    CreateSourceRecord {
        media_type: String,
        bytes: Vec<u8>,
    },
    CreateDerivedArtifact {
        source_id: OpaqueId,
        transform_id: String,
        transform_version: String,
        bytes: Vec<u8>,
    },
    CreateProposal {
        subject_ref: Option<OpaqueId>,
        claim_kind: String,
        payload: Value,
        evidence_refs: Vec<OpaqueId>,
    },
    PromoteProposal {
        proposal_id: OpaqueId,
        authorized_by: OpaqueId,
        subject_ref: OpaqueId,
    },
    CreateIdentityAssertion {
        subject_id: OpaqueId,
        identifier_system: String,
        identifier_value: String,
    },
    DecideIdentityMerge {
        surviving_subject_id: OpaqueId,
        merged_subject_ids: Vec<OpaqueId>,
        authorized_by: OpaqueId,
        rationale: String,
    },
    AppendAudit {
        actor: OpaqueId,
        action: String,
        target_refs: Vec<OpaqueId>,
        detail: Option<Value>,
    },
    TransitionEffect {
        action_id: OpaqueId,
        to: EffectState,
        reconcile_token: Option<String>,
    },
    ReadObject {
        object_id: OpaqueId,
    },
}

/// Successful response body variants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "ok", rename_all = "snake_case")]
pub enum ResponseBody {
    Lease {
        holder_id: OpaqueId,
        vault_id: VaultId,
    },
    Pong {
        schema_version: u32,
    },
    Created {
        object_id: OpaqueId,
    },
    Promoted {
        assertion_id: OpaqueId,
        audit_id: OpaqueId,
    },
    Effect {
        action_id: OpaqueId,
        state: EffectState,
    },
    Object {
        /// JSON encoding of the object; never includes DB/key handles.
        value: Value,
    },
    Released,
}

/// Authority error vocabulary (fail closed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum AuthorityError {
    Unauthorized,
    AlreadyHeld { holder_id: OpaqueId },
    NotHolder,
    NotHeld,
    NotFound,
    WrongScope,
    IllegalTransition,
    UnknownRequiresReconcile,
    DigestMismatch,
    InvalidArgument { message: String },
}

/// Versioned authority request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityRequest {
    pub schema_version: u32,
    pub request_id: OpaqueId,
    pub vault_id: VaultId,
    pub authority_scope_id: crate::objects::AuthorityScopeId,
    pub realm_id: crate::objects::RealmId,
    pub capability: Capability,
    pub deadline_ms: u64,
    pub max_response_bytes: u64,
    pub body: RequestBody,
}

impl AuthorityRequest {
    /// Builds a request at the current schema version.
    #[must_use]
    pub fn new(
        request_id: OpaqueId,
        vault_id: VaultId,
        realm_id: crate::objects::RealmId,
        authority_scope_id: crate::objects::AuthorityScopeId,
        capability: Capability,
        body: RequestBody,
    ) -> Self {
        Self {
            schema_version: AUTHORITY_SCHEMA_VERSION,
            request_id,
            vault_id,
            authority_scope_id,
            realm_id,
            capability,
            deadline_ms: 5_000,
            max_response_bytes: 1_048_576,
            body,
        }
    }
}

/// Versioned authority response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityResponse {
    pub schema_version: u32,
    pub request_id: OpaqueId,
    pub result: Result<ResponseBody, AuthorityError>,
}
