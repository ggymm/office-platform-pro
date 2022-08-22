use base64::encode;

use crate::app::ServiceResult;

#[tauri::command]
pub fn url_encode(val: &str) -> ServiceResult<String> {
    let bytes = val.as_bytes();
    ServiceResult::ok(encode(bytes))
}
