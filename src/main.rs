//#![allow(dead_code)]

#[macro_use]
extern crate clap;
extern crate chrono;
extern crate nimtool;

fn main() {
    use clap::App;
    use chrono::prelude::*;
    use nimtool::app::*;

    // TODO: implement internationalization
    // Pull in our YAML file depending on chosen language
    let yml = {
        //#[cfg(feature = "en")]
        //{load_yaml!("../lang/english.cli.yml").to_owned()}

        //#[cfg(feature = "es")]
        //{load_yaml!("../lang/spanish.cli.yml").to_owned()}

        // default to english
        #[cfg(not(feature))]
        {load_yaml!("../lang/english.cli.yml").to_owned()}
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
    // TODO: finish adding all the currencies
    if let Some(price) = matches.value_of("price") {
        // Lets grab the price data for `current`
        let getdata = get_price_data();

        match price {
            //
            // Each price takes a --currency (-c) flag, lots of choices
            //
            "current" => if let Some(currency) = matches.value_of("currency") {
                match currency {
                    //##############################
                    // PRICE
                    // CURRENT
                    // BTC
                    //##############################
                    "btc" => match getdata {
                        Ok(s) => println!(
                            "{}: ₿{} - %1h: {} - %24h: {}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.btc,
                            s.percent_change_1h.btc,
                            s.percent_change_24h.btc
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // USD
                    //##############################
                    "usd" => match getdata {
                        Ok(s) => println!(
                            "{}: ${} - 1h: %{} - 24h: %{}",
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
                            "{}: ₠{} - 1h: %{} - 24h: %{}",
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
                    // AUD
                    //##############################
                    "aud" => match getdata {
                        Ok(s) => println!(
                            "{}: A${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.aud,
                            s.percent_change_1h.aud,
                            s.percent_change_24h.aud
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // BRL
                    //##############################
                    "brl" => match getdata {
                        Ok(s) => println!(
                            "{}: ‎R${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.brl,
                            s.percent_change_1h.brl,
                            s.percent_change_24h.brl
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // CAD
                    //##############################
                    "cad" => match getdata {
                        Ok(s) => println!(
                            "{}: ‎${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.cad,
                            s.percent_change_1h.cad,
                            s.percent_change_24h.cad
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
                            "{}: ¥{} - 1h: %{} - 24h: %{}",
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
                    // GBP
                    //##############################
                    "gbp" => match getdata {
                        Ok(s) => println!(
                            "{}: £{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.gbp,
                            s.percent_change_1h.gbp,
                            s.percent_change_24h.gbp
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // NZD
                    //##############################
                    "nzd" => match getdata {
                        Ok(s) => println!(
                            "{}: ${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.nzd,
                            s.percent_change_1h.nzd,
                            s.percent_change_24h.nzd
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // DKK
                    //##############################
                    "dkk" => match getdata {
                        Ok(s) => println!(
                            "{}: kr{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.dkk,
                            s.percent_change_1h.dkk,
                            s.percent_change_24h.dkk
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // JPY
                    //##############################
                    "jpy" => match getdata {
                        Ok(s) => println!(
                            "{}: ¥{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.jpy,
                            s.percent_change_1h.jpy,
                            s.percent_change_24h.jpy
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // PLN
                    //##############################
                    "pln" => match getdata {
                        Ok(s) => println!(
                            "{}: zł{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.pln,
                            s.percent_change_1h.pln,
                            s.percent_change_24h.pln
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // KRW
                    //##############################
                    "krw" => match getdata {
                        Ok(s) => println!(
                            "{}: ₩{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.krw,
                            s.percent_change_1h.krw,
                            s.percent_change_24h.krw
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // RUB
                    //##############################
                    "rub" => match getdata {
                        Ok(s) => println!(
                            "{}: руб{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.rub,
                            s.percent_change_1h.rub,
                            s.percent_change_24h.rub
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // MXN
                    //##############################
                    "mxn" => match getdata {
                        Ok(s) => println!(
                            "{}: ${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.mxn,
                            s.percent_change_1h.mxn,
                            s.percent_change_24h.mxn
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // SEK
                    //##############################
                    "sek" => match getdata {
                        Ok(s) => println!(
                            "{}: kr{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.sek,
                            s.percent_change_1h.sek,
                            s.percent_change_24h.sek
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // HKD
                    //##############################
                    "hkd" => match getdata {
                        Ok(s) => println!(
                            "{}: ${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.hkd,
                            s.percent_change_1h.hkd,
                            s.percent_change_24h.hkd
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // MYR
                    //##############################
                    "myr" => match getdata {
                        Ok(s) => println!(
                            "{}: RM{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.myr,
                            s.percent_change_1h.myr,
                            s.percent_change_24h.myr
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // SGD
                    //##############################
                    "sgd" => match getdata {
                        Ok(s) => println!(
                            "{}: S${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.sgd,
                            s.percent_change_1h.sgd,
                            s.percent_change_24h.sgd
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // CHF
                    //##############################
                    "chf" => match getdata {
                        Ok(s) => println!(
                            "{}: CHF{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.chf,
                            s.percent_change_1h.chf,
                            s.percent_change_24h.chf
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // HUF
                    //##############################
                    "huf" => match getdata {
                        Ok(s) => println!(
                            "{}: Ft{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.huf,
                            s.percent_change_1h.huf,
                            s.percent_change_24h.huf
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // NOK
                    //##############################
                    "nok" => match getdata {
                        Ok(s) => println!(
                            "{}: kr{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.nok,
                            s.percent_change_1h.nok,
                            s.percent_change_24h.nok
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // THB
                    //##############################
                    "thb" => match getdata {
                        Ok(s) => println!(
                            "{}: ฿{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.thb,
                            s.percent_change_1h.thb,
                            s.percent_change_24h.thb
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // CLP
                    //##############################
                    "clp" => match getdata {
                        Ok(s) => println!(
                            "{}: ${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.clp,
                            s.percent_change_1h.clp,
                            s.percent_change_24h.clp
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // IDR
                    //##############################
                    "idr" => match getdata {
                        Ok(s) => println!(
                            "{}: Rp{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.idr,
                            s.percent_change_1h.idr,
                            s.percent_change_24h.idr
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // TRY
                    //##############################
                    "try" => match getdata {
                        Ok(s) => println!(
                            "{}: TL{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.try,
                            s.percent_change_1h.try,
                            s.percent_change_24h.try
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // ILS
                    //##############################
                    "ils" => match getdata {
                        Ok(s) => println!(
                            "{}: ₪{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.ils,
                            s.percent_change_1h.ils,
                            s.percent_change_24h.ils
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // PHP
                    //##############################
                    "php" => match getdata {
                        Ok(s) => println!(
                            "{}: ₱{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.php,
                            s.percent_change_1h.php,
                            s.percent_change_24h.php
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // TWD
                    //##############################
                    "twd" => match getdata {
                        Ok(s) => println!(
                            "{}: NT${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.twd,
                            s.percent_change_1h.twd,
                            s.percent_change_24h.twd
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // CZK
                    //##############################
                    "czk" => match getdata {
                        Ok(s) => println!(
                            "{}: Kč{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.czk,
                            s.percent_change_1h.czk,
                            s.percent_change_24h.czk
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // INR
                    //##############################
                    "inr" => match getdata {
                        Ok(s) => println!(
                            "{}: ₹{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.inr,
                            s.percent_change_1h.inr,
                            s.percent_change_24h.inr
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // PKR
                    //##############################
                    "pkr" => match getdata {
                        Ok(s) => println!(
                            "{}: ₨{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.pkr,
                            s.percent_change_1h.pkr,
                            s.percent_change_24h.pkr
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // ZAR
                    //##############################
                    "zar" => match getdata {
                        Ok(s) => println!(
                            "{}: R{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.zar,
                            s.percent_change_1h.zar,
                            s.percent_change_24h.zar
                        ),
                        Err(e) => println!("{}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // FAIL
                    //##############################
                    _ => unreachable!(),
                }; // end of currency flag
            }, // end of current
            // TODO: Add day flag. This will require a lot more work than price.
            // TODO: Add week flag.
            // TODO: Add month flag.
            // TODO: Add year flag.
            //
            "day" => println!("day"),
            _ => unreachable!(),
        }
    // TODO: finish up help text
    //} else if let Some(help) = matches.subcommand_matches("help") {
        //
    } else {
        // TODO: get rid of this english only string
        println!("You must supply arguments. See --help.");
    }
}
