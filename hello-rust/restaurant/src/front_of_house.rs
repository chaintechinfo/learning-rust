pub mod hosting;

mod serving {
    fn take_order() {}

    fn server_order() {}

    fn take_payment() {}

    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(totast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(totast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }

        fn fix_incorrect_order() {
            cook_order();
            super::server_order();
        }

        fn cook_order() {

        }
    }
}