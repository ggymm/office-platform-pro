use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

lazy_static! {
    pub static ref APP: Mutex<App> = Mutex::new(App::new());
}

// 初始化文件目录
pub fn init_app_dir() -> bool {
    // C:/Users/19679/AppData/Roaming/office-platform
    if !Path::new(&APP.lock().app_dir).exists() {
        if let Ok(_) = create_dir_all(&APP.lock().app_dir) {
            return true;
        }
        return false;
    }
    true
}

pub struct App {
    pub app_dir: PathBuf,
}

impl App {
    pub fn new() -> App {
        let cfg = tauri::Config::default();
        match tauri::api::path::app_dir(&cfg) {
            None => App {
                app_dir: PathBuf::new(),
            },
            Some(path) => App {
                app_dir: path.join("office-platform"),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServiceResult<T> {
    pub success: bool,
    pub code: i32,
    pub data: T,
    pub message: String,
}

impl<T> ServiceResult<T> {
    pub fn new(success: bool, code: i32, data: T, message: &str) -> ServiceResult<T> {
        ServiceResult {
            success,
            code,
            data,
            message: message.to_string(),
        }
    }

    pub fn ok(data: T) -> Self {
        Self::new(true, 200, data, "")
    }

    pub fn err(data: T, message: &str) -> Self {
        Self::new(false, 500, data, message)
    }
}
