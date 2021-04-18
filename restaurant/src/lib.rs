mod back_of_house;
mod front_of_house;
// use + Absolute path
// use crate::front_of_house::hosting;
// Re-exporting Names with pub use
// pub use crate::back_of_house::Breakfast;
pub use crate::front_of_house::hosting;

// use + Relative path
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // use
    hosting::add_to_waitlist();
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

pub fn eat_at_restaurant3() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("{:#?}, {:#?}", order1, order2);
}

pub fn serve_order() {}
