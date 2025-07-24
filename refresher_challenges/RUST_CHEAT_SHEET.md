# Quick Reference: Rust Concepts Cheat Sheet

## 🔄 Ownership & Borrowing
```rust
// Ownership transfer
let s1 = String::from("hello");
let s2 = s1; // s1 is moved, no longer valid

// Borrowing (immutable reference)
let s1 = String::from("hello");
let len = calculate_length(&s1); // s1 still valid

// Mutable borrowing
let mut s = String::from("hello");
change(&mut s);

// Rules:
// 1. Either one mutable reference OR any number of immutable references
// 2. References must always be valid
```

## 🏗️ Structs & Enums
```rust
// Struct with methods
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Enum with pattern matching
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to {}, {}", x, y),
    Message::Write(text) => println!("Text: {}", text),
}
```

## 🎯 Option & Result
```rust
// Option for nullable values
let maybe_number: Option<i32> = Some(42);
match maybe_number {
    Some(num) => println!("Got: {}", num),
    None => println!("No value"),
}

// Result for error handling
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Using ? operator
fn process() -> Result<i32, Box<dyn std::error::Error>> {
    let result = risky_operation()?; // Propagates error if Err
    Ok(result * 2)
}
```

## 🧬 Generics & Traits
```rust
// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Trait definition
trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn summarize_author(&self) -> String {
        format!("(Read more from {}...)", self.summarize())
    }
}

// Trait implementation
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

## 🔄 Iterators & Closures
```rust
// Closure examples
let add_one = |x| x + 1;
let multiply = |x: i32, y: i32| -> i32 { x * y };

// Iterator methods
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .filter(|&x| x > 4)
    .collect();

// Iterator with closures
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

## 🏠 Modules & Crates
```rust
// Module definition
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Using modules
use crate::front_of_house::hosting;
// or
use front_of_house::hosting::add_to_waitlist;

// External crates in Cargo.toml
[dependencies]
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
```

## ⚡ Async Programming
```rust
// Async function
async fn fetch_data() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://api.example.com/data").await?;
    let text = response.text().await?;
    Ok(text)
}

// Running async code
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data().await?;
    println!("Data: {}", data);
    Ok(())
}

// Concurrent operations
let (result1, result2) = tokio::join!(
    fetch_data_from_url1(),
    fetch_data_from_url2()
);
```

## 🧵 Concurrency
```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Shared state with Arc and Mutex
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

## 📁 File I/O & Error Handling
```rust
use std::fs;
use std::io::{self, Read};

// Simple file reading
let contents = fs::read_to_string("filename.txt")?;

// Custom error types
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}
```

## 🔧 Common Patterns
```rust
// Builder pattern
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

impl Config {
    fn new() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

// Newtype pattern
struct UserId(u32);
struct ProductId(u32);

// RAII (Resource Acquisition Is Initialization)
struct FileGuard {
    file: File,
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        // Cleanup happens automatically
        println!("File closed");
    }
}
```

## 🧪 Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        panic!("This test should panic");
    }

    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("Math is broken".to_string())
        }
    }
}
```

## 📦 Cargo Commands
```bash
# Project management
cargo new my_project          # Create new project
cargo new --lib my_lib        # Create new library
cargo init                    # Initialize in existing directory

# Building and running
cargo build                   # Build project
cargo build --release         # Optimized build
cargo run                     # Build and run
cargo check                   # Check without building

# Testing and documentation
cargo test                    # Run tests
cargo test test_name          # Run specific test
cargo doc --open             # Generate and open docs

# Dependencies
cargo add serde              # Add dependency
cargo update                 # Update dependencies
cargo tree                   # Show dependency tree
```

This cheat sheet covers the essential concepts you've learned. Keep it handy while working on the challenges! 🦀
