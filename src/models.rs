use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    result: String,
    documentation: String,
    terms_of_use: String,
    time_last_update_unix: u64,
    time_last_update_utc: String,
    time_next_update_unix: u64,
    time_next_update_utc: String,
    base_code: String,
    conversion_rates: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponseError {
    result: String,
    documentation: String,
    terms_of_use: String,
    pub error_type: String
}
