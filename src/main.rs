//#![allow(dead_code)]

#[macro_use]
extern crate clap;
extern crate chrono;
extern crate nimtool;

fn main() {
    use clap::App;
    use chrono::prelude::*;
    use nimtool::app::*;

    // TODO: implement more languages
    // TODO: refactor to handle the case
    // TODO: when a language isn't supported
    // Pull in our YAML file depending on chosen language
    let yml = {
        #[cfg(feature = "en")]
        {load_yaml!("../lang/english.cli.yml").to_owned()}

        #[cfg(feature = "es")]
        {load_yaml!("../lang/spanish.cli.yml").to_owned()}

        // default to english
        //{load_yaml!("../lang/english.cli.yml").to_owned()}
    };

    // Our language specific YAML file with CLI options has
    // been loaded so create a new App instance
    let matches = App::from_yaml(&yml)
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        //.about(crate_description!())
        .get_matches();

    // The --price (-p) flag has been given it takes these possible values
    // current, day, week, month and year
    if let Some(price) = matches.value_of("price") {
        // Lets grab the price data for `current`
        let getdata = get_price_data();

        match price {
            // Each price takes a --currency (-c) flag, lots of choices
            // I'm currently only implementing a handful
            // usd, eur, cny and btc
            "current" => if let Some(currency) = matches.value_of("currency") {
                match currency {
                    //##############################
                    // PRICE
                    // CURRENT
                    // USD
                    //##############################
                    "usd" => match getdata {
                        Ok(s) => println!(
                            // TODO: implement proper language abstraction
                            "{}: {} - Percent change 1 hour: {} - Percent change 24 hour: {}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.usd,
                            s.percent_change_1h.usd,
                            s.percent_change_24h.usd
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // EUR
                    //##############################
                    "eur" => match getdata {
                        Ok(s) => println!(
                            // TODO: implement proper language abstraction
                            "{}: {} - Percent change 1 hour: {} - Percent change 24 hour: {}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.eur,
                            s.percent_change_1h.eur,
                            s.percent_change_24h.eur
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // CNY
                    //##############################
                    "cny" => match getdata {
                        Ok(s) => println!(
                            // TODO: implement proper language abstraction
                            "{}: {} - Percent change 1 hour: {} - Percent change 24 hour: {}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.cny,
                            s.percent_change_1h.cny,
                            s.percent_change_24h.cny
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // BTC
                    //##############################
                    "btc" => match getdata {
                        Ok(s) => println!(
                            // TODO: implement proper language abstraction
                            "{}: {} - Percent change 1 hour: {} - Percent change 24 hour: {}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.btc,
                            s.percent_change_1h.btc,
                            s.percent_change_24h.btc
                        ),
                        Err(e) => println!("{}", e),
                    },
                    _ => unreachable!(),
                }; // end of currency flag
            }, // end of price flag
            "placeholder" => println!("placeholder"),
            _ => unreachable!(),
        }
    } else {
        println!("You must supply arguments. See --help.");
    }
}
