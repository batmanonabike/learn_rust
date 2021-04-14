// Rusts module system terminology
// ===============================
// Packages:  A cargo feature that lets you build, test and share crates.
// Crates:    A tree of modules to produce a library or executable.
//            A crate is binary or library.  The 'crate root' is a source file that the Rust 
//            compiler starts from and makes up the root module of your crate.
// Modules:   Used to organise scope and privacy of paths ('use').
// Paths:     A way of naming an item, such as a struct, function or module.
//   
// Packages
// --------
// A package is one or more crates to provide a set of functionality.
// A package contains Cargo.toml that describes how to build the crates.
// A package 'must' contain zero or one library crate (no more).  It can contain as many binary 
// crates as needed but it must contain at least one crate (either library or binary).
// 
// Crates
// ------
// src/main.rs is the crate root of a package by convention for a binary crate.
// src/lib.rs is crate root of a package by convention for a library crate.
// The Cargo tool passes the crate root source file to the rust compiler: 'rustc'.
// If a package has both main.rs and lib.rs then the package contains a library cate AND a binary 
// crate.
// A package can have multiple binary crates by placing files in the src/bin directory.  Each file
// will be a separate binary crate.
// 
// A crate will group related functionality into scope so that functionality is easy to share 
// between multiple projects.
fn main() {
    println!("Hello, world!");
}
