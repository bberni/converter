mod cache;
mod errors;
mod models;
mod run;
mod utils;

use anyhow::Result;
use clap::{command, Arg, ArgAction, ArgMatches};
use errors::ApiKeyError;
use run::{run_interactive, run_with_arguments};
use std::env;
use utils::{get_exchange_data, parse_amount, parse_code};

fn parse_args() -> ArgMatches {
    command!()
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .action(ArgAction::SetTrue)
                .exclusive(true)
                .help("Starts the program in the interactive mode"),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .num_args(1)
                .value_name("currency code")
                .exclusive(true)
                .help("Lists out all the conversion rates for the specified currency code"),
        )
        .arg(
            Arg::new("from-currency")
                .required_unless_present_any(["interactive", "list"])
                .help("Code of currency that you want to convert money from"),
        )
        .arg(
            Arg::new("to-currency")
                .required_unless_present_any(["interactive", "list"])
                .help("Code of currency that you want to convert money to"),
        )
        .arg(
            Arg::new("amount")
                .required_unless_present_any(["interactive", "list"])
                .help("Amount of money to be converted"),
        )
        .get_matches()
}

fn get_api_key() -> Result<String> {
    match env::var("EXCHANGERATE_API_KEY") {
        Ok(key) => return Ok(key),
        Err(_) => return Err(ApiKeyError::KeyNotFound().into()),
    }
}
fn main() -> Result<()> {
    let conn = cache::init()?;
    let match_result = parse_args();
    let api_key = get_api_key()?;

    if let Some(currency_code) = match_result.get_one::<String>("list") {
        let data = get_exchange_data(&parse_code(currency_code)?, &conn, &api_key)?;
        println!("Listing conversion rates for {}:", &currency_code);
        data.conversion_rates.into_iter().for_each(|(key, value)| {
            println!("{}: {}", key, value);
        });
    } else {
        let results = match match_result.get_flag("interactive") {
            true => run_interactive(&conn, &api_key)?,
            false => {
                // i've decided to use unwrap here to make code shorter and clearer
                // the program shouldn't ever panic anyway, because clap will not allow for it to run this far
                // if arguments are not provided
                let from_currency =
                    parse_code(match_result.get_one::<String>("from-currency").unwrap())?;
                let to_currency =
                    parse_code(match_result.get_one::<String>("to-currency").unwrap())?;
                let amount = parse_amount(match_result.get_one::<String>("amount").unwrap())?;
                run_with_arguments(from_currency, to_currency, amount, &conn, &api_key)?
            }
        };
        println!("{}", results);
    }
    Ok(())
}
