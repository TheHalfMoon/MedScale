#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;

mod navigation_policy;

#[cfg(test)]
mod permission_tests;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ShellStatus {
    schema_version: u8,
    core_connection: &'static str,
    detail: &'static str,
    synthetic_only: bool,
}

#[tauri::command]
fn get_shell_status() -> ShellStatus {
    ShellStatus {
        schema_version: 1,
        core_connection: "unavailable",
        detail: "The Core Host is not connected in this preparatory Tauri build.",
        synthetic_only: true,
    }
}

fn presentation_builder<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    builder.invoke_handler(tauri::generate_handler![get_shell_status])
}

fn main() {
    presentation_builder(tauri::Builder::default())
        .setup(|app| {
            let config = app
                .config()
                .app
                .windows
                .first()
                .ok_or("missing main window")?;
            tauri::WebviewWindowBuilder::from_config(app, config)?
                .on_navigation(|url| {
                    navigation_policy::is_local_application_url(url, cfg!(debug_assertions))
                })
                .on_new_window(|_, _| tauri::webview::NewWindowResponse::Deny)
                .incognito(true)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to start the synthetic-only MedScale Tauri shell");
}
