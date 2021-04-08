fn main() {

    using_if(3);
    using_if(5);
    using_if(49);
    using_if(20);
    using_if(6);
    using_if_with_let();

    using_loop();
    using_loop_to_return_a_value(); 

    println!("Zero: {}", using_while_loop());

    for_looping();
    for_looping_through_collection();
}

fn using_if(x: i32) {

    if x < 4 {
        println!("x is less than 4");        
    } else if x == 5 {
        println!("x is five");        
    } else if x >= 48 {
        println!("x is above 47");        
    } else {
        if x % 4 == 0 {
            println!("x has a factor of 4");        
        } else {
            println!("x is {}", x);        
        }
    }
}

fn using_if_with_let() {
    let condition = true;

    let number = if condition {
        5 // Do not supply a trailing semi-colon
    } else {
        6 // Do not supply a trailing semi-colon
    }; // The resulting expression must be same type.

    println!("number: {}", number);
}

fn using_loop()
{
    let mut val = 0;
    loop {
        println!("using_loop");
        val += 1;
        if val == 5 { // brackets: (val == 5) causes a compilation error
            break;
        }
    } // Note no trailing semi-colon needed.
}

fn using_loop_to_return_a_value() {
    let mut val = 0;
    let ten = loop {        
        val += 1;
        if val == 5 {
            break val * 2;
        }
    }; // Semi-colon needed, this is a 'let'

    println!("ten: {}", ten);
}

fn using_while_loop() -> i32 {
    let mut result = 4;
    while result != 0 {
        result -= 1;
    }
    result // Return values don't want semi-colon
}

fn for_looping_through_collection() {
    let a = [10, 20, 30, 40];

    for element in a.iter() {
        println!("element: {}", element)
    }
}

fn for_looping() {
    for number in 1..4 { // 1, 2, 3 
        println!("up: {}", number);
    }

    for number in (1..4).rev() { // 3, 2, 1
        println!("down: {}", number);
    }

    let max = 5;
    for number in 1..max + 1 { // 1, 2, 3, 4, 5
        println!("up: {}", number);
    }

    let min = 6;
    for number in (min..11).rev() { //10, 9, 8, 7, 6
        println!("down: {}", number);
    }
}
