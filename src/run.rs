use crate::models::Results;
use crate::utils::{convert_currency, get_exchange_data, parse_amount, parse_code, read_line};
use anyhow::Result;
use rusqlite::Connection;
use std::io::{stdout, Write};

pub fn run_interactive(conn: &Connection, api_key: &str) -> Result<Results> {
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
    return run_with_arguments(from_currency, to_currency, amount, conn, api_key);
}

pub fn run_with_arguments(
    from_currency: String,
    to_currency: String,
    amount: f64,
    conn: &Connection,
    api_key: &str,
) -> Result<Results> {
    let api_data = get_exchange_data(&from_currency, &conn, api_key)?;
    let converted_amount = convert_currency(amount, &to_currency, api_data)?;
    return Ok(Results {
        from_currency,
        amount,
        to_currency,
        converted_amount,
    });
}
