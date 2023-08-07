#[derive(Debug, Clone)]
pub struct Kline {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub buy_volume: f64,
}

impl Kline {
    pub fn new(timestamp: u64, open: f64, high: f64, low: f64, close: f64, volume: f64, buy_volume: f64) -> Self {
        Kline {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
            buy_volume,
        }
    }
}