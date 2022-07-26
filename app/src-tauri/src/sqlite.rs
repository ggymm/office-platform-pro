use anyhow::Result;
use std::path::PathBuf;

use rusqlite::Connection;

use crate::app;

// 获取sqlite连接对象
fn create_sqlite_connection() -> Result<Connection> {
    let database = PathBuf::from(&app::APP.lock().app_dir).join("app.db");
    let conn = Connection::open(database)?;
    Ok(conn)
}

// 执行sql
fn exec<F, T>(func: F) -> Result<T>
where
    F: FnOnce(&mut Connection) -> Result<T>,
{
    match create_sqlite_connection() {
        Ok(mut conn) => func(&mut conn),
        Err(e) => Err(e),
    }
}

pub fn init_table() {
    // 初始化代码片段表
    //
    let res = exec(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS 'snippet_tree_node' (
                'node_id'          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                'parent_node_id'   INTEGER,
                'parent_node_path' TEXT,
                'node_name'        TEXT,
                'node_icon'        TEXT,
                'node_description' TEXT,
                'del_flag'         INTEGER
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS 'snippet_tree_node_1' (
               'node_id'          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
               'parent_node_path' TEXT,
               'parent_node_id'   INTEGER,
               'node_icon'        TEXT,
               'node_name'        TEXT,
               'del_flag'         INTEGER
               'node_description' TEXT,
            )",
            [],
        )?;
        Ok(())
    });
    print!("{:?}", res)
}
