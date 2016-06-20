extern crate weather;
#[macro_use]
extern crate clap;

use weather::wunderground::*;

use std::error::Error;
use std::fmt::Display;

static WUNDERGROUND_ABOUT: &'static str = "Wunderground weather source, requires WUNDERGROUND_ACCESS_TOKEN and WUNDERGROUND_LOCATION environment variables to be set";

fn main() {
    let matches = clap_app!(weather =>
        (version: crate_version!())
        (about: "Weather forecasting CLI tool")
        (setting: clap::AppSettings::SubcommandRequiredElseHelp)
        (@subcommand w =>
            (about: WUNDERGROUND_ABOUT)
            (setting: clap::AppSettings::SubcommandRequiredElseHelp)

            (@subcommand c =>
                (about: "Current conditions"))

            (@subcommand h =>
                 (about: "Hourly forecast")
                 (@arg days: -d +takes_value "Number of days to display")))
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("w") {
        process_wunderound(matches);
    }
}

fn process_wunderound(matches: &clap::ArgMatches) {
    let wunderground = Wunderground::new(None);

    if let Some(_) = matches.subcommand_matches("c") {
        print_forecast(wunderground.current_conditions());
    }

    if let Some(matches) = matches.subcommand_matches("h") {
        let days = matches.value_of("days").unwrap_or("10")
            .parse::<u8>().unwrap_or(10);

        print_forecast(wunderground.hourly_forecast().map(|forecast| {
            forecast.days(days)
        }));
    }
}

fn print_forecast<T: Display>(forecast: Result<T, Box<Error>>) {
    match forecast {
        Ok(forecast) => println!("{}", forecast),
        Err(err) => println!("{}", err)
    }
}
