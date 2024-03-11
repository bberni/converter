use rusqlite::types::{FromSql, FromSqlResult, FromSqlError, ValueRef};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    result: String,
    documentation: String,
    terms_of_use: String,
    time_last_update_unix: u64,
    time_last_update_utc: String,
    pub time_next_update_unix: u64,
    time_next_update_utc: String,
    pub base_code: String,
    pub conversion_rates: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponseError {
    result: String,
    documentation: String,
    terms_of_use: String,
    pub error_type: String
}

#[derive(Serialize, Deserialize)]
pub struct CacheData {
    pub cached_response: ApiResponse
}

impl FromSql for CacheData {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(text) => {
                let parsed: Result<ApiResponse, serde_json::Error> = serde_json::from_slice(text);
                match parsed {
                    Ok(api_response) => Ok(CacheData { cached_response: api_response }),
                    Err(e) => Err(FromSqlError::Other(Box::new(e))),
                }
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}