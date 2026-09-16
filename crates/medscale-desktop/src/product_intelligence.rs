//! Model Center and comparative-evidence presentation (Specs 068–070).
//!
//! Spec 070 distinguishes session-admitted Pack inventory from qualification
//! references. Desktop reads Pack state through the Core-owned `CliSession` API;
//! it never imports the runtime crate or treats Hugging Face as an authority plane.

use medscale_contracts::envelopes::AuthorityError;
use medscale_contracts::packs::{PackArtifactKind, PackManifestV0, PackPromotionState};
use medscale_core::CliSession;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelStatusVm {
    pub scope: String,
    pub task: String,
    pub model: String,
    pub source: String,
    pub runtime: String,
    pub device: String,
    pub trust: String,
    pub benchmark: String,
    pub digest: String,
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
    pub model_inventory_summary: String,
    pub openmed_baseline: String,
    pub competitive_summary: String,
    pub models: Vec<ModelStatusVm>,
    pub evidence: Vec<CompetitiveEvidenceVm>,
}

impl ProductIntelligenceVm {
    pub fn from_session(session: &mut CliSession) -> Result<Self, AuthorityError> {
        let packs = session.packs_list()?;
        Ok(Self::from_packs(&packs))
    }

    #[must_use]
    pub fn current_truth() -> Self {
        Self::from_packs(&[])
    }

    #[must_use]
    pub fn from_packs(packs: &[PackManifestV0]) -> Self {
        let mut models = Vec::with_capacity(packs.len() + 4);
        if packs.is_empty() {
            models.push(ModelStatusVm {
                scope: "SESSION INVENTORY".to_owned(),
                task: "Local Pack inventory".to_owned(),
                model: "No Pack admitted in this Desktop session".to_owned(),
                source: "Use local Pack admission to inspect a signed Pack".to_owned(),
                runtime: "No session runtime selected".to_owned(),
                device: "Local only".to_owned(),
                trust: "No Pack authority present".to_owned(),
                benchmark: "No execution or quality claim".to_owned(),
                digest: "—".to_owned(),
                state: "EMPTY".to_owned(),
            });
        } else {
            models.extend(packs.iter().map(pack_row));
        }

        models.push(ModelStatusVm {
            scope: "QUALIFICATION REFERENCE".to_owned(),
            task: "NER runtime qualification".to_owned(),
            model: "onnx-community/bert-base-NER-ONNX".to_owned(),
            source: "Hugging Face · revision 9faa2f4a2d59…".to_owned(),
            runtime: "tract-onnx 0.22.4 · portable CPU".to_owned(),
            device: "Local CPU".to_owned(),
            trust: "Signed Pack provenance · external weights excluded from Git".to_owned(),
            benchmark: "Release baseline PASS · warm p95 596.100 ms · accelerated runtime pending"
                .to_owned(),
            digest: "805168abca14c2131c26ed20c5b46a46e8ccc759f1f78264b139f0aa86c8f93d".to_owned(),
            state: "QUALIFIED BASELINE".to_owned(),
        });
        models.push(ModelStatusVm {
            scope: "CAPABILITY GAP".to_owned(),
            task: "PII / de-identification".to_owned(),
            model: "production clinical model not promoted".to_owned(),
            source: "Curated signed Pack path only".to_owned(),
            runtime: "Portable ONNX runtime available".to_owned(),
            device: "Local worker boundary".to_owned(),
            trust: "Real PHI remains denied".to_owned(),
            benchmark: "High-risk PHI recall qualification pending".to_owned(),
            digest: "—".to_owned(),
            state: "MODEL GAP".to_owned(),
        });
        models.push(ModelStatusVm {
            scope: "CAPABILITY GAP".to_owned(),
            task: "Accelerated inference".to_owned(),
            model: "No accelerated runtime promoted".to_owned(),
            source: "Spec 071 comparison pending".to_owned(),
            runtime: "Portable tract baseline only".to_owned(),
            device: "CPU baseline".to_owned(),
            trust: "No runtime-winner claim".to_owned(),
            benchmark: "Apple Silicon / accelerated comparison pending".to_owned(),
            digest: "—".to_owned(),
            state: "NOT ADMITTED".to_owned(),
        });

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
                medscale: "Portable ONNX runtime + pinned HF NER qualification; clinical-domain model benchmark still pending".to_owned(),
                openmed: "Large public model catalog + rerunnable NER benchmarks".to_owned(),
                verdict: "OPENMED AHEAD".to_owned(),
                evidence: "Spec 069 real HF model execution; Spec 071 owns comparative benchmark qualification".to_owned(),
            },
            CompetitiveEvidenceVm {
                capability: "PII / de-identification".to_owned(),
                medscale: "Real ONNX runtime exists; production de-identification model/PHI benchmark not promoted".to_owned(),
                openmed: "Multilingual local model family and de-identification pipeline".to_owned(),
                verdict: "OPENMED AHEAD".to_owned(),
                evidence: "Spec 007 corpus program; Spec 069 runtime proof; clinical model proof pending 071".to_owned(),
            },
            CompetitiveEvidenceVm {
                capability: "Model ecosystem".to_owned(),
                medscale: "Signed provenance-first Pack lifecycle + pinned HF import qualification; online acquisition still gated".to_owned(),
                openmed: "2,000+ public models and broad runtime coverage".to_owned(),
                verdict: "OPENMED AHEAD".to_owned(),
                evidence: "Specs 008/015/069; raw model count is not a MedScale success metric".to_owned(),
            },
            CompetitiveEvidenceVm {
                capability: "Apple Silicon inference".to_owned(),
                medscale: "Portable tract CPU baseline measured; no accelerated Apple Silicon adapter admitted".to_owned(),
                openmed: "MLX is a first-class accelerated runtime".to_owned(),
                verdict: "OPENMED AHEAD".to_owned(),
                evidence: "Spec 069 portable baseline; Spec 071 owns accelerated runtime comparison".to_owned(),
            },
        ];

        let admitted = packs.len();
        Self {
            model_runtime_summary: format!(
                "Model Fabric · real portable ONNX runtime · {admitted} Pack(s) admitted this session"
            ),
            model_runtime_boundary: "Core owns Pack admission, promotion and execution; real portable ONNX runtime admitted for synthetic/evidence-only evaluation. Model Center is an operator projection: session-local admitted inventory is read through Core; a qualification reference is not an installed model. Real PHI and accelerated-runtime promotion remain denied.".to_owned(),
            model_source_summary: "Hugging Face is an artifact source, never an authority plane. Session inventory shows only Core-admitted signed manifest truth; exact external source revision is shown only for the pinned qualification reference where repository evidence proves it.".to_owned(),
            model_inventory_summary: if packs.is_empty() {
                "No Pack is admitted in this Desktop session. Pack registry state is currently session-local, not restart-persistent.".to_owned()
            } else {
                format!("{admitted} Core-admitted Pack(s) in this Desktop session. Restart persistence is not claimed.")
            },
            openmed_baseline: "OpenMed v2.2.0 · pinned commit 59d9cb0a…".to_owned(),
            competitive_summary: "MedScale is structurally stronger in source custody, identity, longitudinal truth, and controlled-action authority. OpenMed remains ahead in production clinical-model breadth, PII/de-ID breadth, and accelerated local inference; Spec 069 proves the portable HF/ONNX path but does not erase those gaps.".to_owned(),
            models,
            evidence,
        }
    }
}

fn pack_row(pack: &PackManifestV0) -> ModelStatusVm {
    let digest = pack.content_digest.to_hex();
    let task = if pack
        .artifacts
        .iter()
        .any(|artifact| artifact.kind == PackArtifactKind::OnnxModel)
    {
        "Local ONNX Pack"
    } else if pack
        .artifacts
        .iter()
        .any(|artifact| artifact.kind == PackArtifactKind::FixtureBytes)
    {
        "Local fixture Pack"
    } else {
        "Local Pack"
    };
    let benchmark = if pack.benchmark_links.is_empty() {
        "No benchmark links declared".to_owned()
    } else {
        format!("Declared evidence · {}", pack.benchmark_links.join(" · "))
    };
    let state = match pack.promotion_state {
        PackPromotionState::Candidate => "CANDIDATE",
        PackPromotionState::Current => "CURRENT",
        PackPromotionState::LastGreen => "LAST GREEN",
        PackPromotionState::Canary => "CANARY",
    };

    ModelStatusVm {
        scope: "SESSION INVENTORY".to_owned(),
        task: task.to_owned(),
        model: format!("{} · v{}", pack.pack_id.as_str(), pack.version),
        source: format!("Signed Pack · rights {}", pack.rights_uri),
        runtime: pack.runtime_requirements.clone(),
        device: runtime_device(&pack.runtime_requirements).to_owned(),
        trust: format!("Core-admitted · trust root {}", pack.trust_root_id),
        benchmark,
        digest,
        state: state.to_owned(),
    }
}

fn runtime_device(runtime: &str) -> &'static str {
    let lower = runtime.to_ascii_lowercase();
    if lower.contains("cpu") || lower.contains("tract") {
        "Local CPU / worker boundary"
    } else {
        "Local worker boundary"
    }
}

#[cfg(test)]
mod tests {
    use super::ProductIntelligenceVm;
    use medscale_contracts::objects::{DigestSha256, OpaqueId};
    use medscale_contracts::packs::{
        PackArtifactEntry, PackArtifactKind, PackManifestV0, PackPromotionState,
    };

    fn manifest(state: PackPromotionState) -> PackManifestV0 {
        PackManifestV0 {
            pack_id: OpaqueId::new("pack-model-center-070"),
            version: "1.2.3".to_owned(),
            pack_epoch: 7,
            content_digest: DigestSha256::of(b"model-center-070"),
            artifacts: vec![PackArtifactEntry {
                relative_path: "model.onnx".to_owned(),
                kind: PackArtifactKind::OnnxModel,
                digest: DigestSha256::of(b"onnx"),
            }],
            rights_uri: "https://example.test/rights".to_owned(),
            sbom_ref: "sbom:test".to_owned(),
            runtime_requirements: "tract-onnx 0.22.4 · portable CPU".to_owned(),
            benchmark_links: vec!["evidence://070/runtime".to_owned()],
            promotion_state: state,
            trust_root_id: "synthetic-pack-trust-v1".to_owned(),
            signature_hex: "00".repeat(64),
        }
    }

    #[test]
    fn empty_inventory_is_explicit_and_qualification_is_not_installed() {
        let vm = ProductIntelligenceVm::current_truth();
        assert_eq!(vm.models[0].scope, "SESSION INVENTORY");
        assert_eq!(vm.models[0].state, "EMPTY");
        assert!(vm.model_inventory_summary.contains("session-local"));
        assert!(vm.models.iter().any(|row| {
            row.scope == "QUALIFICATION REFERENCE" && row.state == "QUALIFIED BASELINE"
        }));
    }

    #[test]
    fn admitted_pack_row_exposes_runtime_trust_digest_benchmark_and_promotion() {
        let pack = manifest(PackPromotionState::Current);
        let expected_digest = pack.content_digest.to_hex();
        let vm = ProductIntelligenceVm::from_packs(&[pack]);
        let row = &vm.models[0];
        assert_eq!(row.scope, "SESSION INVENTORY");
        assert_eq!(row.state, "CURRENT");
        assert!(row.model.contains("pack-model-center-070"));
        assert!(row.runtime.contains("tract-onnx"));
        assert!(row.trust.contains("synthetic-pack-trust-v1"));
        assert_eq!(row.digest, expected_digest);
        assert!(row.benchmark.contains("evidence://070/runtime"));
        assert!(row.source.contains("https://example.test/rights"));
    }

    #[test]
    fn session_projection_reads_core_admitted_pack_inventory() {
        let mut session =
            medscale_core::CliSession::connect_pack_operator("desktop-model-center-test")
                .expect("connect model center test session");
        let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../evidence/008-local-ai-capability-fabric/fixtures/pack-fixture-ner-v0");
        let admitted = session
            .packs_install_local(&fixture.display().to_string())
            .expect("admit signed local fixture through Core");
        assert!(admitted.admitted);

        let denied = session.open_synthetic_vault("/tmp/model-center-must-not-open-vault");
        assert!(
            matches!(
                denied,
                Err(medscale_contracts::envelopes::AuthorityError::SessionDenied)
            ),
            "Model Center session must not inherit broad CLI authority"
        );

        let vm = ProductIntelligenceVm::from_session(&mut session)
            .expect("read Core-backed session inventory");
        let row = &vm.models[0];
        assert_eq!(row.scope, "SESSION INVENTORY");
        assert!(row.model.contains("pack-fixture-ner-v0"));
        assert_ne!(row.state, "EMPTY");
        assert!(!row.digest.is_empty());
    }

    #[test]
    fn current_truth_never_fakes_model_runtime_or_openmed_superiority() {
        let vm = ProductIntelligenceVm::current_truth();
        assert!(
            vm.model_runtime_summary
                .contains("real portable ONNX runtime")
        );
        assert!(vm.models.iter().any(|row| row.state == "MODEL GAP"));
        assert!(vm.evidence.iter().any(|row| row.verdict == "OPENMED AHEAD"));
        assert!(
            vm.evidence
                .iter()
                .any(|row| row.verdict == "PROVEN ADVANTAGE")
        );
        assert!(!vm.competitive_summary.contains("unqualified superiority"));
    }
}
