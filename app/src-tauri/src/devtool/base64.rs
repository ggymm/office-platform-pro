use base64::{decode, encode};

use crate::app::ServiceResult;

#[tauri::command]
pub fn base64_encode(txt: &str) -> ServiceResult<String> {
    let bytes = txt.as_bytes();
    ServiceResult::ok(encode(bytes))
}

#[tauri::command]
pub fn base64_decode(txt: &str) -> ServiceResult<String> {
    let res = decode(txt);
    match res {
        Ok(val) => {
            let format_res = String::from_utf8(val);
            match format_res {
                Ok(format_val) => ServiceResult::ok(format_val),
                Err(err) => ServiceResult::err(String::from(""), &err.to_string()),
            }
        }
        Err(err) => ServiceResult::err(String::from(""), &err.to_string()),
    }
}
