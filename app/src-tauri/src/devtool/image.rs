use crate::app::ServiceResult;

#[tauri::command]
pub fn image_extractor(val: &str) -> ServiceResult<String> {
    ServiceResult::ok(String::from(val))
}
