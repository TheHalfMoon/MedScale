//! Lexical/structural gate for synthetic FHIR R4 JSON.

mod lexical;

pub use lexical::{LexicalError, LexicalReport, extract_patient_identifiers, gate_fhir_json};

/// Maximum accepted ingest payload size (bytes).
pub const MAX_INGEST_BYTES: usize = 1_048_576;

/// Maximum JSON nesting depth.
pub const MAX_JSON_DEPTH: usize = 64;
