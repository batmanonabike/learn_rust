// cargo doc
// ^^^ runs the 'rustdoc' tool and puts the generated HTML docs in the target/doc directory.  

// cargo doc --open
// ^^^ run 'rustdoc, then opens a browswer.

// We can publish our own packages to crates.io.
// Its useful to document for this...

/// Adds one to the number given.
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
