// Rust doesnt use exceptions but has a strong distinction between recoverable and unrecoverable
// errors.  Unrecoverable errors are bugs like access beyond end of an array.
// Rust uses Result<T, E> for recoverable errors and the panic! macro to stop execution when the 
// program encounters an unrecoverable error.
fn main() {
    //how_to_raise_panic();
    //handling_recoverable_errors_with_match();
    matching_different_errors();
    //using_unwrap();
    //using_expect();
    test_returning_errors_to_caller();
    test_more_terse_returning_errors_to_caller();    
}

// Panics (by default) can provide a backtrace.
// RUST_BACKTRACE=1 cargo run
// debug symbols must be enabled for maximum output.
#[allow(dead_code)]
fn how_to_raise_panic() {
    panic!("oh no! Something went really wrong!")
    // Note that the panic! macro is also used to mark tests as failures.
}

// The Result type looks like this:
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }
use std::fs::File;

#[allow(dead_code)]
#[allow(unused_variables)]
fn handling_recoverable_errors_with_match() {

    let file_name = "non_existance_file.txt";
    let f = File::open(file_name); // returns a Result<T, E>
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open: \"{}\", {:?}", file_name, error)
        }
    };
}

// Closures make this error handling code more concise.
// Discusssed later.
use std::io::ErrorKind;
#[allow(unused_variables)]
fn matching_different_errors() {
 
    let file_name = "some_file.txt";
    let f = File::open(file_name); // returns a Result<T, E>
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(new_file) => new_file,
                Err(new_error) => panic!("Failed to create: \"{}\", {:?}", file_name, new_error)
            },
            other_error => panic!("Failed to open: \"{}\", {:?}", file_name, other_error)
        }
    };

    println!("Successfully have a file: {}", file_name);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn using_unwrap() {
    // If the Result value is the Ok variant this will return the value inside Ok.
    // Otherwise it will call the panic! macro.
    let f = File::open("hello.txt").unwrap();
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn using_expect() {
    // If the Result value is the Ok variant this will return the value inside Ok.
    // Otherwise it will call the panic! macro with the parameter we supply.
    let f = File::open("hello.txt").expect("Failed to open file!");
}

use std::io;
use std::io::Read;
fn returning_errors_to_caller() -> Result<String, io::Error> {
    // This is longhand, there's a terser way of doing this.
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn test_returning_errors_to_caller() {
    let f = returning_errors_to_caller();
    let s = match f {
        Err(_) => String::from("oh dear!"),
        Ok(value) => value
    };
    println!("Found: {}", s);
}

#[allow(dead_code)]
fn terse_returning_errors_to_caller() -> Result<String, io::Error> {
    // The ? placed after a Result value is defined to work similar to the match expression.
    // If the value is Ok then the value inside the Ok will get returned.
    // If the value is an Err then Err will be returned in the same way as if we had used return.

    let mut f = File::open("hello.txt")?; // Return io::Error on failure or assign file to f. 
    let mut s = String::new();
    
    let _a = f.read_to_string(&mut s) ?;  // return io::Error on failure.
    Ok(s)                        // If no error occurs then we return Ok(s)

    // Note that the ? operator can only be used with functions that return Result<T, E> types.
}

fn more_terse_returning_errors_to_caller() -> Result<String, io::Error> {    
    let mut s = String::new();    

    // We can chain these calls to the ? operator...
    File::open("hello.txt")?.read_to_string(&mut s) ?;  // return io::Error on either failure.
    Ok(s)                                               // If no error occurs then we return Ok(s)
}

fn test_more_terse_returning_errors_to_caller() {
    let f = more_terse_returning_errors_to_caller();
    let s = match f {
        Err(_) => String::from("oh dear!"),
        Ok(value) => value
    };
    println!("Found: {}", s);
}

use std::fs;

#[allow(dead_code)]
fn even_more_terse_returning_errors_to_caller() -> Result<String, io::Error> {        
    fs::read_to_string("hello.txt")
    // We can make this even more terse because std::fs happens to define:    
    // fs::read_to_string("hello.txt") -> Result<String, io::Error> {
    //    ...
    // }
    //    
}
