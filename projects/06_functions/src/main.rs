fn main() {
    println!("Hello, world!");
    simple_func();
    expression_by_scope();
    passby_value(123, 45333);    
    println!("five: {}", return_values_can_ommit_last_semicolon());
    println!("size: {}", plus_one(5));
}

fn simple_func() {
    println!("simple_func");
}

fn passby_value(x: i32, y: u64) {
    println!("passby_value: {}, {}", x, y);
}

fn return_values_can_ommit_last_semicolon() -> i32 { // Need to specify return type.
    5 // Do not supply a trailing semi-colon
}

fn plus_one(x: i32) -> i32 {
    x + 1 // Do not supply a trailing semi-colon
}

fn expression_by_scope() {

    let thirty_one = {
        let y = 30;
        y + 1 // Do not supply a trailing semi-colon, this is now a expression.
    };

    println!("thirty_one: {}", thirty_one);
}

