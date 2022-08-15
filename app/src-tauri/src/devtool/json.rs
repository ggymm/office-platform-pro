use base64::encode;

use crate::app::ServiceResult;

#[tauri::command]
pub fn json_to_tree(txt: &str) -> ServiceResult<String> {
    let bytes = txt.as_bytes();
    ServiceResult::ok(encode(bytes))
}
