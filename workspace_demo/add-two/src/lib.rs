mod order_book;

pub fn new_order_book() -> order_book::OrderBook {
    order_book::OrderBook::new_ob()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works_too() {
        let ob = new_order_book();

        // println!("{:?}", ob);
        assert_eq!(ob.get_sell_buckets(), "sell buckets");
    }
}
