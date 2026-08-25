//! Evidence / lexical retrieval contracts (Spec 011).

use serde::{Deserialize, Serialize};

use crate::objects::OpaqueId;

/// One ranked retrieval hit (relevance only — never authority).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetrievalHit {
    pub doc_id: OpaqueId,
    pub score: f64,
    pub snippet: String,
    pub relevance_only: bool,
}

/// Request for deterministic lexical retrieval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LexicalRetrieveRequest {
    pub query: String,
    pub corpus_id: String,
    pub max_hits: u32,
}

/// Response envelope before persistence as EvaluationRecord.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LexicalRetrieveResult {
    pub corpus_id: String,
    pub query: String,
    pub hits: Vec<RetrievalHit>,
    pub evaluation_id: OpaqueId,
    pub evidence_only: bool,
    /// Explicit constitutional reminder.
    pub relevance_is_not_authority: bool,
}
