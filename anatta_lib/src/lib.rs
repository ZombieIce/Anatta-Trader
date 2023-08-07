pub mod market_model;

#[cfg(test)]
mod tests {
    use super::*;
    use market_model::exc_data_feeder::binance_f_feeder::BinanceFDataFeeder;
    use market_model::DataFeeder;

    #[test]
    fn test_binance_f_data_feeder() {
        let test_feeder: &dyn DataFeeder = &BinanceFDataFeeder::new();
        let kline = test_feeder.fetch_kline("BTCUSDT", "1m", 0, 0);
        println!("{:?}", kline);
    }
}
