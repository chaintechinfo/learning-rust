
#[derive(Debug)]
pub struct OrderBook {
    buy_buckets: String,
    sell_buckets: String,

    ltp: f64,
}

impl OrderBook {
    pub fn new_ob() -> OrderBook {
        OrderBook {
            sell_buckets: String::from("sell buckets"),
            buy_buckets: String::from("buy buckets"),
            ltp: 0.0,
        }
    }

    pub fn get_sell_buckets(&self) -> &String {
        &self.sell_buckets
    }
}
