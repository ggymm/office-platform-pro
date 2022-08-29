use std::collections::HashMap;

use crate::app::ServiceResult;

#[tauri::command]
pub fn number_base_convert(num: &str, num_base: u32) -> ServiceResult<HashMap<i32, String>> {
    let mut map = HashMap::new();
    if num_base < 2 || num_base > 36 {
        return ServiceResult::err(map, "参数错误");
    }
    match isize::from_str_radix(num, num_base) {
        Ok(a) => {
            map.insert(2, format!("{:b}", a));
            map.insert(8, format!("{:o}", a));
            map.insert(10, format!("{}", a));
            map.insert(16, format!("{:X}", a));
            ServiceResult::ok(map)
        }
        Err(err) => ServiceResult::err(map, &err.to_string()),
    }
}
