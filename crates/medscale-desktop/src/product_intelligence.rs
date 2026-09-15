//! Product-level model/runtime and competitive-evidence presentation for Spec 068.
//!
//! This module is intentionally honest about the current boundary: MedScale has
//! a pack/fabric contract and FixtureRuntime, but no production ONNX/llama/Candle
//! runtime is admitted yet. Spec 069 owns that admission. Competitive rows may
//! say PROVEN only when the repository already carries a bounded evidence class.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelStatusVm {
    pub task: String,
    pub model: String,
    pub source: String,
    pub runtime: String,
    pub device: String,
    pub trust: String,
    pub benchmark: String,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompetitiveEvidenceVm {
    pub capability: String,
    pub medscale: String,
    pub openmed: String,
    pub verdict: String,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductIntelligenceVm {
    pub model_runtime_summary: String,
    pub model_runtime_boundary: String,
    pub model_source_summary: String,
    pub openmed_baseline: String,
    pub competitive_summary: String,
    pub models: Vec<ModelStatusVm>,
    pub evidence: Vec<CompetitiveEvidenceVm>,
}

impl ProductIntelligenceVm {
    #[must_use]
    pub fn current_truth() -> Self {
        let models = vec![
            ModelStatusVm {
                task: "Clinical NER".to_owned(),
                model: "Production model not admitted".to_owned(),
                source: "Hugging Face pack path".to_owned(),
                runtime: "FixtureRuntime only".to_owned(),
                device: "Local worker boundary".to_owned(),
                trust: "Pack provenance required".to_owned(),
                benchmark: "OpenMed parity not yet proven".to_owned(),
                state: "RUNTIME GAP".to_owned(),
            },
            ModelStatusVm {
                task: "PII / de-identification".to_owned(),
                model: "Production model not admitted".to_owned(),
                source: "Hugging Face pack path".to_owned(),
                runtime: "FixtureRuntime only".to_owned(),
                device: "Local worker boundary".to_owned(),
                trust: "Fail-closed pack admission".to_owned(),
                benchmark: "High-risk PHI recall pending".to_owned(),
                state: "RUNTIME GAP".to_owned(),
            },
            ModelStatusVm {
                task: "Clinical assistant".to_owned(),
                model: "No generative model selected".to_owned(),
                source: "Curated local model path".to_owned(),
                runtime: "Not admitted".to_owned(),
                device: "Desktop-local target".to_owned(),
                trust: "Proposal-only authority".to_owned(),
                benchmark: "No quality claim".to_owned(),
                state: "NOT ADMITTED".to_owned(),
            },
            ModelStatusVm {
                task: "Retrieval".to_owned(),
                model: "Deterministic lexical corpus".to_owned(),
                source: "MedScale core".to_owned(),
                runtime: "Rust".to_owned(),
                device: "Local CPU".to_owned(),
                trust: "Evidence-only evaluation".to_owned(),
                benchmark: "10k synthetic lexical scale".to_owned(),
                state: "ACTIVE".to_owned(),
            },
            ModelStatusVm {
                task: "Model distribution".to_owned(),
                model: "Signed MedScale Pack lifecycle".to_owned(),
                source: "Hugging Face optional source".to_owned(),
                runtime: "Network Broker gated".to_owned(),
                device: "Offline-first".to_owned(),
                trust: "candidate → qualified → current".to_owned(),
                benchmark: "Online acquisition not granted".to_owned(),
                state: "EXTERNAL GATE".to_owned(),
            },
        ];

        let evidence = vec![
            CompetitiveEvidenceVm {
                capability: "Source custody".to_owned(),
                medscale: "Immutable source bytes + explicit transformation/loss chain".to_owned(),
                openmed: "Strong provenance, narrower product authority scope".to_owned(),
                verdict: "PROVEN ADVANTAGE".to_owned(),
                evidence: "Specs 002/003 + source/provenance contracts".to_owned(),
            },
            CompetitiveEvidenceVm {
                capability: "Identity safety".to_owned(),
                medscale: "Explicit IdentityAssertion; no silent patient merge".to_owned(),
                openmed: "Not a category-center capability".to_owned(),
                verdict: "PROVEN ADVANTAGE".to_owned(),
                evidence: "Specs 002/019 adversarial identity semantics".to_owned(),
            },
            CompetitiveEvidenceVm {
                capability: "Longitudinal truth / conflict".to_owned(),
                medscale: "Versioned assertions, absence/freshness/conflict without silent resolution".to_owned(),
                openmed: "Timeline/provenance capabilities".to_owned(),
                verdict: "PROVEN ADVANTAGE".to_owned(),
                evidence: "Specs 004/019 trusted record semantics".to_owned(),
            },
            CompetitiveEvidenceVm {
                capability: "Controlled actions".to_owned(),
                medscale: "Durable intent + payload identity + UNKNOWN reconciliation; blind retry forbidden".to_owned(),
                openmed: "Inspectable agent review and service boundaries".to_owned(),
                verdict: "PROVEN ADVANTAGE".to_owned(),
                evidence: "Specs 014/034/063 outbox/effect-state evidence".to_owned(),
            },
            CompetitiveEvidenceVm {
                capability: "Clinical NER".to_owned(),
                medscale: "No production model runtime admitted yet".to_owned(),
                openmed: "Large public model catalog + rerunnable NER benchmarks".to_owned(),
                verdict: "OPENMED AHEAD".to_owned(),
                evidence: "Spec 008 FixtureRuntime-only; Spec 069 promoted".to_owned(),
            },
            CompetitiveEvidenceVm {
                capability: "PII / de-identification".to_owned(),
                medscale: "Contracts and parity corpus design; no production inference yet".to_owned(),
                openmed: "Multilingual local model family and de-identification pipeline".to_owned(),
                verdict: "OPENMED AHEAD".to_owned(),
                evidence: "Spec 007 corpus program; runtime proof pending 069/071".to_owned(),
            },
            CompetitiveEvidenceVm {
                capability: "Model ecosystem".to_owned(),
                medscale: "Signed/provenance-first pack lifecycle; online HF path gated".to_owned(),
                openmed: "2,000+ public models and broad runtime coverage".to_owned(),
                verdict: "OPENMED AHEAD".to_owned(),
                evidence: "Specs 008/015; raw model count is not a MedScale success metric".to_owned(),
            },
            CompetitiveEvidenceVm {
                capability: "Apple Silicon inference".to_owned(),
                medscale: "No MLX production adapter admitted".to_owned(),
                openmed: "MLX is a first-class accelerated runtime".to_owned(),
                verdict: "OPENMED AHEAD".to_owned(),
                evidence: "Spec 069 must benchmark runtime candidates before admission".to_owned(),
            },
        ];

        Self {
            model_runtime_summary: "Model Fabric · contracts live · production inference not admitted".to_owned(),
            model_runtime_boundary: "Current product truth: signed/offline Pack contracts and FixtureRuntime exist. No production ONNX, llama.cpp, Candle, or MLX execution path is admitted yet. Spec 069 owns real runtime admission.".to_owned(),
            model_source_summary: "Hugging Face is a model source, not an authority plane. Online acquisition must pass the Network Broker and exact pack provenance; runtime remains local and fail closed.".to_owned(),
            openmed_baseline: "OpenMed v2.2.0 · pinned commit 59d9cb0a…".to_owned(),
            competitive_summary: "MedScale is structurally stronger in source custody, identity, longitudinal truth, and controlled-action authority. OpenMed is currently ahead in production clinical models, PII/de-ID breadth, and accelerated local inference.".to_owned(),
            models,
            evidence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProductIntelligenceVm;

    #[test]
    fn current_truth_never_fakes_model_runtime_or_openmed_superiority() {
        let vm = ProductIntelligenceVm::current_truth();
        assert!(
            vm.model_runtime_summary
                .contains("production inference not admitted")
        );
        assert!(vm.models.iter().any(|row| row.state == "RUNTIME GAP"));
        assert!(vm.evidence.iter().any(|row| row.verdict == "OPENMED AHEAD"));
        assert!(
            vm.evidence
                .iter()
                .any(|row| row.verdict == "PROVEN ADVANTAGE")
        );
        assert!(!vm.competitive_summary.contains("unqualified superiority"));
    }
}
