// Ownership rules:
// 1).  Each value in Rust has a variable called its 'owner'
// 2).  There can only be one owner at a time.
// 3).  When the owner goes out of scope, the value will be dropped.
//
// Rust NEVER makes a deep copy by default, though it can MOVE.
// Rust does not allow dangling references, compilation errors will occur.
//
// Reference rules:
// 1).  At any given time you can have EITHER:
// 2).  References must always be valid.
//      a).  ONE mutable reference OR
//      b).  ANY NUMBER of immmutable references.
fn main() {
    string_type();
    test_return_tuples();
    string_moving_ownership();
    string_cloning_ownership();
    test_borrow_const_string();
    test_borrow_mutable_string();
    functions_args_and_ownership();
    mutable_reference_restrictions();
    any_number_of_immutable_references();
    more_on_mutable_reference_restrictions();
}

fn string_type() {
    let mut s = String::from("hello"); // String can be mutated.  'String literals' cannot.
    s.push_str(", world");
    println!("{}", s);    
    // s leaves scope and is dropped.  Like C++ RAII.
}

#[allow(unused_variables)]
fn string_moving_ownership() {

    // Here n1 and n2 are stack only, copies are made.
    // There is NO difference between a deep copy or a shallow copy for i64, therefore a COPY is 
    // made.  When size is known at compile time, copies are quick to make so we dont need to clone.
    let n1: i64 = 5; 
    let n2: i64 = n1;
    println!("n1: {}, n2: {}", n1, n2);
    // Rust has a 'copy' trait.  If a variable has the copy trait then an older variable is still 
    // usable after assignment - a COPY is made.
    // Also, we cannot annotate with the copy trait ifa type or any of its parts has implemented the
    // 'drop' trait.  More on traits later.

    // Copyable types (as a general rule):
    // 1).  Any group of simple scalar values can be copyable and nothing that requires allocation 
    //      or is some form of resource is copyable.
    // 2).  All integer types are copyable.
    // 3).  The character type 'char' is copyable.
    // 4).  All floating point types (f64, etc) are copyable.
    // 5).  Tuples if they only contain types that are also copyable.
    //      E.g.  (i32, i32)
    //      But NOT: (i32, String)

    // ---------------------------------------------------------------------------------------------
    // IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT 
    // String is a heap type.  MOVES are made by default (implicit).    
    // But Rust only allows ONE owner.
    let s1 = String::from("hello"); 
    let s2 = s1; // This does NOT copy heap data.  It MOVES s1 to s2, s2 now owns the String.    
    // s1 is now INVALIDATED, THIS IS KEY!!!
    // IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT 
    // ---------------------------------------------------------------------------------------------

    //println!("{} world!", s1); // This will not compile.  s1 is INVALID now.
    println!("{} world!", s2);

    // s2 leaves scope, heap memory is 'drop'ped
}

fn string_cloning_ownership() {
    let s1 = String::from("clone me");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
}

fn functions_args_and_ownership() {

    let s1 = String::from("takeme");
    take_ownership(s1); // s1 now invalid.  take_ownership is new owner.
    //println!("s1: {}", s1); // Wont compile, s1 is now invalid.
    
    let s2 = String::from("cloneme");
    take_ownership(s2.clone()); // s2 still valid because we explicitly cloned it. take_ownership
                                // takes ownership of a clone leaving s2's ownership alone.
    println!("s2: {}", s2);

    let x = 5;
    make_copy(x); // x is type copy.
    println!("x: {}", x);

    let ret1 = return_ownership();
    println!("ret1: {}", ret1);

    let s3 = String::from("take_and_return"); 
    let s4 = take_and_return_ownership(s3); // We lose s3 and gain s4.
    println!("s4: {}", s4);

    let s5 = String::from("take_and_return2"); 
    take_and_return_ownership(s5);  // We lose s5, return from take_and_return_ownership ignored but
                                    // should be dropped.

    let y = return_copy(); // Copy type so a copy is taken 
    println!("y: {}", y);
    
    // x leaves scope, nothing special needs to happen because no heap involved for i32 type.
    // s1 leaves scope but it not owned by functions_args_and_ownership, nothing special happens.
    // s2 leaves scope and is 'drop'ped.
    // s3 leaves scope but it not owned by take_and_return_ownership, nothing special happens.
    // s4 leaves scope and is 'drop'ped.
    // ret1 leaves scope and is 'drop'ped.
}

fn make_copy(some_int : i32) { // This is a Copy type so a copy is made.
    println!("some_int: {}", some_int);
    // some_int leaves scope, nothing special needs to happen.
}

fn take_ownership(some_string : String) { // Caller loses ownership here.
    println!("some_string: {}", some_string);
    // some_string about to lose scope and 'drop' id called.
}

fn return_ownership() -> String {
    let some_string = String::from("return ownership");
    some_string // No-semicolon for return value.

    // some_string is returned and MOVED out to the calling function.
}

fn return_copy() -> i64 {
    12345
}

fn take_and_return_ownership(some_string: String) -> String { // Takes ownership from caller.    
    some_string // Return ownwship to caller.
}

fn test_return_tuples() {
    let s1 = String::from("some string");
    let (s2, len) = return_tuples(s1);
    println!("some_string: {}, len: {}", s2, len);
}

fn return_tuples(some_string : String) -> (String, usize) // taking ownership is String.
{
    let len = some_string.len();
    (some_string, len) // return tuple, called owner string part of tuple.
}

fn test_borrow_const_string() {
    let s1 = String::from("borrow me");
    let len = borrow_const_string(&s1); // Permit borrow (like address of in C/C++).
    println!("s1: {}, len: {}", s1, len);
}

fn borrow_const_string(s: &String) -> usize { // This is how we 'borrow' a constant    
    //s.push_str("bad!"); // Wont compile we only borrows a constant.
    s.len()
    // s leaves scope but is a reference, nothing happens.
}

fn test_borrow_mutable_string() {
    let mut s1 = String::from("Borrow mutable: "); // Owner.  Needs to be declared as mutable here.
    borrow_mutable_string(&mut s1); // pass a mutable reference.
    println!("s1: {}", s1);
}

fn borrow_mutable_string(s: &mut String) { // borrow a mutable reference.
    s.push_str("good!");
}

fn mutable_reference_restrictions() {

    // ---------------------------------------------------------------------------------------------
    // IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT 
    // We only take ONE mutable reference to a piece of data in a particular scope.
    // IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT IMPORTANT 
    // This behaviour is to restrict data races... code with data races will not even compile.
    // ---------------------------------------------------------------------------------------------

    let mut some_string = String::from("mut reference1"); // Owner
    let ref1 = &mut some_string; // Take a mutable reference - ONLY ONE ALLOWED!
    //let ref2 = &mut some_string; // Can't borrow `some_string` as mutable more than once.

    println!("ref1: {}", ref1);
    println!("some_string: {}", some_string);
    //println!("ref2: {}", ref2);
}

fn more_on_mutable_reference_restrictions() {

    let mut some_string = String::from("mut references2"); // Owner
        
    // We can use scope to make a new reference but NOT 'simultaneous' references.
    {
        let ref1 = &mut some_string; 
        println!("ref1: {}", ref1);
        // ref1 leaves scope, we we can reference again.
    }

    let ref2 = &mut some_string;     
    println!("ref2: {}", ref2);    
}

fn any_number_of_immutable_references() {

    // a).  ONE mutable reference OR
    // b).  ANY NUMBER of immmutable references.

    let mut s = String::from("references3");    

    {
        // ANY Number of immutable references are allowed.
        let r1 = &s;
        let r2 = &s;
        println!("r1: {}, r2: {}", r1, r2);

        //let r3 = &mut s; 
        // Not possible, cannot take mutable reference while we have any other references.
    }

    {
        // ONE mutable reference.
        let r3 = &mut s;
        println!("r3: {}", r3);
        
        //let r4 = &s;
        // Not possible, ONE mutable reference or any number of immutable references.
    }
}
