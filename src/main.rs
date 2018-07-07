//#![allow(dead_code)]

#[macro_use]
extern crate clap;
extern crate chrono;
extern crate nimtool;

fn main() {
    use clap::App;
    use chrono::prelude::*;
    use nimtool::app::*;


    // Pull in our yaml file depending on system language
    // also load up the proper fluent context
    let yml = if env!("LANG") == "en_US.UTF-8" {
        load_yaml!("../lang/english.cli.yaml").to_owned()
    } else {
        panic!()
    };

    let matches = App::from_yaml(&yml).get_matches();

    // The price flag has been given
    if let Some(price) = matches.value_of("price") {
        // Lets grab the price data for `current`
        let getdata = get_data();

        match price {
            "current" => if let Some(currency) = matches.value_of("currency") {
                match currency {
                    "usd" => match getdata {
                        Ok(s) => println!(
                            "{}: {} - Percent change 1 hour: {} - Percent change 24 hour: {}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.usd,
                            s.percent_change_1h.usd,
                            s.percent_change_24h.usd
                        ),
                        Err(e) => println!("{}", e),
                    },
                    "eur" => match getdata {
                        Ok(s) => println!("{}", s.eur),
                        Err(e) => println!("{}", e),
                    },
                    _ => unreachable!(),
                }; // end of currency flag
            }, // end of price flag
            "placeholder" => println!("placeholder"),
            _ => unreachable!(),
        }
    } else {
        println!("--mode <MODE> wasn't used...");
    }
}
