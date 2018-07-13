#![feature(extern_prelude)]
//#![allow(dead_code, unused_imports)]

extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod price;
pub mod supply;
pub mod netstat;
//pub mod lang;
//pub mod error;

pub mod app {
    use super::price::*;
    use super::supply::*;
    use super::netstat::*;

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

}

// TODO: write tests
#[cfg(test)]
mod tests {

}