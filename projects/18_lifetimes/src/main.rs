fn main() {
    test_longest();
}

// Lifetimes can be explicitly annotated and are used to help rust prevent dangling references.
// The Rust compiler has a 'borrow' checker what compares scopes to determine if all borrows are
// valid.

// This function that takes string references and returns a string reference won't compile.  Rust 
// doesnt know enough about the lifetimes and cannot determine whether the returned reference will
// refer to x or y.  In fact, neither can we... it could be either.

/* fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */

// To address this we need to annotate the lifetimes to associate them.
// We give a variable in a method (class method) or function (free function) a label (traditionally)
// starting wih 'a
// &i32         // a reference
// &'a i32      // a reference with an explicit lifetime that we've labelled a
// &'b mut i32  // a mutable reference with an explicit lifetime that we've labelled b

// Lifetime annotations in function signatures
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// Here we are telling Rust that for some lifetime 'a, the function takes two parameters, both of
// which are string slices that live at least as long as lifetime 'a.  The function signature also
// tells Rust that the string slice returned from function will live as long as lifetime 'a.
// Rust will now enforce these constraints.  
// We are NOT changing the lifetime parameters, instead we are telling Rust to reject any values 
// that don't adhere to these constraints.

fn test_longest() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is : \"{}\"", result);
    }
}
