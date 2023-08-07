use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::market_model::{Kline, DataFeeder};

pub struct BinanceFDataFeeder {
    rest_url: String,
    ws_url: String,
}

impl BinanceFDataFeeder {
    pub fn new() -> Self {
        BinanceFDataFeeder {
            rest_url: "https://fapi.binance.com".to_string(),
            ws_url: "wss://fstream.binance.com".to_string(),
        }
    }
}

impl DataFeeder for BinanceFDataFeeder {
    fn fetch_kline(
        &self,
        symbol: &str,
        interval: &str,
        start_time: u64,
        end_time: u64,
    ) -> Result<Vec<Kline>, String> {
        let url = format!(
            "{}/fapi/v1/klines?symbol={}&interval={}&startTime={}&endTime={}&limit=1000",
            self.rest_url, symbol, interval, start_time, end_time
        );
        let resp = reqwest::blocking::get(&url);
        if resp.is_err() {
            return Err("fetch kline network error".to_string());
        }
        let resp = resp.unwrap().text();
        if resp.is_err() {
            return Err("fetch kline text error".to_string());
        }

        let resp = resp.unwrap();
        println!("resp: {}", resp);
        // 解析错误回报为map
        if resp.contains("code") {
            let value: Value = serde_json::from_str(&resp).unwrap();
            let msg: HashMap<String, Value> = serde_json::from_value(value).unwrap();
            return Err(msg["msg"].to_string());
        }

        let klines: Vec<Vec<f64>> = serde_json::from_str(&resp).unwrap();
        let mut result = Vec::new();
        for kline in klines {
            result.push(Kline::new(
                kline[0] as u64,
                kline[1],
                kline[2],
                kline[3],
                kline[4],
                kline[5],
                kline[9],
            ));
        }
        Ok(result)
    }
}
