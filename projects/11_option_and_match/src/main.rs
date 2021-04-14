// Rust does NOT use null!
// Instead, it uses Option type for describing situations where something could be nothing.
// It's defined in the standard library like:
//
// enum Option<T> {
//     Some(T),
//     None
// }
//
// Similar to std::variant

fn main() {
    using_none();
    why_none_is_better_than_null();
    test_value_in_cents();
    test_value_in_cents2();
    test_patterns_that_bind_to_values();
    test_match_with_option();
    test_match_with_placholder();
    terse_matching_with_if_let();
    using_else_with_if_let();
}

#[allow(unused_variables)]
fn using_none() {
    let number1 = Some(5);
    let string1 = Some("some string");
    let absent_number: Option<i32> = None;
}

#[allow(unused_variables)]
fn why_none_is_better_than_null() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // Option<T> and T are different types so this won't compile.
    // I.e.  We can't write code that operates on null... we need specific types
    // and we are compiler enforced to provide all cases.
    // let sum = x + y;
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    // We can use 'match' control flow operator to compare patterns and execute
    // code based matches.  Its similar to 'switch'
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("quarter!");
            25
        }
    }
}

fn test_value_in_cents() {
    println!("{} cent(s) in a penny", value_in_cents(Coin::Penny));
    println!("{} cent(s) in a nickel", value_in_cents(Coin::Nickel));
    println!("{} cent(s) in a dime", value_in_cents(Coin::Dime));
    println!("{} cent(s) in a quarter", value_in_cents(Coin::Quarter));
}

// Alternative using a member function.
impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => {
                println!("quarter!");
                25
            }
        }
    }
}

fn test_value_in_cents2() {
    println!("{} cent(s) in a penny", Coin::Penny.value_in_cents());
    println!("{} cent(s) in a nickel", Coin::Nickel.value_in_cents());
    println!("{} cent(s) in a dime", Coin::Dime.value_in_cents());
    println!("{} cent(s) in a quarter", Coin::Quarter.value_in_cents());
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    //..
}

#[allow(dead_code)]
enum OtherCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState) // Enum value with associated data.
}

fn test_patterns_that_bind_to_values() {

    println!("{} cent(s) in a dime", patterns_that_bind_to_values(OtherCoin::Dime));

    let alsaka_quarter = OtherCoin::Quarter(UsState::Alaska);
    println!("{} cent(s) in a quarter", patterns_that_bind_to_values(alsaka_quarter));
}

fn patterns_that_bind_to_values(coin: OtherCoin) -> u8 {
    match coin {
        OtherCoin::Penny => 1,
        OtherCoin::Nickel => 5,
        OtherCoin::Dime => 10,
        OtherCoin::Quarter(state) => {
            println!("quarter from: {:?}", state);
            25
        }
    }
}

// Combining match and enums is a frequently used pattern in Rust;
// match to a enum, bind a variable and execute an 'arm' of code.
fn match_with_option(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
} 

#[allow(unused_variables)]
fn test_match_with_option() {
    let twelve: Option<i32> = Some(12);
    let thirteen = match_with_option(twelve);
    let six = match_with_option(Some(5));
    let none = match_with_option(None);
}

// Note that: matches are exhaustive
// We have to handle all cases.
// However we can supply a 'catch-all with the '_' placeholder.  This is a bit like 'default'
fn match_with_placholder(my_value: i8) {
    match my_value {
        1 => println!("one"),
        3 => println!("three"),
        _ => println!("something else") // '_' is like default when used with match
    }
}

fn test_match_with_placholder() {
    match_with_placholder(1);
    match_with_placholder(2);
    match_with_placholder(3);
    match_with_placholder(4);   
}

fn terse_matching_with_if_let() {
    let my_value = Some(6);

    // Match and take action only on Some(6)
    match my_value {
        Some(6) => println!("We have 6"),
        _ => ()
    }

    // An more terse way of expressing the above:
    if let Some(6) = my_value {
        println!("We have 6");
    }
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn using_else_with_if_let() {
    let mut count = 0;
    let coin = OtherCoin::Quarter(UsState::Alaska);

    if let OtherCoin::Quarter(state) = coin {
        println!("Quarter is from: {:?}", state);
    } else {
        count += 1;
    }

    //println!("Count: {}", count);
}
