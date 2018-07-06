#![allow(dead_code)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate reqwest;
extern crate serde_json;

// Create the price struct that will
// hold the deserialized response
#[derive(Debug, Deserialize)]
struct Price {
    timestamp: i64,
    btc: String,
    usd: f64,
    eur: f64,
    aud: f64,
    brl: f64,
    cad: f64,
    cny: f64,
    gbp: f64,
    nzd: f64,
    dkk: f64,
    jpy: f64,
    pln: f64,
    krw: f64,
    rub: f64,
    mxn: f64,
    sek: f64,
    hkd: f64,
    myr: f64,
    sgd: f64,
    chf: f64,
    huf: f64,
    nok: f64,
    thb: f64,
    clp: f64,
    idr: f64,
    try: f64,
    ils: f64,
    php: f64,
    twd: f64,
    czk: f64,
    inr: f64,
    pkr: f64,
    zar: f64,
    percent_change_1h: PricePercentChange1h,
    percent_change_24h: PricePercentChange24h,
}

#[derive(Debug, Deserialize)]
struct PricePercentChange1h {
    btc: String,
    usd: String,
    eur: String,
    aud: String,
    brl: String,
    cad: String,
    cny: String,
    gbp: String,
    nzd: String,
}

#[derive(Debug, Deserialize)]
struct PricePercentChange24h {
    btc: String,
    usd: String,
    eur: String,
    aud: String,
    brl: String,
    cad: String,
    cny: String,
    gbp: String,
    nzd: String,
}


fn main() {
    use clap::App;
    use chrono::prelude::*;

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

fn get_data() -> Result<Price, reqwest::Error> {
    reqwest::get("https://api.nimiqx.com/price/")?.json()
}
