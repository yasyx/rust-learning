
#![cfg(test)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}

use front_of_house::hosting;
/// This function is used to simulate a restaurant experience.
/// It demonstrates the usage of modules and functions from the `front_of_house` module.
///
/// # Examples
/// ```rust
/// eat_at_restaurant()
/// ```
///
#[cfg(test)]
pub fn eat_at_restaurant(){
   
    hosting::add_to_waitlist();
}