mod models;
mod errors;
mod cache;

use anyhow::Result;
use clap::{command, Arg, ArgAction, ArgMatches};
use crate::{}
const API_KEY: &str = "cf1879d2cbcc3030df333526";


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
