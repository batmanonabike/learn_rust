// cargo test --test integration_test
// ^^^ run tests
// 
// Also see comments in src/lib.rs

use automated_tests;

mod common;

#[test]
//#[cfg(test)] Supposedly not needed for integration tests in the 'tests' directory.
fn test_public_interface() {
    common::setup();
    assert_eq!(automated_tests::add_stuff(2, 2), 4);
}
