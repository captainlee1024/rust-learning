#![allow(unused)]
fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
}

pub fn cook_order() {}

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}
