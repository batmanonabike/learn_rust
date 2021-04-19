fn main() {
    //test_largest();
    test_generic_method();
    test_mixup();
    test_tweet();
    test_show_user();
    test_notify();
    test_notify2();
}

////////////////////////////////////////////////////////////////////////////////
// Generics
////////////////////////////////////////////////////////////////////////////////

// Using generic type parameters in structs.
#[allow(dead_code)]
struct Point<T> {
     x: T,
     y: T
 }

#[allow(dead_code)]
#[allow(unused_variables)]
fn test_points() {
    let int_point = Point { x: 2, y: 30 }; // Needs to be the same types with Point (above).
    let float_point = Point { x: 1.2, y: 33.3 };
}

#[allow(dead_code)]
struct AnotherPoint<T, U> {
    x: T,
    y: U
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn test_more_points() {
    let mixed_point1 = AnotherPoint { x: 2, y: 30.3 };
    let mixed_point2 = AnotherPoint { x: 'A', y: 30.3 };    
}

// Using generic types in method definitions.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn test_generic_method() {
    let point1 = Point { x: 2, y: 30 };
    println!("point1.x = {}", point1.x());

    let point2 = Point { x: 1.2, y: 33.3 };
    println!("point2.x = {}", point2.x());    
}

// Specific implementations (a bit like template specialisation in c++)
// This only applies to a struct with a particular concrete type for the generic 
// type parameter T.
#[allow(dead_code)]
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// A method that uses different generic types than its struct definition
impl<T, U> AnotherPoint<T, U> {
    fn mixup<V, W>(self, other: AnotherPoint<V, W>) -> AnotherPoint<T, W> {
        AnotherPoint {
            x: self.x,
            y: other.y
        }
    }
}

fn test_mixup() {
    let p1 = AnotherPoint { x: 5, y: 10.3 };
    let p2 = AnotherPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Generic functions...
//
// This wont work... we need to restrict T to types that can be compared.
// For this example, we need to enable comparisons via the std::cmp::PartialOrd 
// trait.
//
/* fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn test_largest() {

    let num_list = vec![34, 54, 12, 34];
    println!("largest1: {}", largest(&num_list));

    let char_list = vec!['y', 'a', 'b', 'm'];
    println!("largest2: {}", largest(&char_list));
}
 */
 // For this ^^^ we need TRAITS...

////////////////////////////////////////////////////////////////////////////////
// Traits - similar to interfaces in other languages
////////////////////////////////////////////////////////////////////////////////
// We use these for different types to share the same behaviour and allow us to 
// call the same method on those types.

// Define a trait which enforces method signatures.
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username : String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn test_tweet() {
    let tweet = Tweet {
        username: String::from("ebooks"),
        content: String::from("blah blah blah"),
        reply: false,
        retweet: false
    };

    println!("tweet: {}", tweet.summarize());
}

// Note that we can only implement a trait on a type if either the trait or type
// is local to our crate.
// E.g. We can implement a trait from std on our own types but we can't 
// implement a trait from std on say, std:Vec<T> for example.

// Default implementations.
pub trait ShowUser {
    fn summarize_user(&self) -> String {
        String::from("Fred Bloggs")
    }
}

// Use the default implemnation for a type with an empty impl block.
impl ShowUser for NewsArticle {    
}

fn test_show_user() {
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author")
    };

    println!("user: {}", article.summarize_user());   
}

// Default trait implementations can call other methods in the same trait.
pub trait AnotherSummary {    
    fn summarize_author(&self) -> String;

    // We dont need to implement summarize on implementing types.
    fn summarize(&self) -> String {
        format!("blah blah: {}", self.summarize_author())
    }
}

// Defining functions that type traits.
pub fn notify(item: impl Summary) {
    println!("breaking news! {}", item.summarize());
}

fn test_notify() {

    let tweet = Tweet {
        username: String::from("ebooks"),
        content: String::from("blah blah blah"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author")
    };

    notify(tweet);
    notify(article);
}

// More complex trait bound syntax.
// Now we are binding a generic type that must implement a specific trait.
pub fn notify2<T: Summary>(item: T) {
    println!("breaking news! {}", item.summarize());    
}

pub fn notify3<T: Summary>(item1: T, item2: T) {    
    println!("breaking news! 1:{}", item1.summarize());    
    println!("breaking news! 2:{}", item2.summarize());    
}

fn test_notify2() {

    let tweet1 = Tweet {
        username: String::from("Bilbo"),
        content: String::from("blah blah blah"),
        reply: false,
        retweet: false
    };

    let tweet2 = Tweet {
        username: String::from("Sam"),
        content: String::from("blah blah blah"),
        reply: false,
        retweet: false
    };

    let tweet3 = Tweet {
        username: String::from("Smaud"),
        content: String::from("blah blah blah"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("Frodo")
    };

    notify2(tweet1);
    notify2(article);

    notify3(tweet2, tweet3);
    //notify3(tweet, article);
}

// Specifying multiple trait bounds with the + syntax.
use std::fmt::Display;

#[allow(unused_variables)]
pub fn notify4(item: impl Summary + Display) {    
}

#[allow(unused_variables)]
pub fn notify5<T: Summary + Display>(item: T) {    
}

// Alternative syntax for multiple trait bounds.
use std::fmt::Debug;

#[allow(dead_code)]
#[allow(unused_variables)]
fn some_overly_verbose_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    23
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn terse_function_with_multiple_trait_bounds<T, U>(t: T, u: T) -> i32 
    where   T: Display + Clone,
            U: Clone + Debug 
{
    23
}

// Returning types that implement traits.
// This is useful in the contet of closures and iterators.
#[allow(dead_code)]
fn returns_summarisable() -> impl Summary {
    Tweet {
        username: String::from("Smaud"),
        content: String::from("blah blah blah"),
        reply: false,
        retweet: false
    }    
}

// Note that we can NOT return different concrete types.
// This wont compile...
/* fn returns_summarisable2(switch: bool) -> impl Summary {
    if switch {
        Tweet {
            username: String::from("Smaud"),
            content: String::from("blah blah blah"),
            reply: false,
            retweet: false
        }    
    } else {
        NewsArticle {
            headline: String::from("headline"),
            location: String::from("location"),
            author: String::from("Frodo")
        }
    }
} */
// This is due to restrictions on how the impl Trait syntax is implemented in the compiler.
// This can be overcome (more later).
