//! In-memory scoped object store.

use std::collections::HashMap;

use medscale_contracts::objects::{
    ActionAuditRecord, AuthorityScopeId, ClinicalAssertion, DerivedSourceArtifact,
    IdentityAssertion, IdentityMergeDecision, OpaqueId, Proposal, RealmId, SourceRecord,
};
use serde_json::Value;

/// Stored object variants for read-back.
#[derive(Debug, Clone)]
pub enum StoredObject {
    Source(SourceRecord),
    Derived(DerivedSourceArtifact),
    Proposal(Proposal),
    Assertion(ClinicalAssertion),
    Audit(ActionAuditRecord),
    Identity(IdentityAssertion),
    Merge(IdentityMergeDecision),
}

impl StoredObject {
    pub fn to_json(&self) -> Value {
        match self {
            Self::Source(v) => serde_json::to_value(v).unwrap_or(Value::Null),
            Self::Derived(v) => serde_json::to_value(v).unwrap_or(Value::Null),
            Self::Proposal(v) => serde_json::to_value(v).unwrap_or(Value::Null),
            Self::Assertion(v) => serde_json::to_value(v).unwrap_or(Value::Null),
            Self::Audit(v) => serde_json::to_value(v).unwrap_or(Value::Null),
            Self::Identity(v) => serde_json::to_value(v).unwrap_or(Value::Null),
            Self::Merge(v) => serde_json::to_value(v).unwrap_or(Value::Null),
        }
    }

    fn scope_ids(&self) -> (&RealmId, &AuthorityScopeId, &OpaqueId) {
        match self {
            Self::Source(v) => (
                &v.header.realm_id,
                &v.header.authority_scope_id,
                &v.header.id,
            ),
            Self::Derived(v) => (
                &v.header.realm_id,
                &v.header.authority_scope_id,
                &v.header.id,
            ),
            Self::Proposal(v) => (
                &v.header.realm_id,
                &v.header.authority_scope_id,
                &v.header.id,
            ),
            Self::Assertion(v) => (
                &v.header.realm_id,
                &v.header.authority_scope_id,
                &v.header.id,
            ),
            Self::Audit(v) => (
                &v.header.realm_id,
                &v.header.authority_scope_id,
                &v.header.id,
            ),
            Self::Identity(v) => (
                &v.header.realm_id,
                &v.header.authority_scope_id,
                &v.header.id,
            ),
            Self::Merge(v) => (
                &v.header.realm_id,
                &v.header.authority_scope_id,
                &v.header.id,
            ),
        }
    }
}

/// In-memory store keyed by object id with scope checks on read.
#[derive(Debug, Default)]
pub struct InMemoryAuthorityStore {
    objects: HashMap<String, StoredObject>,
    next_seq: u64,
}

impl InMemoryAuthorityStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alloc_id(&mut self, prefix: &str) -> OpaqueId {
        self.next_seq += 1;
        OpaqueId::new(format!("{prefix}-{}", self.next_seq))
    }

    pub fn insert(&mut self, object: StoredObject) {
        let id = object.scope_ids().2.as_str().to_owned();
        self.objects.insert(id, object);
    }

    pub fn get_scoped(
        &self,
        id: &OpaqueId,
        realm_id: &RealmId,
        scope_id: &AuthorityScopeId,
    ) -> Result<&StoredObject, ScopeError> {
        let Some(obj) = self.objects.get(id.as_str()) else {
            return Err(ScopeError::NotFound);
        };
        let (r, s, _) = obj.scope_ids();
        if r != realm_id || s != scope_id {
            return Err(ScopeError::WrongScope);
        }
        Ok(obj)
    }

    pub fn get_proposal(
        &self,
        id: &OpaqueId,
        realm_id: &RealmId,
        scope_id: &AuthorityScopeId,
    ) -> Result<&Proposal, ScopeError> {
        match self.get_scoped(id, realm_id, scope_id)? {
            StoredObject::Proposal(p) => Ok(p),
            _ => Err(ScopeError::NotFound),
        }
    }

    pub fn get_source(
        &self,
        id: &OpaqueId,
        realm_id: &RealmId,
        scope_id: &AuthorityScopeId,
    ) -> Result<&SourceRecord, ScopeError> {
        match self.get_scoped(id, realm_id, scope_id)? {
            StoredObject::Source(s) => Ok(s),
            _ => Err(ScopeError::NotFound),
        }
    }

    pub fn get_audit_mut(&mut self, id: &OpaqueId) -> Option<&mut ActionAuditRecord> {
        match self.objects.get_mut(id.as_str()) {
            Some(StoredObject::Audit(a)) => Some(a),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeError {
    NotFound,
    WrongScope,
}
