//#![allow(dead_code)]

#[macro_use]
extern crate clap;
extern crate chrono;
extern crate nimtool;
extern crate textplots;
extern crate separator;


fn main() {
    use clap::App;
    use nimtool::app::*;
    use nimtool::datastore::apikey;
    use chrono::prelude::*;
    use textplots::{Chart, Plot, Shape};
    use separator::Separatable;

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
        .get_matches();

    //
    // The --price (-p) flag has been given it takes these possible values
    // current, day, week, month and year
    //
    if let Some(price) = matches.value_of("price") {

        match price {
            //
            // Each price takes a --currency (-c) flag, lots of choices
            //
            "current" => if let Some(currency) = matches.value_of("currency") {
                // Lets grab the price data for `current`
                let getdata = get_price_data();

                match currency {
                    //##############################
                    // PRICE
                    // CURRENT
                    // BTC
                    //##############################
                    "btc" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ₿{} - %1h: {} - %24h: {}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.btc,
                            s.percent_change_1h.btc,
                            s.percent_change_24h.btc
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // USD
                    //##############################
                    "usd" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.usd,
                            s.percent_change_1h.usd,
                            s.percent_change_24h.usd
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // EUR
                    //##############################
                    "eur" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ₠{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.eur,
                            s.percent_change_1h.eur,
                            s.percent_change_24h.eur
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // AUD
                    //##############################
                    "aud" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: A${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.aud,
                            s.percent_change_1h.aud,
                            s.percent_change_24h.aud
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // BRL
                    //##############################
                    "brl" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ‎R${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.brl,
                            s.percent_change_1h.brl,
                            s.percent_change_24h.brl
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // CAD
                    //##############################
                    "cad" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ‎${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.cad,
                            s.percent_change_1h.cad,
                            s.percent_change_24h.cad
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // CNY
                    //##############################
                    "cny" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ¥{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.cny,
                            s.percent_change_1h.cny,
                            s.percent_change_24h.cny
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // GBP
                    //##############################
                    "gbp" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: £{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.gbp,
                            s.percent_change_1h.gbp,
                            s.percent_change_24h.gbp
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // NZD
                    //##############################
                    "nzd" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ${} - 1h: %{} - 24h: %{} GMT",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.nzd,
                            s.percent_change_1h.nzd,
                            s.percent_change_24h.nzd
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // DKK
                    //##############################
                    "dkk" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: kr{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.dkk,
                            s.percent_change_1h.dkk,
                            s.percent_change_24h.dkk
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // JPY
                    //##############################
                    "jpy" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ¥{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.jpy,
                            s.percent_change_1h.jpy,
                            s.percent_change_24h.jpy
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // PLN
                    //##############################
                    "pln" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: zł{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.pln,
                            s.percent_change_1h.pln,
                            s.percent_change_24h.pln
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // KRW
                    //##############################
                    "krw" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ₩{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.krw,
                            s.percent_change_1h.krw,
                            s.percent_change_24h.krw
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // RUB
                    //##############################
                    "rub" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: руб{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.rub,
                            s.percent_change_1h.rub,
                            s.percent_change_24h.rub
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // MXN
                    //##############################
                    "mxn" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ${} - 1h: %{} - 24h: %{} GMT",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.mxn,
                            s.percent_change_1h.mxn,
                            s.percent_change_24h.mxn
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // SEK
                    //##############################
                    "sek" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: kr{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.sek,
                            s.percent_change_1h.sek,
                            s.percent_change_24h.sek
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // HKD
                    //##############################
                    "hkd" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ${} - 1h: %{} - 24h: %{} GMT",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.hkd,
                            s.percent_change_1h.hkd,
                            s.percent_change_24h.hkd
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // MYR
                    //##############################
                    "myr" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: RM{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.myr,
                            s.percent_change_1h.myr,
                            s.percent_change_24h.myr
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // SGD
                    //##############################
                    "sgd" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: S${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.sgd,
                            s.percent_change_1h.sgd,
                            s.percent_change_24h.sgd
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // CHF
                    //##############################
                    "chf" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: CHF{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.chf,
                            s.percent_change_1h.chf,
                            s.percent_change_24h.chf
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // HUF
                    //##############################
                    "huf" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: Ft{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.huf,
                            s.percent_change_1h.huf,
                            s.percent_change_24h.huf
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // NOK
                    //##############################
                    "nok" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: kr{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.nok,
                            s.percent_change_1h.nok,
                            s.percent_change_24h.nok
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // THB
                    //##############################
                    "thb" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ฿{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.thb,
                            s.percent_change_1h.thb,
                            s.percent_change_24h.thb
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // CLP
                    //##############################
                    "clp" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ${} - 1h: %{} - 24h: %{} GMT",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.clp,
                            s.percent_change_1h.clp,
                            s.percent_change_24h.clp
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // IDR
                    //##############################
                    "idr" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: Rp{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.idr,
                            s.percent_change_1h.idr,
                            s.percent_change_24h.idr
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // TRY
                    //##############################
                    "try" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: TL{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.try,
                            s.percent_change_1h.try,
                            s.percent_change_24h.try
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // ILS
                    //##############################
                    "ils" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ₪{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.ils,
                            s.percent_change_1h.ils,
                            s.percent_change_24h.ils
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // PHP
                    //##############################
                    "php" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ₱{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.php,
                            s.percent_change_1h.php,
                            s.percent_change_24h.php
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // TWD
                    //##############################
                    "twd" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: NT${} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.twd,
                            s.percent_change_1h.twd,
                            s.percent_change_24h.twd
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // CZK
                    //##############################
                    "czk" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: Kč{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.czk,
                            s.percent_change_1h.czk,
                            s.percent_change_24h.czk
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // INR
                    //##############################
                    "inr" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ₹{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.inr,
                            s.percent_change_1h.inr,
                            s.percent_change_24h.inr
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // PKR
                    //##############################
                    "pkr" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: ₨{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.pkr,
                            s.percent_change_1h.pkr,
                            s.percent_change_24h.pkr
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // ZAR
                    //##############################
                    "zar" => match getdata {
                        Ok(s) => println!(
                            "GMT {}: R{} - 1h: %{} - 24h: %{}",
                            NaiveDateTime::from_timestamp(s.timestamp, 0),
                            s.zar,
                            s.percent_change_1h.zar,
                            s.percent_change_24h.zar
                        ),
                        Err(e) => println!("{:#?}", e),
                    },
                    //##############################
                    // PRICE
                    // CURRENT
                    // FAIL
                    //##############################
                    _ => unreachable!(),
                }; // end of currency flag
            }, // end of current

            //
            // The daily stats
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
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    // I know using unwrap() is kinda dirty however, in
                                    // this case I am positive that there is a value here
                                    // every time since there would have been many errors
                                    // before this if there wasn't. If you're combing
                                    // through this code and have a better way for me to
                                    // handle this situation please let me know.
                                    .map(|(i, p)| (i as f32, p.btc.clone().unwrap_or_default().parse::<f32>().unwrap() as f32))
                                    .collect();

                                println!("\r\ny = BTC \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // USD
                    //##############################
                    "usd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.usd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = USD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // EUR
                    //##############################
                    "eur" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.eur.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = EUR \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // AUD
                    //##############################
                    "aud" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.aud.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = AUD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // BRL
                    //##############################
                    "brl" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.brl.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = BRL \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // CAD
                    //##############################
                    "cad" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.cad.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CAD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // CNY
                    //##############################
                    "cny" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.cny.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CNY \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // GBP
                    //##############################
                    "gbp" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.gbp.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = GBP \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // NZD
                    //##############################
                    "nzd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.nzd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = NZD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // DKK
                    //##############################
                    "dkk" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.dkk.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = DKK \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // JPY
                    //##############################
                    "jpy" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.jpy.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = JPY \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // PLN
                    //##############################
                    "pln" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.pln.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = PLN \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // KRW
                    //##############################
                    "krw" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.krw.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = KRW \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // RUB
                    //##############################
                    "rub" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.rub.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = RUB \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // MXN
                    //##############################
                    "mxn" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.mxn.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = MXN \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // SEK
                    //##############################
                    "sek" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.sek.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = SEK \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // HKD
                    //##############################
                    "hkd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.hkd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = HKD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // MYR
                    //##############################
                    "myr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.myr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = MYR \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // SGD
                    //##############################
                    "sgd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.sgd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = SGD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // CHF
                    //##############################
                    "chf" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.chf.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CHF \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // HUF
                    //##############################
                    "huf" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.huf.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = HUF \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // NOK
                    //##############################
                    "nok" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.nok.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = NOK \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // THB
                    //##############################
                    "thb" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.thb.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = THB \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // CLP
                    //##############################
                    "clp" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.clp.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CLP \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // IDR
                    //##############################
                    "idr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.idr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = IDR \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // TRY
                    //##############################
                    "try" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.try.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = TRY \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // ILS
                    //##############################
                    "ils" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.ils.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = ILS \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // PHP
                    //##############################
                    "php" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.php.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = PHP \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // TWD
                    //##############################
                    "twd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.twd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = TWD \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // CZK
                    //##############################
                    "czk" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.czk.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CZK \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // INR
                    //##############################
                    "inr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.inr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = INR \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // PKR
                    //##############################
                    "pkr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.pkr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = PKR \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // DAY
                    // ZAR
                    //##############################
                    "zar" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.zar.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = ZAR \r\nx = 24h");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // CURRENCY
                    // FAIL
                    //##############################
                    _ => unreachable!()
                }// end currency

            }// end of "day"

            //
            // The weekly stats
            //
            "week" => if let Some(currency) = matches.value_of("currency") {
                let getdata = get_price_week_data();

                match currency {
                    //##############################
                    // PRICE
                    // WEEK
                    // BTC
                    //##############################
                    "btc" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    // I know using unwrap() is kinda dirty however, in
                                    // this case I am positive that there is a value here
                                    // every time since there would have been many errors
                                    // before this if there wasn't. If you're combing
                                    // through this code and have a better way for me to
                                    // handle this situation please let me know.
                                    .map(|(i, p)| (i as f32, p.btc.clone().unwrap_or_default().parse::<f32>().unwrap() as f32))
                                    .collect();

                                println!("\r\ny = BTC \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // USD
                    //##############################
                    "usd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.usd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = USD \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // EUR
                    //##############################
                    "eur" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.eur.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = EUR \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // AUD
                    //##############################
                    "aud" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.aud.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = AUD \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // BRL
                    //##############################
                    "brl" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.brl.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = BRL \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // CAD
                    //##############################
                    "cad" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.cad.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CAD \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // CNY
                    //##############################
                    "cny" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.cny.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CNY \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // GBP
                    //##############################
                    "gbp" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.gbp.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = GBP \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // NZD
                    //##############################
                    "nzd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.nzd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = NZD \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // DKK
                    //##############################
                    "dkk" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.dkk.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = DKK \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // JPY
                    //##############################
                    "jpy" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.jpy.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = JPY \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // PLN
                    //##############################
                    "pln" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.pln.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = PLN \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // KRW
                    //##############################
                    "krw" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.krw.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = KRW \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // RUB
                    //##############################
                    "rub" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.rub.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = RUB \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // MXN
                    //##############################
                    "mxn" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.mxn.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = MXN \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // SEK
                    //##############################
                    "sek" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.sek.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = SEK \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // Week
                    // HKD
                    //##############################
                    "hkd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.hkd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = HKD \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // MYR
                    //##############################
                    "myr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.myr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = MYR \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // SGD
                    //##############################
                    "sgd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.sgd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = SGD \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // CHF
                    //##############################
                    "chf" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.chf.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CHF \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // HUF
                    //##############################
                    "huf" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.huf.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = HUF \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // NOK
                    //##############################
                    "nok" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.nok.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = NOK \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // THB
                    //##############################
                    "thb" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.thb.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = THB \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // CLP
                    //##############################
                    "clp" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.clp.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CLP \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // IDR
                    //##############################
                    "idr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.idr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = IDR \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // TRY
                    //##############################
                    "try" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.try.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = TRY \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // ILS
                    //##############################
                    "ils" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.ils.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = ILS \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // PHP
                    //##############################
                    "php" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.php.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = PHP \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // TWD
                    //##############################
                    "twd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.twd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = TWD \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // CZK
                    //##############################
                    "czk" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.czk.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CZK \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // INR
                    //##############################
                    "inr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.inr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = INR \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // PKR
                    //##############################
                    "pkr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.pkr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = PKR \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // WEEK
                    // ZAR
                    //##############################
                    "zar" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.zar.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = ZAR \r\nx = Week");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // CURRENCY
                    // FAIL
                    //##############################
                    _ => unreachable!()
                }// end currency

            }// end of week

            //
            // The monthly stats
            //
            "month" => if let Some(currency) = matches.value_of("currency") {
                let getdata = get_price_month_data();

                match currency {
                    //##############################
                    // PRICE
                    // MONTH
                    // BTC
                    //##############################
                    "btc" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                println!("{:#?}", count);
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    // I know using unwrap() is kinda dirty however, in
                                    // this case I am positive that there is a value here
                                    // every time since there would have been many errors
                                    // before this if there wasn't. If you're combing
                                    // through this code and have a better way for me to
                                    // handle this situation please let me know.
                                    .map(|(i, p)| (i as f32, p.btc.clone().unwrap_or_default().parse::<f32>().unwrap() as f32))
                                    .collect();

                                println!("\r\ny = BTC \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // USD
                    //##############################
                    "usd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.usd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = USD \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // EUR
                    //##############################
                    "eur" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.eur.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = EUR \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // AUD
                    //##############################
                    "aud" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.aud.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = AUD \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // BRL
                    //##############################
                    "brl" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.brl.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = BRL \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // CAD
                    //##############################
                    "cad" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.cad.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CAD \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // CNY
                    //##############################
                    "cny" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.cny.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CNY \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // GBP
                    //##############################
                    "gbp" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.gbp.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = GBP \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // NZD
                    //##############################
                    "nzd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.nzd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = NZD \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // DKK
                    //##############################
                    "dkk" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.dkk.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = DKK \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // JPY
                    //##############################
                    "jpy" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.jpy.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = JPY \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // PLN
                    //##############################
                    "pln" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.pln.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = PLN \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // KRW
                    //##############################
                    "krw" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.krw.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = KRW \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // RUB
                    //##############################
                    "rub" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.rub.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = RUB \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // MXN
                    //##############################
                    "mxn" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.mxn.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = MXN \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // SEK
                    //##############################
                    "sek" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.sek.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = SEK \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // HKD
                    //##############################
                    "hkd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.hkd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = HKD \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // MYR
                    //##############################
                    "myr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.myr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = MYR \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // SGD
                    //##############################
                    "sgd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.sgd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = SGD \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // CHF
                    //##############################
                    "chf" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.chf.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CHF \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // HUF
                    //##############################
                    "huf" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.huf.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = HUF \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // NOK
                    //##############################
                    "nok" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.nok.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = NOK \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // THB
                    //##############################
                    "thb" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.thb.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = THB \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // CLP
                    //##############################
                    "clp" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.clp.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CLP \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // IDR
                    //##############################
                    "idr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.idr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = IDR \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // TRY
                    //##############################
                    "try" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.try.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = TRY \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // ILS
                    //##############################
                    "ils" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.ils.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = ILS \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // PHP
                    //##############################
                    "php" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.php.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = PHP \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // TWD
                    //##############################
                    "twd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.twd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = TWD \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // CZK
                    //##############################
                    "czk" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.czk.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CZK \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // INR
                    //##############################
                    "inr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.inr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = INR \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // PKR
                    //##############################
                    "pkr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.pkr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = PKR \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // MONTH
                    // ZAR
                    //##############################
                    "zar" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.zar.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = ZAR \r\nx = Month");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // CURRENCY
                    // FAIL
                    //##############################
                    _ => unreachable!()
                }// end currency

            }// end of month

            //
            // The yearly stats
            //
            "year" => if let Some(currency) = matches.value_of("currency") {
                let getdata = get_price_year_data();

                match currency {
                    //##############################
                    // PRICE
                    // YEAR
                    // BTC
                    //##############################
                    "btc" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    // I know using unwrap() is kinda dirty however, in
                                    // this case I am positive that there is a value here
                                    // every time since there would have been many errors
                                    // before this if there wasn't. If you're combing
                                    // through this code and have a better way for me to
                                    // handle this situation please let me know.
                                    .map(|(i, p)| (i as f32, p.btc.clone().unwrap_or_default().parse::<f32>().unwrap() as f32))
                                    .collect();

                                println!("\r\ny = BTC \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // USD
                    //##############################
                    "usd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.usd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = USD \r\nx = YEAR");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // EUR
                    //##############################
                    "eur" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.eur.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = EUR \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // AUD
                    //##############################
                    "aud" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.aud.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = AUD \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // BRL
                    //##############################
                    "brl" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.brl.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = BRL \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // CAD
                    //##############################
                    "cad" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.cad.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CAD \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // CNY
                    //##############################
                    "cny" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.cny.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CNY \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // GBP
                    //##############################
                    "gbp" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.gbp.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = GBP \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // NZD
                    //##############################
                    "nzd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.nzd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = NZD \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // DKK
                    //##############################
                    "dkk" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.dkk.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = DKK \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // JPY
                    //##############################
                    "jpy" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.jpy.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = JPY \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // PLN
                    //##############################
                    "pln" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.pln.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = PLN \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // KRW
                    //##############################
                    "krw" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.krw.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = KRW \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // RUB
                    //##############################
                    "rub" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.rub.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = RUB \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // MXN
                    //##############################
                    "mxn" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.mxn.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = MXN \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // SEK
                    //##############################
                    "sek" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.sek.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = SEK \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // HKD
                    //##############################
                    "hkd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.hkd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = HKD \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // MYR
                    //##############################
                    "myr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.myr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = MYR \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // SGD
                    //##############################
                    "sgd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.sgd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = SGD \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // CHF
                    //##############################
                    "chf" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.chf.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CHF \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // HUF
                    //##############################
                    "huf" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.huf.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = HUF \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // NOK
                    //##############################
                    "nok" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.nok.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = NOK \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // THB
                    //##############################
                    "thb" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.thb.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = THB \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // CLP
                    //##############################
                    "clp" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.clp.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CLP \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // IDR
                    //##############################
                    "idr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.idr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = IDR \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // TRY
                    //##############################
                    "try" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.try.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = TRY \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // ILS
                    //##############################
                    "ils" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.ils.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = ILS \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // PHP
                    //##############################
                    "php" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.php.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = PHP \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // TWD
                    //##############################
                    "twd" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.twd.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = TWD \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // CZK
                    //##############################
                    "czk" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.czk.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = CZK \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // INR
                    //##############################
                    "inr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.inr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = INR \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // PKR
                    //##############################
                    "pkr" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.pkr.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = PKR \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // YEAR
                    // ZAR
                    //##############################
                    "zar" =>
                        match getdata {
                            Ok(data) => {
                                let count = data.iter().count();
                                let points: Vec<_> = data
                                    .iter()
                                    .enumerate()
                                    .map(|(i, p)| (i as f32, p.zar.unwrap_or_default() as f32))
                                    .collect();

                                println!("\r\ny = ZAR \r\nx = Year");
                                Chart::new(180, 60, 0.0, count as f32)
                                    .lineplot(Shape::Lines(&points))
                                    .display();

                                // TODO: get these numbers and display them
                                //println!("min: {} max: {}: average: {}", 22, 33, 24.5);
                            },
                            Err(e) => println!("{:#?}", e)
                        }// end getdata
                    //##############################
                    // PRICE
                    // CURRENCY
                    // FAIL
                    //##############################
                    _ => unreachable!()
                }// end currency

            }// end of month

            _ => unreachable!(), //match price fail

        }// end of match price
    }// end of price flag

    //
    // The --hashrate (-h) flag has been given
    //
    if let Some(hashrate) = matches.value_of("hashrate") {
        let hashratedata = get_hashrate_data();
        match hashrate {
            "current" => match hashratedata {
                Ok(s) => {
                    if s.hashrate <= 999999.00 {
                        println!("Hashrate: {} kH/s", s.hashrate/100000.00);
                    } else if (s.hashrate >= 1000000.00) && (s.hashrate <= 999999999.00) {
                        println!("Hashrate: {} MH/s", s.hashrate / 1000000.00);
                    } else if s.hashrate >= 100000000.00 {
                        println!("Hashrate: {} GH/s", s.hashrate/100000000.00);
                    }
                },
                Err(e) => println!("{:#?}", e)
            }

            _ => unreachable!(),
        }
    }

    //
    // The --supply (-s) flag has been given
    //
    if matches.is_present("supply") {
        // Grab supply data
        let supply = get_supply_data();

        match supply {
            Ok(s) => {
                println!("Block height: {}", s.height.separated_string());
                println!("Market cap: ${}", s.market_cap.separated_string());
                println!("Existing supply: {}", s.existing_supply.separated_string());
                println!("Circulating supply: {}", s.circulating_supply.separated_string());
            },
            Err(e) => println!("{:#?}", e)
        }
    }// end of supply

    //
    // The --network-stats (-n) flag has been given
    //
    if matches.is_present("network-stats") {
        // Grab supply data
        let network_stats = get_network_stats_data();

        match network_stats {
            Ok(s) => {
                if s.hashrate <= 999999.00 {
                    println!("Hashrate: {} kH/s", s.hashrate/100000.00);
                } else if (s.hashrate >= 1000000.00) && (s.hashrate <= 999999999.00) {
                    println!("Hashrate: {} MH/s", s.hashrate / 1000000.00);
                } else if s.hashrate >= 100000000.00 {
                    println!("Hashrate: {} GH/s", s.hashrate/100000000.00);
                }

                if s.last_found <= 60 {
                    println!("Last found block: {}s ago", s.last_found);
                } else if (s.last_found >= 61) && (s.last_found <= 3600) {
                    let minutes = s.last_found / 60;
                    let seconds = s.last_found % 60;
                    println!("Last found block: {}m {}s ago", minutes, seconds);
                } else if s.last_found >= 3601 {
                    let minutes = s.last_found / 60;
                    let seconds = s.last_found % 60;
                    let hours = minutes / 60;
                    let minutes = minutes % 60;
                    println!("Last found block: {}h {}m {}s ago", hours, minutes, seconds);
                }

                println!("Block height: {}", s.height.separated_string());
                println!("Difficulty: {}", s.difficulty.separated_string());
                println!("Last reward: {}", (s.last_reward/100000.00).separated_string());
                println!("Nim per day per kH: {}", s.nim_day_kh);
            },
            Err(e) => println!("{:#?}", e)
        }
    }// end of network-stats

    //
    // The --transaction (-t) option has been given
    //
    if let Some(hash) = matches.value_of("transaction") {
        // Grab supply data
        let transaction = get_transaction_data(hash);

        match transaction {
            Ok(s) => {
                if s.error == "Invalid Hash" {
                    println!("Transaction hash is invalid.");
                } else {
                    println!("Transaction hash: {}", s.hash);
                    println!("Block height: {}", s.height.separated_string());
                    println!("Block hash: {}", s.block_hash);
                    println!("Transaction index: {}", s.transaction_index);
                    println!("From address: {}", s.from_address);
                    println!("To address: {}", s.to_address);
                    println!("Value: {}", (s.value/100000.00).separated_string());
                    println!("Fee: {}", (s.fee/100000.00).separated_string());
                    println!("Timestamp: Unix - {} Human - {} GMT", s.timestamp, NaiveDateTime::from_timestamp(s.timestamp, 0));
                }

            },
            Err(e) => println!("-b{}", e)
        }// end of transaction match
    }// end of transaction option

    //
    // The --blockinfo (-b) option has been given
    //
    if let Some(blockinfo) = matches.value_of("blockinfo") {
        // Grab supply data
        let blockinfo = get_blockinfo_data(blockinfo);

        match blockinfo {
            Ok(s) => {
                if s.error == "Block not found" {
                    println!("Block not found.");
                } else {
                    println!("Block hash: {}", s.hash);
                    println!("Block height: {}", s.height.separated_string());
                    println!("Parent hash: {}", s.parent_hash);
                    println!("Interlink hash: {}", s.interlink_hash);
                    println!("Body hash: {}", s.body_hash);
                    println!("Accounts hash: {}", s.accounts_hash);
                    println!("POW: {}", s.pow);
                    println!("NONCE: {}", s.nonce);
                    println!("Reward: {}", (s.reward/100000.00).separated_string());
                    println!("Miner address: {}", s.miner_address);
                    println!("Difficulty: {}", s.difficulty.separated_string());
                    println!("Size: {}", s.size);
                    println!("Extra data: {}", s.extra_data);
                    println!("Transactions: {}", s.transactions);
                    println!("Timestamp: Unix - {} Human - {} GMT", s.timestamp, NaiveDateTime::from_timestamp(s.timestamp, 0));
                }

            },
            Err(e) => println!("{:#?}", e)
        }// end of block match
    }// end of block flag

    //
    // The --addressbook (-a) option has been given
    //
    if let Some(address) = matches.value_of("addressbook") {
        // Grab supply data
        let label = get_addressbook_data(address);

        match label {
            Ok(s) => {
                if s.error != "" {
                    println!("{}", s.error);
                } else {
                    println!("Label: {}\r\nCreated: {} GMT", s.label, NaiveDateTime::from_timestamp(s.created, 0),);
                }

            },
            Err(e) => println!("{:#?}", e)
        }// end of addressbook match
    }// end of addressbook flag

    //
    // The --label (-l) option has been given
    //
    if let Some(label) = matches.value_of("label") {
        // Grab supply data
        let address = get_labelbook_data(label);

        match address {
            Ok(s) => {
                if s.error != "" {
                    println!("{}", s.error);
                } else {
                    println!("Address: {}\r\nCreated: {} GMT", s.address, NaiveDateTime::from_timestamp(s.created, 0),);
                }

            },
            Err(e) => println!("{:#?}", e)
        }// end of address match
    }// end of label flag

    //
    // The --appkey option has been given
    //
    if let Some(appkey) = matches.value_of("appkey") {
        // Grab supply data
        let appkey = apikey(appkey);

        match appkey {
            Ok(s) => {
                println!("{:#?}", s)
            }
            Err(e) => println!("{:#?}", e)
        }// end of appkey match
    }// end of appkey flag

    // TODO: hashrate for hour, day, week, month and year
    // TODO: latest blocks https://api.nimiqx.com/docs/latest-blocks
    // TODO: global hashrate https://api.nimiqx.com/docs/global-hashrate
    // TODO: hashing distribution https://api.nimiqx.com/docs/hashing-distribution
    // TODO: latest transactions https://api.nimiqx.com/docs/latest-transactions
    // TODO: account information https://api.nimiqx.com/docs/account-information
    // TODO: account blocks https://api.nimiqx.com/docs/account-blocks
    // TODO: account transactions https://api.nimiqx.com/docs/account-blocks
    // TODO: top 500 solo miners https://api.nimiqx.com/docs/top500-solo-miners
    // TODO: solo miner ranks https://api.nimiqx.com/docs/solo-miners-ranks
}// end of main
