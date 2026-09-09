//! Source-versioned local lexical evidence corpus (Spec 025 / Q10).

use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use medscale_contracts::evidence::{
    EvidenceCorpusDocument, EvidenceCorpusManifest, EvidenceCorpusRights, EvidenceDocStatus,
};
use medscale_contracts::objects::DigestSha256;
use serde::Deserialize;
use thiserror::Error;

/// Default MedScale-owned synthetic corpus identity (not a content hash).
pub const DEFAULT_CORPUS_ID: &str = "synthetic-lexical";
/// Default admitted synthetic corpus version.
pub const DEFAULT_CORPUS_VERSION: &str = "1.0.0";

/// Corpus admission / load failures (fail-closed).
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CorpusAdmitError {
    #[error("invalid corpus manifest: {0}")]
    InvalidManifest(String),
    #[error("missing or non-synthetic rights")]
    RightsDenied,
    #[error("digest mismatch")]
    DigestMismatch,
    #[error("io: {0}")]
    Io(String),
}

#[derive(Debug, Deserialize)]
struct WireDoc {
    doc_id: String,
    text: String,
    content_digest: String,
    status: EvidenceDocStatus,
    #[serde(default)]
    freshness_marker: Option<String>,
    #[serde(default)]
    conflict_marker: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WireManifest {
    corpus_id: String,
    version: String,
    content_digest: String,
    rights: EvidenceCorpusRights,
    rights_uri: String,
    documents: Vec<WireDoc>,
}

fn parse_hex_digest(hex: &str) -> Result<DigestSha256, CorpusAdmitError> {
    let hex = hex.trim();
    if hex.len() != 64 {
        return Err(CorpusAdmitError::InvalidManifest(
            "digest must be 64 hex chars".into(),
        ));
    }
    let mut bytes = [0_u8; 32];
    for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
        let s = std::str::from_utf8(chunk)
            .map_err(|_| CorpusAdmitError::InvalidManifest("digest utf8".into()))?;
        bytes[i] = u8::from_str_radix(s, 16)
            .map_err(|_| CorpusAdmitError::InvalidManifest("digest hex".into()))?;
    }
    Ok(DigestSha256::from_bytes(bytes))
}

fn compute_corpus_content_digest(docs: &[EvidenceCorpusDocument]) -> DigestSha256 {
    let mut ordered: Vec<&EvidenceCorpusDocument> = docs.iter().collect();
    ordered.sort_by(|a, b| a.doc_id.cmp(&b.doc_id));
    let mut concat = String::new();
    for d in ordered {
        concat.push_str(&d.content_digest.to_hex());
        concat.push('\n');
    }
    DigestSha256::of(concat.as_bytes())
}

fn validate_and_build(wire: WireManifest) -> Result<EvidenceCorpusManifest, CorpusAdmitError> {
    if !matches!(wire.rights, EvidenceCorpusRights::SyntheticOwned) {
        return Err(CorpusAdmitError::RightsDenied);
    }
    if wire.rights_uri.trim().is_empty() {
        return Err(CorpusAdmitError::RightsDenied);
    }
    if !wire.rights_uri.contains("synthetic") {
        return Err(CorpusAdmitError::RightsDenied);
    }
    if wire.corpus_id.trim().is_empty() || wire.version.trim().is_empty() {
        return Err(CorpusAdmitError::InvalidManifest(
            "corpus_id and version required".into(),
        ));
    }
    if wire.documents.is_empty() {
        return Err(CorpusAdmitError::InvalidManifest(
            "documents must be non-empty".into(),
        ));
    }

    let mut documents = Vec::with_capacity(wire.documents.len());
    for doc in wire.documents {
        let computed = DigestSha256::of(doc.text.as_bytes());
        let declared = parse_hex_digest(&doc.content_digest)?;
        if computed != declared {
            return Err(CorpusAdmitError::DigestMismatch);
        }
        documents.push(EvidenceCorpusDocument {
            doc_id: doc.doc_id,
            text: doc.text,
            content_digest: computed,
            status: doc.status,
            freshness_marker: doc.freshness_marker,
            conflict_marker: doc.conflict_marker,
        });
    }

    let expected = compute_corpus_content_digest(&documents);
    let declared = parse_hex_digest(&wire.content_digest)?;
    if expected != declared {
        return Err(CorpusAdmitError::DigestMismatch);
    }

    Ok(EvidenceCorpusManifest {
        corpus_id: wire.corpus_id,
        version: wire.version,
        content_digest: declared,
        rights: wire.rights,
        rights_uri: wire.rights_uri,
        documents,
    })
}

/// Admit a corpus directory containing `corpus.manifest.json`.
pub fn admit_corpus_dir(path: &Path) -> Result<EvidenceCorpusManifest, CorpusAdmitError> {
    let manifest_path = path.join("corpus.manifest.json");
    let raw = fs::read(&manifest_path).map_err(|e| CorpusAdmitError::Io(e.to_string()))?;
    admit_corpus_bytes(&raw)
}

/// Admit corpus manifest JSON bytes (fixture or embedded).
pub fn admit_corpus_bytes(raw: &[u8]) -> Result<EvidenceCorpusManifest, CorpusAdmitError> {
    let wire: WireManifest = serde_json::from_slice(raw)
        .map_err(|e| CorpusAdmitError::InvalidManifest(e.to_string()))?;
    validate_and_build(wire)
}

fn builtin_v1_json() -> &'static str {
    include_str!(
        "../../../../evidence/025-evidence-corpus-lifecycle/fixtures/synthetic-lexical-v1/corpus.manifest.json"
    )
}

/// Built-in Spec 025 synthetic corpus (admitted once, fail-closed on digest error).
#[must_use]
pub fn default_synthetic_corpus() -> &'static EvidenceCorpusManifest {
    static CORPUS: OnceLock<EvidenceCorpusManifest> = OnceLock::new();
    CORPUS.get_or_init(|| {
        admit_corpus_bytes(builtin_v1_json().as_bytes())
            .expect("builtin synthetic-lexical@1.0.0 must admit")
    })
}

pub(crate) fn matches_request(m: &EvidenceCorpusManifest, request: &str) -> bool {
    request == m.corpus_id || request == m.source_identity()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_admits_and_identity_is_not_digest() {
        let c = default_synthetic_corpus();
        assert_eq!(c.corpus_id, DEFAULT_CORPUS_ID);
        assert_eq!(c.version, DEFAULT_CORPUS_VERSION);
        assert_eq!(c.source_identity(), "synthetic-lexical@1.0.0");
        assert_ne!(c.source_identity(), c.content_digest.to_hex());
        assert!(matches!(c.rights, EvidenceCorpusRights::SyntheticOwned));
        assert!(
            c.documents
                .iter()
                .any(|d| matches!(d.status, EvidenceDocStatus::Retracted))
        );
    }
}
