// this is where the data will be requested

use reqwest::header::{ValuesMut, USER_AGENT, WARNING};
use reqwest::{Error as reqwError, Result as reqwResult};
use serde::{Deserialize, Serialize};
use serde_json::{Result as sjsonResult, Value as sjsonValue};
use std::collections::BTreeMap;
use std::env;
use std::error::Error as stdError;
use std::io::Error;

type DynError = Box<dyn stdError + Sync + Send + 'static>;

pub enum time_series {
    Daily(String),
    Hourly(String),
}

#[derive(Deserialize, Serialize)]
pub struct ApiResponse {
    #[serde(rename = "Time Series (Daily)")]
    daily: BTreeMap<String, Ohlc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Ohlc {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

pub fn daily_data(ts: &str, symbol: &str) -> Result<Vec<(String, f64)>, DynError> {
    let key: String = env::var("ZOORPKEY")
        .expect("No API key found. Set the key in your OS's environment variables : ");

    let vantage_url =
        format!("https://www.alphavantage.co/query?function={ts}&symbol={symbol}&apikey={key}");

    let response = reqwest::blocking::get(vantage_url)?
        .error_for_status()?
        .text()?;

    let v: ApiResponse = serde_json::from_str(&response)?; // Parse to a serde json
    let data_list = v
        .daily
        .into_iter()
        .map(|(day, ohlc)| {
            let price = ohlc
                .open
                .trim()
                .parse::<f64>()
                .map_err(|e| -> DynError { Box::new(e) })?;
            Ok((day, price))
        })
        .collect::<Result<Vec<_>, DynError>>()?;
    Ok(data_list)
}
