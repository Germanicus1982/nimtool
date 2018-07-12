//#![allow(dead_code)]

#[macro_use]
extern crate clap;
//extern crate statrs;
extern crate chrono;
extern crate nimtool;
extern crate textplots;


fn main() {
    use clap::App;
    use nimtool::app::*;
    use chrono::prelude::*;
    //use statrs::statistics::Statistics;
    use textplots::{Chart, Plot, Shape};

    //
    // TODO: implement internationalization
    // Pull in our YAML file depending on chosen language
    //
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

    //
    // The --price (-p) flag has been given it takes these possible values
    // current, day, week, month and year
    //
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
            "day" => if let Some(currency) = matches.value_of("currency") {
                let getdata = get_price_day_data();

                match currency {
                    //##############################
                    // PRICE
                    // DAY
                    // BTC
                    //##############################
                    "btc" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    // I know using unwrap() is kinda dirty however, in
                                    // this case I am positive that there is a value here
                                    // every time since there would have been many errors
                                    // before this if there wasn't. If you're combing
                                    // through this code and have a better way for me to
                                    // handle this situation please let me know.
                                    .map(|(i, p)| (i as f32, p.btc.parse::<f32>().unwrap() as f32))
                                    .collect();

                                println!("\r\ny = BTC \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // USD
                    //##############################
                    "usd" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.usd as f32))
                                    .collect();

                                println!("\r\ny = USD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // EUR
                    //##############################
                    "eur" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.eur as f32))
                                    .collect();

                                println!("\r\ny = EUR \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // AUD
                    //##############################
                    "aud" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.aud as f32))
                                    .collect();

                                println!("\r\ny = AUD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // BRL
                    //##############################
                    "brl" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.brl as f32))
                                    .collect();

                                println!("\r\ny = BRL \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // CAD
                    //##############################
                    "cad" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.cad as f32))
                                    .collect();

                                println!("\r\ny = CAD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // CNY
                    //##############################
                    "cny" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.cny as f32))
                                    .collect();

                                println!("\r\ny = CNY \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // GBP
                    //##############################
                    "gbp" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.gbp as f32))
                                    .collect();

                                println!("\r\ny = GBP \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // NZD
                    //##############################
                    "nzd" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.nzd as f32))
                                    .collect();

                                println!("\r\ny = NZD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // DKK
                    //##############################
                    "dkk" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.dkk as f32))
                                    .collect();

                                println!("\r\ny = DKK \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // JPY
                    //##############################
                    "jpy" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.jpy as f32))
                                    .collect();

                                println!("\r\ny = JPY \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // PLN
                    //##############################
                    "pln" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.pln as f32))
                                    .collect();

                                println!("\r\ny = PLN \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // KRW
                    //##############################
                    "krw" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.krw as f32))
                                    .collect();

                                println!("\r\ny = KRW \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // RUB
                    //##############################
                    "rub" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.rub as f32))
                                    .collect();

                                println!("\r\ny = RUB \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // MXN
                    //##############################
                    "mxn" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.mxn as f32))
                                    .collect();

                                println!("\r\ny = MXN \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // SEK
                    //##############################
                    "sek" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.sek as f32))
                                    .collect();

                                println!("\r\ny = SEK \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // HKD
                    //##############################
                    "hkd" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.hkd as f32))
                                    .collect();

                                println!("\r\ny = HKD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // MYR
                    //##############################
                    "myr" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.myr as f32))
                                    .collect();

                                println!("\r\ny = MYR \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // SGD
                    //##############################
                    "sgd" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.sgd as f32))
                                    .collect();

                                println!("\r\ny = SGD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // CHF
                    //##############################
                    "chf" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.chf as f32))
                                    .collect();

                                println!("\r\ny = CHF \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // HUF
                    //##############################
                    "huf" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.huf as f32))
                                    .collect();

                                println!("\r\ny = HUF \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // NOK
                    //##############################
                    "nok" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.nok as f32))
                                    .collect();

                                println!("\r\ny = NOK \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // THB
                    //##############################
                    "thb" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.thb as f32))
                                    .collect();

                                println!("\r\ny = THB \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // CLP
                    //##############################
                    "clp" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.clp as f32))
                                    .collect();

                                println!("\r\ny = CLP \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // IDR
                    //##############################
                    "idr" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.idr as f32))
                                    .collect();

                                println!("\r\ny = IDR \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // TRY
                    //##############################
                    "try" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.try as f32))
                                    .collect();

                                println!("\r\ny = TRY \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // ILS
                    //##############################
                    "ils" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.ils as f32))
                                    .collect();

                                println!("\r\ny = ILS \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // PHP
                    //##############################
                    "php" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.php as f32))
                                    .collect();

                                println!("\r\ny = PHP \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // TWD
                    //##############################
                    "twd" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.twd as f32))
                                    .collect();

                                println!("\r\ny = TWD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // CZK
                    //##############################
                    "czk" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.czk as f32))
                                    .collect();

                                println!("\r\ny = CZK \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // INR
                    //##############################
                    "inr" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.inr as f32))
                                    .collect();

                                println!("\r\ny = INR \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // PKR
                    //##############################
                    "pkr" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.pkr as f32))
                                    .collect();

                                println!("\r\ny = PKR \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // ZAR
                    //##############################
                    "zar" =>
                        match getdata {
                            Ok(data) => {
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.zar as f32))
                                    .collect();

                                println!("\r\ny = ZAR \r\nx = 24h");
                                Chart::new(180, 60, 0.0, 288.0)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // CURRENCY
                    // FAIL
                    //##############################
                    _ => unreachable!()
                }// end currency

            }// end of "day"
            //##############################
            // PRICE
            // DAY
            // FAIL
            //##############################
            _ => unreachable!(),
        }
    // TODO: finish up help text
    //} else if let Some(help) = matches.subcommand_matches("help") {
        //
    } // end of price

    //
    // The --supply (-s) flag has been given
    //
    if matches.is_present("supply") {
        // Grab supply data
        let supply = get_supply_data();

        match supply {
            Ok(s) => {
                println!("Block height: {}", s.height);
                println!("Market cap: {}", s.market_cap);
                println!("Existing supply: {}", s.existing_supply);
                println!("Circulating supply: {}", s.circulating_supply);
            },
            Err(e) => println!("{}", e)
        }
    } // end of supply

} // end of main
