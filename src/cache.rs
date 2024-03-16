use std::{fs::{create_dir_all, File}, path::Path};
use rusqlite::{params, Connection, OptionalExtension};
use anyhow::Result;

use crate::models::{ApiResponse, CacheData};

pub fn cleanup(conn: &Connection) -> Result<usize> {
    let rows_deleted = conn.execute("
        DELETE FROM cache
        WHERE expiry < strftime('%s','now')
    ", ()
    )?;
    return Ok(rows_deleted)
}

pub fn init() -> Result<Connection> {
    if !Path::new("./.cache/cache.db").exists() {
        create_dir_all("./.cache")?;
        File::create("./.cache/cache.db")?;
    }
    let conn = Connection::open("./.cache/cache.db")?;
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

pub fn get(code: &str, conn: &Connection) -> Result<Option<ApiResponse>> {
    let mut stmt = conn.prepare("SELECT data FROM cache WHERE code = ?1")?;
    let data: Option<CacheData> = stmt.query_row([code], |r| r.get(0)).optional()?;
    if let Some(data) = data {
        return Ok(Some(data.cached_response))
    } else {
        return Ok(None)
    }
}

pub fn add(response: &ApiResponse, conn: &Connection) -> Result<()>{
    let code = &response.base_code;
    let expiry = &response.time_next_update_unix;
    let data = serde_json::to_string(response)?;
    conn.execute("
        INSERT INTO cache (code, expiry, data) 
        VALUES (?1, ?2, ?3);
    ", 
    params![code, expiry, data])?;
    Ok(())
}