#![feature(extern_prelude)]
//#![allow(dead_code, unused_imports)]

extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod price;
//pub mod lang;
//pub mod error;

pub mod app {
    use super::price::*;
    //use super::lang::*;
    //use std::error::Error;
    //use serde_yaml;

    // TODO: Handle all other endpoints and return proper errors
    pub fn get_price_data() -> Result<Price, reqwest::Error> {
        reqwest::get("https://api.nimiqx.com/price/")?.json()
    }

}

// TODO: write tests
#[cfg(test)]
mod tests {
    use super::*;


}