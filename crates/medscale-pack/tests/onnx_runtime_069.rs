use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use medscale_pack::{OnnxRuntimeError, OnnxTokenClassifierRuntime, admit_pack_dir};

fn fixture_pack() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../evidence/069-real-local-model-runtime-hf-pack-path/fixtures/pack-tiny-token-classifier-v0",
    )
}

fn temp_copy() -> PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let root = std::env::temp_dir().join(format!(
        "medscale-069-runtime-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&root).expect("create temp pack");
    for name in [
        "model.onnx",
        "tokenizer.json",
        "labels.json",
        "model.meta.json",
        "pack.manifest.json",
    ] {
        fs::copy(fixture_pack().join(name), root.join(name)).expect("copy fixture");
    }
    root
}

#[test]
fn real_onnx_fixture_runs_evidence_only() {
    let root = fixture_pack();
    let manifest = admit_pack_dir(&root).expect("signed pack admits");
    let runtime = OnnxTokenClassifierRuntime::new(4).expect("runtime");
    let output = runtime
        .run(&root, &manifest, "alice visited clinic today")
        .expect("real ONNX inference");

    assert!(output.output.evidence_only);
    assert_eq!(output.output.pack_id, manifest.pack_id);
    assert_eq!(output.provenance.source_kind, "synthetic_fixture");
    assert_eq!(
        output.output.proposal_payload["kind"],
        "onnx_token_classification_v1"
    );
    assert_eq!(output.output.proposal_payload["token_count"], 4);
    let predictions = output.output.proposal_payload["predictions"]
        .as_array()
        .unwrap();
    assert!(predictions.iter().all(|item| item["label"] == "ENTITY"));
}

#[test]
fn post_admission_artifact_swap_fails_closed() {
    let root = temp_copy();
    let manifest = admit_pack_dir(&root).expect("initial admission");
    fs::write(root.join("labels.json"), b"[\"O\"]\n").expect("tamper");
    let runtime = OnnxTokenClassifierRuntime::new(4).expect("runtime");
    assert!(matches!(
        runtime.run(&root, &manifest, "alice visited clinic today"),
        Err(OnnxRuntimeError::ArtifactDigestMismatch)
    ));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn runtime_never_accepts_unsafe_artifact_paths() {
    let root = fixture_pack();
    let mut manifest = admit_pack_dir(&root).expect("admission");
    manifest.artifacts[0].relative_path = "../outside".to_owned();
    let runtime = OnnxTokenClassifierRuntime::new(4).expect("runtime");
    assert!(matches!(
        runtime.run(Path::new(&root), &manifest, "alice visited clinic today"),
        Err(OnnxRuntimeError::UnsafeArtifactPath)
    ));
}

#[cfg(unix)]
#[test]
fn runtime_rejects_post_admission_symlink_escape_even_when_bytes_match() {
    use std::os::unix::fs::symlink;

    let root = temp_copy();
    let manifest = admit_pack_dir(&root).expect("initial admission");
    let outside = std::env::temp_dir().join(format!(
        "medscale-069-runtime-outside-{}-{}",
        std::process::id(),
        uid_for_test()
    ));
    fs::copy(root.join("model.onnx"), &outside).expect("copy outside model");
    fs::remove_file(root.join("model.onnx")).expect("remove in-pack model");
    symlink(&outside, root.join("model.onnx")).expect("create escaping symlink");
    let runtime = OnnxTokenClassifierRuntime::new(4).expect("runtime");
    assert!(matches!(
        runtime.run(&root, &manifest, "alice visited clinic today"),
        Err(OnnxRuntimeError::UnsafeArtifactPath)
    ));
    let _ = fs::remove_dir_all(root);
    let _ = fs::remove_file(outside);
}

#[test]
fn admission_and_runtime_reject_oversized_model_before_loading() {
    let root = temp_copy();
    let manifest = admit_pack_dir(&root).expect("initial admission");
    let model_path = root.join("model.onnx");
    fs::OpenOptions::new()
        .write(true)
        .open(&model_path)
        .expect("open model")
        .set_len(1_073_741_825)
        .expect("create sparse oversized model");

    let admission_error = admit_pack_dir(&root).expect_err("oversized model must be refused");
    assert!(admission_error.to_string().contains("byte bound"));

    let runtime = OnnxTokenClassifierRuntime::new(4).expect("runtime");
    assert!(matches!(
        runtime.run(&root, &manifest, "alice visited clinic today"),
        Err(OnnxRuntimeError::ArtifactBound)
    ));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn pack_admission_rejects_parent_traversal_before_reading_artifact() {
    let root = temp_copy();
    let manifest_path = root.join("pack.manifest.json");
    let mut manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(&manifest_path).expect("manifest bytes"))
            .expect("manifest json");
    manifest["artifacts"][0]["relative_path"] = serde_json::Value::String("../outside".to_owned());
    fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest).expect("serialize manifest") + "\n",
    )
    .expect("write traversal manifest");
    let err = admit_pack_dir(&root).expect_err("parent traversal must be refused");
    assert!(err.to_string().contains("escapes pack root"));
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn pack_admission_rejects_symlink_escape_even_when_bytes_match() {
    use std::os::unix::fs::symlink;

    let root = temp_copy();
    let outside = std::env::temp_dir().join(format!(
        "medscale-069-outside-{}-{}",
        std::process::id(),
        uid_for_test()
    ));
    fs::copy(root.join("model.onnx"), &outside).expect("copy outside model");
    fs::remove_file(root.join("model.onnx")).expect("remove in-pack model");
    symlink(&outside, root.join("model.onnx")).expect("create escaping symlink");
    let err = admit_pack_dir(&root).expect_err("symlink escape must be refused");
    assert!(err.to_string().contains("escapes pack root"));
    let _ = fs::remove_dir_all(root);
    let _ = fs::remove_file(outside);
}

#[cfg(unix)]
fn uid_for_test() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(10_000);
    NEXT.fetch_add(1, Ordering::Relaxed)
}
