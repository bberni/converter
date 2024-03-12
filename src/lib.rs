mod models;
mod errors;
pub mod cache;

use std::{collections::HashMap, io::{stdin, stdout, Write}};
use models::Results;
use reqwest::{self, blocking::Response, StatusCode};
use errors::{RequestError, ApiError, ConversionError, InputError};
use anyhow::Result;
use rusqlite::Connection;
use crate::models::{ApiResponse, ApiResponseError};

const API_KEY: &str = "cf1879d2cbcc3030df333526";

fn get_response(from_currency: &String) -> Result<Response> {
    println!("[+] Fetching data from API...");
     match reqwest::blocking::get(
        format!("https://v6.exchangerate-api.com/v6/{}/latest/{}", API_KEY, from_currency)
    ) {
        Ok(r) => return Ok(r),
        Err(e) => return Err(RequestError::RequestError(e).into())
    }
}

fn parse_response(from_currency: &String, r: Response, conn: &Connection) -> Result<ApiResponse> {
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
                "unsupported-code" => ApiError::UnsupportedCode(from_currency.to_owned()),
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

pub fn get_exchange_data(from_currency: &String, conn: &Connection) -> Result<ApiResponse> {
    match cache::cleanup(conn) {
        Ok(_) => {
            let response = match cache::get(&from_currency, conn) {
                Ok(Some(cache_response)) => {
                    println!("[+] Using data from cache...");
                    return Ok(cache_response)
                },
                Ok(None) => get_response(&from_currency)?,
                Err(e) => {
                    println!("[!] Error getting data from cache: {}, continuing with data from API", e);
                    get_response(&from_currency)?
                }
            };
            return parse_response(&from_currency, response, conn)
        },
        Err(e) => {
            println!("[!] Cannot clear out old data from cache: {}, continuing with data from API", e);
            return parse_response(&from_currency, get_response(&from_currency)?, conn)
        }
    };
}

fn convert_currency(amount: f64, to_currency: &String, api_data: ApiResponse) -> Result<f64> {
    let conversion_rate = if let Some(rate) = api_data.conversion_rates.get(to_currency) {
        rate
    } else {
        return Err(ConversionError::CurrencyNotFound(api_data.base_code, to_currency.clone()).into())
    };
    
    return Ok(((amount * conversion_rate) * 100 as f64).floor() / 100 as f64)
}

fn read_line() -> Result<String> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)
        .map_err(|e| InputError::ReadLineError(e.to_string()))?;
    return Ok(buffer)
}

pub fn parse_code(buffer: &String) -> Result<String> {
    if !buffer.trim().chars().all(|c| c.is_uppercase()) {
        return Err(InputError::InvalidCode().into())
    }
    return Ok(buffer.trim().to_string())
}

pub fn parse_amount(buffer: &String) -> Result<f64> {
    match buffer.trim().parse::<f64>() {
        Ok(amount) => {
            if amount > 0 as f64 {
                return Ok(amount)
            } else {
                return Err(InputError::InvalidAmount().into())
            }
        },
        Err(_) => return  Err(InputError::InvalidAmount().into())
    }
}

pub fn run_interactive(conn: &Connection) -> Result<Results> {
    println!("Welcome to the currency converter tool.");
    print!("Enter the code of currency that you want to convert from: ");
    stdout().flush()?;
    let from_currency = parse_code(&read_line()?)?;
    print!("Enter the code of currency that you want to convert to: ");
    stdout().flush()?;
    let to_currency = parse_code(&read_line()?)?;
    print!("Enter the amount of money that you want to convert: ");
    stdout().flush()?;
    let amount = parse_amount(&read_line()?)?;
    return run_with_arguments(from_currency, to_currency, amount, conn)
}

pub fn run_with_arguments(from_currency: String, to_currency: String, amount: f64, conn: &Connection) -> Result<Results> {
    let api_data = get_exchange_data(&from_currency, &conn)?;
    let converted_amount = convert_currency(amount, &to_currency, api_data)?;
    return Ok(Results {from_currency, amount, to_currency, converted_amount}) 
}
