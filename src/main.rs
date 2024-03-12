mod models;
mod errors;
mod cache;

use reqwest::{self, blocking::Response, StatusCode};
use errors::{RequestError, ApiError, ConversionError};
use anyhow::Result;
use rusqlite::Connection;
use models::{ApiResponse, ApiResponseError};
use clap::{command, Arg, ArgAction, ArgMatches};

const API_KEY: &str = "cf1879d2cbcc3030df333526";

fn get_response(user_currency: &str) -> Result<Response> {
    println!("[+] Fetching data from API...");
     match reqwest::blocking::get(
        format!("https://v6.exchangerate-api.com/v6/{}/latest/{}", API_KEY, user_currency)
    ) {
        Ok(r) => return Ok(r),
        Err(e) => return Err(RequestError::RequestError(e).into())
    }
}

fn parse_response(user_currency: &str, r: Response, conn: &Connection) -> Result<ApiResponse> {
    match r.status() {
        StatusCode::OK => {
            let data: ApiResponse = serde_json::from_str(&r.text()?)?;
            cache::add(&data, conn).unwrap_or_else(|e| {
                println!("[!] Cannot cache recieved response: {}", e)
            });
            return Ok(data)
        },
        _ => {
            let text = &r.text()?;
            println!("{:?}", text);
            let data: ApiResponseError = serde_json::from_str(text)?;
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

fn get_exchange_data(user_currency: &str, conn: &Connection) -> Result<ApiResponse> {
    match cache::cleanup(conn) {
        Ok(_) => {
            let response = match cache::get(user_currency, conn) {
                Ok(Some(cache_response)) => {
                    println!("[+] Using data from cache...");
                    return Ok(cache_response)
                },
                Ok(None) => get_response(user_currency)?,
                Err(e) => {
                    println!("[!] Error getting data from cache: {}, continuing with data from API", e);
                    get_response(user_currency)?
                }
            };
            return parse_response(user_currency, response, conn)
        },
        Err(e) => {
            println!("[!] Cannot clear out old data from cache: {}, continuing with data from API", e);
            return parse_response(user_currency, get_response(user_currency)?, conn)
        }
    };
}

fn convert_currency(amount: f64, target_currency: String, api_data: ApiResponse) -> Result<f64> {
    let conversion_rate = if let Some(rate) = api_data.conversion_rates.get(&target_currency) {
        rate
    } else {
        return Err(ConversionError::CurrencyNotFound(api_data.base_code, target_currency).into())
    };
    
    return Ok(((amount * conversion_rate) * 100 as f64).floor() / 100 as f64)
}

fn parse_args() -> ArgMatches {
    command!()
    .arg(
        Arg::new("interactive").short('i').long("interactive")
        .action(ArgAction::SetTrue)
        .exclusive(true)
    )
    .arg(Arg::new("list").short('l').long("list").num_args(1).value_name("currency code")
        .exclusive(true)
    )
    .arg(
        Arg::new("from-currency")
        .required_unless_present_any(["interactive", "list"])
    )
    .arg(
        Arg::new("to-currency")
        .required_unless_present_any(["interactive", "list"])
    )
    .arg(
        Arg::new("amount")
        .required_unless_present_any(["interactive", "list"])
    ).get_matches()
}
fn main() -> Result<()> {
    let match_result = parse_args();

    let is_interactive = match_result.get_flag("interactive");



    let conn = cache::init()?;
    let user_currency = "PLN";
    let target_currency = "GBP";
    let amount: f64 = 10.00;
    let api_data = get_exchange_data(user_currency, &conn)?;
    let final_amount = convert_currency(amount, target_currency.to_string(), api_data)?;
    println!("{:.2} {} -> {:.2} {}", amount, user_currency, final_amount, target_currency);
    Ok(())
}
