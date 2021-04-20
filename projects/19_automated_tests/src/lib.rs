// cargo new automated_tests --lib
// When creating a lib, an initial test is added automatically.

// cargo test
// ^^^ Run all tests

// Some command line arguments go to 'cargo test', some go to the resulting binary.
// First you list arguments that go to cargo test, followed by -- and the arguments you want for
// the test binary.
//
// cargo test --help
//            ^^^ display cargo test options.
// cargo test -- --help
//            ^^^^^ display test binary options.
//
// cargo test -- --test-threads=1
//                ^^^ Don't run tests in parallel
// cargo test -- --show-output
//                ^^^ show ALL output, not just failing tests.
// cargo test oh_no_it_panicked
//            ^^^ run a single test
// cargo test are_we
//            ^^^ wildcard match on tests.
// cargo test -- --ignored
//            ^^^ run tests that are normally ignored.

// Test organisation
// =================
// Tests generally fit into one of two categories in Rust:
// 1).  Unit tests
//      These are small, more focused testing of one module in isolation at a time and we CAN test
//      private interfaces. 
//      These belong in the src directory in each file with the code being tested.  The convention 
//      is to create a module named 'tests' in each file to contain the test function and to 
//      annotate the tests with 'cgf(test)'
// 2).  Integration tests
//      Are entirely external to your library, using only the public interface and potentially 
//      exercising multiple modules per test.
//      We create a 'tests' directory at the top level of our projects diectory, next to 'src'.
//      We can make as manay tests files as needed here.  Note that Cargo will COMPILE EACH FILE IN
//      'tests' as a different crate.
//      If we need satellite helper files for 'tests' that dont want to be in a different crate then 
//      can create a 'tests/commmon' directory and place setup functions in 'tests/common/mod.rs'
//      See integrations_tests.rs

// IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTA
// Note that: If a project is a binary crate that only contains src/main.rs and doesn't have 
// src/lib.rs then we can't create integration tests in the 'tests' directory.  Only library creates 
// expose functions that other crates can use; binary crates are meant to be run on their own.
// For this reason its a good practice to have Rust projects that provide a binary to have a 
// straightforward src/main.rs and call logic that lives in src/lib.rs.  That way we can test the 
// library crate with 'use' and only use a small amount of code in src/main.rs

// '#[cfg(test)]' is used to configure for UNIT TESTS (builds into unit test onlu) and are only run 
// with 'cargo test', not when you run 'cargo build'.
// INTEGRATION TESTS go into a different directory so do not need: '#[cfg(test)]'
#[cfg(test)] 
mod tests {

    #[test] // to add tests.
    fn it_works() {
        
        // The assert macros panic on failure and display formatted ouput.
        // For this formatting the need to implement the PartialEq and Debug traits.
        // All primitive types and most standard library type already do implement these traits.
        // For user struct and enums to be used with the assert_* macros we need to implement
        // PartialEq and Debug.

        assert_eq!(2 + 2, 4); // assert equals (uses == internally)
        assert_ne!(3, 4); // assert not equals (uses != internally)
    }
    
    #[test]
    fn oh_no_it_panicked() {
        panic!("oh dear!"); // Tests fail when something panics.
    }

    use super::*; // provide access to Rectangle.

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        // This calls panic! for a boolean of false.
        assert!(larger.can_hold(&smaller));        
    }

    #[test]
    fn providing_custom_failure_messages() {
        assert!(
            23 == 25, 
            "This gets passed to the format! macro: {}", 6
        )
    }

    #[test]
    #[should_panic]
    fn are_we_panicking() {
        test_should_panic();
    }

    #[test]
    #[should_panic(expected = "for demonstrating")] // matches substring from panic or fails test.
    fn are_we_explicit_panicking() {
        test_should_panic();
    }

    #[test]
    // Note that you cannot use #[should_panic] when using Result<T, E> like this.
    fn using_result_generics_in_tests() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())            
        } else {
            Err(String::from("something went wrong!"))
        }
    }

    #[test]
    #[ignore] // Dont run by default, we need: cargo test -- --ignored
    fn marking_expensive_tests() {
        // code that takes an hour to run.    
    }
}

#[derive(Debug)] // For debug formatting so we can use with with assert!()
pub struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[allow(dead_code)]
fn test_should_panic() {
    panic!("just for demonstrating should_panic")
}

pub fn add_stuff(val1: i32, val2: i32) -> i32 {
    val1 + val2
}
