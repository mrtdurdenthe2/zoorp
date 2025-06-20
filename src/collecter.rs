// this is where the data will be requested

use reqwest::header::{ValuesMut, USER_AGENT, WARNING};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::env;

pub enum time_series {
    Daily(String),
    Hourly(String),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OHLCpacket {
    // called it a bag because its bigger than a packet. genius i know
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u64,
}

pub fn daily_data(ts: &str, symbol: &str) -> Result<(), Error> {
    let key: String = env::var("ZOORPKEY").expect("No API key set");
    let vantage_url =
        format!("https://www.alphavantage.co/query?function={ts}&symbol={symbol}&apikey={key}");
    let response = reqwest::blocking::get(vantage_url)?.text()?;

    response;
    Ok(())
}
