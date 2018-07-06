#![allow(dead_code)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate reqwest;
extern crate serde_json;

use chrono::prelude::*;

// Create the price struct that will
// hold the deserialized response
#[derive(Debug, Deserialize)]
struct Price {
    timestamp: i64,
    usd: f64,
    eur: f64,
    aud: f64,
    brl: f64,
    cad: f64,
    cny: f64,
    gbp: f64,
    nzd: f64,
    btc: String,
    #[serde(flatten)]
    inner1h: PricePercentChange1h,
    percent_change_1h: PricePercentChange1h,
    #[serde(flatten)]
    inner24h: PricePercentChange24h,
    percent_change_24h: PricePercentChange24h,
}

#[derive(Debug, Deserialize)]
struct PricePercentChange1h {
    usd: String,
    eur: String,
    aud: String,
    brl: String,
    cad: String,
    cny: String,
    gbp: String,
    nzd: String,
    btc: String,
}

#[derive(Debug, Deserialize)]
struct PricePercentChange24h {
    usd: String,
    eur: String,
    aud: String,
    brl: String,
    cad: String,
    cny: String,
    gbp: String,
    nzd: String,
    btc: String,
}

fn main() {
    use clap::App;

    // Pull in our yaml file
    let yaml = load_yaml!("../i18n/english.cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

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
                            s.inner1h.usd,
                            s.inner24h.usd
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

fn get_data() -> Result<Price, reqwest::Error> {
    reqwest::get("https://api.nimiqx.com/price/")?.json()
}
