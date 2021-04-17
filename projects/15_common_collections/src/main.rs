fn main() {
    using_vectors();
    using_vectors_of_enums_with_values();

    string_basics();
    string_catenation();
    string_format_macro();
    string_lengths_are_in_bytes();
    strings_slices();
    iterating_over_strings_as_scalars();
    iterating_over_strings_as_bytes();
    string_grapheme_cluster();

    hash_map_basics();
    hash_map_ownership();
}

fn using_vectors() {
    let v1 = vec![1, 2, 3];

    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    let third: &i32 = &v1[2]; // causes panic if element does not exist.
    println!("third: {}", third);

    // We handle None so no panic...
    match v2.get(2) {
        Some(another_third) => println!("another_third: {}", another_third),
        None => println!("No third element")
    }

    for i in &mut v2 {
        *i += 10; // Using dereference operator
    }

    for i in &v2 {
        println!("{}", i);
    }
}

enum SpreadsheetCall {
    Int(i32),
    Float(f64),
    Text(String)
}

fn using_vectors_of_enums_with_values() {
    let row = vec![
        SpreadsheetCall::Int(3),
        SpreadsheetCall::Text(String::from("blue")),
        SpreadsheetCall::Float(10.12),
    ];

    for i in &row {
        match i {
            SpreadsheetCall::Int(val) => println!("Int:{}", val),
            SpreadsheetCall::Text(val) => println!("Text:{}", val),
            SpreadsheetCall::Float(val) => println!("Float:{}", val)
        }
    }
}

// Strings are by default stored as UTF8 in the core language.  
// They CANNOT be indexed due to multi-byte to character implications and Rust 
// honours safety first.
// So standard strings are UTF8 but there are also OsString/OsStr, CString/CStr 
// which are different types (owned/borrowed).
fn string_basics() {
    let string_literal = "initial contents";

    let s1 = string_literal.to_string();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");

    let bar = String::from("bar");
    let mut s4 = String::from("foo");
    s4.push_str(&bar); // can take a string 'slice' so as not to take ownership.
    println!("bar: {}", bar);
    s4.push('!');

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);
}

fn string_catenation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");

    // CAREFUL! 
    let s3 = s1 + &s2; // Note this takes ownership of s1!
    // This actually call something like:
    // fn add(self, s: &str) -> String {}

    // println!("s1: {}", s1); <-- This wont compile.
    println!("s2: {}", s2);
    println!("s3: {}", s3);
}

fn string_format_macro() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // The format macro does not take ownership
    let tic_tac_toe = format!("{} {} {}", s1, s2, s3);
    println!("tic_tac_toe: {}", tic_tac_toe);

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
}

#[allow(unused_variables)]
fn string_lengths_are_in_bytes() {
    let len1 = String::from("Hello").len(); // in bytes
    println!("len1: {}", len1); // 5

    let len2 = String::from("Здравствуйте").len(); // in bytes
    println!("len2: {}", len2); // 25 NOT 12

    // Strings are UTF8 by default.  So not necessarily a one byte to letter mapping.
    // This is why Rust prohibits access by index.
    let hello = "Здравствуйте";
    //let val = &hello[0]; // We cannot access by index!
}

// Strings are stored internally as Vec<u8>, a growable generic vector of unsigned bytes.
// They can also be treat as:
// 1).  SCALARS - Unicode scalars, which is what Rust's char type is.
//      These are not necessarily a one to one mapping for letters, e.g. accents/diacritics
//      which dont make any sense on their own (they augment another character).
//      If a letter had a diactric then the letter and the diacritic would add TWO scalars.
// 2).  GRAPHEME CLUSTERS -  these are what we would expect for letters. 

fn strings_slices() {
    
    // string slices are possible but risky.  We may cause a PANIC if the slice is not on a char
    // boundary.
    let hello = "Здравствуйте";

    let s1 = &hello[0..4];
    println!("s1: {}", s1); // "Зд"

    // This would cause a runtime PANIC.  byte zero does not constitute a full letter and is not
    // on a char boundary.
    //let s2 = &hello[0..1];
}

fn iterating_over_strings_as_scalars() {
    // Diacritics (accents) will add additional entries.
    for c in "नमस्ते".chars() { // 'Hello' in Hindi is: "नमस्ते" but there are 6 scalars.
        println!("{}", c);  
    }
    // न
    // म
    // ...
}

fn iterating_over_strings_as_bytes() {
    for c in "नमस्ते".bytes() { 
        println!("{}", c);
    }
    // 224
    // 164
    // ...
}

fn string_grapheme_cluster() {
    // TODO
}

// HashMaps (like std::map in C++).
// Note that the default hash is cryptographically secure so is not the best performance.  
// Though this can be changed.

// HashMaps are not commonly used so need to be brought in.
use std::collections::HashMap;

fn hash_map_basics() {

    let mut scores = HashMap::new();

    // Inserts.  These will overwrite existing entries.
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    scores.insert(String::from("yellow"), 60); // overwrites

    // For the common use case for update or insert...
    // Returns mutable reference IF exits OR inserts with the specified value.
    scores.entry(String::from("yellow")).or_insert(40); // 60
    let red = scores.entry(String::from("red")).or_insert(20); // 20
    *red += 5; // We have a mutable reference and so need to dereference (*).

    // Iterating through the map.
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // We can also build from a pair of vectors using collect and zip...
    let teams = vec![String::from("Red Team"), String::from("Green Team")];
    let initial_scores = vec![10, 50];
    let more_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // We have to give the explicit type but we can use underscores so that Rust can infer the type.

    for (key, value) in &more_scores {
        println!("{}: {}", key, value);
    }

    // Searching through a map.
    let find_red = String::from("Red Team");
    let score_red = more_scores.get(&find_red); // Can be None so we use match.
    match score_red {
        Some(i) => println!("Found red: {}", i),
        _ =>()
    }

    let score_green = more_scores.get(&String::from("Green Team"));
    if let Some(i) = score_green {
        println!("Found green: {}", i);
    }
}

fn hash_map_ownership() {
    // For types that implement the Copy trait, such as i32, the values are copied into the hash 
    // map.  For owned values such as String, the values will be moved and the hashmap becomes the 
    // new owner.

    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // The map takes ownership.
    // field_name and file_value can no longer be used.

    // Wont compile.
    // println!("field_name: {}", field_name);
    // println!("field_value: {}", field_value);
}
