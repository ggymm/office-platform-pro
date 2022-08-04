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
pub fn exec<F, T>(func: F) -> Result<T>
where
    F: FnOnce(&mut Connection) -> Result<T>,
{
    match create_sqlite_connection() {
        Ok(mut conn) => func(&mut conn),
        Err(e) => Err(e),
    }
}

pub fn init_table() {}
