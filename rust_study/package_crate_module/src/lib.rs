mod front_of_house {
    mod hosting {}
}

fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct BreatFast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl BreatFast {
        pub fn summer(toast: &str) -> BreatFast {
            BreatFast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        crate::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::BreatFast::summer("RYE");
    meal.toast = String::from("Wheat");
    println!("{}", meal.toast);
}

use std::collections::HashMap;


use std::fmt;
use std::io::Result as IoResult;

fn f1() -> fmt::Result() {
    Result
}
fn f2() -> IoResult<i32> {
    IoResult
}
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}