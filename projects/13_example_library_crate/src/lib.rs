// cargo new --lib restaurant

// 'mod' defines a module.  Allowing us to group related definitions.  Very similar to namespaces.
// Modules also define the 'privacy boundary.  Inner details are hidden by default.
// Items in a parent module cannot use private items inside child modules however items in child
// modules can use itesm in their ancestor modules.
mod front_of_house { 

    // HIDING INNER DETAILS IS THE DEFAULT.  We use 'pub' to expose the path to this module.
    // We also need to declare 'public': functions, structs, enums, methods aswell as modules when
    // needed.
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// The Module Tree
// ===============
// The above defines a module named 'crate' at the tthe root of the crates module structure, knowns
// as the 'module tree'.
//
//  crate
//      front_of_house 
//          hosting
//              add_to_waitlist
//              seat_at_table
//          serving
//              take_order
//              serve_order
//              take_payment
//

// Paths
// =====
// Paths are used to refer to an item in the module tree (similar to a file system).
// To call a function we need its path.  Paths can take two forms:
// 1).  absolute paths: starting from crate root by using a crate name or a literal crate.
// 2).  relative paths: from the current path by using 'self', 'super' or an identifer in the 
//      current module.
//
// 'pub' makes this function part of the publically exposed API (like 'public:').
pub fn eat_at_restaurant() { 
    
    // We need the module 'hosting' to be public to call this from here..  
    // HIDING INNER DETAILS IS THE DEFAULT.

    crate::front_of_house::hosting::add_to_waitlist(); // Absolute path.
    front_of_house::hosting::add_to_waitlist(); // Relative path.
}

fn server_order() {
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::server_order(); // Calling a function using a relative path using 'super' 
    }

    fn cook_order() {
    }

    pub struct Breakfast {
        pub toast: String, // fields are private by default.
        seasonal_fruit: String
    }

    pub enum Appetizer { // 'pub' affects all of its variants, Soup and Salad.
        Soup,
        Salad
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches")
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("{} toast", meal.toast);

    // This wont compile, seasonal_fruit is not public.
    //meal.seasonal_fruit = String::from("blueberries");

    // We can access both Soup and Salid, Appetizer is 'pub'.
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Note that a 'module' can only be defined only once, this won't compilre
// mod back_of_house {
// } 

// Pulling paths to modules into scope using 'use'
pub fn eat_at_restaurant3() {
    hosting::add_to_waitlist();
}
use crate::front_of_house::hosting; // Affects ABOVE and BELOW.  This differs from C++.
pub fn eat_at_restaurant4() {
    hosting::add_to_waitlist();
}

// Pulling paths to functions into scope using 'use'.  Not that for functions, pulling in modules
// as above is the usual way its done.
pub fn eat_at_restaurant5() {
    add_to_waitlist();
}
use crate::front_of_house::hosting::add_to_waitlist; // Affects ABOVE and BELOW.  This differs from C++.
pub fn eat_at_restaurant6() {
    add_to_waitlist();
}

// We can also pull in specific structs, enums and other items.
// This is generally the convention for items other than functions (structs, enums and other items).
use std::collections::HashMap;
fn use_map() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// This is how we resolve ambiquous names with 'use'
use std::fmt;
use std::io;
fn function1(result: fmt::Result) {}
fn function2(result: io::Result<()>) {}

// We can redefine names with 'use'
use std::io::Result as IoResult;
fn function3(result: IoResult<()>) {}

// We can also 're-export' what we have imported so that other code has public access.
// pub use crate::front_of_house::hosting;
// This would give external access to call hosting::add_to_waitlist().

// Using External packages.
// We can modify 'Cargo.toml' tu pull in external packages.
// [dependencies]
// rand = "0.5.5"
//
// https://crates.io contains many publically available packages.
// 
// Tne standard library (std) is also an external crate but comes with the compiler so we don't 
// need to pull this in as a dependency.
use rand::Rng; // This is an absolute path 
fn use_external_package() {
    let num = rand::thread_rng().gen_range(1, 101);
}

// We can simplify 'use' when pulling in nested paths.
//   use std::{io, cmp::Ordering};  
// This is equivalent to:
//   use std::io;
//   use std::cmp::Ordering;

// We can use self:
// use std::io::{self, Write};
// This is the same as:
// use std::io;
// use std::io::Write;

// We can also bring in ALL public items into scope using the glob operator, '*':
use std::collections::*; // This can be a bit dangerous.
