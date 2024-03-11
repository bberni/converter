use std::{fs::File, path::Path};
use rusqlite::Connection;
use anyhow::Result;

use crate::models::{ApiResponse, CacheData};

pub fn cleanup(conn: &Connection) -> Result<usize> {
    let rows_deleted = conn.execute("
        DELETE FROM cache
        WHERE expiry < strftime('%s','now');, params)
    ",
    ()
    )?;
    return Ok(rows_deleted)
}

pub fn init() -> Result<Connection> {
    if !Path::new("./cache.db").exists() {
        File::create("./cache.db")?;
    }
    let conn = Connection::open("cache.db")?;
    conn.execute("
        CREATE TABLE IF NOT EXISTS cache (
            code TEXT,
            expiry INTEGER,
            data TEXT
        )
      ",
      ()
    )?;
    return Ok(conn)
}

pub fn get(code: &str, conn: &Connection) -> Result<ApiResponse> {
    let mut stmt = conn.prepare("SELECT data FROM cache WHERE code = ?1")?;
    let data: CacheData = stmt.query_row(&[code], |r| r.get(0))?;
    return Ok(data.cached_response);
}