#![feature(extern_prelude)]
//#![allow(dead_code, unused_imports)]

extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod price;
pub mod supply;
pub mod netstat;
pub mod transaction;
pub mod block;
pub mod addressbook;

pub mod app {
    use super::price::*;
    use super::supply::*;
    use super::netstat::*;
    use super::transaction::*;
    use super::block::*;
    use super::addressbook::*;

    use reqwest::Url;

    // TODO: Refactor all of these into one function

    // TODO: Handle all other endpoints and return proper errors
    // grab price data for current flag
    pub fn get_price_data() -> Result<Price, reqwest::Error> {
        reqwest::get("https://api.nimiqx.com/price/")?.json()
    }

    // grab price every 5 minutes for the last day
    pub fn get_price_day_data() -> Result<Vec<PriceDay>, reqwest::Error> {
        reqwest::get("https://api.nimiqx.com/price/day/")?.json()
    }

    // grab price every 15 minutes for the last week
    pub fn get_price_week_data() -> Result<Vec<PriceDay>, reqwest::Error> {
        reqwest::get("https://api.nimiqx.com/price/week/")?.json()
    }

    // grab supply data for supply flag
    pub fn get_supply_data() -> Result<Supply, reqwest::Error> {
        reqwest::get("https://api.nimiqx.com/supply/")?.json()
    }

    // grab supply data for supply flag
    pub fn get_network_stats_data() -> Result<NetStat, reqwest::Error> {
        reqwest::get("https://api.nimiqx.com/network-stats/")?.json()
    }

    // grab transaction data for transaction option
    pub fn get_transaction_data(hash: &str) -> Result<Transaction, reqwest::Error> {
        // using unwrap() here because I know the string is always valid
        // and I'm relying on the response from the api to tell me if the
        // passed hash is invalid.
        let base = Url::parse("https://api.nimiqx.com/transaction/").unwrap();
        let url = base.join(hash).unwrap();
        reqwest::get(url)?.json()
    }

    // grab blockinfo data for blockinfo option
    pub fn get_blockinfo_data(identifier: &str) -> Result<BlockInfo, reqwest::Error> {
        // using unwrap() here because I know the string is always valid
        // and I'm relying on the response from the api to tell me if the
        // passed block is invalid.
        let base = Url::parse("https://api.nimiqx.com/block/").unwrap();
        let url = base.join(identifier).unwrap();
        reqwest::get(url)?.json()
    }

    // grab addressbook data for addressbook option
    pub fn get_addressbook_data(address: &str) -> Result<AddressBook, reqwest::Error> {
        // using unwrap() here because I know the string is always valid
        // and I'm relying on the response from the api to tell me if the
        // passed block is invalid.
        let base = Url::parse("https://api.nimiqx.com/address-book/").unwrap();
        let url = base.join(address).unwrap();
        reqwest::get(url)?.json()
    }

}

// TODO: write tests
#[cfg(test)]
mod tests {

}