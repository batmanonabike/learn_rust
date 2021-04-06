#[allow(dead_code)]
#[allow(unused_variables, unused_mut)]
fn everything_is_const_by_default() {
    //let someconst = 5;
    //someconst = 6; <-- compile error

    let mut nonconst = 5; // mut for mutable.    
    println!("nonconst: {}", nonconst);
    nonconst = 6;    

    println!("nonconst: {}", nonconst);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn variables_can_shadowed() {
    let reuse_me = 5;        
    let reuse_me = 5 * reuse_me;  // new variable, same name.
    
    let spaces = "    ";
    let spaces = spaces.len();    
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn integer_types_and_type_annotation() {
    // Rust IS statically typed but often deduces type from context.
    // To expliclty set the type (aka type annotation):

    // signed primitive integers:
    let my_i8 : i8 = -1;
    let my_i16 : i16 = -2;
    let my_i32 : i32 = -3;
    let my_i64 : i64 = -4;
    let my_i128 : i128 = -6;
    let my_iarch : isize = 0; // i32 or i64 depending on architecture.
    
    // unsigned primitive integers:
    let my_u8 : u8 = 1;
    let my_u16 : u16 = 2;
    let my_u32 : u32 = 3;
    let my_u64 : u64 = 4;
    let my_u128 : u128 = 6;
    let my_uarch : usize = 0; // u32 or u64 depending on architecture.
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn integer_literals() {

    let hex = 0xFF;
    let octal = 0o77;
    let decimal = 98222;
    let readible_decimal = 98_222; // 98222
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 only

    println!("readible_decimal: {}", readible_decimal);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn floating_point_primitives() {

    let fp64 = 2.12; //f64 (default)
    let fp32: f32 = 3.14; //f32

    println!("fp32: {}", fp32);
    println!("fp64: {}", fp64);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn boolean_primitives() {
    let t = true;
    let f: bool = false;
}

#[allow(dead_code)]
#[allow(unused_variables)]
const MY_I64: u64 = 666_001;
//const USE_SCREAMING_SNAKE_FOR_CONSTS: str = "this can never change";

#[allow(dead_code)]
#[allow(unused_variables)]
fn const_specifics() {
    // A type is REQUIRED
    // Constants can also be at top level scope.
    // The standard is to use 'screaming snake style for naming constants.
    const MY_I8: i8 = 124;
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn character_type() {    
    // Rust char type is four bytes and represents a Unicode scalre value so it CAN represent more
    // than just ASCII.

    let c = 'z';
    let cat = '😼';
    println!("cat: {}", cat);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn tuple_type() {
    let tup1: (i32, f64, u8) = (500, 6.4, 1);

    let tup2 = (512, 6.4, "Some string");
    let (x, y, z) = tup2;
    println!("x: {}, y: {}, z: \"{}\"", x, y, z);

    let x2 = tup2.0;
    let y2 = tup2.1;
    let z2 = tup2.2;
    println!("x2: {}, y2: {}, z2: \"{}\"", x2, y2, z2);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn array_type() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // [3, 3, 3, 3, 3]
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", 
        "Jul", "Aug", "Sep", "Oct", "Now,", "Dec"];

    let val1 = a[0];
    let val2 = a[1];
}

fn main() {

    tuple_type();
    character_type();
    const_specifics();
    integer_literals();
    boolean_primitives();
    variables_can_shadowed();
    floating_point_primitives();
    everything_is_const_by_default();
    integer_types_and_type_annotation();    
}
