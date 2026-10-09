//! Spec 104: a catalog row whose snapshot ships `model.safetensors` builds a
//! signed candle-runtime Pack, admits, prepares and runs. Synthetic weights
//! (randomly initialised tiny BERT) and synthetic Hub metadata; no network.

use std::path::PathBuf;

use candle_core::{DType, Device};
use candle_nn::{VarBuilder, VarMap};
use candle_transformers::models::bert;
use medscale_pack::{
    CANDLE_TOKEN_CLASSIFIER_RUNTIME_ID, CandleRunOptions, HfRepoMetadata, ModelCatalog,
    TokenClassifierSnapshot, admit_pack_dir, build_token_classifier_pack,
    prepare_candle_token_classifier, reproducibility_hash,
};
use serde_json::json;
use sha2::{Digest, Sha256};

const COMMIT: &str = "ea920f36fadd7b45935247d639f0ffa1ef493b23";
const SHA: &str = "3c914174cf099fda326b43084f679ad31425f88c";
const REPO: &str = "OpenMed/Synthetic-NER-safetensors";

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn git_blob(b: &[u8]) -> String {
    let mut h = sha1::Sha1::new();
    h.update(format!("blob {}\0", b.len()).as_bytes());
    h.update(b);
    hex(&h.finalize())
}

fn dir(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("medscale-104-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

#[test]
fn safetensors_snapshot_builds_admits_prepares_and_runs() {
    let config = json!({
        "model_type": "bert",
        "architectures": ["BertForTokenClassification"],
        "vocab_size": 8, "hidden_size": 8, "num_hidden_layers": 1,
        "num_attention_heads": 2, "intermediate_size": 16, "hidden_act": "gelu",
        "hidden_dropout_prob": 0.0, "max_position_embeddings": 16, "type_vocab_size": 2,
        "initializer_range": 0.02, "layer_norm_eps": 1e-12, "pad_token_id": 0,
        "classifier_dropout": null,
        "id2label": {"0": "O", "1": "B-X"}
    });
    let config_bytes = serde_json::to_vec(&config).unwrap();
    let tokenizer = std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
            "../../evidence/069-real-local-model-runtime-hf-pack-path/fixtures/pack-tiny-token-classifier-v0/tokenizer.json",
        ),
    )
    .unwrap();
    let snapshot = dir("snapshot");
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &Device::Cpu);
    let cfg: bert::Config = serde_json::from_value(config.clone()).unwrap();
    bert::BertModel::load(vb.pp("bert"), &cfg).unwrap();
    candle_nn::linear(8, 2, vb.pp("classifier")).unwrap();
    varmap.save(snapshot.join("model.safetensors")).unwrap();
    let weights = std::fs::read(snapshot.join("model.safetensors")).unwrap();
    std::fs::write(snapshot.join("config.json"), &config_bytes).unwrap();
    std::fs::write(snapshot.join("tokenizer.json"), &tokenizer).unwrap();

    let meta_json = json!({
        "id": REPO, "sha": SHA, "lastModified": "2026-07-13T07:43:16.000Z",
        "gated": false, "private": false,
        "siblings": [
            {"rfilename": "config.json", "size": config_bytes.len(), "blobId": git_blob(&config_bytes)},
            {"rfilename": "model.safetensors", "size": weights.len(),
             "lfs": {"sha256": hex(&Sha256::digest(&weights)), "size": weights.len()}},
            {"rfilename": "tokenizer.json", "size": tokenizer.len(), "blobId": git_blob(&tokenizer)}
        ]
    });
    let meta: HfRepoMetadata = serde_json::from_value(meta_json).unwrap();
    let row = format!(
        r#"{{"repo_id":"{REPO}","family":"NER","task":"token-classification","architecture":"bert","formats":["pytorch"],"license":"apache-2.0","reproducibility_hash":"{}"}}"#,
        reproducibility_hash(&meta)
    );
    let catalog = ModelCatalog::import("maziyarpanahi/openmed", COMMIT, row.as_bytes()).unwrap();

    let out = dir("pack");
    let pack_id = build_token_classifier_pack(
        &TokenClassifierSnapshot {
            meta: &meta,
            row: catalog.get(REPO).unwrap(),
            snapshot_dir: &snapshot,
            onnx_file: "model.safetensors",
            fixed_sequence_length: 16,
        },
        &out,
    )
    .expect("verified safetensors snapshot builds a Pack");
    assert_eq!(pack_id, "hf-synthetic-ner-safetensors-safetensors");
    assert!(out.join("config.json").is_file());
    assert!(!out.join("model.onnx").exists());

    let manifest = admit_pack_dir(&out).expect("signed Pack admits");
    assert!(
        manifest
            .runtime_requirements
            .starts_with(CANDLE_TOKEN_CLASSIFIER_RUNTIME_ID)
    );
    let prepared = prepare_candle_token_classifier(&out, &manifest).expect("prepares");
    assert_eq!(prepared.provenance().export_format, "safetensors");
    assert_eq!(prepared.provenance().revision, SHA);
    let output = prepared
        .run_document(
            &manifest.pack_id,
            "alice visited clinic today",
            CandleRunOptions::default(),
        )
        .expect("runs");
    assert!(output.evidence_only);
    assert_eq!(output.proposal_payload["covered_tokens"], 4);

    // A tampered weight file fails closed at prepare.
    let mut tampered = std::fs::read(out.join("model.safetensors")).unwrap();
    let last = tampered.len() - 1;
    tampered[last] ^= 0xFF;
    std::fs::write(out.join("model.safetensors"), tampered).unwrap();
    assert!(prepare_candle_token_classifier(&out, &manifest).is_err());

    let _ = std::fs::remove_dir_all(snapshot);
    let _ = std::fs::remove_dir_all(out);
}
