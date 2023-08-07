use super::Kline;

pub trait DataFeeder {
    fn fetch_kline(&self, symbol: &str, interval: &str, start_time: u64, end_time: u64) -> Result<Vec<Kline>, String>;
}
