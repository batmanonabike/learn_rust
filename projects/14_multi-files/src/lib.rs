// cargo new --lib multi-files    

mod front_of_house; // pull in the other module from another file (front_of_house.rs)

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
