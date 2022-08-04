use serde::{Deserialize, Serialize};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

use crate::{app::ServiceResult, sqlite};

#[derive(Default)]
struct CodeSnippetState {}

#[derive(Serialize, Deserialize, Debug)]
struct CodeSnipperCatalog {}

// 通过 `invoke('plugin:code_snipper|catalog_list')`. 调用
#[tauri::command]
fn catalog_list() -> ServiceResult<Vec<CodeSnipperCatalog>> {
    ServiceResult::ok(vec![], "")
}

// 代码片段结构
// 节点 -> 代码片段项 -> 代码片段fragment -> 代码片段内容
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("code_snipper")
        .invoke_handler(tauri::generate_handler![catalog_list])
        .setup(|app_handle| {
            app_handle.manage(CodeSnippetState::default());
            Ok(())
        })
        .build()
}

pub fn init_table() {
    let res = sqlite::exec(|conn| {
        // 初始化代码片段目录表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS 'code_snippet_catalog' (
            'catalog_id'          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            'parent_catalog_id'   INTEGER,
            'parent_catalog_path' TEXT,
            'catalog_name'        TEXT,
            'catalog_icon'        TEXT,
            'catalog_description' TEXT,
            'del_flag'            INTEGER)",
            [],
        )?;

        // 初始化代码片段项表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS 'code_snippet' (
            'snippet_id'            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            'catalog_id'            INTEGER,
            'snippet_name'          TEXT,
            'snippet_description'   TEXT,
            'snippet_tag'           TEXT,
            'snippet_main_language' TEXT,
            'del_flag'              INTEGER)",
            [],
        )?;

        // 初始化代码片段内容表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS 'code_snippet_fragment' (
            'fragment_id'   INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            'fragment_name' TEXT,
            'code_content'  TEXT,
            'del_flag'      INTEGER)",
            [],
        )?;
        Ok(())
    });
    print!("{:?}", res)
}
