// Slices let you retain a reference to a contiguous sequence of elements in a collection rather 
// than the whole collection.
// Slices let the compile catch scope related errors.  See test_first_word().
fn main() {
    string_slice();
    test_first_word();
    test_first_word_improved();
    string_literals_are_actually_slices();
    more_general_slicing();
}

fn string_slice() {

    let s = String::from("hello world");
    let hello_slice = &s[0..5]; //0 to 4, [start_index..last_index], last_index is + 1
    let world_slice = &s[6..11]; //6 to 10

    let hello_slice2 = &s[..5]; // We can drop the begin index if we want to start from 0
    let world_slice2 = &s[6..]; // We can drop the last index is we want the end.

    let hello_world_slice = &s[..]; // We can slice then entire string.

    println!("hello_slice: {}, world_slice: {}", hello_slice, world_slice);
    println!("hello_slice2: {}, world_slice2: {}", hello_slice2, world_slice2);
    println!("hello_world_slice: {}", hello_world_slice);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn string_literals_are_actually_slices() {
    let s = "hello Martyn"; // s is of type &str... 
    // It's a slice pointing to that specific point in the binary.  This is also why string literals
    // are mutable; &str is an immutable reference.
}

fn test_first_word() {
    let mut s = String::from("Ginger was here");

    {
        let first_word_slice = first_word(&s);    

        //s.clear(); // This won't compile because we have a reference and cannot mutate.    
        // We cannot take a mutable reference when we have an immutable reference.  s.clear() 
        // attempts to take a mutable reference and the compiler catches this error for us.

        // In short:
        // USE SLICES LIBERALLY!!

        println!("Ginger: {}", first_word_slice);
    }

    s.clear();
}

fn first_word(s: &String) -> &str { // &str means 'string slice'

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { // form of enumerate that gives index + elements.
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_improved(s: &str) -> &str {
    // If we use a string reference for the method args rather than a String then the function can
    // work on string slices AND Strings.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn test_first_word_improved() {
    let string_literal = "Jasper the cat";
    let jasper = first_word_improved(&string_literal[..]); // Take entire slice of string literal.
    println!("Jasper: {}", jasper);

    let string1 = String::from("Mac M1 is great");
    println!("Mac: {}", first_word_improved(&string1[..]));

    // Works because string literals are also string slices.
    println!("GPD: {}", first_word_improved("GPD Pocket is good too"));
}

fn more_general_slicing() {

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // i32 values of 2, 3

    for element in slice.iter() {
        println!("Array slice: {}", element);
    }
}