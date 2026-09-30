fn main() {
    let attributes = tauri_build::Attributes::new()
        .app_manifest(tauri_build::AppManifest::new().commands(&["get_shell_status"]));
    tauri_build::try_build(attributes).expect("failed to build the constrained Tauri manifest");
}
