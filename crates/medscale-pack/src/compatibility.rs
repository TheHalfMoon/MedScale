//! Pre-download runtime compatibility for catalog rows (Spec 103, T103-09).
//!
//! This is an **expectation** derived from recorded execution evidence for the
//! local `tract_onnx_token_classification_v1` runtime
//! (`specs/103-local-model-catalog/research.md` sections 9–10). It is never a
//! status promotion: a row only becomes `RUNTIME_COMPATIBLE` / `EXECUTED_TESTED`
//! through its own evidence.

use serde::Serialize;

use crate::catalog::CatalogRow;

/// Architectures with at least one OpenMed v3.0.0 model executed end to end
/// (catalog → verified snapshot → signed Pack → admission → run). DeBERTa-v2
/// and XLM-R were added with qualification run 37977100903 (XLM-R through its
/// fp16 export, widened to fp32 at load).
pub const EVIDENCED_ARCHITECTURES: &[&str] = &[
    "bert",
    "distilbert",
    "roberta",
    "modernbert",
    "deberta-v2",
    "xlm-roberta",
];

/// Architectures recorded as not supported by the current runtime, with the cause.
pub const KNOWN_UNSUPPORTED: &[(&str, &str)] = &[];

/// Published exports known to be defective, matched by architecture and a
/// repository-id prefix, with the cause.
pub const KNOWN_DEFECTIVE_EXPORTS: &[(&str, &str, &str)] = &[(
    "xlm-roberta",
    "OpenMed/OpenMed-NER-",
    "the published tokenizer.json is a BPE model with zero merges, so text is split into single characters; the runtime refuses it",
)];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RuntimeExpectation {
    /// Same task, an ONNX file, and an architecture with recorded execution
    /// evidence. Expected to run; still unverified for this row.
    ExpectedRunnable,
    /// Known not to run on the current runtime.
    KnownUnsupported { reason: String },
    /// No ONNX file is listed (for example MLX or PyTorch only).
    NoOnnxArtifact,
    /// The task is not token classification.
    UnsupportedTask { task: String },
    /// ONNX token classifier with an architecture not yet tried.
    Untested,
}

/// Expectation for `row` on the local ONNX token-classification runtime.
pub fn runtime_expectation(row: &CatalogRow) -> RuntimeExpectation {
    if row.task != "token-classification" {
        return RuntimeExpectation::UnsupportedTask {
            task: row.task.clone(),
        };
    }
    if !row.formats.iter().any(|f| f.eq_ignore_ascii_case("onnx")) {
        return RuntimeExpectation::NoOnnxArtifact;
    }
    let arch = row.architecture.to_ascii_lowercase();
    if let Some((_, _, reason)) = KNOWN_DEFECTIVE_EXPORTS
        .iter()
        .find(|(a, prefix, _)| *a == arch && row.repo_id.starts_with(prefix))
    {
        return RuntimeExpectation::KnownUnsupported {
            reason: (*reason).to_string(),
        };
    }
    if let Some((_, reason)) = KNOWN_UNSUPPORTED.iter().find(|(a, _)| *a == arch) {
        return RuntimeExpectation::KnownUnsupported {
            reason: (*reason).to_string(),
        };
    }
    if EVIDENCED_ARCHITECTURES.contains(&arch.as_str()) {
        return RuntimeExpectation::ExpectedRunnable;
    }
    RuntimeExpectation::Untested
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::ModelCatalog;

    const COMMIT: &str = "ea920f36fadd7b45935247d639f0ffa1ef493b23";

    fn row(task: &str, arch: &str, formats: &str) -> CatalogRow {
        named("Org/m", task, arch, formats)
    }

    fn named(repo: &str, task: &str, arch: &str, formats: &str) -> CatalogRow {
        let line = format!(
            r#"{{"repo_id":"{repo}","family":"NER","task":"{task}","architecture":"{arch}","formats":{formats},"license":"apache-2.0"}}"#
        );
        ModelCatalog::import("s/f", COMMIT, line.as_bytes())
            .unwrap()
            .get(repo)
            .unwrap()
            .clone()
    }

    #[test]
    fn expectations_follow_recorded_evidence() {
        assert_eq!(
            runtime_expectation(&row("token-classification", "bert", r#"["onnx"]"#)),
            RuntimeExpectation::ExpectedRunnable
        );
        assert_eq!(
            runtime_expectation(&row(
                "token-classification",
                "ModernBERT",
                r#"["pytorch","onnx"]"#
            )),
            RuntimeExpectation::ExpectedRunnable
        );
        assert_eq!(
            runtime_expectation(&row("token-classification", "deberta-v2", r#"["onnx"]"#)),
            RuntimeExpectation::ExpectedRunnable
        );
        assert_eq!(
            runtime_expectation(&named(
                "OpenMed/OpenMed-PII-Spanish-BigMed-Large-278M-v1-onnx-android",
                "token-classification",
                "xlm-roberta",
                r#"["onnx"]"#
            )),
            RuntimeExpectation::ExpectedRunnable
        );
        assert!(matches!(
            runtime_expectation(&named(
                "OpenMed/OpenMed-NER-DiseaseDetect-BigMed-278M-v1-onnx-android",
                "token-classification",
                "xlm-roberta",
                r#"["onnx"]"#
            )),
            RuntimeExpectation::KnownUnsupported { .. }
        ));
        assert_eq!(
            runtime_expectation(&row("token-classification", "bert", r#"["mlx-fp"]"#)),
            RuntimeExpectation::NoOnnxArtifact
        );
        assert_eq!(
            runtime_expectation(&row("token-classification", "gliner", r#"["onnx"]"#)),
            RuntimeExpectation::Untested
        );
        assert_eq!(
            runtime_expectation(&row("text-generation", "qwen", r#"["onnx"]"#)),
            RuntimeExpectation::UnsupportedTask {
                task: "text-generation".into()
            }
        );
    }

    #[test]
    fn expectation_never_changes_the_row_status() {
        let r = row("token-classification", "bert", r#"["onnx"]"#);
        let before = r.status;
        let _ = runtime_expectation(&r);
        assert_eq!(r.status, before);
        assert!(before <= crate::catalog::CatalogStatus::RightsPending);
    }
}
