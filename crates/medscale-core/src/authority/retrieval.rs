//! Deterministic lexical retrieval (Spec 011) — relevance ≠ authority.

use medscale_contracts::AUTHORITY_SCHEMA_VERSION;
use medscale_contracts::envelopes::AuthorityError;
use medscale_contracts::evidence::{LexicalRetrieveRequest, LexicalRetrieveResult, RetrievalHit};
use medscale_contracts::objects::{EvaluationRecord, ObjectHeader, OpaqueId};

use super::store::{InMemoryAuthorityStore, StoredObject};

#[derive(Clone)]
struct CorpusDoc {
    id: &'static str,
    text: &'static str,
}

const SYNTHETIC_CORPUS: &[CorpusDoc] = &[
    CorpusDoc {
        id: "doc-htn",
        text: "synthetic hypertension note blood pressure elevated",
    },
    CorpusDoc {
        id: "doc-dm",
        text: "synthetic diabetes mellitus metformin dose discussion",
    },
    CorpusDoc {
        id: "doc-asthma",
        text: "synthetic asthma inhaler rescue critical number trap",
    },
];

fn tokenize(s: &str) -> Vec<String> {
    s.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(str::to_owned)
        .collect()
}

fn score(query: &str, doc: &str) -> f64 {
    let q = tokenize(query);
    if q.is_empty() {
        return 0.0;
    }
    let d = tokenize(doc);
    let mut hits = 0_u32;
    for t in &q {
        if d.iter().any(|x| x == t) {
            hits += 1;
        }
    }
    f64::from(hits) / q.len() as f64
}

/// Rank synthetic corpus; persist evidence-only EvaluationRecord.
pub(crate) fn retrieve_lexical(
    store: &mut InMemoryAuthorityStore,
    realm_id: medscale_contracts::objects::RealmId,
    scope_id: medscale_contracts::objects::AuthorityScopeId,
    request: LexicalRetrieveRequest,
) -> Result<LexicalRetrieveResult, AuthorityError> {
    if request.corpus_id != "synthetic-lexical-v0" {
        return Err(AuthorityError::InvalidArgument {
            message: format!("unknown corpus_id {}", request.corpus_id),
        });
    }
    let max = request.max_hits.clamp(1, 32) as usize;
    let mut scored: Vec<(f64, &CorpusDoc)> = SYNTHETIC_CORPUS
        .iter()
        .map(|d| (score(&request.query, d.text), d))
        .filter(|(s, _)| *s > 0.0)
        .collect();
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(max);

    let hits: Vec<RetrievalHit> = scored
        .into_iter()
        .map(|(s, d)| RetrievalHit {
            doc_id: OpaqueId::new(d.id),
            score: s,
            snippet: d.text.to_owned(),
            relevance_only: true,
        })
        .collect();

    let evaluation_id = store.alloc_id("eval");
    let target_refs: Vec<OpaqueId> = hits.iter().map(|h| h.doc_id.clone()).collect();
    store.insert(StoredObject::Evaluation(EvaluationRecord {
        header: ObjectHeader {
            id: evaluation_id.clone(),
            schema_version: AUTHORITY_SCHEMA_VERSION,
            realm_id,
            authority_scope_id: scope_id,
        },
        target_refs,
        evaluator: "medscale.retrieval.lexical.v0".to_owned(),
        result: serde_json::json!({
            "corpus_id": request.corpus_id,
            "query": request.query,
            "hits": hits,
            "relevance_is_not_authority": true,
            "freshness": "synthetic_fixture",
            "conflict": null
        }),
        evidence_only: true,
    }));

    Ok(LexicalRetrieveResult {
        corpus_id: request.corpus_id,
        query: request.query,
        hits,
        evaluation_id,
        evidence_only: true,
        relevance_is_not_authority: true,
    })
}
