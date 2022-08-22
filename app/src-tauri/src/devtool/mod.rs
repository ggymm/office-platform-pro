use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub mod base64;
pub mod json;
pub mod url;

#[derive(Default)]
struct DevBoxState {}

// 通过 `invoke('plugin:devtool|test_command')`. 调用
#[tauri::command]
fn test_command(args: &str) -> String {
    format!("Hello, {}!", args)
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("devtool")
        .invoke_handler(tauri::generate_handler![
            json::json_to_tree,
            base64::base64_encode,
            base64::base64_file_encode,
            base64::base64_decode,
            url::url_encode,
            test_command
        ])
        .setup(|app_handle| {
            app_handle.manage(DevBoxState::default());
            Ok(())
        })
        .build()
}
