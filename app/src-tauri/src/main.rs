#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod app;
pub mod sqlite;

pub mod devtool;
pub mod socket_client;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            if !app::init_app_dir() {
                panic!("工作目录初始化失败！");
            }

            sqlite::init_table();
            Ok(())
        })
        .plugin(devtool::init()) // 注册开发工具箱插件
        .plugin(socket_client::TauriLibSocketClient::new())
        .run(tauri::generate_context!())
        .expect("error while running coder-desktop app");
}
