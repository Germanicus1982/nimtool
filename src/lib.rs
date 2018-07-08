#![feature(extern_prelude)]
//#![allow(dead_code, unused_imports)]

extern crate reqwest;
//#[macro_use]
//extern crate serde_derive;
//extern crate serde_json;
//extern crate serde_yaml;

pub mod price;
pub mod lang;
pub mod error;

pub mod app {
    use super::price::*;
    use super::lang::*;
    use std::error::Error;
    use serde_yaml;

    // TODO: Handle all other endpoints and return proper errors
    pub fn get_price_data() -> Result<Price, Error> {
        let req = reqwest::get("https://api.nimiqx.com/price/")?.json();
    }

    /*pub fn get_translations() -> Result<(), ErrorHandler> {
        let f = File::open("../lang/strings.yaml")?;
        let mut buf_reader = BufReader::new(f);
        let mut contents = String::new();
        let mut result = buf_reader.read_to_string(&mut contents)?;
        let trans: Lang = serde_yaml::from_str(&result)?.unwrap()?;
        Ok(())
    }*/
}

// TODO: write tests
#[cfg(test)]
mod tests {
    use super::*;


}