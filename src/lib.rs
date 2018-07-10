#![feature(extern_prelude)]
//#![allow(dead_code, unused_imports)]

extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod price;
pub mod supply;
//pub mod lang;
//pub mod error;

pub mod app {
    use super::price::*;
    use super::supply::*;
    //use super::lang::*;
    //use std::error::Error;
    //use serde_yaml;

    // TODO: Handle all other endpoints and return proper errors
    // grab price data for current flag
    pub fn get_price_data() -> Result<Price, reqwest::Error> {
        reqwest::get("https://api.nimiqx.com/price/")?.json()
    }

    // grab supply data for supply flag
    pub fn get_supply_data() -> Result<Supply, reqwest::Error> {
        reqwest::get("https://api.nimiqx.com/supply/")?.json()
    }
}

// TODO: write tests
#[cfg(test)]
mod tests {

}