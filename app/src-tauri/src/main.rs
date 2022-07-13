#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

pub mod code_snippet;
pub mod socket_client;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            Ok(())
        })
        .plugin(socket_client::TauriLibSocketClient::new())
        .run(tauri::generate_context!())
        .expect("error while running coder-desktop app");
}
