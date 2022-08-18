use std::fs;

use crate::app::ServiceResult;
use base64::{decode, encode};

#[tauri::command]
pub fn base64_encode(val: &str) -> ServiceResult<String> {
    let bytes = val.as_bytes();
    ServiceResult::ok(encode(bytes))
}

#[tauri::command]
pub fn base64_file_encode(filepath: &str) -> ServiceResult<String> {
    let res = fs::read(filepath);
    match res {
        Ok(buf) => ServiceResult::ok(encode(buf)),
        Err(err) => ServiceResult::err(String::from(""), &err.to_string()),
    }
}

#[tauri::command]
pub fn base64_decode(val: &str) -> ServiceResult<String> {
    let res = decode(val);
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

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn base64_encode_test() {
        let res = super::base64_encode("Hello Base64");
        if res.success {
            print!("{}", res.data)
        } else {
            print!("{:?}", res)
        }
    }

    #[test]
    fn base64_file_encode_test() {
        let res = super::base64_file_encode("D:\\Temp\\test.png");
        if res.success {
            // 输出到文件中
            fs::write("D:\\Temp\\base64.txt", res.data).unwrap()
        } else {
            print!("{:?}", res)
        }
    }
}
