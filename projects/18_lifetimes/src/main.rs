fn main() {
    test_longest();
    test_lifetime_within_struct();    
    test_longest2();
}

// Lifetimes can be explicitly annotated and are used to help rust prevent dangling references.
// The Rust compiler has a 'borrow checker' what compares scopes to determine if all borrows are
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

// Lifetime annotations in function signatures...

// Here we are telling Rust that for some lifetime 'a, the function takes two parameters, both of
// which are string slices that live at least as long as lifetime 'a.  The function signature also
// tells Rust that the string slice returned from function will live as long as lifetime 'a.
// Rust will now enforce these constraints.  
// We are NOT changing the lifetime parameters, instead we are telling Rust to reject any values 
// that don't adhere to these constraints.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// In this example we dont need to annote the lifetime for y because the lifetime of y does not have
// any relationship with the lifetime of x or the return value.
#[allow(dead_code)]
#[allow(unused_variables)]
fn another_example<'a>(x: &'a str, y: &str) -> &'a str {
    x
} 

fn test_longest() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is : \"{}\"", result);
    }
}

// Lifetime annotations within structures...
struct ImportantExcerpt<'a> {
    part: &'a str // string ref.  An instance of this struct cannot exceed the reference it holds.
}

fn test_lifetime_within_struct() {
    let novel = String::from("My name is Martyn.  Some years ago...");
    let first_sentance = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentance };
    println!("except: {}", i.part);
}

// Lifetime Elision
// Rust uses three rules built into the compiler it will use if lifetime annotations are not 
// provided.  If all three fail and lifetimes cannot be inferred there will be a compiler error.

// 1).  Each function parameter that is a reference gets is OWN lifetime parameter 'a, 'b, etc..  
//      ONE per reference.
// 2).  If there is exactly ONE input lifetime parameter, that lifetime will be assigned to all
//      output lifetime parameters.
// 3).  If there are multiple input lifetime parameters but one of them is &self or &mut self then
//      the lifetime of self is assigned to all output lifetime parameters.

#[allow(dead_code)]
#[allow(unused_variables)]
impl<'a> ImportantExcerpt<'a> {
     // Lifetime annotation needed, see rule 1.
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

// Static livetime (use sparingly)
#[allow(dead_code)]
fn example_static() -> &'static str {
    let s: &'static str = "This will last for the programs durarion";
    return s;
}

// Using generic type parameters, trait bounds and lifetimes together.
use std::fmt::Display;
fn longest2<'a, T>(x: &'a str, y: &'a str, ann: T) -> & 'a str 
    where T: Display 
{
    println!("Announcing: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test_longest2() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest2(string1.as_str(), string2.as_str(), "zxczxc");
    println!("The longest string is : \"{}\"", result);    
}
