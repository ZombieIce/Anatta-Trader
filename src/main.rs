use anatta_lib::market_model;

fn main() {
    let k = market_model::Kline::new(1, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
    println!("{:?}", k);
}
