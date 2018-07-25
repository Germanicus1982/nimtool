//#![allow(dead_code, unused_imports)]

extern crate reqwest;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate directories;
#[macro_use] extern crate failure;
//#[macro_use] extern crate failure_derive;

pub mod price;
pub mod supply;
pub mod netstat;
pub mod transaction;
pub mod block;
pub mod addressbook;
pub mod labelbook;
pub mod hashrate;

pub mod app {
    use super::price::*;
    use super::supply::*;
    use super::netstat::*;
    use super::transaction::*;
    use super::block::*;
    use super::addressbook::*;
    use super::datastore::getkey;
    use super::hashrate::*;
    use super::labelbook::*;
    use reqwest::{Error, get, Url};

    // TODO: Refactor all of these into one function

    // TODO: Handle all other endpoints and return proper errors
    // grab price data for current flag
    pub fn get_price_data() -> Result<Price, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/price/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab price every 5 minutes for the last day
    pub fn get_price_day_data() -> Result<Vec<PriceDay>, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/price/day/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab price every 15 minutes for the last week
    pub fn get_price_week_data() -> Result<Vec<PriceDay>, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/price/week/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab price every 1 hour for the last month
    pub fn get_price_month_data() -> Result<Vec<PriceDay>, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/price/month/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab price every 6 hours for the last year
    pub fn get_price_year_data() -> Result<Vec<PriceDay>, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/price/year/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab supply data for supply flag
    pub fn get_supply_data() -> Result<Supply, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/supply/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab network data for network flag
    pub fn get_network_stats_data() -> Result<NetStat, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/network-stats/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab transaction data for transaction option
    pub fn get_transaction_data(hash: &str) -> Result<Transaction, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url base
        let base = match Url::parse("https://api.nimiqx.com/transaction/") {
            Ok(base) => base,
            Err(_) => panic!("Parse error: lib.rs building base url")
        };
        // append input data
        let base = match base.join(hash) {
            Ok(base) => base,
            Err(_) => panic!("Parse error: lib.rs appending input data")
        };
        // add the api key
        let url = match Url::parse_with_params(
            &*base.to_string(),
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab blockinfo data for blockinfo option
    pub fn get_blockinfo_data(identifier: &str) -> Result<BlockInfo, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url base
        let base = match Url::parse("https://api.nimiqx.com/block/") {
            Ok(base) => base,
            Err(_) => panic!("Parse error: lib.rs ln: 165")
        };
        // append input data
        let base = match base.join(identifier) {
            Ok(base) => base,
            Err(_) => panic!("Parse error: lib.rs ln: 170")
        };
        // add the api key
        let url = match Url::parse_with_params(
            &*base.to_string(),
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab addressbook data for addressbook option
    pub fn get_addressbook_data(address: &str) -> Result<AddressBook, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url base
        let base = match Url::parse("https://api.nimiqx.com/address-book/") {
            Ok(base) => base,
            Err(_) => panic!("Parse error: lib.rs building base url")
        };
        // append input data
        let base = match base.join(address) {
            Ok(base) => base,
            Err(_) => panic!("Parse error: lib.rs appending input data")
        };
        // add the api key
        let url = match Url::parse_with_params(
            &*base.to_string(),
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab addressbook data for addressbook option
    pub fn get_labelbook_data(label: &str) -> Result<LabelBook, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url base
        let base = match Url::parse("https://api.nimiqx.com/label-book/") {
            Ok(base) => base,
            Err(_) => panic!("Parse error: lib.rs building base url")
        };
        // append input data
        let base = match base.join(label) {
            Ok(base) => base,
            Err(_) => panic!("Parse error: lib.rs appending input data")
        };
        // add the api key
        let url = match Url::parse_with_params(
            &*base.to_string(),
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab current hashrate data for current flag
    pub fn get_hashrate_data() -> Result<Hashrate, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/hashrate/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab hour hashrate data for hour flag
    pub fn get_hour_hashrate_data() -> Result<Vec<Hashrate>, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/hashrate/hour/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab day hashrate data for day flag
    pub fn get_day_hashrate_data() -> Result<Vec<Hashrate>, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/hashrate/day/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab week hashrate data for week flag
    pub fn get_week_hashrate_data() -> Result<Vec<Hashrate>, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/hashrate/week/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab month hashrate data for month flag
    pub fn get_month_hashrate_data() -> Result<Vec<Hashrate>, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/hashrate/month/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }

    // grab year hashrate data for year flag
    pub fn get_year_hashrate_data() -> Result<Vec<Hashrate>, Error> {
        // grab the api key
        let apikey = match getkey() {
            Ok(apikey) => apikey,
            Err(e) => panic!("{:#?}", e)
        };
        // build the url
        let url = match Url::parse_with_params(
            "https://api.nimiqx.com/hashrate/year/",
            &[("nimtool", &*apikey)]) {
            Ok(url) => url,
            Err(e) => panic!("{:#?}", e)
        };
        // make the call and deserialize
        get(url)?.json()
    }
}

pub mod datastore {
    use std::fs::File;
    use std::fs;
    use std::fs::create_dir_all;
    use std::io::prelude::*;
    use directories::ProjectDirs;
    use failure::Error;

    pub fn apikey(key: &str) -> Result<bool, Error> {
        //let mut success = false;
        if let Some(path) = ProjectDirs::from("com", "nimtool", "nimtool") {
            // create the path if it doesn't exist
            create_dir_all(path.config_dir())?;

            // append our file to the path
            let path = path.config_dir().join("data.db");

            // Overwrite the file or create it if it doesn't exist
            let mut file = File::create(&path)?;

            // and write the key newly created file
            file.write_all(key.as_bytes())?;
        } else {
            return Err(format_err!("The path to store the API key could not be created."));
        }
        // Return true since everything worked without error
        Ok(true)
    }// end apikey

    // get the api key
    pub fn getkey() -> Result<String, Error> {
        if let Some(path) = ProjectDirs::from("com", "nimtool", "nimtool") {
            let path = path.config_dir().join("data.db");

            // get the key
            if path.exists() {
                Ok(fs::read_to_string(&path)?)
            } else {
                Err(format_err!("Please add your API key from api.nimiqx.com using the --appkey option."))
            }
        } else {
            Err(format_err!("Could not get key from keystore file."))
        }

    }

}// end mod datastore

// TODO: write tests
#[cfg(test)]
mod tests {

}