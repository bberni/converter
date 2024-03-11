mod models;
mod errors;
use reqwest::{self, blocking::Response, StatusCode};
use errors::{RequestError, ApiError};
use anyhow::Result;
use std::process::exit;
use models::{ApiResponse, ApiResponseError};

const API_KEY: &str = "cf1879d2cbcc3030df333526";

fn get_response(user_currency: &str) -> Result<Response> {
     match reqwest::blocking::get(
        format!("https://v6.exchangerate-api.com/v6/{}/latest/{}", API_KEY, user_currency)
    ) {
        Ok(r) => return Ok(r),
        Err(e) => return Err(RequestError::RequestError(e).into())
    }
}

fn parse_response(user_currency: &str, r: Response) -> Result<ApiResponse> {
    match r.status() {
        StatusCode::OK => {
            let data: ApiResponse = serde_json::from_str(&r.text()?)?;
            return Ok(data)
        },
        _ => {
            let data: ApiResponseError = serde_json::from_str(&r.text()?)?;
            let error = match data.error_type.as_str() {
                "unsupported-code" => ApiError::UnsupportedCode(user_currency.to_owned()),
                "malformed-request" => ApiError::MalformedRequest(),
                "invalid-key" => ApiError::InvalidKey(),
                "inactive-account" => ApiError::InactitveAccount(),
                "quota-reached" => ApiError::QuotaReached(),
                e => ApiError::UnknownError(e.to_string())
            };
            return Err(error.into())
        }
    }
}

fn get_exchange_data(user_currency: &str) -> Result<ApiResponse> {
    let response = get_response(user_currency)?;
    return parse_response(user_currency, response)
}

fn convert_currency(target_currency: &str, api_data: &ApiResponse) -> Result<f64> {
    todo!()
}
fn main() {
    let user_currency = "PLN";
    let target_currency = "USD";
    let amount: f64 = 15.50;
    let api_data = get_exchange_data(user_currency).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });
   println!("{:?}", api_data)
}
