use anyhow::Result;
use clap::{command, Arg, ArgAction, ArgMatches};
use converter::run_interactive;
use converter::run_with_arguments;


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

    let results = match match_result.get_flag("interactive") {
        true => {
            run_interactive()?
        }
        false => {run_with_arguments("PLN".to_string(), "USD".to_string(), 10.00)?}
    };
    println!("{:.2} {} -> {:.2} {}", results.amount, results.from_currency, results.converted_amount, results.to_currency);
    Ok(())
}
