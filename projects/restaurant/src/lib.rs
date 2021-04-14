mod front_of_house; // loads module from another file

pub use crate::front_of_house::hosting;

// eat_at_restaurant and front_of_house are siblings
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let _order1 = front_of_house::serving::back_of_house::Appetizer::Soup;
    let _order2 = front_of_house::serving::back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use std::collections::*; // Glob operator
use std::fmt;
use std::io::{self, Write}; // Nested paths

use std::fmt::Result;
use std::io::Result as IoResult; // alias
